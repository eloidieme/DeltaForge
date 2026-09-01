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

Set `gates.enforce = false` only when a machine cannot reliably meet a pack's performance environment. `next` still requires correctness, warns that performance gates were skipped, and does not fabricate a passing gate result. `bench` and `status` continue to measure and report gates.

## Pack discovery

Packs are discovered from these locations, in order:

1. `--packs-dir <path>` (per invocation).
2. `DELTAFORGE_PACKS_DIR` (environment override).
3. The builtin dev-tree packs directory (when running from a source checkout).
4. The bundled packs embedded in the binary.

## Environment variables

- `DELTAFORGE_HOME` overrides the per-user application directory that holds the project registry and private workbench discovery record.
- `DELTAFORGE_BIN` tells the pack MCP server which `deltaforge` executable to invoke.
- `DELTAFORGE_PACKS_DIR` overrides pack discovery after the per-invocation flag.
- `DELTAFORGE_WORKSPACE` changes the browser creation flow's default project parent.
- `DELTAFORGE_NO_BROWSER=1` prints the workbench URL instead of opening it.
- `DELTAFORGE_NO_PAGER=1` disables the terminal pager.

Embedded packs are extracted to a per-user cache directory rather than the shared system temp directory:

- Unix: `$XDG_CACHE_HOME/deltaforge` (or `~/.cache/deltaforge`).
- Windows: `%LOCALAPPDATA%\deltaforge`.

The cache subdirectory is keyed by a content digest of the embedded pack tree, so changed content refreshes automatically and extraction is atomic (extract to a sibling directory, then rename into place). The system temp directory is used only as a last-resort fallback when no per-user cache location is available.
