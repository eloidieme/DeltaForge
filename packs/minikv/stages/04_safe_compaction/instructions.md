# Stage 07 — Replace compacted output safely

## Goal

Finish the compaction command's file behavior: create missing destination parents, replace every stale destination byte, and leave the input log unchanged.

The compacted records define logical state. Safe output handling ensures those records become the complete destination rather than only its prefix.

## Background

Suppose yesterday's compacted file was long:

```text
SET alpha one
SET beta two
SET gamma three
```

Today's compacted state contains only:

```text
SET alpha final
```

If MiniKV overwrites only the beginning without truncating the old file, the bytes after the new record remain. The result might look like:

```text
SET alpha final
SET beta two
SET gamma three
```

The first record is current, but the trailing records are stale leftovers from a different artifact. A successful replacement must define the entire destination, not only its prefix.

The destination may also be nested beneath directories that do not exist. MiniKV creates those parent directories as part of producing the requested output.

Finally, compaction reads one artifact and writes another. The input is evidence of the original history. This command must not quietly turn an out-of-place compaction into an in-place mutation.

These rules make the filesystem result dependable, but they do not promise survival if power fails midway through writing. Crash-atomic replacement would usually involve writing a temporary sibling, synchronizing it, and renaming it under a stated failure model. MiniKV does not make that stronger guarantee.

## Requirements

Keep:

```console
minikv compact <input-log> <output-log>
```

Preserve the compacted key/value state and ascending key order. Create missing parent directories for `<output-log>`. Replace the destination's complete previous contents, including any stale trailing bytes. Leave `<input-log>` byte-for-byte unchanged. Print a line containing `compacted` only after successful output.

## Example

```console
$ minikv compact data/store.log snapshots/current/store.log
compacted snapshots/current/store.log
```

The nested destination is created, and it contains only the current compacted records.

## Edge cases

- Missing destination parent directories are created.
- A new result shorter than the old destination leaves no stale trailing record.
- The input file remains unchanged.

## Success criteria

All `deltaforge test` cases pass and the destination bytes describe exactly one current compacted artifact.

## Non-goals

- Crash-atomic replacement across power failure.
- In-place compaction of the input path.
- File locking or concurrent readers and writers.
- Tombstone semantics.
