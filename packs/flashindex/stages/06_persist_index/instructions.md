# Stage 07 — Write the index to disk

## Goal

Let `flashindex index` write a reusable index file instead of limiting its result to terminal output.

## Background

An in-memory index lasts only as long as its process. When the command exits, the maps disappear, and the next search would have to scan and tokenize the project again.

Saving the index separates preparation from use. A project can be indexed once, then queried later by another process. For that to work, the file needs an exact structure that both writer and reader understand.

FlashIndex uses UTF-8 text with one token record per line. A record begins with the token and continues with its sorted paths. Fields are separated by tab bytes:

```text
retry\tsrc/main.rs\tsrc/network.rs
timeout\tsrc/network.rs
```

Tokens never contain tabs or newlines. The file format is intended for ordinary source paths and does not provide an escape syntax for those two characters in a filename.

The destination may be inside a directory that does not exist yet. It may also contain bytes from an older, longer index. A successful rebuild must leave one complete current file, not a current prefix followed by stale data.

This is persistence, but not a promise of recovery from power loss during the write. That would require a more detailed failure model and a stronger replacement protocol.

## Requirements

Extend the index command:

```console
flashindex index <path> --out <index-file>
```

Write the complete canonical index to `<index-file>`. Each line must contain one token followed by its sorted, deduplicated relative `/` paths. Separate every field with one tab byte and terminate every record with `\n`. Token lines retain canonical ascending order.

Create missing parent directories and replace all stale destination contents. On success, exit 0 and print a line containing `wrote`.

## Example

```console
$ flashindex index project --out .flashindex/project.idx
wrote .flashindex/project.idx
```

After the command succeeds, `.flashindex/project.idx` is a regular file containing the current index rather than a transcript of earlier builds.

## Edge cases

- A destination in missing parent directories is created successfully.
- Rebuilding over an older, longer file removes every stale trailing byte.
- An empty corpus still produces a valid readable file.
- The source directory and output file may have different locations; stored project paths remain root-relative.

## Success criteria

All `deltaforge test` cases pass, a non-empty corpus produces token records, and rebuilding replaces rather than extends the previous file.

## Non-goals

- Querying the saved index.
- Escaping tabs or newlines in tokens and paths.
- Compression, checksums, version migration, or incremental updates.
- Crash-safe atomic replacement or concurrent readers and writers.
