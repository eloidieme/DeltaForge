# Stage 08 — Query a saved index

## Goal

Read the artifact written in the previous stage and return the files associated with one exact token without scanning the source tree again.

Writing made the index durable. Querying completes the round trip and proves that another process can understand what was saved.

## Background

Imagine building an index, closing the terminal, and later asking for `retry`. The new process does not have the in-memory maps from the earlier build. It has only the artifact on disk.

If the writer preserved every relationship clearly, the reader can recover the posting for `retry`:

```text
retry → src/main.rs, src/network.rs
```

The source files are not needed for this lookup. That separation is the practical value of persistence: preparation and use can happen at different times and in different processes.

The reader must also distinguish two kinds of absence. A well-formed index may simply contain no record for the requested token; that is a successful query with no results. An unreadable artifact is different. In that case, printing nothing would falsely suggest that the search completed normally.

Format-specific corruption matters too, but a shared black-box fixture cannot assume one learner's representation when the format is intentionally open. Validation beyond an unreadable file belongs to the format each learner chose.

## Requirements

Add:

```console
flashindex query <index-file> <token>
```

Read the artifact produced by `index --out`. Print the matching relative `/` paths one per line in ascending order, with no heading or summary.

If the token is absent, exit 0 with empty stdout. An unreadable artifact must exit non-zero.

## Example

After saving an index for a project containing `retry` in two files:

```console
$ flashindex query .flashindex/project.idx retry
src/main.rs
src/network.rs
```

The command reads the index artifact; it does not need the original project path as an argument.

## Edge cases

- A missing token is an empty successful result.
- Returned paths are sorted and contain no duplicates.
- Query matching is exact and case-sensitive.
- An unreadable artifact exits non-zero.

## Success criteria

All `deltaforge test` cases pass and building in one invocation followed by querying in another reproduces the Stage 06 postings.

## Non-goals

- Rebuilding or updating the index during a query.
- Searching by prefix, substring, or regular expression.
- Sharing one artifact format between different learner implementations.
- A shared malformed-artifact grammar; deeper validation depends on the chosen format.
- Ranking the returned paths.
