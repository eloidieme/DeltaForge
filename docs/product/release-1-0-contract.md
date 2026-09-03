# DeltaForge 1.0 product contract

Status: **Frozen**, with recorded amendments
Date: 2026-08-31 (amended 2026-09-01)
Branch: `codex/product-workbench`

This document is the authoritative scope for DeltaForge 1.0. Where it conflicts with
`vision.md`, `experience.md`, or `architecture.md`, this document wins. Those documents
remain correct on principles, learner states, and architecture; this one decides what
ships.

Phase 1 (correctness loop) and Phase 2 (application shell) are complete and recorded in
`phase-1-checkpoint.md` and `phase-2-application-shell.md`. This contract covers the
work between that boundary and a public 1.0.

## Purpose

DeltaForge 1.0 is a **public open-source product**: something another programmer can
install on macOS, Linux, or Windows and use to build a serious systems project locally.
It is not a portfolio artifact and not a personal tool. That decision sets the bar for
distribution, content quality, and validation below.

## The sixteen decisions

| # | Decision | Answer |
|---|---|---|
| 0 | What 1.0 is for | A public open-source product |
| 1 | Vision breadth | Narrow: correctness loop, performance lab, in-browser creation |
| 2 | Catalog breadth | FlashIndex to 1.0 quality; the other three ship as a lighter tier |
| 3 | Languages | Rust only |
| 4 | Browser completeness | The entire learner journey is completable in the browser; the terminal is used only to write code |
| 5 | Project creation | In-browser wizard; `deltaforge init` retained as the automation form |
| 6 | CLI role | Shrink what is taught, keep what works |
| 7 | Performance lab | Moves into the browser for 1.0 |
| 8 | Performance gates | Block progression only once measurable in the browser; warn until then |
| 9 | Prediction and reflection | Offered and skippable, reusing existing `design_prompt.md` content |
| 10 | FlashIndex numbering | Renumber stage IDs to 01-14; accept the state break |
| 11 | Help ladder | Five levels on every FlashIndex stage; three elsewhere, labeled |
| 12 | Pack authoring | Maintainer and AI tooling; documented, not promoted |
| 13 | Visual identity | The current graphite/blue shell is a placeholder; a design pass precedes 1.0 |
| 14 | Distribution | Prebuilt release binaries for three platforms, plus `cargo install` |
| 15 | External learner research | Not performed. Replaced by the validation contract below |
| 16 | AI in the learner experience | Deferred past 1.0; MCP remains authoring-only |

## In scope for 1.0

### 1. Creation loop (new)

- A first-launch welcome and project catalog in the browser.
- A creation flow: choose project, choose an installed language, choose or confirm a
  location, pass an environment preflight, create the repository, open the workbench.
- The preflight surfaces the toolchain checks that `doctor` performs today.
- `deltaforge init` keeps its current behavior for scripting and CI.

A learner must never be told to run a terminal command in order to begin.

### 2. Correctness loop (complete; one content gap)

The loop shipped in Phase 1 stands as built. The remaining gap is content: the
five-level help ladder currently exists only on FlashIndex Stage 1.

### 3. Performance loop (new)

- Benchmarks run from the browser as jobs on the same run lease, event journal, and
  cancellation model as tests. A benchmark started from the CLI and one started from
  the browser are indistinguishable to project state and the event stream.
- Gate status is visible on the step it belongs to, before the learner reaches it.
- A prediction is requested before a stage's first benchmark run, sourced from that
  stage's existing `design_prompt.md`. It is the primary path and it is skippable.
- A reflection is offered once results are available. Also skippable.
- Results compare against prior saved runs for the same project and machine.
- Gates block `Begin next step` only once a learner can measure them without leaving
  the browser. Until that lands, gates annotate and warn.

### 4. Completion and export (minimal)

- A stage snapshot (commit plus tag) is offered at the pass moment from the browser,
  showing the relevant changes first. It is never automatic without explicit opt-in.
- The existing report export becomes reachable from the browser, with its hardcoded
  advisory text replaced by claims that trace to recorded runs, measurements, and
  commits.

### 5. Content

- FlashIndex stage IDs renumbered to a coherent 01-14, with manifest titles, document
  headings, and stage numbers agreeing. This intentionally breaks existing project
  state and stage tags, which frozen decision 11 permits.
- Five help levels on all fourteen FlashIndex stages.
- MiniKV, TinyHTTP, and ByteForgeVM ship at current quality, presented as a lighter
  tier rather than as equals to the flagship.

### 6. Visual identity

The design deliverable required by `visual-direction.md` is produced before further
frontend expansion: a state-flow wireframe, high-fidelity designs for the canonical
states now in scope, light and dark behavior, keyboard and focus behavior, typography
and color rationale, and motion samples. Implementation follows the design, not the
reverse.

### 7. Distribution

Prebuilt binaries for macOS, Linux, and Windows published through GitHub Releases,
alongside `cargo install`. The existing three-platform CI matrix is the gate.

## Explicitly deferred past 1.0

These are conscious cuts, not oversights:

- the final challenge, in every pack;
- the chronicle as a distinct product feature (run history remains as it is today);
- a second implementation language;
- the other three packs at flagship quality;
- the proactive `Want a nudge?` prompt;
- editor preference memory, copy-path, and copy-command conveniences;
- AI coaching of any kind in the learner experience;
- a native application shell;
- promoting pack authoring as a user-facing surface;
- accounts, cloud, synchronization, telemetry, and anything multi-learner.

## Validation contract

DeltaForge 1.0 ships without external learner research.
`phase-1-observation-protocol.md` is retained as an unexecuted design, not as evidence.
The following three practices replace it. Each produces evidence a single author can
actually obtain.

### A. Scripted cold dogfood

A written journey script executed against a fresh project on a clean machine, recording
every hesitation, dead end, and moment requiring source or documentation. Formalizes
the ad hoc dogfood recorded as Phase 1 Slice 14.

Gate: the complete FlashIndex journey, creation through final step, is completed in the
browser without consulting the DeltaForge source, the pack files, or the docs.

### B. Agent-as-learner content sufficiency

An AI agent receives only the content a learner can see in the workbench for one stage:
instructions, why it matters, expected behavior, requirements, example, edge cases,
exclusions, and hints. It receives no `tests.yaml`, no fixtures, and no reference
solution. It must produce an implementation that passes that stage's checks.

This measures the one thing external learners were most needed for: whether a
specification is genuinely complete, or only appears complete to its author.

Gate: every FlashIndex stage is passable from published content alone.

Limitation, stated plainly: this measures specification completeness. It measures
nothing about visual hierarchy, orientation, or how a person feels while stuck.

### C. Expanded failure corpus

The existing corpus runs deliberately-wrong implementations through the real
application and asserts the exact primary diagnosis, priority, kind, headline,
contract, evidence, and absence of leaked temporary paths. It currently covers seven
implementations of one stage.

Gate: every FlashIndex stage has corpus coverage for its most likely wrong turns, and
each case yields the intended primary diagnosis.

## Success measures

These replace the measures in `vision.md`, which assumed observed participants.

| Measure | Target | Evidence |
|---|---|---|
| Activation | A clean machine with a working Rust toolchain reaches the first behavioral run in under five minutes | A |
| Surface completeness | The full journey is completable in the browser; the terminal is used only to write code | A, plus an automated audit of routes and application operations |
| Content sufficiency | Every FlashIndex stage is passable from published content alone | B |
| Diagnosis quality | Every corpus case produces the intended primary diagnosis | C |
| Cross-surface agreement | A CLI-started job and a browser-started job are indistinguishable in state and events | Existing workbench integration suite |
| Recovery | Kill, restart, edit, or cancel at any point leaves a coherent, resumable state with no invented progress | Existing lifecycle suite, extended to benchmark jobs |

The following remain **design intents** rather than measures, because they cannot be
evidenced without participants: that a learner can state the current mission unprompted,
explain a contradiction before revealing a hint, or resume useful work within thirty
seconds.

## Amendment procedure

Changing anything above requires an explicit amendment naming the decision, the reason,
and the consequences for active work. Adding scope back from the deferred list requires
the same.

## Amendments

### A1. Prediction content gets its own file (decision 9)

*2026-09-01.* Decision 9 said prediction prompts reuse the existing `design_prompt.md`
files. Measured against the pack, prompts and benchmarks overlapped on exactly one
stage, and that stage carried no gate: the reuse was empty. Prediction now has its own
`prediction.md`, authored for each of the four benchmark-carrying FlashIndex stages
(01, 03, 06, 12). `design_prompt.md` keeps its separate purpose.

Consequence: `prediction.md` is an optional per-stage pack file. A stage without one
offers no prediction, which is correct for a stage with nothing to measure.

### A2. The performance lab stays a four-stage surface (decision 7)

*2026-09-01.* Four of FlashIndex's fourteen stages carry benchmarks and one carries a
gate. Adding benchmarks to more stages was considered and rejected: a benchmark on a
stage with nothing interesting to measure teaches nothing and costs CI time on every
run. The lab is small on purpose, and the Performance surface says so on a stage that
has no measurement rather than hiding.

### A3. Creation accepts a location (decision 5)

*2026-09-01.* `architecture.md` states that browser requests resolve only opaque
identifiers and never carry a filesystem path. In-browser creation necessarily breaks
that. The exception, its exact refusals, and the reasoning are recorded as an amendment
in `architecture.md` under *Local security boundary*. Creation accepts a parent
directory and a leaf name, never a full path, and both are resolved by one guarded
function.

### A4. Gate enforcement is on (decision 8)

*2026-09-01.* Decision 8 said gates block progression only once a learner can measure
them without leaving the browser, and warn until then. Benchmarks now run from the
browser on the same run lease, journal, and cancellation path as checks, and a gate's
status is visible on the step before the learner reaches it. Enforcement is therefore
active, which is `gates.enforce = true`, the default it already had. A project may still
turn it off in `.deltaforge/config.toml`.

### A5. The default creation workspace is self-creating (decision 5)

*2026-09-02.* A clean account does not yet have DeltaForge's default workspace. Treating
that absence as a refusal made the browser creation flow impossible on exactly the
machine it was designed for. When the learner leaves Location untouched, the browser
now omits `parent_directory` and the guarded creation policy creates its own configured
default. A path the learner types must still exist and pass every A3 boundary check.

Consequence: creation may create one parent directory chosen by DeltaForge, never an
arbitrary missing parent supplied by a browser request. Preflight names that default as
“will be created” before any write occurs.

### A6. Saved progress migrates instead of breaking (decision 10)

*2026-09-03.* Decision 10 renumbered FlashIndex's stage IDs and *accepted the
state break*, and the code generalised that into a rule: `check_schema_version`
refused every older schema and told the learner to run `deltaforge init` again.
That was a defensible trade for a project with no users and is not one for a
project that has shipped — it makes every future schema change a promise to
destroy work someone has done.

A migration ladder now runs on load: one rung per schema version, operating on
the parsed JSON rather than on `ProjectState`, because `ProjectState` is the
current shape and a migration exists precisely to handle a document that is not
that shape yet. A checked-in schema-1 fixture keeps rung one honest.

Consequence: decision 10's renumber stands and its historical break is not
undone — a pre-1.0 FlashIndex project still cannot be carried forward, because
the stage IDs it recorded no longer exist. What changes is the rule going
forward: from schema 2 onward, a bump owes the learner a migration, and shipping
one without a rung is a defect rather than an accepted cost.

### A7. Every finding gets a regression test (validation practice)

*2026-09-03.* The 1.0 ship review found 33 defects, and its own conclusion was
that the root cause was singular: the software had only ever been driven by its
author, on the machine where it was written, along paths already known to work.
Three review passes missed a dead creation flow because all three harnesses
created the directory whose absence was the bug.

The contract now carries the practice that answers it. A fix is not done until
something other than a person re-runs the path it fixed:

- `tests/browser/journey.mjs` drives the real page in a headless browser, from a
  home directory and a workspace that do not exist when it starts.
- `tests/ui/` executes the page's pure decisions under `node --test`.
- `tools/a11y/contrast_check.py` and `tools/perf/idle_cpu.py` assert in CI the
  two numbers this review had to measure by hand.
- `validate-pack --strict` proves every pack survives its own renderer by round
  trip, rather than by a list of constructs someone remembered to ban.

Consequence: CI gained five jobs and roughly twenty minutes of wall time. That
is the price of the answer to the pattern named above, and it is worth paying.

## Status at 1.0

| Contract item | State |
|---|---|
| Creation loop in the browser | Shipped |
| Correctness loop | Shipped in Phase 1 |
| Performance loop in the browser | Shipped |
| Prediction and reflection | Shipped, offered and skippable |
| Gate visibility and enforcement | Shipped |
| Stage snapshot from the browser | Shipped |
| Record export with traced claims | Shipped |
| FlashIndex renumber | Shipped |
| Five help levels on fourteen stages | Shipped |
| Lighter tier presentation | Shipped as `tier: preview` |
| Visual design pass | Shipped |
| Release binaries | Shipped as a tagged workflow |
| Validation A (cold dogfood) | See `cold-dogfood.md` |
| Validation B (content sufficiency) | See `content-sufficiency.md` |
| Validation C (failure corpus) | See `phase1_failure_corpus.rs` |
