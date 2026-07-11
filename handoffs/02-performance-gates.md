# Task 02 — Performance gates

*(Prepend `handoffs/00-shared-context.md`. Depends on Task 01 — benchmark matrix, peak memory, speedup, versioned history must already be merged; verify before starting.)*

## Objective

Implement Spec §15: stages may declare performance requirements that are checked against benchmark results and gate stage progression, with educational (not punitive) failure output.

## Design

### Schema
- `benchmarks.yaml` gains an optional top-level `performance_gates` list. Each gate:
  ```yaml
  performance_gates:
    - name: tokenizer throughput
      benchmark: tokenize_medium_project     # must reference a benchmark in this file
      metric: throughput_mb_s                # runtime_median_ms | runtime_p95_ms | throughput_mb_s | peak_memory_mb | speedup
      min: 150                               # exactly one of min/max
      params: { threads: "8" }               # optional matrix-point selector; required when the benchmark has a matrix (except metric: speedup)
      advice:                                # optional, shown on failure
        - excessive string allocation
        - repeated path normalization
  ```
- `metric: speedup` uses the derived speedup from Task 01 and needs no `params`.
- Validation (`src/pack.rs` + MCP schema in `deltaforge-pack-mcp.rs`): gate references an existing benchmark name, valid metric, exactly one bound, params keys exist in that benchmark's matrix.

### Evaluation and recording
- `deltaforge bench` (and `bench --save`) evaluates the current stage's gates after running its benchmarks and prints results. Gate outcomes are recorded in `state.json` under a new `#[serde(default)]` field, e.g. `gate_results: BTreeMap<String, GateRecord>` keyed by stage id, where `GateRecord` stores pass/fail per gate, timestamp, and the learner `project_digest` at evaluation time (mirror the `CompletionProof` pattern in `src/state.rs`/`src/context.rs` so results are invalidated when the code changes).
- `deltaforge next`: if the current stage defines gates, require a recorded passing `GateRecord` whose project digest still matches, in addition to the existing completion proof. Failure message must say exactly what to run.
- Escape hatch (gates must not hard-block learners on weak machines): add `[gates] enforce = true` to `.deltaforge/config.toml` (`src/config.rs`, serde default true). When false, `next` prints a clear warning ("performance gates skipped: gates.enforce = false") instead of blocking. Document in `docs/config.md`.

### Output (educational, per Spec §15)
```
Correctness: passed
Performance: not yet

Gate: tokenizer throughput
  required: throughput_mb_s >= 150
  measured: 91.4

Likely areas to investigate:
  - excessive string allocation
  - repeated path normalization
```
- `deltaforge status` shows gate state for stages that define gates (passed / not yet / not measured).

## Files you will touch
`src/commands/bench.rs`, `src/commands/next.rs`, `src/commands/status.rs`, `src/state.rs`, `src/config.rs`, `src/context.rs`, `src/pack.rs`, `src/bin/deltaforge-pack-mcp.rs`, `docs/benchmark-format.md`, `docs/config.md`, `docs/commands.md`, `tests/cli_flow.rs`.

## Acceptance criteria
- Integration tests with a temp pack: (a) gate fails → `next` blocked with educational output; (b) gate passes → `next` proceeds; (c) code change after passing gate invalidates the record; (d) `gates.enforce = false` warns instead of blocking; (e) validation rejects a gate referencing a missing benchmark.
- Legacy state files (no `gate_results`) still parse; stages without gates behave exactly as before.
- Full quality bar passes.

## Out of scope
Quests (Task 06), pack content that uses gates (Task 03), boss fights.
