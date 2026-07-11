# Task 04 — Content depth pass across all bundled packs

*(Prepend `handoffs/00-shared-context.md`. Works best after Tasks 01–02 (matrix benchmarks, gates) so new benchmark content can use them, but can proceed without — check `git log` and use whatever benchmark schema is current.)*

## Objective

The runtime is far ahead of the content. Many stage instructions are 2–9 lines (e.g. `packs/byteforgevm/stages/03_control_flow/instructions.md` is two sentences), only 1 of 26 stages has a `benchmarks.yaml`, and only 1 has a `design_prompt.md`. Bring every stage of every bundled pack (minikv, tinyhttp, byteforgevm, flashindex stages 01–08) up to the Spec's stage template (§12) and give the bench/design features real content to exercise.

## Requirements

### Instructions (every stage)
Rewrite each `instructions.md` to contain, in order: **Goal** (one paragraph), **Background** (the concept being practiced and why it matters — e.g. tombstones, HTTP keep-alive semantics, stack-machine control flow), **Requirements** (precise observable behavior: exact command shape, exact output format), **Example** (concrete command + expected output block), **Edge cases** (explicit list), **Success criteria** ("all `deltaforge test` cases pass" plus anything else), **Non-goals**. Follow Spec Principle 3: specify *what* must be true, never a step-by-step implementation recipe. `packs/flashindex/stages/02_filter_files/instructions.md` is the closest existing example of the right depth.

### Tests (every stage)
- Every edge case listed in the instructions must have a corresponding test in `tests.yaml`. Add tests (and fixtures) where missing — e.g. malformed input handling, empty fixtures, ordering stability. Keep tests deterministic and black-box; use `stdout_exact` where output is fully specified, `stdout_contains`/`stdout_not_contains` otherwise.
- New fixtures follow existing conventions (small, committed, immutable). Remember packs are embedded in the binary — keep fixtures small.

### Hints (every stage)
Exactly three progressive hints per `hints.md` (`# Hint 1/2/3` format parsed by `src/commands/hint.rs`): conceptual → structural → concrete (may mention a std-library facility, never full code).

### Benchmarks (targeted)
Add `benchmarks.yaml` to stages where measurement teaches something, at minimum: minikv `02_append_log` and `04_compaction`, tinyhttp `01_parse_request` (parse throughput over a large request fixture), byteforgevm `06_trace_mode` or the execution stage, flashindex `03_tokenize` and `05_inverted_index`. Use realistic-but-small fixtures; no gates in this task (gates belong to intentional performance stages).

### Design prompts (targeted)
Add `design_prompt.md` (question list, Spec §17 style) to: minikv `04_compaction`, minikv `05_delete_tombstones`, byteforgevm `05_call_return`, tinyhttp `06_range_requests`, flashindex `06_persist_index`.

### Reference solutions must keep passing
Extend `tools/reference_solutions/{minikv,tinyhttp,byteforgevm,flashindex}_rust` wherever new tests demand behavior they don't yet implement. The CI test running all reference solutions against all stages must stay green.

### Tooling follow-through
- Bump each pack's `version` in `project.yaml`.
- Extend `pack doctor` / `diagnose_pack` (`src/authoring.rs`) heuristics to flag what this task fixes: instructions missing an `Edge cases` or `Non-goals` heading, fewer than 3 hints, stages with < 2 tests. Keep them as `--strict`/doctor findings, not hard validation failures.

## Acceptance criteria
- Every stage instruction file contains all seven template sections; every listed edge case has a test; every stage has exactly 3 hints.
- `cargo run -- validate-pack --strict` clean; all reference-solution CI tests pass; full quality bar passes.
- In your final report, include a per-pack summary table (stage → what changed: instructions rewritten / tests added / benchmark added / design prompt added).

## Out of scope
New stages (Task 03 handles flashindex 09–10), performance gates content, interview content (Task 10), second-language templates (Task 09).
