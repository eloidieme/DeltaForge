# Stage 03 — Tokenize files

## Goal

Turn the selected source corpus into identifier-like token occurrences, recording each token's portable path and precise one-based source position.

## Background

Tokenization is the bridge from bytes to searchable units. Compiler lexers use rich language grammars; search tools often prefer a smaller, language-neutral rule so one index can cover Rust, C++, Python, and documentation. Position reporting follows the long tradition of concordances and compiler diagnostics: a match is more useful when it takes a reader directly to its line and column.

## Requirements

Expose `flashindex tokenize <path>`. In Stage 02 files, recognize maximal runs of ASCII letters, digits, and `_` that contain an ASCII letter or underscore before any leading digits; leading digit runs are not tokens. Print every occurrence as `relative/path:line:column token`, with one-based line and byte-column positions. Preserve original token case. Order by sorted path, then source order. Comments and string literals are ordinary text at this stage.

## Example

```console
$ flashindex tokenize project
src/main.rs:1:1 fn
src/main.rs:1:4 main
src/main.rs:2:9 fetch_or
```

## Edge cases

- `_` remains part of identifiers such as `fetch_or` and `value_32`.
- Punctuation separates tokens and is never emitted.
- A digit-leading run such as `123abc` emits `abc` at its actual column, never `123abc` or `123`.
- Empty source files produce no occurrences.
- Nested files and occurrences have stable path/source ordering.

## Success criteria

All `deltaforge test` cases pass, every position points at the first byte of the printed token, and the tokenizer benchmark completes.

### Benchmark interpretation worksheet

After measuring tokenization, record fixture bytes, number of printed occurrences, median, and p95, then answer:

1. How much measured work belongs to scanning bytes versus formatting and writing occurrence lines?
2. Would a file containing a few enormous tokens behave like one containing many short tokens?
3. What fixture pair could reveal allocation sensitivity while keeping total bytes similar?
4. Which correctness check must remain unchanged after any tokenizer optimization?

### Reflection

Choose a punctuation-heavy line and mark every transition into and out of a token. Why are byte columns an explicit part of this pack's contract?

## Non-goals

- Full language parsing, keyword classification, or comment/string removal.
- Unicode identifiers or grapheme-based columns.
- Case folding, stemming, or deduplication.
