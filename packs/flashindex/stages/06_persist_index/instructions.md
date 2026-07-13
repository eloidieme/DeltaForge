# Stage 07 — Write the index to disk

## Goal

Let `flashindex index` write a reusable index artifact instead of limiting the result to terminal output.

This stage is about the writing half of persistence. Reading and querying the artifact will follow once the file contract exists.

## Background

Until now, the index has lived only for the duration of one command. When the process exits, a later search must scan and tokenize the project again.

Consider a project that takes several seconds to index. If a developer performs fifty searches, repeating the full build fifty times wastes the preparation work. A saved artifact allows the expensive phase to happen once and the result to survive afterward.

Writing an index creates a new agreement. A later reader must be able to distinguish records and recover every token and path. Two learners may choose different valid representations—a line-oriented text format, escaped fields, or another UTF-8 structure—so DeltaForge does not prescribe the internal format. It does prescribe the observable facts that the artifact represents.

The destination may be inside a directory that does not exist yet. It may also contain bytes from an older, longer index. A successful rebuild must leave one complete current artifact, not a current prefix followed by stale data.

This is persistence, but not yet crash-safe replacement. Guaranteeing recovery from power loss during the write would require a more detailed failure model than this stage claims.

## Requirements

Extend the index command:

```console
flashindex index <path> --out <index-file>
```

Write the complete Stage 06 canonical index to `<index-file>` using a UTF-8 representation that the next stage can read.

Create missing parent directories. Replace all stale destination contents. On success, exit 0 and print a line containing `wrote`. The artifact must contain enough information to recover every token's sorted, deduplicated relative `/` paths.

## Example

```console
$ flashindex index project --out .flashindex/project.idx
wrote .flashindex/project.idx
```

After the command succeeds, `.flashindex/project.idx` is a regular file containing the current index rather than a transcript of earlier builds.

## Edge cases

- A destination in missing parent directories is created successfully.
- Rebuilding over an older, longer artifact removes every stale trailing byte.
- An empty corpus still produces a valid readable artifact.
- The source directory and output artifact may have different locations; stored project paths remain root-relative.

## Success criteria

All `deltaforge test` cases pass, the output file contains token data for a non-empty corpus, and a rebuild replaces rather than extends the previous artifact.

## Non-goals

- Querying the artifact; that is the next stage.
- Requiring one shared on-disk format from every learner.
- Compression, checksums, version migration, or incremental updates.
- Crash-safe atomic replacement or concurrent readers and writers.
