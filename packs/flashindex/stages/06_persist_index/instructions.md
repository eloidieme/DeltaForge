# Stage 06 — Persist the index

## Goal

Save an inverted index as a reusable file and query that file later without rescanning the source tree.

## Background

Persistence separates the expensive build phase from cheap repeated lookup. Early search systems stored postings on disk for exactly this reason. An on-disk format is also a compatibility promise: writers and readers must agree about escaping, ordering, and replacement. DeltaForge leaves the concrete UTF-8 representation to you, but its observable round trip is fixed.

## Requirements

Expose `flashindex index <path> --out <index-file>` and `flashindex query <index-file> <token>`. The first command writes the complete Stage 05 index, creates missing parent directories, replaces any stale destination contents, exits 0, and prints a line containing `wrote`. `query` reads that artifact and prints matching relative `/` paths one per line in ascending order. A missing token exits 0 with empty stdout; unreadable or malformed index data exits non-zero.

## Example

```console
$ flashindex index project --out cache/index.fi
wrote cache/index.fi
$ flashindex query cache/index.fi durable_token
src/main.rs
```

## Edge cases

- A new destination and its missing parent directories are created.
- Rebuilding replaces stale destination content rather than appending another index.
- A shared token round-trips to multiple sorted, de-duplicated paths.
- A missing query token succeeds with empty stdout.

## Success criteria

All `deltaforge test` cases pass, a written artifact can be queried by a later process, and the design prompt has been considered before choosing a format.

## Non-goals

- A prescribed binary format, compression, checksums, or memory mapping.
- Concurrent writers or incremental updates.
- Ranking persisted results.
