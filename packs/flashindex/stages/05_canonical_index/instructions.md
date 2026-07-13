# Stage 06 — Make the index canonical

## Goal

Make `flashindex index` produce one byte-stable representation: tokens sorted once, paths sorted once, and no duplicate file within a posting.

Stage 05 established the inverted relationship. This stage makes that relationship dependable enough to save, compare, and reproduce.

## Background

Suppose `retry` appears twenty times in `src/network.rs`. The token occurrence stream contains twenty useful locations, but a document-level posting answers a different question: which files contain `retry`?

For that question, the path should appear once:

```text
retry src/network.rs
```

Printing the same path twenty times would confuse occurrence frequency with document membership.

Order presents another problem. Filesystem traversal can discover files in different sequences, and a map may not reveal its keys in a guaranteed order. These two outputs describe the same relationships:

```text
retry src/network.rs src/main.rs
open src/storage.rs
```

```text
open src/storage.rs
retry src/main.rs src/network.rs
```

For a human reader, either may be understandable. For saved artifacts, regression tests, and later parallel construction, “equivalent” is not precise enough. FlashIndex chooses one **canonical** representation: a single agreed form for the same logical data.

Tokens are ordered by their byte text. Paths within each token are also ordered, after duplicates have been removed. The result depends only on the corpus, never on discovery order or the timing of later workers.

## Requirements

Keep `flashindex index <path>` and the Stage 05 line format.

Tighten its output as follows:

- sort distinct tokens in ascending bytewise order;
- list each containing path exactly once per token;
- sort paths within each token in ascending order;
- print no blank or summary lines; and
- succeed with empty stdout when the source corpus contains no tokens.

## Example

If `retry` appears several times in `src/network.rs` and once in `src/main.rs`, canonical output contains:

```text
retry src/main.rs src/network.rs
```

The number and discovery order of occurrences cannot change that line.

## Edge cases

- Repeated occurrences in one file produce one path in that token's posting.
- Token lines are sorted independently of source discovery order.
- Paths inside each posting are sorted and deduplicated.
- An empty source corpus succeeds with empty stdout.

## Success criteria

All `deltaforge test` cases pass and repeated indexing of unchanged input produces byte-identical stdout.

### Reading the benchmark

After measuring index construction, record input bytes, token occurrences, distinct tokens, median, and p95. Then consider:

1. Which work belongs to tokenization, grouping, deduplication, and ordering?
2. How would many repeated tokens affect the relationship between occurrence count and output size?
3. Which fixture would stress a very large posting list?
4. Which byte-identical comparison should protect every later optimization?

## Non-goals

- Persisting the canonical bytes to a named artifact.
- Recording term frequency or exact positions in the posting.
- Compressing paths or tokens.
- Parallel construction.
