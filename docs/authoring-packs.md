# Authoring Packs

Use `deltaforge validate-pack <pack-id>` while developing packs. Validation checks manifests, language templates, stage files, tests, fixtures, benchmarks, duplicate stage ids, and unsafe benchmark fixture paths.

Keep tests black-box and deterministic. Prefer small fixtures that make failures clear.

For bundled packs, add an internal reference solution under `tools/reference_solutions/<pack>_rust/` and an integration test that initializes the pack, copies the reference solution into the learner template, and runs `deltaforge test --all`.

## Authoring Commands

```bash
deltaforge pack new miniredis --name "MiniRedis" --description "RESP-compatible toy server" --dest packs
deltaforge pack add-stage --pack-dir packs/miniredis 02_resp_arrays --title "Parse RESP arrays"
deltaforge --packs-dir packs validate-pack miniredis --strict
deltaforge --packs-dir packs pack doctor miniredis
deltaforge --packs-dir packs pack check-reference miniredis --reference tools/reference_solutions/miniredis_rust/src/main.rs
```

The scaffold intentionally includes placeholders. A pack is not considered complete until validation and reference checks pass.

## MCP Server

DeltaForge also ships a stdio MCP server:

```bash
deltaforge-pack-mcp
```

It exposes pack-authoring tools for AI agents:

- `inspect_packs`
- `read_pack_manifest`
- `read_stage_document`
- `read_stage_tests`
- `read_stage_benchmarks`
- `list_fixture_files`
- `read_fixture_file`
- `create_pack`
- `add_stage`
- `update_pack_metadata`
- `update_stage_metadata`
- `write_stage_document`
- `replace_stage_tests`
- `write_fixture_file`
- `replace_stage_benchmarks`
- `delete_fixture_file`
- `diagnose_pack`
- `validate_pack`
- `check_reference`

The MCP server returns structured reports with:

- `status`
- `pack`
- `path`
- `problems`
- `next_actions`

Agents should treat `status: blocked` as a hard stop. They should not claim a pack is ready until `validate_pack` and `check_reference` return `status: ok`.

Use a grounded authoring loop:

1. Call `inspect_packs`, then `read_pack_manifest` for the selected pack.
2. Read the relevant stage documents, structured tests, structured benchmarks, and fixture listings/content.
3. Modify only the returned fields that need to change. Pass the complete arrays back to `replace_stage_tests` or `replace_stage_benchmarks`; always preserve `performance_gates` during benchmark updates.
4. Read the changed content again, then run `validate_pack` and `check_reference`.

For example, read `tests` with `read_stage_tests`, change one test definition in that array, pass the whole array to `replace_stage_tests`, and read it again to verify the result. Benchmark editing follows the same pattern with both the `benchmarks` and `performance_gates` arrays. Missing optional `design_prompt.md` and `benchmarks.yaml` files return `status: ok` with null content rather than blocking the workflow.

Mutation tools require an explicit `packs_dir`; they will not fall back to bundled packs. Stage documents are limited to instructions, hints, and design prompts. Tests are accepted as structured arrays. `replace_stage_benchmarks` accepts its existing `benchmarks` array plus an optional `performance_gates` array, validates both sections and their cross-references, then atomically replaces the single `benchmarks.yaml` file. Fixture writes are limited to safe relative paths beneath a named stage fixture, reject symbolic-link crossings, and require `overwrite: true` before replacing an existing file. Fixture reads, recursive listing, and deletion traverse from opened directory capabilities with no-follow semantics, so concurrent symlink replacement cannot redirect them outside the fixture. `delete_fixture_file` requires `confirm: true`, deletes only one regular file beneath the named fixture, and never removes the fixture root.

Fixture reads are UTF-8 text only and are limited to 1 MiB per file. The size is checked before UTF-8 decoding. Binary fixtures, devices, sockets, symbolic links, and other special files are not supported by the MCP read tools; manage binary fixture content outside this constrained tool surface and validate the pack before use.

Set `DELTAFORGE_BIN=/path/to/deltaforge` when running the MCP server from a location where the `deltaforge` binary is not installed next to `deltaforge-pack-mcp`.

The server uses stdio transport. MCP hosts such as Codex start and stop the configured binary as a child process; authors do not need to keep it running in a separate terminal or background service.
