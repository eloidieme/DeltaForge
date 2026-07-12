# Stage 05 — Build an inverted index

## Goal

Invert the token stream into a deterministic map from each distinct token to the source files that contain it.

## Background

A book's index maps a subject to pages; an inverted index applies the same reversal to a document collection, mapping terms to documents. This structure powered early information-retrieval systems and remains central to modern search engines because lookup no longer requires rescanning every file. At this stage presence matters, not frequency: repeating a token within one document must not repeat the document identifier.

## Requirements

Expose `flashindex index <path>`. Build from Stage 03 token occurrences and print one line per token as `token path1 path2 ...`. Sort tokens by bytewise ascending text. For each token, include every containing relative `/` path exactly once, sorted ascending. Print no blank or summary lines; an empty corpus succeeds with empty stdout.

## Example

```text
alpha src/a.rs src/b.rs
beta src/a.rs
gamma src/b.rs
```

## Edge cases

- Repeated occurrences in one file list that path once.
- A token shared by several files lists every path once in ascending order.
- Token lines are in ascending token order and remain byte-stable across runs.
- An empty corpus produces empty stdout.

## Success criteria

All `deltaforge test` cases pass, output is deterministic, and the index benchmark runs successfully.

### Benchmark interpretation worksheet

After `deltaforge bench`, record corpus bytes, distinct tokens, printed postings, median, and p95, then answer:

1. Would a corpus with the same bytes but a much larger vocabulary stress the same operations?
2. How does repeating one token within one file affect tokenization work versus final posting count?
3. Which costs belong to index construction and which belong only to deterministic serialization?
4. What measurement would help identify whether ordering or allocation dominates?

### Reflection

Describe the information discarded when occurrences become file-only postings. Why is that loss acceptable for this stage but not for Stage 04 output?

## Non-goals

- Recording positions or term frequencies in the printed index.
- Persistence, compression, ranking, or incremental updates.
- Parallel construction, introduced later.
