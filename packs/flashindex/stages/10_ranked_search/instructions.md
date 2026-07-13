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

This is a teaching ranking function, not a universal model of relevance. Production search systems may consider document frequency, field importance, proximity, recency, or learned signals. Our smaller rule keeps every score visible enough to calculate by hand.

The command searches a directory and builds its information in memory. Stage 07 allowed every learner to choose an on-disk format, so black-box tests cannot provide one shared persisted artifact that all implementations must understand.

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

The next stage will finish the ordering contract for exact ties and result limits.

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

All `deltaforge test` cases for this stage pass and each displayed score can be reproduced from Stage 03 occurrences.

## Non-goals

- A deterministic order for files tied on both scores; the next stage adds it.
- Limiting the number of returned files.
- TF-IDF, BM25, proximity, phrase, or fuzzy scoring.
- Reading a learner-specific persisted index format.
