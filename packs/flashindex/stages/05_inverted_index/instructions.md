# Stage 05 — Group files by token

## Goal

Reorganize the token occurrences so that each distinct token points to the files that contain it.

Exact search currently begins with the corpus and examines its occurrences. This stage prepares the relationship in the direction a search query needs.

## Background

Imagine that tokenization produced this simplified record:

```text
src/main.rs     → open, file, error
src/storage.rs  → file, write, error
src/network.rs  → open, connection, error
```

This view begins with a file and tells us what the file contains. To answer “which files contain `open`?”, the program still has to examine each file's tokens.

We can turn the relationship around:

```text
connection → src/network.rs
error      → src/main.rs, src/network.rs, src/storage.rs
file       → src/main.rs, src/storage.rs
open       → src/main.rs, src/network.rs
write      → src/storage.rs
```

This is an **inverted index**. The word “inverted” refers to the reversal: documents once pointed to tokens; tokens now point to documents.

A back-of-the-book index makes the same kind of trade. Someone does the organizing work in advance so that a later reader can begin with a subject and go directly to the relevant pages.

At this stage, the important change is the shape of the information. Every distinct token receives a posting containing its file paths. We will tighten duplicate removal and canonical ordering in the next stage, after the basic grouping is visible.

## Requirements

Add:

```console
flashindex index <path>
```

Build from Stage 03 token occurrences. Print one line for each distinct token using this shape:

```text
token path1 path2 ...
```

Every line must begin with the complete case-sensitive token and contain the root-relative `/` paths in which it appears. Do not print headings, blank separator lines, or a summary.

## Example

For two files containing:

```text
src/main.rs     → open, file
src/storage.rs  → file, write
```

the index contains:

```console
$ flashindex index project
file src/main.rs src/storage.rs
open src/main.rs
write src/storage.rs
```

The command now begins each record with the value a future lookup will know: the token.

## Edge cases

- A token that appears in several files lists every containing file.
- A token appearing in only one file still receives an index record.
- Complete token text is preserved, including case and underscores.
- Index records contain only token and path data, so another command can search their line-oriented shape.

## Success criteria

All `deltaforge test` cases for this stage pass and every token/file relationship in the output can be traced back to Stage 03 occurrences.

## Non-goals

- Guaranteeing final token and path ordering; the next stage makes it canonical.
- Recording occurrence counts or line and column positions.
- Saving the index to disk for another process.
- Ranking documents.
