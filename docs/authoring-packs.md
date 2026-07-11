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
- `create_pack`
- `add_stage`
- `update_pack_metadata`
- `update_stage_metadata`
- `write_stage_document`
- `replace_stage_tests`
- `write_fixture_file`
- `replace_stage_benchmarks`
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

Mutation tools require an explicit `packs_dir`; they will not fall back to bundled packs. Stage documents are limited to instructions, hints, and design prompts. Tests and benchmarks are accepted as structured arrays and validated before atomic replacement. Fixture writes are limited to safe relative paths beneath a named stage fixture, reject symbolic-link crossings, and require `overwrite: true` before replacing an existing file.

Set `DELTAFORGE_BIN=/path/to/deltaforge` when running the MCP server from a location where the `deltaforge` binary is not installed next to `deltaforge-pack-mcp`.

The server uses stdio transport. MCP hosts such as Codex start and stop the configured binary as a child process; authors do not need to keep it running in a separate terminal or background service.
