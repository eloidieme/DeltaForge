# Stage 08 — Report summary

## Goal

Produce a concise human-readable inventory of the indexed corpus: how many files were selected, how many token occurrences were found, and how many distinct token texts those occurrences represent.

## Background

Counts turn an opaque indexing run into something an engineer can sanity-check. “Tokens” and “unique tokens” are intentionally different statistics: one measures corpus volume, while the other approximates vocabulary size. Concordance makers and modern search pipelines use the same distinction to spot unexpectedly empty, duplicated, or noisy inputs.

## Requirements

Expose `flashindex summary <path>`. Print exactly three labelled lines in this order: `files: <N>`, `tokens: <N>`, and `unique_tokens: <N>`. Use Stage 02 for file selection and Stage 03 for token occurrences. `tokens` counts every occurrence, including repeats; `unique_tokens` counts distinct case-sensitive token strings across all files. All counts are non-negative decimal integers.

## Example

```text
files: 2
tokens: 10
unique_tokens: 8
```

## Edge cases

- Non-source assets and ignored directories do not contribute to any count.
- Repeated occurrences increase `tokens` but not `unique_tokens` after the first.
- The same token in two files is one unique token.
- An empty corpus reports zero for all three fields.

## Success criteria

All `deltaforge test` cases pass and the reported counts agree with `scan` and `tokenize` for the same tree.

## Non-goals

- Listing individual files or tokens.
- Markdown, charts, JSON, or benchmark-history analysis.
- Approximate counting or language-specific statistics.
