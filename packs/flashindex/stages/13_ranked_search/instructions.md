# Stage 13 — Score multi-token matches

## Goal

Accept a query containing several tokens and score every matching file by how much of the query it covers and how many matching occurrences it contains.

Exact search answers where one token appears. This stage begins answering a different question: when several files match a larger query, which files contain the strongest evidence?

## Background

Suppose the query is:

```text
cache retry error
```

Three files match:

```text
src/a.rs → cache, retry, error
src/b.rs → cache, retry, retry
src/c.rs → error
```

`src/a.rs` covers all three distinct query tokens. `src/b.rs` covers two, even though one of them occurs twice. `src/c.rs` covers only one.

FlashIndex first scores **coverage**: the number of distinct query tokens found in a file. Coverage rewards a file that speaks to more parts of the question.

When coverage ties, it scores the total number of matching **occurrences**. That gives repeated evidence a smaller, secondary role.

This is a small ranking function, not a universal model of relevance. Larger search systems may consider document frequency, field importance, proximity, recency, or learned signals. FlashIndex keeps every score visible enough to calculate by hand.

The command searches a directory and builds occurrence counts in memory. The saved index records only file membership, so it does not contain the occurrence totals required by the secondary score.

Repeated words in the query count once for coverage. Otherwise `retry retry` would pretend to ask for two distinct ideas when it asks for one.

## Requirements

Add:

```console
flashindex rank <path> "<query>"
```

Split `<query>` on whitespace into distinct, case-sensitive query tokens. Reject a query containing no tokens.

For each file containing at least one query token, calculate:

1. the number of distinct query tokens matched; and
2. the total occurrences of those tokens in the file.

Order by matched-token count descending, then occurrence count descending. Print each result as:

```text
<rank>. <path> (matched <matched>/<query-total> tokens, <occurrences> occurrences)
```

Files tied on both numeric scores may appear in either order. Result limits and a complete tie-break are outside this command's current contract.

## Example

For the three files above:

```text
1. src/a.rs (matched 3/3 tokens, 3 occurrences)
2. src/b.rs (matched 2/3 tokens, 3 occurrences)
3. src/c.rs (matched 1/3 tokens, 1 occurrences)
```

Coverage decides the order before occurrence count is considered.

## Edge cases

- A file matching no query token is absent from the result set.
- A token absent from every file produces empty stdout and status 0.
- Repeated query tokens count once in the coverage denominator.
- Matching remains exact and case-sensitive.
- A query containing only whitespace is an error.

## Success criteria

All `deltaforge test` cases pass and each displayed score can be reproduced from token occurrences.

## Non-goals

- A deterministic order for files tied on both scores.
- Limiting the number of returned files.
- TF-IDF, BM25, proximity, phrase, or fuzzy scoring.
- Reading the file-membership-only persisted index.
