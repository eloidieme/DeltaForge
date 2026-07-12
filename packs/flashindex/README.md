# FlashIndex

## What you are building

FlashIndex is a local source-code search engine. By the end, you will scan a project, select a text corpus, tokenize identifier-like terms, find exact occurrences, build and persist an inverted index, measure and summarize indexing, parallelize construction without losing determinism, and rank multi-token results.

## Why this is useful

Fast local code search is a realistic systems problem. It touches directory traversal, corpus policy, lexical boundaries, information-retrieval data structures, persistence, CLI design, deterministic concurrency, and performance measurement. These are building blocks behind developer tools, language servers, indexing services, and build systems.

## Big picture

```text
directory tree
    ↓ scan and filter
ordered source corpus
    ↓ tokenize
positioned occurrences
    ↓ invert
token → documents
    ↓ persist / parallelize / rank
reusable search results
```

## Concept map

| Stage | New representation | Invariant to protect |
|---|---|---|
| 01–02 | Ordered source paths | Corpus policy is deterministic and portable. |
| 03 | Positioned token occurrences | Every location points to the printed token's first byte. |
| 04 | Exact-match occurrence stream | Search and tokenization share boundaries. |
| 05 | Token-to-document postings | Tokens and paths are sorted and de-duplicated. |
| 06 | Persistent index artifact | A later process can recover the same postings. |
| 07–08 | Measurements and summaries | Metrics use the same corpus and token definitions. |
| 09 | Merged worker-local indexes | Thread count changes time, never bytes. |
| 10 | Ranked candidate files | Scoring has a deterministic total order. |

## Search glossary

- **Corpus:** the collection of documents admitted to indexing.
- **Token:** a searchable unit produced from source text.
- **Occurrence:** one token at one path, line, and column.
- **Posting list:** the documents or positions associated with one token.
- **Inverted index:** a mapping from tokens to documents, reversing document-to-token input.
- **Term frequency:** how often a term occurs within a document.
- **Document frequency:** how many documents contain a term.
- **Determinism:** identical observable output for identical input, independent of enumeration or thread timing.
- **Speedup:** baseline runtime divided by parallel runtime for the same work.

## Retrieval field note

Book concordances already inverted words into locations. Twentieth-century systems such as SMART turned that idea into ranked computerized retrieval. Production search engines commonly use TF-IDF descendants such as BM25: term frequency rewards evidence inside a document, while inverse document frequency discounts terms appearing almost everywhere. FlashIndex's final ranking is intentionally smaller—coverage, then occurrence density, then path—so every decision remains visible and testable.

## Persistence field note

An index file is an agreement between a writer and a later reader. Delimiters, ordering, truncation, malformed data, and future format versions all matter. Stage 06 leaves the format open, but a thoughtful design can still answer: how is a record bounded, how is stale output replaced, and how would a future reader recognize an incompatible artifact?

## Failure-analysis lab

Diagnose the violated layer before proposing a fix:

1. `scan` returns correct paths in a different order on Windows. Is traversal wrong, or is canonicalization missing?
2. `123alpha` is indexed as one token. Which lexical start rule was ignored?
3. Searching `main` also returns `main_index`. Did corpus selection, tokenization, or matching fail?
4. A token repeated 20 times in one file lists that path 20 times. Which occurrence/posting distinction was lost?
5. Four-thread indexing contains the right lines in varying order. Is the computation incomplete, or is the merge non-canonical?
6. Two tied ranked results swap places across runs. Which final ordering key is missing?

## What good looks like

Good solutions keep output deterministic, separate scanning from tokenization, reuse one definition at every later stage, avoid irrelevant files, preserve portable paths, and use benchmarks to guide optimization rather than guessing. Each optimization should be checked against byte-identical correctness before its speed is celebrated.

## Optional extensions

Natural experiments include Unicode-aware tokenization, ignoring comments and strings, index-format versioning, incremental updates, deleted-file detection, phrase search, prefix search, or BM25-style scoring. Each changes the product contract; write examples and non-goals before choosing data structures.
