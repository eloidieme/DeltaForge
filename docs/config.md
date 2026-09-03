# Config

Project config lives at `.deltaforge/config.toml`.

```toml
schema_version = 1

[runner]
timeout_ms = 5000
build_timeout_ms = 120000
keep_temp = false

[bench]
iterations = 7
warmup = 2

[git]
auto_commit = false
auto_tag = true

[integrity]
exclude = []

[gates]
enforce = true
```

`integrity.exclude` adds names to the built-in staleness-digest exclusion list (`target`, `node_modules`, ...). Entries are plain file or directory names matched only at the project root — no paths. A nested `src/target`, for example, remains source-visible. Use the setting when a tool creates a generated root directory or directory symlink that DeltaForge does not already know about.

`bench.iterations` and `bench.warmup` must both be greater than zero.

Run `deltaforge config validate` after manual edits.

DeltaForge writes project progress atomically and preserves the previous
complete document as `.deltaforge/state.json.prev` before every update. If the
current file becomes unreadable, run `deltaforge doctor --repair` inside the
project. Repair validates the backup before restoring it and keeps the damaged
file for diagnosis.

Set `gates.enforce = false` only when a machine cannot reliably meet a pack's performance environment. `next` still requires correctness, warns that performance gates were skipped, and does not fabricate a passing gate result. `bench` and `status` continue to measure and report gates.

## Pack discovery

Packs are discovered from these locations, in order:

1. `--packs-dir <path>` (per invocation).
2. `DELTAFORGE_PACKS_DIR` (environment override).
3. The path `packs/` was found under at build time (`env!("CARGO_MANIFEST_DIR")/packs`, baked into the binary), if that path still exists on the machine running it — normally only true on the machine that built the binary, most often a source checkout.
4. The bundled packs embedded in the binary.

## Environment variables

- `DELTAFORGE_HOME` overrides the platform application-data directory that holds the project registry, panic log, and private workbench discovery record. Without the override, DeltaForge uses `$XDG_DATA_HOME/deltaforge` (or `~/.local/share/deltaforge`) on Linux and other Unix systems, `~/Library/Application Support/DeltaForge` on macOS, and `%LOCALAPPDATA%\DeltaForge` on Windows.
- `DELTAFORGE_BIN` tells the pack MCP server which `deltaforge` executable to invoke.
- `DELTAFORGE_PACKS_DIR` overrides pack discovery after the per-invocation flag.
- `DELTAFORGE_WORKSPACE` changes the browser creation flow's default project parent, and also widens where the browser creation flow is allowed to create a project: a parent directory under `DELTAFORGE_WORKSPACE` is accepted in addition to the learner's home directory.
- `DELTAFORGE_NO_BROWSER=1` prints the workbench URL instead of opening it.
- `DELTAFORGE_NO_PAGER=1` disables the terminal pager.
- `DELTAFORGE_PANIC_PROBE=1` adds an authenticated `POST /api/v1/__panic` route that panics on purpose. It exists so panic recovery can be tested against a real service; a workbench started without it has no such route.
- `VISUAL` / `EDITOR` choose the editor the workbench's *Open editor* button and `deltaforge design --edit` launch. Without either set, DeltaForge falls back to a short list of known graphical editors and refuses with "no supported graphical editor was found" if none are on `PATH`.

Embedded packs are extracted to a per-user cache directory rather than the shared system temp directory:

- Unix: `$XDG_CACHE_HOME/deltaforge` (or `~/.cache/deltaforge`).
- Windows: `%LOCALAPPDATA%\deltaforge`.

## Workbench service

The workbench binds loopback only and exits after thirty minutes with no
connected client and no run in flight; see
[Idle shutdown](commands.md#idle-shutdown). A panic in one request is contained
to that request — it answers 500, the service keeps running, and the panic is
appended to `panic.log` in the DeltaForge home alongside `projects.json`.

The cache subdirectory is keyed by a content digest of the embedded pack tree, so changed content refreshes automatically and extraction is atomic (extract to a sibling directory, then rename into place). The system temp directory is used only as a last-resort fallback when no per-user cache location is available.
