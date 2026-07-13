# Stage 14 — Make ranking stable

## Goal

Finish ranked search with a deterministic path tie-break and a maximum of ten results.

Coverage and occurrence counts compare different scores. Stable ranking defines what happens when those scores are exactly equal and when more matches exist than a concise result page should show.

## Background

Consider two files for the query `gamma`:

```text
src/tie_a.rs → 1 matched token, 1 occurrence
src/tie_b.rs → 1 matched token, 1 occurrence
```

Coverage cannot separate them. Occurrence count cannot separate them either.

If the program stops comparing at that point, their visible order may depend on filesystem discovery, map iteration, or thread timing. The scores are valid, but the result list is not reproducible.

A complete ordering needs one final comparison that can distinguish any remaining pair. FlashIndex uses the portable relative path in ascending order. The path is not claiming that `a` is more relevant than `b`; it is a neutral rule that makes ties stable.

Ranking also needs a result boundary. A query may match hundreds of files, but the first screen should remain small enough to inspect. FlashIndex prints at most ten. The limit is another explicit product choice, not a universal property of search engines.

Rank numbers are assigned only after the final order is known. They begin at one and remain consecutive through the printed prefix.

## Requirements

Keep the coverage and occurrence scoring rules and output format. Complete the ordering as follows:

1. distinct query-token coverage descending;
2. total matching occurrences descending; and
3. relative `/` path ascending.

Print only the first ten results after applying the complete order. Number the printed rows from `1` through at most `10`.

## Example

For two exact ties:

```console
$ flashindex rank project gamma
1. src/tie_a.rs (matched 1/1 tokens, 1 occurrences)
2. src/tie_b.rs (matched 1/1 tokens, 1 occurrences)
```

Reversing file creation or discovery order must not reverse these lines.

## Edge cases

- Exact score ties break by ascending portable path.
- A single-token query orders by occurrence count before the path tie-break.
- Eleven or more candidates still print no more than ten rows.
- Rank numbers are consecutive and correspond to the final sorted order.

## Success criteria

All `deltaforge test` cases pass and every pair of candidates has a deterministic comparison under the complete ordering.

## Non-goals

- Pagination or a configurable result limit.
- Explaining scores beyond the required summary text.
- Statistical relevance models.
- Interactive query refinement.
