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

The remaining nine stages were run in execution 2, below.

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

## Execution 2 — 2026-09-03

The remaining nine stages, run the same way: each attempt made by an agent with no
knowledge of this project, given only its sandbox and told not to read the repository.
Three agents, three stages each, in curriculum order.

| Stage | Cumulative content | Result | Submissions | Guesses |
|---|---:|---|---:|---:|
| 01 Scan files | 157 lines | **Pass** | 1 | 2 |
| 03 Recognize tokens | 464 lines | **Pass** | 1 | 3 |
| 04 Find an exact token | 607 lines | **Pass** | 1 | 1 |
| 06 Make the index canonical | 912 lines | **Pass** | 1 | 3 |
| 07 Write the index to disk | 1,049 lines | **Pass** | 1 | 3 |
| 09 Describe a scan as data | 1,329 lines | **Pass** | 1 | 3 |
| 10 Summarize the corpus | 1,476 lines | **Pass** | 1 | 4 |
| 12 Measure parallel speedup | 1,771 lines | **Pass** | 1 | 3 |
| 13 Score multi-token matches | 1,938 lines | **Pass** | 2 | 4 |

**The contract's gate is met: fourteen of fourteen FlashIndex stages are passable from
published content alone.** Eight of the nine passed on first submission; stage 13 took
two.

### What the attempts found

Two of these are worse than anything execution 1 turned up, because they are places where
following the specification *literally* fails the checks.

**Stage 09's requirements contradicted its own checker.** The Requirements block shows
the output object as `{"files": <N>, "runtime_ms": <N>}` — with a space after each colon —
and the prose says "print exactly one valid JSON object". The checks asserted
`"files":2` as a substring and pinned the whole object with a byte-exact regex, both
without spaces. A learner who copied the shape from the requirements failed, and the
failure said nothing about whitespace. Verified by building exactly that implementation:
it failed two of six checks before the fix and passes all six after. The assertions now
allow whitespace wherever JSON does, and the requirement says so.

**Stage 13 required an error string it never named.** "Reject a query containing no
tokens" was the whole specification; the check required stderr to contain `non-empty
query`. This was the only failing submission in the exercise, and it cost a full
iteration of trial and error to discover a phrase no reader could have inferred. Step 11
already sets the right precedent — it pins `positive integer` explicitly — so the
inconsistency, not the requirement, was the defect. Stage 13 now pins its string the same
way.

Two further gaps were reported independently across stages and are now closed:

- **No general invocation contract.** Stages 1, 3 and 4 each specify the happy-path
  command shape precisely and name one or two error conditions, but nothing states the
  general rule for a wrong shape — an unknown command word, or the wrong argument count.
  Every attempt guessed the same way and none were tested on it. Stage 1 now states the
  rule once, for every command the project gains later.
- **`--out` and standard output.** Stage 7's "this step adds a destination; it does not
  replace one" left it genuinely unclear whether the canonical index is still printed as
  well as written. Two attempts reasoned it out from the example rather than the prose.
  Stage 7 now says it.

Everything else reported was a decision the content explicitly declares open — symlink
treatment, mid-tree read errors, partitioning strategy, tie-break order among equal
scores. Those are stated non-goals, not gaps.

### The recurring shape, across both executions

Execution 1 found six gaps of the form *a rule stated once and relied on later*.
Execution 2 found a different shape and a sharper one: **the prose promises more freedom
than the checker allows.** Stage 09's "exactly one valid JSON object" and stage 13's
unnamed error string are the same defect seen twice — the specification describes a
class of acceptable answers, and the checks accept one member of it.

That is the failure mode this practice is uniquely able to find, because it is invisible
to the author. The author's own implementation is the member of the class that passes.
