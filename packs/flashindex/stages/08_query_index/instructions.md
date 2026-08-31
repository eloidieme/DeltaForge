# Stage 08 — Query a saved index

## Goal

Read a saved index and return the files associated with one exact token without scanning the source tree again.

## Background

Imagine building an index, closing the terminal, and later asking for `retry`. The new process does not have the maps created during indexing. It has only the file on disk.

The tab-separated records preserve the relationship the query needs:

```text
retry\tsrc/main.rs\tsrc/network.rs
```

The reader can recover the posting for `retry` without opening either source file. Preparation and lookup can happen at different times and in different processes.

The command must also distinguish two kinds of absence. A readable index may contain no record for the requested token; that is a successful query with no results. An unreadable file is different. Printing nothing in that case would falsely suggest that lookup completed normally.

This command defines behavior for readable, well-formed index files. Recovery from damaged records belongs to a format-hardening feature with its own error contract.

## Requirements

Add:

```console
flashindex query <index-file> <token>
```

Read the tab-separated UTF-8 records produced by `index --out`. Match the complete token field exactly and case-sensitively. Print its relative `/` paths one per line in ascending order, with no heading or summary.

If the token is absent, exit 0 with empty stdout. An unreadable index file must exit non-zero.

## Example

```console
$ flashindex query .flashindex/project.idx retry
src/main.rs
src/network.rs
```

The command needs the index path and token; it does not need the original project path.

## Edge cases

- A missing token is an empty successful result.
- Returned paths are sorted and contain no duplicates.
- Query matching is exact and case-sensitive.
- An unreadable index file exits non-zero.

## Success criteria

All `deltaforge test` cases pass, and an index written in one invocation can be queried in another with the same postings.

## Non-goals

- Rebuilding or updating the index during a query.
- Prefix, substring, regular-expression, or ranked search.
- Malformed-record recovery or multi-version format negotiation.
