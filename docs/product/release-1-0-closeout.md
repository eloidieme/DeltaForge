# DeltaForge 1.0 closeout

Date: 2026-09-01
Measured against: [release-1-0-contract.md](release-1-0-contract.md)
Supersedes: [release-1-0-gap.md](release-1-0-gap.md)

What shipped against each contract item, and — more usefully — what the work turned up
that the gap analysis had not predicted.

## Against the contract

| Item | Shipped |
|---|---|
| Job model generalized beyond tests | `JobKind` on the active job and every attempt; benchmark variants on `RunEvent`; state schema 2 |
| Performance loop in the browser | `benchmarks.rs` + `application::run_benchmarks` on the shared lease, journal, and cancellation path; `POST /api/v1/benchmarks` |
| Gate visibility | `WorkbenchState.performance`, including a marker for every measured step in the journey |
| Prediction and reflection | Per-stage notes in state, two routes, a prompt file per benchmark-carrying stage |
| Creation loop in the browser | `creation.rs`, catalog, preflight, and creation routes, with the path boundary documented as an architecture amendment |
| Five help levels on fourteen stages | Done, and pinned by a test |
| Lighter tier | `tier: preview` on three packs, presented differently in the catalog |
| Snapshot from the browser | `snapshot.rs`, preview-then-commit, offered at the pass moment |
| Record export with traced claims | `reporting.rs`, reachable from the browser |
| Visual design pass | `src/ui/`, recorded in [design-1-0.md](design-1-0.md) |
| Release binaries | `.github/workflows/release.yml`, three platforms with checksums |
| Validation A / B / C | [cold-dogfood.md](cold-dogfood.md) / [content-sufficiency.md](content-sufficiency.md) / `tests/phase1_failure_corpus.rs` |

## What the work found

The gap analysis was accurate about the shape of the work. It could not have predicted
these, because each was only visible once something new was pointed at the product.

### The workbench was not showing what the packs said

Stage content reached the browser through two flatteners: the first paragraph of the
background, and the bullet items of everything else. Packs write requirements as prose
with a fenced list about as often as they write them as bullets — so *"What your program
must do"* and *"Done when"* rendered **empty on thirteen of fourteen FlashIndex stages**,
and every Background lost all but its first paragraph.

Nothing had surfaced it because the Phase 1 vertical slice was built on stage 01, which
happens to be the one stage written entirely in bullets. It became visible the moment
`pack content` was written to dump exactly what a learner sees — which was built for a
different purpose entirely.

Stage sections are now parsed into blocks and rendered whole.
`every_shipped_stage_fills_every_panel` pins it for all four packs.

### Exporting the record invalidated the record

Writing the export into the project changed the project digest, which invalidated the
completion proof the export described, which made the next step refuse to unlock. Found
by the browser-journey test on its first full run. DeltaForge's own exports are now
excluded from the digest.

### The specification had six holes, and they were not where anyone would look

Five agents attempted five stages from published content alone. All five passed, and all
five reported guesses. Six gaps came back from more than one attempt: a root that exists
but is not a directory; invalid UTF-8; symbolic links; `--out` combined with `--threads`;
stage 5 disclaiming an ordering guarantee beside two fully sorted examples; and stage 7
being the one place the pack broke its habit of restating what carries forward — which
is exactly where the ambiguity landed.

Three were answered. Three were named as deliberately open, because no fixture exercises
them and answering them would be a promise the checks do not keep. Naming an open
question specifies it; leaving it unsaid does not.

### The content told learners to use a surface the contract had closed

Learner-facing prose instructed the reader to run `deltaforge test`, `deltaforge bench`,
or `deltaforge next` in **thirty-nine places** across the four packs. Every one was a
dead end for a reader in the browser. Decision 4 had been made and the content had not
been swept for it.

### Three refusals nobody had specified

Creation's path policy needed answers to questions the contract did not raise: a project
inside another project's source tree, a hidden directory, and a parent that does not
exist. Each is now refused with a reason, and each has a test.

## Deviations

**The design deliverable was written after implementation.** `visual-direction.md`
requires a design reviewed before production frontend work expands. The interface was
designed and built in one pass and [design-1-0.md](design-1-0.md) records the system as
built. It is a real deviation from the stated process, recorded rather than glossed.

**The cold dogfood was performed by the author.** Its two objective parts — activation
timing and the route audit — hold regardless of who drives. Its subjective part measures
nothing. Stated in the record itself.

**Content sufficiency covers five stages of fourteen.** The contract's gate is all
fourteen. The practice, its scripts, and its five results are recorded; the remaining
nine are a known, repeatable gap.

## Known limits at 1.0

- The performance lab is four stages of fourteen, one of them gated. Accepted as
  amendment A2.
- Only FlashIndex is at flagship content quality. The other three packs keep their
  repeating stage IDs and their three-level help ladders, and the catalog says so.
- The source watcher re-hashes each registered project's full tree every 500 ms rather
  than gating on mtime. Learner projects are small and generated directories are
  excluded, so this is fine today and will not be at ten times the size.
- No release has been published yet, so the install path in the README is untested
  end to end. The workflow is written and the three-platform CI matrix proves the build.
