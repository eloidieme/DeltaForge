# Changelog

## 0.1.0

- V1 local CLI surface for staged project packs.
- FlashIndex pack expanded to eight stages.
- Project discovery, config validation, test runner options, benchmarks, reports, portfolio summaries, design prompts, and git commits.

## V2 working tree

- Added grounded MCP pack-authoring reads for manifests, stage documents, structured tests and benchmarks, and recursively listed UTF-8 fixtures, plus confirmed fixture-file deletion with traversal, symlink, special-file, and 1 MiB protections.
- Added fail-closed, digest-pinned performance gates for benchmarked stages, including authoring validation, MCP support, progression enforcement, status, JSON, and the `gates.enforce` escape hatch.
- Hardened gate reporting and validation so partial runs cannot claim individual passes, display-only gate renames preserve proofs, recorded outcomes are recomputed from measurements, and speedup matrices reject duplicate or nonnumeric thread values.

- Hardened stage completion with full-run eligibility, pinned pack identity, and learner/pack integrity proofs.
- Made pack scaffolding and installation transactional and tightened path/schema validation.
- Added bounded concurrent process capture, process-tree timeouts, isolated benchmark fixtures, and truthful benchmark exit statuses.
- Added MCP protocol negotiation and malformed-message recovery.
- Added legacy state compatibility coverage and strict Ubuntu, macOS, and Windows CI.
- Added an end-to-end stdio integration test using the official Rust MCP SDK.
- Added constrained MCP tools for pack/stage metadata, stage documents, structured tests and benchmarks, and fixture files.

- Added `pack list`, `pack show`, and local `pack install`.
- Added `doctor` and `explain-failure`.
- Added JSON report output.
- Extended tests with stdin, per-test env, and file-content assertions.
- Added MiniKV, TinyHTTP, and ByteForgeVM bundled packs.
- Added learner-facing `overview` output and richer generated project READMEs.
- Deepened MiniKV, TinyHTTP, and ByteForgeVM to 6 stages each.
- Added Rust reference solutions for MiniKV, TinyHTTP, and ByteForgeVM.
- Added integration coverage proving all bundled packs are passable by reference solutions.
- Added deterministic pack authoring commands: `pack new`, `pack add-stage`, `pack doctor`, and `pack check-reference`.
- Added `deltaforge-pack-mcp`, a stdio MCP server for AI-assisted pack creation and validation.

### Foundation repair

- Pinned bundled/embedded packs logically as `"bundled"` instead of by absolute temp path, so version bumps no longer brick existing learner projects; old pins under any embedded-cache location are treated as bundled.
- Added `deltaforge sync-pack` to adopt an updated pack by re-pinning the project's pack version, source, and digest. Supports `--json`. All pin-mismatch errors now point at it.
- Moved the embedded-pack cache to a per-user directory (`$XDG_CACHE_HOME`/`~/.cache/deltaforge` on Unix, `%LOCALAPPDATA%\deltaforge` on Windows), keyed by a content digest of the embedded tree, and made extraction atomic via extract-to-sibling-then-rename.
- Applied digest exclusions at every directory depth and expanded default project-digest exclusions (plus each pack's `ignored_paths`), so generated directories no longer invalidate completion proofs. (Symlink handling is described under "Integrity truthfulness".)
- Added an optional `bench_run` command to language specs and set it for all bundled packs so benchmarks time the built binary rather than `cargo run` startup overhead.
- Made pack discovery resilient to a single malformed pack: `list` warns, `doctor` reports it, and `validate-pack` reports and fails.
- Showed actual program stdout/stderr beneath test failures (truncated; full output with `--verbose`).
- Prevented `hint --level N` from lowering recorded progress, expanded `{fixture_path}`/`{temp_dir}` in test `stdin` and `env` values, defaulted `report`/`portfolio` `--output`, and added `--json` to `status`.

### Integrity truthfulness

- Completion proofs now pin a per-stage behavioral digest — the stage's `tests.yaml`, its fixtures, and the language `build`/`run` commands — instead of the whole-pack digest. Documentation-only pack updates no longer invalidate completed stages; changes to tests, fixtures, or commands invalidate exactly the stages they affect.
- `sync-pack` updates only the project-level pin and never rewrites what a proof claims was proven. It reports each completed stage as valid or needing revalidation, and `next`/`commit` block stale stages with a message pointing at `deltaforge test`. Legacy proofs (no behavioral digest) are upgraded automatically only when the pack is bit-identical to the one that passed.
- `status` marks completed-but-stale stages with `!` (JSON: `needs_revalidation`).
- Split integrity digests into two modes. Pack digests reject symlinks and special files outright (pack behavior must be self-contained). Learner project digests hash file symlinks as link path + target path + target contents — so editing or relinking a symlinked source file is detected — and reject directory symlinks with an actionable error.
- Added `[integrity] exclude = [...]` to `.deltaforge/config.toml`: learner-controlled names appended to the digest exclusion list (matched at any depth, like the built-ins).

### Benchmark engine v2

- Versioned `.deltaforge/benchmark_history.json` as `{"schema_version": 2, "runs": [...]}`; each run now carries per-configuration `points` (with `params` and a reserved `peak_memory_mb`) instead of a single flat `results` object. Legacy bare-array history files convert losslessly on read; newer schema versions are rejected. `bench --json` emits the new `points` shape.
- Added optional benchmark parameter matrices (`matrix: { threads: [1, 2, 4] }` in `benchmarks.yaml`): the cartesian product of all parameters is measured independently, `{name}` placeholders in command args expand per point, and each saved history entry records its `params`. Matrix declarations are validated by `validate-pack` (identifier names, non-empty scalar value lists, no undeclared command placeholders) and by the MCP `replace_stage_benchmarks` schema.
- Added best-effort peak-memory capture to `deltaforge bench`: each point's `peak_memory_mb` reports the approximate peak resident set size (max across measured iterations), sampled per-OS (Linux `VmHWM`, macOS `proc_pid_rusage`, Windows `PeakWorkingSetSize`) and `null` when unavailable — sampling never fails a benchmark. The test-runner path is unchanged.
- Reworked human `bench` output into an aligned per-benchmark table (one row per matrix point: params, median, p95, throughput, peak memory) and added a derived `speedup_<min>_to_<max>` line when a numeric `threads` matrix parameter is the only varying parameter. `bench --json` attaches the speedup as a per-record `derived` object; derived metrics are computed at display time and never written to history.
- Added `bench --compare`: each new benchmark point is compared with its most recent prior saved point using exact benchmark parameters, with median, throughput, and peak-memory deltas, improved/regressed wording, machine OS/architecture warnings, and machine-readable JSON comparison output. Comparisons are computed before saving and are never persisted.
