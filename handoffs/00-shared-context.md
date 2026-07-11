# Shared context — prepend this to every DeltaForge handoff prompt

You are working on **DeltaForge**, at `/Users/eloidieme/Projects/DeltaForge` (Rust 2024 edition, single crate, git repo). It is a local, CodeCrafters-style learning framework for staged systems-programming projects: a learner runs `deltaforge init <pack> --lang rust`, gets a repo they own, reads behavioral specs, implements each stage, runs black-box tests (`deltaforge test`), unlocks stages (`deltaforge next`), records benchmarks (`deltaforge bench --save`), and exports reports/portfolio material. The full product vision is in `Spec.md` at the repo root — read the sections referenced by your task.

Core principles (do not violate):
- Packs specify observable CLI behavior, not implementation details.
- Tests and benchmarks are black-box and deterministic; fixtures are immutable (copied to temp dirs before use).
- Learner commands are never passed through a shell (`src/process.rs::run_command` execs argv directly).
- JSON command modes (`--json`) must emit JSON only on stdout.
- Reference solutions under `tools/reference_solutions/` prove packs are passable but are never copied into learner projects.
- Correctness before optimization; performance pressure appears gradually and failures must be educational, not punitive.

Layout:
- `src/main.rs` → `src/cli.rs` (clap derive) → `src/commands/*.rs` (one module per subcommand, dispatched in `src/commands/mod.rs`).
- `src/pack.rs` — pack discovery/loading/validation; packs embedded via `include_dir!` and discoverable via `--packs-dir`, `DELTAFORGE_PACKS_DIR`, dev fallback.
- `src/runner.rs` — test execution; `src/commands/bench.rs` — benchmark execution + history (`.deltaforge/benchmark_history.json`).
- `src/state.rs` — `.deltaforge/state.json` (schema_version 1, `deny_unknown_fields`; any new field needs `#[serde(default)]` so legacy state files keep parsing — there is a legacy-state test).
- `src/context.rs` — project root discovery, pack pinning, completion-proof verification.
- `src/integrity.rs` — tree digests + `is_safe_relative_path`.
- `src/authoring.rs` + `src/bin/deltaforge-pack-mcp.rs` — constrained pack-authoring API (CLI `pack` subcommands + stdio MCP server, newline-delimited JSON).
- `src/terminal.rs` — learner-facing renderer (colors, minimal markdown, pager).
- `tests/cli_flow.rs` — end-to-end tests spawning the real binaries; `tests/mcp_standard_client.rs` — MCP tests using the official `rmcp` client. Add coverage for your changes here; there is precedent for temp projects, `--packs-dir` overrides, env overrides, and MCP calls.
- Bundled packs in `packs/` (flashindex, minikv, tinyhttp, byteforgevm), each with `project.yaml`, per-stage `instructions.md`/`hints.md`/`tests.yaml`/`fixtures/`.

Conventions: `anyhow` with `.with_context()`, `crate::fs_util::atomic_write` for all file writes, `#[serde(deny_unknown_fields)]` on schemas, transactional directory replacement (prepare → rename → backup, see `authoring.rs::replace_directory`), path safety via `is_safe_relative_path` on anything pack-supplied, avoid new dependencies unless clearly justified.

Quality bar — all must pass locally before you finish (CI runs them on Ubuntu, macOS, Windows):
```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -- validate-pack --strict
```

Process requirements:
- A foundation-repair pass and other feature tasks may have landed before yours. **Start by running `git log --oneline -15` and reading the files you plan to touch** — verify assumptions against the actual code, not this description.
- Commit per coherent step with clear messages.
- Update `docs/` (commands.md, pack-format.md, test-format.md, benchmark-format.md, config.md as relevant) and append to `CHANGELOG.md` for any behavior or schema change.
- Keep Windows in mind for every change.
- When you change a YAML schema, update all three places that know about it: the serde structs, the validation in `src/pack.rs`, and the MCP JSON schemas in `src/bin/deltaforge-pack-mcp.rs`.
- Report results honestly, including anything you skipped.
