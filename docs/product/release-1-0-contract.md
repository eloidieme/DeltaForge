# DeltaForge 1.0 product contract

Status: **Frozen**
Date: 2026-08-31
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
