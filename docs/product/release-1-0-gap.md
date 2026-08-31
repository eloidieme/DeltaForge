# DeltaForge 1.0 implementation gap

Status: **Analysis — not an approved plan**
Date: 2026-08-31
Measured against: [release-1-0-contract.md](release-1-0-contract.md)

This records what exists, what the frozen 1.0 contract requires, and the work between
them. Every claim was verified against the code at `5f8cd2b`.

## Summary

| Contract item | Status | Size |
|---|---|---|
| Correctness loop | Complete | — |
| Application shell, hub, routes | Complete | — |
| Job model generalized beyond tests | Absent | M |
| Performance loop in the browser | Absent | L |
| Gate visibility (currently a dead end) | Absent | S |
| Prediction and reflection | Absent; content nearly absent | M |
| Creation loop in the browser | Absent | L |
| FlashIndex renumber | **Done** | — |
| Five-level help ladder on 14 stages | 1 of 14 | M (content) |
| Stage snapshot from the browser | Absent | S |
| Report export with traced claims | Boilerplate | S |
| Visual design pass | Not started | L |
| Release binaries | Absent | S |
| Validation A / B / C | Absent / Absent / 1 stage of 14 | M |

Roughly: the correctness half of the product is done. The other half is greenfield, and
two foundations (a general job model, a path-accepting API) have to land before it.

## W1. Generalize the job model — foundation, blocks W2

`RunEvent` has eleven variants, all test-shaped: `JobStarted`, `Build*`, `Test*`,
`RunCompleted`, `SourceChanged`, `ProjectStateChanged`, `JobInterrupted`
([application.rs:24](../../src/application.rs:24)). `architecture.md` lists
`BenchmarkSampleRecorded` as representative; it does not exist.

`ActiveJob` and `TestAttempt` ([state.rs:139](../../src/state.rs:139)) carry
`stage_ids`, `passed`, `failed` and no job kind. A benchmark job cannot be represented.

Work:

- add a job kind to `ActiveJob` / `TestAttempt`, and benchmark variants to `RunEvent`;
- bump `state.json` schema. Do this in the same break as the W4 renumber, not separately.

`run_journal` needs no change — it stores `serde_json::Value` and truncates generically.

## W2. Performance loop — the largest item

`commands/bench.rs` is 1,929 lines with **zero** integration with the shared machinery:
no `RunLease`, no `run_journal`, no `EventSink`, no cancellation path. It loads a
context and runs to completion, printing as it goes. It is the last major command that
never went through the Phase 1 application extraction.

Consequence today: the only browser-reachable code path that touches gates is
`begin_next_capability` ([application.rs:506](../../src/application.rs:506)), which
*blocks*. A learner reaching FlashIndex `09_parallel_performance` in the browser gets a
409 reading `Run: deltaforge bench`. Gates can currently only ever appear as a wall.

Work:

- split `bench.rs` the way `test` was split: execution into an application operation
  taking an `EventSink`, holding the run lease, honoring the cancellation path;
  rendering stays in `commands/bench.rs`;
- benchmark events flow through the same journal and SSE stream as tests;
- `POST /api/v1/benchmarks`; cancellation reuses the existing route;
- `WorkbenchState` gains a benchmark/gate summary — it has no gate field at all today
  ([application.rs:137](../../src/application.rs:137)), though
  `context.gate_status()` already computes exactly what is needed and is used only by
  `status --json`;
- prediction and reflection persistence: new state fields plus two routes;
- flip gate enforcement on only once the above lands (contract decision 8).

### Content problem discovered during verification

The contract says prediction prompts reuse existing `design_prompt.md` files. Measured:

| FlashIndex stage | design_prompt | benchmarks | gate |
|---|---|---|---|
| 01_scan_files | yes | yes | — |
| 03_tokenize | — | yes | — |
| 06_canonical_index | — | yes | — |
| 07_persist_index | yes | — | — |
| 11_parallel_indexing | yes | — | — |
| 12_parallel_performance | — | yes | **yes** |

Design prompts and benchmarks overlap on exactly one stage, and that stage has no gate.
The one gated stage has no prediction prompt. **The reuse assumption in decision 9 is
effectively empty and needs a decision** (see Open decisions).

Also note the perf lab is a 4-of-14-stage surface. That is a small footprint for a
whole product loop.

## W3. Creation loop — highest risk

`commands/init.rs` interleaves creation and printing in one `run()`. The creation work
itself (copy template, write state and config, write README, `git init`) is in private
helpers that lift cleanly. `--name` already accepts an absolute path — `check_reference`
relies on that — so the engine can target an arbitrary location today.

Missing for a catalog: `ProjectPack` ([pack.rs:17](../../src/pack.rs:17)) has
`description` and `topics` but **no difficulty, no time estimate, and no toolchain
declaration**. `LanguageSpec` declares `build` / `run` / `bench_run` commands but not
what must be installed, so preflight cannot know that `rust` needs `cargo`; `doctor`
hardcodes cargo and git. These are additive `#[serde(default)]` manifest fields — no
schema bump needed.

**The security item.** `architecture.md` states browser requests resolve only opaque
registry identifiers and *never* provide a filesystem path. Creation necessarily breaks
that invariant: the learner chooses where the project goes. This is the first endpoint
accepting a path from the browser, in a service that then executes pack-defined build
commands inside the result. It needs a deliberate design — a configured parent
directory, canonicalization, traversal rejection, a required non-existent leaf, and an
explicit amendment to the architecture document. Do not let it arrive as a side effect
of a UI wizard.

## W4. Content

**Renumber — done.** FlashIndex stage IDs now run 01–14, and the IDs, manifest titles,
and guide headings agree. The document headings were already correct; the directory IDs
and ten of fourteen manifest titles had drifted, so the headings won and the manifest
was synced to them. `commit.rs:107` derives its message from the ID prefix, so the
duplicate `Complete Stage 05` resolved itself once prefixes became unique. Pack bumped
to 2.0.0. Existing FlashIndex projects break, as decision 10 permits.

The blast radius was smaller than first estimated — four live files
(`project.yaml`, `tests/cli_flow.rs`, `tools/gen_flashindex_bench_fixture.py`,
`docs/curriculum-map.md`) rather than the ~90 references guessed from a coarse grep,
most of which were synthetic test-pack IDs. Changelog and handoff entries were left
alone as historical record.

MiniKV, TinyHTTP, and ByteForgeVM keep their repeating IDs until they are brought to
flagship quality.

**Help ladder.** Stage 01 has five labeled levels; the other thirteen have three
unlabeled ones (`# Hint 1` with no label — `parse_help` falls back to the literal
"Hint"). Work is 13 × (2 new levels + labels on 3 existing), authored to the
Observation / Concept / Experiment / Structure / Retrospective ladder. This is writing,
not engineering, and it is the largest single content task in 1.0.

**Lighter tier.** MiniKV, TinyHTTP, ByteForgeVM need a manifest marker and hub
presentation so they read as a second tier rather than as flagship equals.

## W5. Completion and export

`commit.rs` is CLI-only with no application boundary; the pass moment offers nothing.
Lift to `create_stage_snapshot`, add a route, show changed files before committing,
keep it opt-in.

`report.rs` and `portfolio.rs` emit fixed advisory text — `portfolio.rs:96` literally
writes *"Profile benchmark hot paths before optimizing."* regardless of what happened.
Replace with claims traced to recorded runs, gate results, and commits, and make it
reachable from the browser.

## W6. Design

`workbench.html` is one 460-line file: inline CSS, inline JS, no build step, no
component structure. It is honest scaffolding, not a design. The
`visual-direction.md` deliverable (state-flow wireframe, high-fidelity states, light and
dark, keyboard and focus, typography and color rationale, motion samples) was never
produced.

Sequencing consequence: W2 and W3 each add substantial new surface. Building them
against the placeholder means building the frontend twice. Prefer landing their
application and API layers first, then designing, then implementing both frontends once.

## W7. Distribution

Only `ci.yml` exists. Needs a tagged release workflow producing macOS, Linux, and
Windows binaries with checksums. The three-platform matrix already proves the build;
this is packaging, and it is small.

## W8. Validation

- **A (dogfood script):** does not exist. A document plus one honest execution.
- **B (agent-as-learner):** does not exist and is real infrastructure, not a checkbox —
  extract workbench-visible content into a fixture, sandbox the attempt away from
  `packs/`, decide what a flaky pass means, wire into CI. Scope it as its own slice.
- **C (failure corpus):** `tests/phase1_failure_corpus.rs` is 271 lines covering seven
  wrong implementations of one stage. Extend to fourteen. The existing structure
  generalizes well and this is the best evidence-per-hour item in the plan.

## Suggested sequence

1. W1 job model + W4 renumber — one combined state break.
2. W2 application and API layer; gates visible but still warning-only.
3. W3 application and API layer, including the path-security design and amendment.
4. W6 design pass across the now-known surface.
5. W2 and W3 frontends, built once against the design.
6. W4 help-ladder content, W5, W8-C in parallel throughout.
7. W8-A and W8-B, then W7, then flip gate enforcement.

## Open decisions

1. **Prediction content.** Reuse is empty as written. Author prediction prompts for the
   four benchmark stages, or narrow prediction to the single gated stage, or add
   benchmarks to more stages.
2. **Perf-lab footprint.** Four of fourteen stages carry benchmarks and one carries a
   gate. Accept a small lab, or invest in benchmark coverage first.
3. **Path acceptance.** Confirm the architecture amendment before W3 implementation.

## Smaller findings

- `runner.rs:316–339` has four nested `if false {` blocks left from the `EventSink`
  refactor; `print_actual_output`, `print_actual_stream`, and `truncate_output` are
  reachable only from inside them. About sixty dead lines.
- The source watcher re-hashes every registered project's full tree every 500 ms
  ([workbench.rs:475](../../src/workbench.rs:475)). Generated directories are excluded,
  so a learner project is small and this is currently fine — but it is full-content
  hashing on a timer, not mtime-gated, and it scales with project size times project
  count. Worth revisiting if projects get large.
