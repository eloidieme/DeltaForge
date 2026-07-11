# Task 08 — Design reflection loop

*(Prepend `handoffs/00-shared-context.md`. Depends lightly on Task 01 (benchmark history shape) — build against whatever history schema is current.)*

## Objective

Implement the second half of Spec §17. Today `deltaforge design` shows a static prompt and `--edit` opens `$EDITOR` on a notes file (`src/commands/design.rs`). The Spec's point is the *after* moment: once real measurements exist, confront the learner's predictions with the data ("You expected tokenization to dominate. Merge took 41% of total time. What changed your understanding?"). This teaches engineering reflection.

## Design

### `deltaforge design --reflect [--stage <id>]`
- Preconditions (each with a clear, actionable error): the stage has a design note in `.deltaforge/design_notes/<stage>.md` (else: "write your predictions first: deltaforge design --edit"), and saved benchmark history exists for that stage (else: "record a measurement first: deltaforge bench --save").
- Behavior: append to the note file (via `atomic_write` of the full updated content) a new section:
  ```markdown
  ## Reflection — 2026-07-12T14:03:00Z

  ### Measured results

  - index_with_threads {threads: 1}: median 812.40 ms
  - index_with_threads {threads: 8}: median 143.10 ms (speedup 5.7x)
  - peak memory: 412 MB

  ### Questions

  1. Which prediction from your design note held up? Which didn't?
  2. What surprised you most in the measurements?
  3. What is the single next experiment you would run, and what result would confirm your hypothesis?
  ```
  - Measured results come from the **latest** saved history records for benchmarks belonging to this stage (median, throughput, peak memory, speedup when present).
  - Questions: if the stage's `design_prompt.md` contains a `## Reflection` section, use its questions; otherwise use the generic three above.
  - Running `--reflect` twice appends a second dated section (idempotence is not required; each reflection is a journal entry).
- After appending, print the file path and open `$EDITOR` **only** when `--edit` is also passed (reuse `open_editor`); otherwise just print the appended section to stdout so the learner sees the questions immediately.

### Nudge
- `deltaforge bench --save`: when the current stage has a `design_prompt.md`, print a one-line nudge after saving: `Reflect on your predictions: deltaforge design --reflect`. No nudge in `--json` mode.

### Docs & content
- Document the predict → measure → reflect loop in `docs/commands.md` (design section).
- Add a `## Reflection` section with 2–3 tailored questions to any existing `design_prompt.md` files (currently flashindex `01_scan_files`; more if Tasks 03/04 have landed — check).

## Acceptance criteria
- Integration tests: `--reflect` with no note errors with the edit hint; with a note but no history errors with the bench hint; with both, the note file gains a dated section containing a measured median and the questions; a second `--reflect` appends rather than overwrites; the bench nudge appears (and is absent in `--json` mode).
- No change to existing `design` / `design --edit` behavior.
- Full quality bar passes.

## Out of scope
AI-generated reflection analysis (Spec §19 modes), percentage-of-runtime breakdowns (needs profiling — deferred).
