# Stage 10 — Summarize the corpus

## Goal

Add a human-readable summary showing how many files, token occurrences, and distinct token texts FlashIndex found.

The index now works, but a learner still needs a quick way to check whether it indexed roughly the project they expected.

## Background

Consider two source files:

```rust
fn open() {}
fn close() { open(); }
```

There are two files. There are several token occurrences because `fn` and `open` appear more than once. There are fewer distinct token texts because repeated occurrences share the same spelling.

These counts answer different questions:

- **Files** describes the size of the selected corpus.
- **Tokens** describes the amount of identifier-like material encountered, including repetition.
- **Unique tokens** describes the size of the case-sensitive vocabulary.

Keeping the quantities separate makes the summary useful for diagnosis. Zero files suggests corpus selection failed or the directory is empty. Many files but zero tokens suggests the selected files contain no matching text. A very large token count with a small vocabulary may be perfectly valid generated repetition—or a clue that the corpus policy admitted something unexpected.

The labels and order are fixed so a person can scan the report and a simple script can still read it line by line.

## Requirements

Add:

```console
flashindex summary <path>
```

Use Stage 02 file selection and Stage 03 tokenization. Print exactly these three labelled lines in this order:

```text
files: <N>
tokens: <N>
unique_tokens: <N>
```

`tokens` counts every occurrence. `unique_tokens` counts distinct case-sensitive token strings across the entire corpus. All values are non-negative decimal integers.

## Example

```console
$ flashindex summary project
files: 3
tokens: 84
unique_tokens: 31
```

The summary does not list the tokens themselves. It describes the collection that later commands search.

## Edge cases

- Repeated occurrences increase `tokens` but not `unique_tokens` after the first spelling.
- `Token` and `token` count as two unique token texts.
- Non-source assets do not contribute to any counter.
- An empty corpus reports zero for all three counters.
- The labels and their order do not change.

## Success criteria

All `deltaforge test` cases pass and the counts agree with the Stage 02 corpus and Stage 03 occurrence stream for the same input.

## Non-goals

- Listing the most frequent tokens or files.
- Measuring runtime; `bench` has a different purpose.
- JSON, interactive, or graphical output.
- Changing the corpus or index while producing the summary.
