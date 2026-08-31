# Content sufficiency

Validation practice B from [release-1-0-contract.md](release-1-0-contract.md).

## What this measures

Whether a stage's published specification is genuinely complete, or only appears
complete to the person who wrote it.

DeltaForge ships without external learner research, and this practice is the closest
substitute for the thing external learners were most needed for. An agent receives only
what a learner can see in the workbench — the guide, its worked example, its edge cases,
its exclusions, and the help ladder — and must produce an implementation that passes the
stage's real checks. It sees no `tests.yaml`, no fixtures, and no reference solution.

**Limitation, stated plainly.** This measures specification completeness. It measures
nothing about visual hierarchy, orientation, or how a person feels while stuck. An agent
that reads a whole guide in one pass is also not a tired human reading it at 11pm, and a
guess an agent makes calmly is one a person might make angrily.

## The protocol

```bash
cargo build --release

# One sandbox per stage: the content a learner has seen up to that point, and a
# project to implement it in. Nothing else.
tools/content_sufficiency/prepare.sh 05_inverted_index /tmp/attempt-05

# The attempt happens inside the sandbox, reading CONTENT.md only.

# The real checks, unchanged.
tools/content_sufficiency/check.sh 05_inverted_index /tmp/attempt-05
```

`prepare.sh` builds `CONTENT.md` from `deltaforge pack content flashindex --stage <id>`
for that stage and every earlier one, because a learner reaching stage 8 has read
stages 1 through 8 and their program must satisfy all of them.

The attempt is asked for three things beyond the code: every place it had to guess,
every place the content contradicted itself, and the one addition that would most have
helped. **Those notes are the product of the practice.** A pass with a long list of
guesses is a weaker result than a pass with a short one.

### Honesty of the sandbox

The sandbox contains no tests, fixtures, or reference solution, so nothing inside it can
leak the contract. The attempt is instructed not to read the DeltaForge repository,
which sits elsewhere on the same machine; that instruction is not enforced by the
filesystem. An attempt that ignored it would be visible in its own report, because it
would have no guesses to list. This is a stated weakness of the method, not a hidden
one.

## Execution 1 — 2026-09-01

Five stages, chosen to span the curriculum: an early one, one at each third, and the
last. Each attempt was made by a separate agent with no prior knowledge of this project,
given only its sandbox.

| Stage | Cumulative content | Result | Guesses reported |
|---|---:|---|---:|
| 02 Choose searchable files | 286 lines | **Pass**, 7 of 7 checks | 6 |
| 05 Group files by token | 749 lines | **Pass**, 4 of 4 checks | 5 |
| 08 Query a saved index | 1,167 lines | **Pass**, 7 of 7 checks | 6 |
| 11 Build the index with several workers | 1,613 lines | **Pass**, 7 of 7 checks | 8 |
| 14 Make ranking stable | 2,069 lines | **Pass**, 5 of 5 checks | 8 |

Every attempt passed on its first and only submission. Every attempt returned the
verdict *yes-with-guesswork*: implementable from the content, with decisions the content
did not settle.

The remaining nine stages have not been run. The contract's gate — every FlashIndex
stage passable from published content alone — is therefore **met on five of fourteen**,
and the practice is repeatable for the rest with the two scripts above.

### What the attempts found

Six gaps, each independently reported by two or more attempts, which is what makes them
worth acting on rather than noise from one reader:

1. **A root that exists but is not a directory.** Four attempts. The error contract said
   "missing or unreadable"; a plain file is neither, yet clearly not scannable.
2. **Invalid UTF-8 in a corpus file.** Four attempts. Stage 3's own retrospective raises
   reading-as-text against reading-as-bytes and never resolves it, and no requirement
   settles it either.
3. **Symbolic links.** Three attempts. Never mentioned anywhere in fourteen stages, and
   stage 1's help points at `Path::is_dir`, which follows them.
4. **`index --out` combined with `--threads`.** Three attempts. Each flag is introduced
   alone and no example shows them together.
5. **Stage 5's ordering non-goal beside two sorted examples.** One attempt, but it is the
   sharpest finding in the set: the prose disclaims an ordering guarantee while every
   example on the page is fully sorted, so a learner who believes the prose and emits
   insertion order has followed the text and failed to reproduce the example.
6. **Stage 7 never restating that `index <path>` still prints.** One attempt. Every other
   stage restates what carries forward; stage 7 is the one place that habit breaks, and
   it is exactly where the ambiguity landed.

### What was changed as a result

All six, in the same commit as this record:

- stage 1's error contract now reads "missing, unreadable, or is not a directory", and a
  check was added for it — the corpus for stage 1 grew from nine cases to ten;
- stage 1's non-goals now name symbolic links and part-way-down-the-tree errors as
  deliberately open;
- stage 3's non-goals now say that every corpus here is valid UTF-8, that both readings
  are acceptable, and that neither is checked;
- stage 5's ordering non-goal now says the examples are sorted for readability and that
  a canonical order becomes a contract in the next step;
- stage 7 now states that `index <path>` without `--out` keeps printing as before;
- stage 11 now states that `--threads` combines with `--out`.

Three findings were left as they are, on purpose. Non-UTF-8 handling, symbolic links, and
errors below the root are now **named as open** rather than answered: no fixture exercises
them, so answering them would be a promise the checks do not keep. Naming an open
question is a complete specification of it; leaving it unmentioned is not.

## Running it again

Run the two scripts for a stage and record the outcome in a new *Execution* section.
Never edit an earlier one: a practice whose history can be rewritten proves nothing.
