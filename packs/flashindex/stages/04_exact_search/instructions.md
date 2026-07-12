# Stage 04 — Exact token search

## Goal

Find every occurrence whose complete token equals a query and print it in the same positional format as tokenization.

## Background

Exact retrieval answers a crisp predicate: is this indexed term identical to the requested term? Developer tools depend on token boundaries because substring search for `main` inside `main_index` creates false positives. Classic inverted-file systems were built around the same distinction between terms and arbitrary character sequences; ranking can wait until the exact semantics are trustworthy.

## Requirements

Expose `flashindex search <path> <token>`. Reuse Stage 03's corpus, token grammar, positions, case, portable paths, and ordering. Print only occurrences whose token is byte-for-byte equal to `<token>`, one per line as `path:line:column token`. If nothing matches, exit 0 with empty stdout. Missing arguments or an unreadable root exit non-zero.

## Example

```console
$ flashindex search project main
src/main.rs:1:4 main
```

## Edge cases

- Whole-token matching excludes a longer token that merely contains the query.
- Matching is case-sensitive.
- Multiple occurrences remain in stable path and source order.
- An absent token succeeds with empty stdout.
- Generated, binary-looking, and ignored-directory files remain outside the corpus.

## Success criteria

All `deltaforge test` cases pass and search never disagrees with the occurrence stream produced by `tokenize`.

## Non-goals

- Substring, prefix, fuzzy, phrase, or regular-expression search.
- Ranking or limiting results.
- Persisting an index.
