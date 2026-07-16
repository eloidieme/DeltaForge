# Phase 1 release audit

Status: **Local release candidate green; hosted Linux and Windows execution pending**

Audit date: 2026-07-16

Candidate branch: `codex/product-workbench`

Candidate base commit: `9c289a30f56cbc761ea8a4a41c1c0cbd6d4b1acb` plus the current
uncommitted Phase 1 worktree

## Scope

This audit closes the Phase 1 engineering checklist defined by
`phase-1-vertical-slice.md` and tracked in `phase-1-checkpoint.md`. It covers:

- removal of assertions tied to the retired generated-page/viewer product;
- release compilation and warning-free linting;
- all unit, CLI, MCP, failure-corpus, and real-service workbench tests;
- strict bundled-pack validation and direct reference-solution proof;
- macOS execution evidence;
- Linux and Windows target-specific compilation and linting;
- hosted CI configuration for native execution on macOS, Linux, and Windows.

## Audit environment

- macOS 26.5.1, build 25F80
- Apple Silicon (`aarch64-apple-darwin`)
- Rust 1.97.0 (`2d8144b78`, 2026-07-07)
- Cargo 1.97.0

## Defects found and resolved

### 1. Stale test-run headings

Three CLI integration assertions still expected the retired `Stage <id>: <title>` test
heading even though the terminal runner now announces `Checking <stage ids>`.

Resolution:

- updated current-stage, multi-stage, and bundled-reference assertions to the new
  terminal contract;
- updated the Stage 1 passing count from eight to the current nine checks;
- reran the affected progression and reference-solution tests, then the complete suite.

### 2. Labeled help headings rejected by strict validation

The workbench correctly parsed headings such as `# Hint 1 — Observation`, but the pack
authoring validator counted only headings containing a bare integer and therefore
reported zero Stage 1 hints.

Resolution:

- taught the validator to accept a numeric level followed by an em-dash or hyphen
  label;
- added a regression assertion for both label styles;
- updated the pack-format documentation to allow at least three progressive levels and
  optional labels;
- confirmed every bundled pack passes strict validation.

### 3. Windows fixture normalization risk

An earlier intentional CRLF fixture exception had replaced the repository-wide LF
checkout rule. On Windows, ordinary text fixtures could consequently be rewritten by
`core.autocrlf`, changing byte-exact MiniKV and TinyHTTP behavior.

Resolution:

- restored `* text=auto eol=lf` for ordinary repository text;
- retained `-text` for the intentional FlashIndex CRLF fixture subtree;
- verified Git attributes select LF for ordinary MiniKV/TinyHTTP fixtures while the
  CRLF fixture remains byte-preserved.

## Local macOS results

| Gate | Command | Result |
|---|---|---|
| Patch integrity | `git diff --check` | Pass |
| Formatting | `cargo fmt --all -- --check` | Pass |
| Release compilation | `cargo check --release --offline` | Pass |
| Release lint | `cargo clippy --release --offline --all-targets -- -D warnings` | Pass |
| Complete Rust suite | `cargo test --offline` | Pass |
| Strict pack validation | `cargo run --offline -- validate-pack --strict` | Pass, 4/4 packs |
| FlashIndex reference proof | `deltaforge pack check-reference flashindex ...` | Pass, all stages |
| MiniKV reference proof | `deltaforge pack check-reference minikv ...` | Pass, all stages |
| TinyHTTP reference proof | `deltaforge pack check-reference tinyhttp ...` | Pass, all stages |
| ByteForgeVM reference proof | `deltaforge pack check-reference byteforgevm ...` | Pass, all stages |

The complete Rust suite executed:

- 68 library tests;
- 51 CLI integration tests;
- 4 official MCP client integration tests;
- 1 seven-case Phase 1 diagnosis-corpus test;
- 9 real-service workbench integration tests;
- binary and documentation test harnesses with no additional tests.

Total executed tests: 133 passed, 0 failed, 0 ignored.

The first sandboxed test attempt could not bind loopback ports. The authoritative rerun
used normal local-service permission and passed; this was an execution-environment
restriction, not a product failure.

## Linux and Windows target audit

Installed Rust standard-library targets:

- `x86_64-unknown-linux-gnu`
- `x86_64-pc-windows-gnu`

| Target | Release all-target check | Release all-target Clippy |
|---|---|---|
| Linux x86-64 GNU | Pass | Pass with warnings denied |
| Windows x86-64 GNU | Pass | Pass with warnings denied |

These checks compile platform-only service launch, browser fallback, process
measurement, cancellation, run-lease, filesystem, and test code. Cross-compilation
cannot execute the resulting Linux or Windows test binaries on the macOS host.

## Hosted CI matrix

`.github/workflows/ci.yml` defines native jobs for:

- `ubuntu-latest`;
- `macos-latest`;
- `windows-latest`.

Each job now runs:

1. `cargo fmt --check`;
2. `cargo check --release`;
3. `cargo clippy --release --all-targets -- -D warnings`;
4. `cargo test`;
5. `cargo run -- validate-pack --strict`.

The complete test command includes bundled reference solutions, the Phase 1 failure
corpus, and the real-service workbench suite.

## Remaining release evidence

The current worktree has not been committed or pushed, so hosted CI cannot yet execute
this exact candidate. Phase 1 remains open until the native `ubuntu-latest` and
`windows-latest` jobs pass for the candidate commit. The hosted macOS job should also be
recorded, although the same worktree already has complete native macOS evidence above.

After the user authorizes a commit and push:

1. commit the complete Phase 1 worktree on `codex/product-workbench`;
2. push the branch;
3. wait for all three matrix jobs;
4. fix and rerun any platform-owned failure;
5. record the candidate commit and CI run in this document;
6. change this audit to `Complete` and mark Phase 1 complete in the checkpoint.
