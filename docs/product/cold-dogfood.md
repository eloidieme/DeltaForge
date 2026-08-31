# Cold dogfood

Validation practice A from [release-1-0-contract.md](release-1-0-contract.md).

## What this measures

Whether a person with a working Rust toolchain and no knowledge of DeltaForge can go
from nothing to a first behavioral result, and onward through the journey, **without
leaving the browser and without reading the DeltaForge source, the pack files, or these
docs.**

The gate is stated in two parts, because they fail differently:

- **Activation.** A clean machine reaches its first behavioral run in under five minutes.
- **Surface completeness.** The whole journey is completable in the browser; the terminal
  is used only to write code.

## The script

Run each step in order. Record, for every step: whether it was obvious what to do next,
anything that had to be guessed, and anything that sent you outside the browser.

1. Install: download a release archive for your platform, verify its checksum, put the
   binary on `PATH`. Or `cargo install deltaforge`.
2. Run `deltaforge` with no arguments, from any directory.
3. From the page that opens, find the catalog and choose a project.
4. Choose a language. Read what the environment preflight says.
5. Accept or change the location and the folder name.
6. Create the project.
7. Read the current step. Run its checks without changing anything, and read the failure.
8. Open the project in an editor from the page. Write enough to pass the step.
9. Run the checks again. Reach a pass.
10. Take the snapshot the page offers.
11. Open the performance surface. Record a prediction. Run the benchmark. Read the
    numbers against the prediction.
12. Continue to the next step.
13. Close the browser tab, leave for a while, and come back. Judge whether the page tells
    you where you were.
14. Export the record. Read it and decide whether every claim in it is one the project
    actually earned.

Note every moment you had to consult something outside the browser. Each one is a defect
against decision 4, whatever else it is.

## Execution 1 — 2026-09-01

Performed by the author, on macOS, against a release build.

**This execution is contaminated and does not satisfy the gate.** The author wrote the
software; the whole value of a cold dogfood is that the participant has not. It is
recorded because the two objective parts of it — the activation timing and the
route audit — are measurable regardless of who is driving, and because an honest partial
record is worth more than an unexecuted protocol.

### Activation timing

`tools/dogfood/activation.py` measures the machine time on the activation path: a clean
DeltaForge home, a launched service, and every request the page makes from catalog to
the first behavioral result. It excludes human reading and typing time.

```
service ready              0.0s
catalog loaded             0.2s
environment preflight      0.0s
project created            0.1s
first behavioral run       3.2s
------------------------------
total machine time         3.5s
```

The dominant cost is the learner project's own first `cargo build --release`, which is
what "first behavioral run" almost entirely consists of. On a machine without a warm
cargo registry it will be longer.

Against the five-minute target this leaves roughly four and a half minutes for a person
to read a catalog card, pick a language, and confirm a location. That is comfortable but
it is not proof: the human half is exactly the half this execution cannot measure.

### Surface audit

Every step of the script above was performed through the browser, and the route and
operation coverage is pinned by `the_whole_journey_is_reachable_from_the_browser` in
`tests/browser_journey.rs`, which drives catalog, preflight, creation, a failing run and
its diagnosis, help, a passing run, prediction, benchmark, reflection, snapshot, record
export, and progression as the exact HTTP exchanges the page makes. The only
terminal-shaped action in that test is writing source, which is what the terminal is for.

Two things were found and fixed during this execution:

- Exporting the record wrote a file into the project, which changed the project digest
  and invalidated the completion proof the record described. Continuing to the next step
  then failed. DeltaForge's own exports are now excluded from the digest.
- Learner-facing pack prose instructed the reader to run `deltaforge test`,
  `deltaforge bench`, and `deltaforge next` in nineteen places across the flagship. Every
  one was a dead end for a reader in the browser. All are now written in
  surface-neutral language.

### Not established by this execution

- Whether a person unfamiliar with the product can infer the next action without being
  told. Nothing here measures that.
- Whether the failure diagnosis reads as helpful rather than merely correct.
- Anything about the install step: this execution used a locally built binary, not a
  published release archive.

## Running it again

Run the script with someone who has not seen DeltaForge, and add a new *Execution*
section. Never edit an earlier one.
