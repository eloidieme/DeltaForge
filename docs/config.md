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

`integrity.exclude` adds names to the built-in integrity-digest exclusion list (`target`, `node_modules`, ...). Entries are plain file or directory names matched at any depth — no paths. Use it when a tool creates a generated directory or a directory symlink that DeltaForge does not already know about.

Run `deltaforge config validate` after manual edits.

Set `gates.enforce = false` only when a machine cannot reliably meet a pack's performance environment. `next` still requires correctness, warns that performance gates were skipped, and does not fabricate a passing gate result. `bench` and `status` continue to measure and report gates.

## Pack discovery

Packs are discovered from these locations, in order:

1. `--packs-dir <path>` (per invocation).
2. `DELTAFORGE_PACKS_DIR` (environment override).
3. The builtin dev-tree packs directory (when running from a source checkout).
4. The bundled packs embedded in the binary.

Embedded packs are extracted to a per-user cache directory rather than the shared system temp directory:

- Unix: `$XDG_CACHE_HOME/deltaforge` (or `~/.cache/deltaforge`).
- Windows: `%LOCALAPPDATA%\deltaforge`.

The cache subdirectory is keyed by a content digest of the embedded pack tree, so changed content refreshes automatically and extraction is atomic (extract to a sibling directory, then rename into place). The system temp directory is used only as a last-resort fallback when no per-user cache location is available.
