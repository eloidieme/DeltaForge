# Task 06 — Optimization quests

*(Prepend `handoffs/00-shared-context.md`. Depends on Task 01 (matrix/memory/speedup metrics, versioned history); pairs naturally with Task 02 (gates) — reuse its metric-evaluation code if it has landed. Verify with `git log` before starting.)*

## Objective

Implement Spec §16: optional performance challenges unlocked after a stage's correctness passes. Quests are the "deeper learning" track — never required for progression.

## Design

### Schema
- New optional per-stage file `quests.yaml`:
  ```yaml
  quests:
    - id: improve_parallel_scaling
      title: Improve parallel scaling
      description: Achieve at least 4x speedup from 1 to 8 threads.
      benchmark: index_with_threads          # benchmark in this stage's benchmarks.yaml
      metric: speedup                        # same metric set as performance gates
      min: 4.0                               # exactly one of min/max/target_delta_percent
    - id: reduce_median
      title: Cut indexing time in half
      description: Reduce median runtime by 50% versus your first recorded run.
      benchmark: index_medium_project
      metric: runtime_median_ms
      target_delta_percent: 50               # relative to the FIRST saved history record for this benchmark
      params: { threads: "8" }               # optional matrix-point selector, as in gates
  ```
- Validation (`src/pack.rs` + stage validation flow): quest ids unique per stage, referenced benchmark exists, exactly one bound, valid metric, params keys exist in the benchmark's matrix. `validate_pack` treats a `quests.yaml` referencing a missing `benchmarks.yaml` as a problem.
- Add a `LoadedPack::quests_path` helper mirroring the other stage-file helpers.

### Commands
- `deltaforge quest list [--json]`: all quests across stages with status — `locked` (stage not completed), `available`, `done` (with the recorded value and timestamp). Human output grouped by stage.
- `deltaforge quest check [<id>] [--json]`: for one quest (or all available quests in the current stage when omitted): run the referenced benchmark (reuse the bench execution path — do not duplicate it), evaluate, record, and print pass/not-yet with required vs measured. `target_delta_percent` quests compare against the **first** saved history record for that benchmark+params; if no baseline exists, say so and instruct `deltaforge bench --save` first.
- Completion recorded in `state.json` under a `#[serde(default)]` field, e.g. `completed_quests: BTreeMap<String, QuestRecord>` (quest id → value achieved, timestamp). Once done, stays done (a later regression doesn't revoke it — quests reward reaching the target).

### Surfacing
- `deltaforge status`: after the stage list, a quest summary for completed stages (`○ improve_parallel_scaling`, `✓ reduce_median`), matching the Spec §10.8 example.
- Report (markdown/JSON) and portfolio: completed-quests section with achieved values (coordinate with Task 05's structure if it has landed; extend additively otherwise).

### Authoring
- Add an MCP tool `replace_stage_quests` in `src/bin/deltaforge-pack-mcp.rs` + `src/authoring.rs`, following the exact pattern of `replace_stage_tests`/`replace_stage_benchmarks` (structured JSON array → validated → atomic YAML write, `status: blocked` on validation problems).
- Add one real quest set to a bundled pack that has benchmarks (flashindex stage 09 if Task 03 landed, otherwise the best available benchmarked stage) so content exercises the feature.

## Acceptance criteria
- Integration tests: quest lifecycle (locked → available after stage completion → done after `quest check` passes), `target_delta_percent` against a seeded baseline history, `--json` outputs on both commands, validation failure for a quest referencing a missing benchmark, MCP `replace_stage_quests` round-trip via the rmcp test client.
- Legacy state files parse; projects/packs without quests behave exactly as before.
- Docs: new `docs/commands.md` entries + a quests section in `docs/pack-format.md` (or `benchmark-format.md`). Full quality bar passes.

## Out of scope
Gates (Task 02), boss fights, allocation-count metrics (needs profiling integration — deferred).
