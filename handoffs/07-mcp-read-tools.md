# Task 07 — MCP read/grounding tools for pack authoring

*(Prepend `handoffs/00-shared-context.md`. Independent of other tasks; check `git log` in case the tool surface moved.)*

## Objective

The MCP server (`src/bin/deltaforge-pack-mcp.rs`) is write-only: an AI course designer can mutate packs (`write_stage_document`, `replace_stage_tests`, …) but cannot read anything back except the pack list (`inspect_packs`). It cannot see current instructions, tests, or fixtures without out-of-band filesystem access — exactly what the constrained-tool design is meant to prevent. Add a read surface, plus explicit fixture management.

## New tools

All follow existing conventions: `project` + optional `packs_dir` args, path safety via `is_safe_relative_path` and `reject_symlink_components` (`src/authoring.rs`), structured JSON results, `status: blocked` + `problems` + `next_actions` on failure, accurate `readOnlyHint`/`destructiveHint`/`idempotentHint` annotations.

1. **`read_pack_manifest`** (read-only) — `{project, packs_dir?}` → the parsed manifest (id, name, version, description, topics, languages incl. commands, stages with id/title/path).
2. **`read_stage_document`** (read-only) — `{project, packs_dir?, stage, document}` where `document ∈ {instructions, hints, design_prompt}` (extend the enum if quests/interview docs exist by the time you build this) → `{content, path}`. Missing optional documents (e.g. no design_prompt) return `status: ok` with `content: null`, not an error.
3. **`read_stage_tests`** (read-only) — parses `tests.yaml` and returns the **structured** test list (same shape `replace_stage_tests` accepts), so an agent can round-trip read → modify → replace. Unparseable YAML → `status: blocked` with the parse error.
4. **`read_stage_benchmarks`** (read-only) — same pattern for `benchmarks.yaml`; absent file → `ok` with `benchmarks: null`.
5. **`list_fixture_files`** (read-only) — `{project, packs_dir?, stage, fixture?}`: without `fixture`, list fixture names for the stage; with it, list relative file paths + sizes within that fixture (recursive, sorted).
6. **`read_fixture_file`** (read-only) — `{project, packs_dir?, stage, fixture, path}` → `{content, path}`. UTF-8 only; non-UTF-8 or > 1 MiB → `status: blocked` explaining the limit (mirror `MAX_AUTHORED_TEXT_BYTES`).
7. **`delete_fixture_file`** (destructive) — `{project, packs_dir, stage, fixture, path, confirm}`: requires explicit `packs_dir` (like all mutations — route through `load_explicit_authoring_pack`) and `confirm: true`; refuses to delete anything outside `stages/<stage>/fixtures/<fixture>/`; removes now-empty parent directories up to (not including) the fixture root. Missing file → `blocked`, not silent success.

## Implementation notes

- Put the logic in `src/authoring.rs` (new request structs + functions with unit-testable cores), keep `deltaforge-pack-mcp.rs` as thin dispatch + JSON schemas, matching the existing split.
- Reads should accept discovery-resolved packs (optional `packs_dir`, like `diagnose_pack`); only `delete_fixture_file` requires the explicit `packs_dir` + identity check like other mutations.
- Use `ensure_only_arguments` for every new tool; add each tool to `tools()` with a schema (`additionalProperties: false` on mutating tools).
- Update the server `instructions` string in the `initialize` result: agents should ground with `inspect_packs` → `read_*` before mutating.
- Consider exposing the read tools as CLI too only if trivial (`pack show` already covers the manifest) — not required.

## Acceptance criteria
- `tests/mcp_standard_client.rs` (rmcp client) extended: full round-trip — create a pack, `read_stage_tests`, `replace_stage_tests` with a modification of what was read, read again and assert the change; `read_fixture_file` on the scaffold fixture; `delete_fixture_file` without `confirm` is blocked, with `confirm` deletes; path-escape attempts (`../`, absolute, symlink) are blocked.
- `tools/list` count and schemas assert the new tools exist with correct annotations.
- `docs/authoring-packs.md` documents the read/ground-then-mutate workflow. Full quality bar passes.

## Out of scope
Binary fixture support (base64 content) — note it as a known limitation in the docs; multi-language reference checking (Task 09).
