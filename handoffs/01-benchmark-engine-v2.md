# Task 01 — Benchmark engine v2: matrix, peak memory, speedup, compare

*(Prepend `handoffs/00-shared-context.md`.)*

## Objective

Upgrade `deltaforge bench` from "time one command" to the measurement engine the Spec describes (§14): parameter matrices (e.g. threads), peak-memory capture, derived speedup, and comparison against history. Every later performance feature (gates, quests, performance stages) builds on this.

## Design

### Matrix parameters
- `benchmarks.yaml` entries gain an optional `matrix` map: parameter name → list of scalar values, e.g. `matrix: { threads: [1, 2, 4, 8] }`. Expand the cartesian product of all parameters; each point is measured independently (own warmup + iterations, fixture reset between runs as today).
- Expand `{<param>}` placeholders in command args the same way `{fixture_path}`/`{temp_dir}` are expanded (`src/commands/bench.rs::expand_variables`).
- Validate: matrix values non-empty, parameter names are identifiers, placeholders in the command reference declared parameters. Sync `src/pack.rs::ValidationBenchmark` and the MCP `benchmark_definition_schema`.

### Peak memory (best-effort, per-OS)
- Add a measured variant of process execution (e.g. `process::run_command_measured`) returning the existing `Output` plus `peak_rss_bytes: Option<u64>`. Use the existing 10 ms poll loop to sample:
  - Linux: read `/proc/<pid>/status` `VmHWM` while polling (it is a high-water mark, so last successful read before exit is correct).
  - macOS: `proc_pid_rusage` via a small `extern "C"` declaration (precedent: the raw `kill` extern in `src/process.rs`).
  - Windows: `GetProcessMemoryInfo` → `PeakWorkingSetSize` (raw extern, no new crate if avoidable).
  - Return `None` when unsupported/failed — never fail a benchmark because memory sampling failed. Document that the value is approximate.
- Only `bench` needs this; don't slow down the test runner path.

### Derived metrics
- When a benchmark has a `threads` matrix parameter, compute speedup = median(min threads) / median(max threads) and report it (`speedup_1_to_N` naming, spec §14 output example).

### History schema
- Current history is a bare JSON array of `BenchmarkRecord` with a single flat `results` object. Restructure so a record carries one entry per matrix point, each with its `params: BTreeMap<String, String>`, `runtime_median_ms`, `runtime_p95_ms`, `throughput_mb_s`, `peak_memory_mb: Option<f64>`.
- Version the history file (e.g. `{ "schema_version": 2, "runs": [...] }`). On read, detect the legacy bare-array format and convert it losslessly (empty params). Write only the new format. Add a test that reads a checked-in legacy history fixture.

### `bench --compare`
- New flag: after running (or with `--compare` alone against saved history without running — pick one, document it; recommended: compare the just-completed run against the most recent prior saved run), print per benchmark+params: median delta (ms and %), throughput delta, peak-memory delta, with clear improved/regressed wording. This is the "182 MB/s → 417 MB/s" moment — make the output legible and honest (note machine metadata differences if os/arch changed).

### Human output
- Aligned table per benchmark (spec §14): one row per matrix point (param values, median, p95, throughput, peak mem), then the speedup line. Keep `--json` emitting only JSON.

## Files you will touch
`src/commands/bench.rs`, `src/process.rs`, `src/pack.rs` (validation), `src/bin/deltaforge-pack-mcp.rs` (schema), `src/cli.rs`, `packs/flashindex/stages/01_scan_files/benchmarks.yaml` (leave as-is unless schema forces change — old files without `matrix` must keep working), `docs/benchmark-format.md`, `docs/commands.md`, `tests/cli_flow.rs`.

## Acceptance criteria
- Unit tests: matrix expansion (cartesian, placeholder substitution), speedup computation, percentile behavior, legacy-history conversion.
- Integration test: a temp pack whose benchmark uses a matrix param; assert the saved history contains one entry per point with correct `params`, and that `bench --compare` output appears on a second run. Don't assert wall-clock values.
- Peak memory: integration test asserts `peak_memory_mb` is `Some` on the current OS for a trivial command (skip assertion on OSes where you couldn't implement it, but implement at least Linux + macOS).
- Existing benchmarks (no matrix) behave exactly as before; full quality bar passes.

## Out of scope
Performance gates, quests, profiling, benchmark content for packs (separate tasks).
