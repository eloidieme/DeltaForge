# DeltaForge

DeltaForge is a local CodeCrafters-style learning framework for staged systems projects. It gives learners project packs, instructions, black-box tests, hints, progress tracking, benchmarks, reports, and portfolio summaries while keeping the implementation locally owned.

Bundled V2 packs:

- FlashIndex: local source-code search engine
- MiniKV: persistent key-value store with tombstones and log stats
- TinyHTTP: HTTP parser/static response engine with MIME and range behavior
- ByteForgeVM: stack-based bytecode virtual machine with calls and tracing

## Install

```bash
cargo install --path .
```

## Quickstart

```bash
deltaforge list
deltaforge pack list
deltaforge init flashindex --lang rust
cd flashindex-rust
deltaforge
deltaforge test
deltaforge explain-failure
deltaforge hint
deltaforge status
deltaforge next
deltaforge bench --save
deltaforge bench --compare
deltaforge report --format markdown --output report.md
deltaforge report --format json --output report.json
deltaforge portfolio --output PORTFOLIO.md
deltaforge doctor
```

## Local workbench

Bare `deltaforge` starts or focuses the project's token-protected loopback workbench. It presents the current mission, live checks, durable diagnosis, progressive help, recovery, and capability progression. `DELTAFORGE_NO_BROWSER=1` prints the local URL and leaves browser opening to the user.

## Safety

DeltaForge runs learner commands directly without a shell, copies fixtures to temporary directories for tests and benchmarks, and keeps state under `.deltaforge/`. Pack fixtures are treated as immutable inputs. The workbench binds only to loopback, requires a per-service capability token, and never serves repository files.

## Pack Authoring

See [docs/pack-format.md](docs/pack-format.md), [docs/test-format.md](docs/test-format.md), and [docs/authoring-packs.md](docs/authoring-packs.md).

Internal reference solutions under `tools/reference_solutions/` are used to prove bundled packs are passable. They are not copied into learner projects.

## AI-Assisted Pack Authoring

DeltaForge includes deterministic pack-authoring commands and a stdio MCP server for AI agents:

```bash
deltaforge pack new example --name "Example" --description "Example pack" --dest packs
deltaforge pack add-stage --pack-dir packs/example 02_next --title "Next behavior"
deltaforge-pack-mcp
```

The MCP server returns structured `ok`/`blocked` reports with problems and next actions. It is designed so agents create packs through scaffold, constrained metadata/document/test/fixture/benchmark updates, validation, and reference-solution proof instead of guessing the format or editing arbitrary files.
