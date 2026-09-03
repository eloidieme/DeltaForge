# DeltaForge 1.0 ship-review closeout

Date: 2026-09-03
Worked at: `f3bcb85` → `HEAD` on `ship-1-0`
Answers: `release-1-0-ship-review.md`, 2026-09-02, 33 findings

The review asked whether DeltaForge would ship as an Apple or a Google 1.0 and
answered no, for 33 specific reasons. This records what happened to each one,
what the work found that the review did not predict, and what is still true
about the blind spot the review named at the end.

Every finding is annotated in place in `release-1-0-ship-review.md` with its
resolution and the commit that closed it. None were deleted. This document is
the account; that one is the ledger.

---

## 1. Disposition of all 33 findings

**Fixed: 31. Deferred with a stated reason: 2. Not-a-defect: 0.**

The two deferred are P0-4's final step (cutting the tag, which is a push and
therefore the user's to make) and P3-8 (the cold dogfood, which by construction
cannot be performed by whoever did this work). P3-3, the two `wip` commits, is
closed as will-not-fix. Everything else is closed with a commit against it.

Nothing in the review turned out to be wrong. That is worth stating plainly:
every finding reproduced, including the four that were only reachable by running
the product rather than reading it.

### Blockers

| # | Finding | Disposition |
|---|---|---|
| P0-1 | Browser creation dead on every clean machine | Fixed — `c8a937c` |
| P0-2 | CI never executes the page a learner uses | Fixed — `08099e4` |
| P0-3 | Section renderer corrupts learner content | Fixed — `fe3c2fd`, swept `a7b407e` |
| P0-4 | Distribution never executed, first run irreversible | Fixed except the tag — `b4ebf7c`, `71395ad` |

### Review-gate failures

| # | Finding | Disposition |
|---|---|---|
| P1-1 | Build layout hides the primary action below 1120px | Fixed — `69e19a4` |
| P1-2 | `--text-3` and `--line-strong` fail WCAG AA | Fixed — `69e19a4`, guarded `b1ea8c8` |
| P1-3 | SPA never announces a route change | Fixed — `69e19a4` |
| P1-4 | 7% of a core while the learner reads | Fixed — `47c8059`, guarded `b1ea8c8` |
| P1-5 | One panic permanently bricks the workbench | Fixed — `47c8059`, `69e19a4` |

### Finish and hygiene

P2-1 through P2-15 and P3-1 through P3-6 and P3-9 are all fixed; see the ledger
for the commit against each.

### Content sufficiency reached its gate

P3-7 was five stages of fourteen against a contract gate of fourteen. The
remaining nine were run: all nine passed, eight on first submission. **The gate
is met.**

What they found is in §2 — it includes the single worst content defect this
whole review turned up, and the review did not name it.

### The two that are not closed

**P0-4's last step — cut `v1.0.0-rc.1` and install from the published archive on
macOS, Linux and Windows.** Everything the tag needs is in place and verified as
far as it can be verified without pushing: the crate packages, the dry run
passes, the matrix builds five targets, archives are checksummed and attested,
and the release body carries this version's changelog. The push itself is the
one action in this list that cannot be undone, and it is the user's to take. See
§4 for the exact sequence.

**P3-8 — the cold dogfood by someone who is not the author.** This one cannot be
performed by the agent doing the work, by construction: the whole point is a
reader with no prior knowledge. `cold-dogfood.md` now carries a ready-to-run
protocol — participant and machine preconditions, observer script including the
exact sentence to say and the refusal to say anything else, the nine timestamps
that make up the activation measurement, a per-step record sheet, and pass
criteria. It needs one unfamiliar programmer and about an hour.

**P3-3 — the two `wip` commits — is closed as "will not fix".** They are in
published history; rewriting it would break every clone and every commit hash
cited in `docs/product/`. `CONTRIBUTING.md` states the practice going forward.

---

## 2. What the work found that the review did not predict

This is the section the previous closeout did best, so it gets the same
treatment. In descending order of how much they would have hurt.

**Seven of the eleven were found by pushing.** Everything before the first push
had been verified on one macOS machine and was green there. The first CI run
failed on five of eight jobs, and it took five rounds to get to green — an
untrue MSRV, a security advisory that the untrue MSRV was holding out, two
Windows-only failures, a cross-process registry bug, and a race in the new
harness itself. That is the review's own thesis playing out one level up: the
harnesses were also only ever run by their author, on the machine where they
were written.

### A release candidate would have published the real crate version

The review found that `publish-crate` had no `needs:`, so a failed build could
leave a permanent crates.io version with no binaries. That was true and is
fixed. Underneath it was something worse, which the review did not name and
which its own remediation plan would have triggered.

`cargo publish` publishes the version in `Cargo.toml`, not the version in the
tag. The plan's next step was to cut `v1.0.0-rc.1` and install from it *before*
`v1.0.0` exists. Pushing that tag would have published crate version 1.0.0 to
crates.io — permanently, since publishes can only be yanked — on the way to
finding out whether the release worked at all. The validation step would have
destroyed the thing it was validating.

`publish-crate` now compares the tag against `cargo metadata` and publishes only
on an exact match. A candidate still builds every target, checksums and attests
every archive, and cuts a GitHub Release marked as a pre-release.

The general lesson is the review's own, one level deeper: the first execution of
an untested irreversible path is not a validation, it is the thing itself.

### A stage's requirements contradicted its own checker

The worst content defect found anywhere in this work, and the review did not
see it — because it is invisible unless someone who has not read the code tries
to implement from the text.

FlashIndex stage 9 asks for a benchmark result as JSON. Its Requirements block
shows the object as `{"files": <N>, "runtime_ms": <N>}`, with a space after each
colon, and its prose says *print exactly one valid JSON object*. Its checks
asserted `"files":2` as a substring and pinned the whole object with a
byte-exact regex — neither of which that shape satisfies.

A learner who copied the shape out of the requirements failed two of six checks,
and the failure message said nothing about whitespace. Confirmed by writing
exactly that implementation and running it: two failures before the fix, six
passes after.

Stage 13 is the same defect in a different costume: *reject a query containing
no tokens* was the entire specification, while the check required the string
`non-empty query` in stderr — a phrase no reader could infer. It cost the
exercise its only failing submission. Stage 11 pins its own error string
explicitly, so the inconsistency was the defect rather than the requirement.

The general shape, which is worth more than either instance: **the prose
promises a class of acceptable answers and the checker accepts one member of
it.** That is precisely the defect an author cannot see, because their own
implementation is the member that passes. It is the reason the
content-sufficiency practice exists, and it is the first time the practice has
returned something that would have blocked a learner outright rather than merely
made them guess.

### The renderer was losing two constructs nobody had noticed

P0-3 named two symptoms: deleted inline code and leaked `###` headings. Building
a real parser and then running a round-trip check over all four packs found the
rest of the class:

- **164 authored numbered list items across the four packs were rendering as
  bullets.** Ordered lists are how a stage says *do this, then this*; presenting
  them as an unordered set silently removes the ordering from every sequence in
  the curriculum. No one had reported it because nothing looked broken.
- **Table column alignment was dropped**, in the two packs that use tables.

Neither would have been found by looking for the symptoms the review described.
They were found because the guard was written as a property — parse, re-emit,
compare — rather than as a list of constructs someone remembered to ban. A
checklist only refuses what its author thought of.

### `rust-version = "1.85"` had never been true

The review's own words: *a promise CI never checks*. The MSRV job was added to
check it, and failed on its first run against a real 1.85 toolchain — 35 errors,
all the same one. Let-chains (`if x && let Some(y) = z`) are used throughout the
codebase and stabilised in **1.88**; 1.85 was the edition-2024 floor and was
never the real minimum. The crate has never compiled on the version it told
users to install.

Bisected to confirm rather than assumed: 1.87 fails with 35 errors, 1.88 is
clean. `rust-version` now says 1.88 and `CONTRIBUTING.md` says the same.

Stating the truth then produced a second, smaller finding for free: clippy gates
some lints on the declared MSRV, so `collapsible_if` had been silently
suppressed. With the real minimum declared, it fired, and one nested `if` became
the let-chain the rest of the file already uses.

### A Windows-only warning failed a `-D warnings` build

`create_private_scratch_dir` binds `let mut builder` and mutates it only inside
`#[cfg(unix)]`, where `DirBuilderExt::mode` takes `&mut self`. On Windows the
mutation does not exist, so `unused_mut` fired and `-D warnings` turned it into
a failed build. macOS and Linux CI were green; nothing local could have caught
it. This is the same shape as every P0 in the review — a path the author's
machine never takes — and it is the reason the matrix is worth its runtime.

### The false MSRV was holding a security fix out

`time` was pinned to exactly `0.3.45`, with a comment explaining why: *0.3.46
raised `time`'s MSRV above DeltaForge's documented Rust 1.85*. That pin was
therefore load-bearing on a claim that was never true.

`cargo deny`, on its first run, reported **RUSTSEC-2026-0009** — denial of
service via stack exhaustion in `time`'s RFC 2822 parser, fixed in 0.3.47. The
pin was holding the fix out to preserve compatibility with a Rust version the
crate had never compiled on.

Making the MSRV honest dissolved the reason for the pin, and `time` moved to
0.3.55. Verified to still build on the declared 1.88.

Two gates found this between them, and neither could have alone: the MSRV job
proved the claim false, and `cargo deny` proved what the false claim was
costing.

### Reading the project registry could make another process fail to write it

A cross-platform correctness bug, found only because `deltaforge init` now
registers the project it creates (P2-7) and so many more processes touch
`projects.json` at once.

Registry writes are already serialised by a lease and already use `MoveFileEx`
with `MOVEFILE_REPLACE_EXISTING`. Registry *reads* were not, and on Windows a
handle opened without `FILE_SHARE_DELETE` blocks a replace — so an ordinary
reader makes a concurrent writer fail with *Access is denied*. The lease
serialises writers against each other and cannot help here, because the reader
never takes it.

That is a real defect, not a test artefact: `deltaforge status` in one terminal
could make `deltaforge init` in another fail to register. Every file DeltaForge
writes atomically — the registry, `state.json`, the service discovery record —
is now read through `read_to_string_shared`, which sets the share mode on
Windows and is a plain read everywhere else. The replace also retries briefly on
a transient `ACCESS_DENIED`/`SHARING_VIOLATION`, because an antivirus scanner or
the search indexer can hold a handle no care on DeltaForge's side prevents.

### `set_modified` needs a writable handle on Windows

`fs::File::open(path).set_modified(…)` works on Unix, where `futimens` accepts a
read-only descriptor, and fails on Windows with *Access is denied*. One test
used it. Now opened for writing.

### Windows reports a closed connection as an error, not as end-of-stream

Both integration suites read each response with
`stream.read_to_string(&mut response).unwrap()`. On Windows a peer that closes
after replying surfaces as `ECONNRESET` rather than as EOF, so a *complete*
response arrived alongside an error and the unwrap failed — on Windows only,
while macOS and Linux stayed green.

It had to be found twice. `browser_journey.rs` and `workbench_flow.rs` each
carried their own copy of "send a request, read the reply", so fixing one left
the other to fail on the next run with the same error at a different line. The
helper now lives in `tests/common/mod.rs` and both call it — a rule written
twice being, again, the defect shape this project keeps finding.

### The `--check` gate was already failing on `main`

`cargo fmt --check` is a CI gate. Ten hunks across four files did not satisfy it
at `f3bcb85`, because rustfmt's stable line-breaking had changed since the code
was last formatted. CI would have failed on the first push regardless of any
work done here. A gate that has not been run since the toolchain moved is not a
gate.

### The health screen was the one screen with no live connection

P1-5 asked for a disconnected state after N failed reconnects, and got one. The
headless journey then failed on it — because the journey's last act before
killing the service was to visit the project-health screen, and `loadProject`
returns before opening any event stream when health is not `healthy`.

So the one screen a learner reaches *because something has already gone wrong*
was the one screen that could not tell them the workbench had stopped. It now
opens the application stream. This was found only because the harness drove the
real page in the real order, which is exactly the argument P0-2 makes.

### The harness asserted a title before the page could know it

Not a product defect, and recorded because the distinction matters. A screen is
unhidden as soon as its route is known; its title is set once the data that
names it arrives. `renderCreate` shows the form and *then* fetches the catalog
and the workspace. Locally that gap is a few milliseconds and the assertion
never lost the race. On a Linux CI runner it did, reading the previous route's
title.

The page was right and the harness was wrong. All five title assertions now wait
for the title rather than sampling it, and the waiter was checked against a
deliberately impossible expectation to confirm it still fails when the title is
genuinely wrong.

The general point: a harness that only ever ran on one fast machine has the same
blind spot as the product did.

### Preflight was creating a directory just for being looked at

Fixing P0-1 meant separating "can this be created" from "create it".
`resolve_target` did both, and preflight called it — so opening the creation
screen and walking away left a `~/DeltaForge` behind, on a screen whose entire
purpose is to report before anything is written. Nobody would have filed this;
it is invisible unless you go looking at the boundary between a decision and its
effect.

---

## 3. The numbers

| | At review | Now |
|---|---:|---:|
| Rust `#[test]` (all targets) | 194 | 222 |
| JavaScript unit tests | 0 | 13 |
| Headless-browser journeys | 0 | 1 (11 asserted steps) |
| CI jobs | 1 | 7 |
| Idle CPU, release, browser tab open | 7.0% of a core | **0.400%** |
| Idle CPU, release, no client | 2.0% of a core | **0.067%** |
| Contrast pairs below WCAG AA | 4 of 34 | **0 of 34** |
| CI jobs green on Linux / macOS / Windows | never run | **8 of 8** |
| Known security advisories | 1 (unreported) | **0** |
| Packs failing the renderer round trip | (not measured) | 0 of 4 |
| Stage files leaking literal markdown | 21 | 0 |
| FlashIndex stages proven passable from content alone | 5 of 14 | **14 of 14** |

CI jobs: `rust` (fmt, clippy, test, validate-pack on three OSes), `page` (Node
unit suite, contrast check, headless journey), `msrv` (pinned 1.88),
`publish-dry-run`, `energy` (idle CPU ceiling), `dependency-policy`
(`cargo deny`: advisories, bans, licences, sources).

---

## 4. The remaining release sequence

Everything below is ready and unexecuted. It needs a push.

The branch is pushed and **CI is green on all eight jobs across Linux, macOS and
Windows** (run `33803509969`). What remains:

```bash
git tag -a v1.0.0-rc.1 -m "Release candidate 1"
git push origin v1.0.0-rc.1
```

That builds five targets, checksums and attests each archive, and cuts a
pre-release. It does **not** touch crates.io, because the tag does not name the
manifest version.

Then, on each of macOS, Linux and Windows — on a machine that has never built
this project — download the archive, verify its checksum, run `deltaforge`, and
complete FlashIndex step 1 in the browser. On macOS this includes the Gatekeeper
step the README documents; that step is part of what is being tested.

Only then:

```bash
git tag -a v1.0.0 -m "DeltaForge 1.0"
git push origin v1.0.0
```

which publishes the crate.

If Linux or Windows hardware is not reachable, the honest substitute is a
workflow-dispatch job that downloads the published archive on those runners and
runs the same journey — and the result must be reported as *validated on CI
runners*, not as *validated on three platforms*.

---

## 5. The blind spot, one year of commits later

The review's closing paragraph:

> Every P0 exists because the software was only ever driven by the person who
> wrote it, on the machine where it was written, through the paths that person
> already knew worked.

That is still the right diagnosis, and it is still only partly answered.

**What is now structurally different.** Four things run the product without the
author's hands on it, on every commit:

- a headless browser drives the real page from a home directory and a workspace
  that do not exist when it starts — the exact machine state that hid P0-1;
- three operating systems build and test every commit, which found five defects
  in the first four runs that no amount of local care would have;
- `node --test` executes the page's own decisions, which had zero coverage;
- two scripts assert the two numbers the review had to measure by hand, so
  contrast and idle CPU cannot drift back unobserved;
- `validate-pack --strict` proves content survives its renderer by round trip
  rather than by a list of banned constructs.

Contract amendment A7 records this as a practice: a fix is not done until
something other than a person re-runs the path it fixed.

**What is not answered.** Three things, stated plainly because the value of this
document is that it does not flinch.

1. **No human who is not the author has ever used this product.** P3-8 is
   deferred, not solved, and the review is right that one session with one
   unfamiliar programmer would have found P0-1 in ninety seconds. Every
   automated gate added here was designed by someone who already knows how the
   product works, and that is a category of blindness no amount of CI removes.
2. **The install path is still a claim.** Until an archive is downloaded onto a
   machine that has never built this project, "install from Releases" is
   documentation, not a tested path. The rc sequence in §4 exists precisely
   because that has to stop being a claim before 1.0.0 is real.
3. **The content-sufficiency practice grades its own homework.** The attempts
   are made by agents told not to read the repository. That instruction is not
   enforced by the filesystem, which `content-sufficiency.md` already says. It
   is a good practice with a stated weakness, not a proof — though it has now
   earned some credit: it found a stage whose requirements a learner could not
   satisfy by following them, which nothing else in this review did.

The pattern the review named is a solo-project pattern, and the durable fix is
the one it gave: make something other than the author run the product, on every
path, on every build. That is now true for four paths. It is not yet true for
the one that matters most, which is a person.
