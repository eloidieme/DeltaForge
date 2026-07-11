# Task 01 — Benchmark engine v2: frozen subtask plan

Decomposition of `handoffs/01-benchmark-engine-v2.md`. Subtasks are ordered by
dependency; each ends with a commit, updated docs/CHANGELOG where behavior
changes, and a green quality bar (`cargo fmt --check`, `cargo clippy
--all-targets -- -D warnings`, `cargo test`, `cargo run -- validate-pack
--strict`).

## S1 — History schema v2: per-point records + legacy conversion

Foundational restructure; every later subtask writes into this shape.

- Replace `BenchmarkRecord.results: BenchmarkResult` with
  `points: Vec<BenchmarkPoint>`; each point carries
  `params: BTreeMap<String, String>`, `success`, `iterations`, `warmup`,
  `runtime_median_ms`, `runtime_p95_ms`, `throughput_mb_s`,
  `peak_memory_mb: Option<f64>` (always `None` until S3), `error`.
  Non-matrix benchmarks produce exactly one point with empty `params`.
- Version `.deltaforge/benchmark_history.json` as
  `{ "schema_version": 2, "runs": [...] }`. On read, detect the legacy
  bare-array format and convert losslessly (empty params, `peak_memory_mb:
  null`); reject unknown newer schema versions with a clear error. Write only
  the v2 format (`read_history` keeps returning `Vec<BenchmarkRecord>`).
- Update consumers: human `bench` output, `bench --json` (now emits `points`),
  `src/commands/report.rs` (add a Params column and peak-memory column to the
  history table), `src/commands/portfolio.rs`.
- Tests: checked-in legacy history fixture under `tests/fixtures/` read
  through `read_history`; unit tests for legacy conversion, v2 round-trip,
  newer-version rejection; update the integration assertion on
  `results.success` → `points[0].success`.
- Docs: `docs/benchmark-format.md` (history file section), `CHANGELOG.md`.

## S2 — Matrix parameters: schema, expansion, validation

- `benchmarks.yaml` entries gain optional `matrix: { name: [scalar, ...] }`.
  Expand the cartesian product deterministically (params in `BTreeMap` key
  order, values in listed order); each point runs independently (own warmup +
  iterations + fixture reset) and records its `params` (values stringified).
- Expand `{<param>}` in command args alongside `{fixture_path}`/`{temp_dir}`
  in `expand_variables`. The record-level `command` stores the pre-expansion
  argv (matrix placeholders intact) since the expanded argv differs per point.
- Validation (three places: serde structs in `src/commands/bench.rs`,
  `src/pack.rs::ValidationBenchmark` + checks, MCP
  `benchmark_definition_schema` in `src/bin/deltaforge-pack-mcp.rs`):
  matrix value lists non-empty, values scalars, parameter names are
  identifiers (`[A-Za-z_][A-Za-z0-9_]*`) and not `fixture_path`/`temp_dir`,
  every `{identifier}` placeholder in the command references a declared
  parameter or a built-in.
- Tests: unit tests for cartesian expansion and placeholder substitution;
  validation-error cases; packs without `matrix` behave exactly as before.
- Docs: `docs/benchmark-format.md` matrix section, `CHANGELOG.md`.

## S3 — Peak memory capture (best-effort, per-OS)

- Add `process::run_command_measured` returning the existing `Output` plus
  `peak_rss_bytes: Option<u64>`, sampled from the existing 10 ms poll loop:
  Linux `/proc/<pid>/status` `VmHWM`; macOS `proc_pid_rusage` raw extern
  (precedent: the `kill` extern); Windows `GetProcessMemoryInfo` →
  `PeakWorkingSetSize` raw extern. `None` on failure — sampling must never
  fail a benchmark. `run_command` itself stays untouched so the test-runner
  path pays nothing.
- `bench` uses the measured variant; a point's `peak_memory_mb` is the max
  across measured iterations, converted to MB. Document that it is
  approximate.
- Tests: integration test asserting `peak_memory_mb` is `Some` for a trivial
  command on Linux/macOS/Windows (skip on other OSes).
- Docs: `docs/benchmark-format.md`, `CHANGELOG.md`.

## S4 — Derived speedup + human table output

- When a benchmark has a `threads` matrix parameter with ≥ 2 distinct values,
  compute speedup = median(min threads) / median(max threads); report as
  `speedup_1_to_8`-style key in JSON (record-level derived metric) and as a
  line after the table in human output. Skip when either median is missing.
- Human output becomes an aligned table per benchmark: one row per point
  (param values, median ms, p95 ms, throughput MB/s, peak mem MB), then the
  speedup line. `--json` still emits JSON only on stdout.
- Tests: unit test for speedup computation (incl. missing-median and
  single-value cases); human-output integration assertion on table + speedup.
- Docs: `docs/commands.md` output example, `CHANGELOG.md`.

## S5 — `bench --compare`

- New `--compare` flag on `deltaforge bench`: after the run completes, compare
  it against the most recent prior saved run in history for the same
  benchmark+params (chosen semantics: compare-after-run; requires existing
  history, warns when none). Print per benchmark+params: median delta (ms and
  %), throughput delta, peak-memory delta, with improved/regressed wording,
  and a note when machine os/arch metadata differs between the two runs.
- Tests: integration test — save a run, run again with `--compare`, assert
  comparison output appears (no wall-clock value assertions); unit test for
  the pairing/delta formatting logic.
- Docs: `docs/commands.md`, `CHANGELOG.md`.

## S6 — End-to-end matrix integration test + final polish

- Integration test with a temp pack (via `--packs-dir`, existing precedent)
  whose benchmark uses a matrix param: assert saved history contains one
  entry per matrix point with correct `params`, and `bench --compare` output
  appears on a second run.
- Verify `packs/flashindex/stages/01_scan_files/benchmarks.yaml` unchanged
  and still valid; sweep docs for consistency; full quality bar on the final
  tree; honest report of anything skipped (e.g. per-OS memory paths not
  testable locally — macOS is the local dev OS; Linux/Windows verified by CI).

## Out of scope (per handoff)

Performance gates, quests, profiling, new benchmark content for packs.
