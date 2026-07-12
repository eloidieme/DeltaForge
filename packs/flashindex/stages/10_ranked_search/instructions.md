# Stage 10 — Ranked search

## Goal

Turn your index into a search engine that answers *multi-token* queries and
returns the most relevant files first, using a small, fully specified ranking
function with a deterministic tie-break.

## Background

Every search you have built so far answers a yes/no question: does this token
appear here? Real search answers a *better* question — of all the files that
match, which ones should I read first? That shift, from boolean retrieval to
**ranked retrieval**, is the idea that made web search usable. Early systems
like the 1960s SMART system and every search engine since rank results rather
than dumping an unordered set on the user.

Ranking needs a **scoring function**, and scoring functions embody a judgement
about relevance. Production engines reach for weighting schemes like TF-IDF
(term frequency × inverse document frequency, formalised by Karen Spärck Jones
in 1972) and BM25. You will implement something deliberately simpler and fully
specified, so the behaviour is testable to the byte: rank first by **coverage**
(how many of the distinct query tokens a file contains), then by **density**
(how many times those tokens occur), and only then fall back to a tie-break.

That last point matters more than it looks. Two files can tie on every score
you compute. A search engine that returns tied results in a random order is not
reproducible — the same query yields different output run to run, tests flake,
and users lose trust. A **total order** requires a deterministic final
tie-break, and sorting the path ascending gives you one that never depends on
filesystem or thread timing.

### What this command searches

This command takes a **directory** and builds the index in memory before
ranking:

```bash
flashindex rank <path> "<query>"
```

Stage 06 let you persist an index whose on-disk format is yours to choose. Two
learners can pick two perfectly valid formats, so a single shared, pre-built
index *file* cannot be a fair black-box fixture for everyone. Ranking a
directory keeps the test deterministic while reusing everything you have already
built. Interactively you would of course persist an index once (Stage 06) and
query it repeatedly; here the tested interface rebuilds from the directory so
the grader never has to assume your file format.

Note the command is `rank`, not `search`: Stage 04's `search` prints raw token
occurrences, and ranked retrieval is a different question with a different
output shape, so it gets its own verb.

## Requirements

```bash
flashindex rank <path> "<token1> <token2> ...">
```

- The query is a single argument: one or more whitespace-separated tokens.
- Consider only files that contain **at least one** query token.
- Rank them by, in order:
  1. number of **distinct** query tokens the file contains (descending),
  2. total number of query-token **occurrences** in the file (descending),
  3. relative path, `/`-separated (ascending) — the deterministic tie-break.
- Print the top **10** files, one per line, numbered from 1:

  ```txt
  <rank>. <path> (matched <X>/<Y> tokens, <Z> occurrences)
  ```

  where `X` is distinct query tokens matched in that file, `Y` is the total
  number of distinct query tokens, and `Z` is total query-token occurrences.

## Example

```bash
$ flashindex rank ./project "alpha beta gamma"
1. src/three.rs (matched 3/3 tokens, 3 occurrences)
2. src/two_hi.rs (matched 2/3 tokens, 3 occurrences)
3. src/two_lo.rs (matched 2/3 tokens, 2 occurrences)
4. src/one.rs (matched 1/3 tokens, 1 occurrences)
```

`three.rs` wins on coverage; `two_hi.rs` and `two_lo.rs` tie on coverage so the
denser one ranks higher; `one.rs` matches least.

## Edge cases

- **Tie-break:** two files with identical coverage and identical occurrence
  counts must be ordered by ascending path, every run.
- **Absent tokens:** a query whose tokens appear in no file prints nothing and
  exits 0.
- **Empty query:** an empty or whitespace-only query is an error — exit non-zero
  with a message on stderr.
- **Repeated query tokens:** `Y` counts *distinct* query tokens, so a query of
  `"alpha alpha"` has `Y = 1`.
- More than 10 matching files: print only the top 10.

## Success criteria

- All `deltaforge test` cases for this stage pass.
- Output is deterministic: the same directory and query always produce the same
  bytes.

### Reflection

1. Build the ranking tuple for each file in the example before sorting it.
2. Which result could outrank another through coverage even with fewer total occurrences?
3. Why does a final path comparison turn a partial relevance order into a total deterministic order?
4. Which user expectation would change first if the pack moved from this score to TF-IDF or BM25?

## Non-goals

- TF-IDF, BM25, or any statistical weighting — the ranking function is exactly
  the three keys above.
- Phrase, prefix, fuzzy, or regular-expression matching.
- Persisting or reloading a ranked result.
- Paging beyond the top 10.
