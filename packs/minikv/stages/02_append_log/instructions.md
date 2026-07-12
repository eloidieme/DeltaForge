# Stage 02 — Append-only log

## Goal

Make a write durable by appending a `SET` record to a log file. Repeated commands must extend the history rather than replacing it, giving later stages enough information to reconstruct the store after a restart.

## Background

Append-only storage turns a mutation into a sequential write. Database systems call closely related structures write-ahead logs; filesystems and event-sourced applications use the same principle because appending is simple, inspectable, and less vulnerable to half-rewritten state than editing records in place. The trade-off is deliberate duplication: old values remain until compaction reclaims them.

## Requirements

Expose:

```bash
minikv write-log <path> <key> <value>
```

Append exactly one UTF-8 line, `SET <key> <value>`, followed by `\n`, to `<path>`. Preserve all existing bytes in the log. Create the file and any missing parent directories. A successful command exits 0 and prints a line containing `wrote`; invalid arity exits non-zero.

## Example

```console
$ minikv write-log data/store.log language Rust
wrote language
```

The log then ends with:

```text
SET language Rust
```

## Edge cases

- A missing log file is created.
- Missing parent directories are created.
- Existing records remain before the newly appended record.
- Spaces inside a value argument are preserved after the key separator.

## Success criteria

All `deltaforge test` cases pass, repeated writes form a readable history, and `deltaforge bench` can measure append cost against the supplied log fixture.

### Benchmark interpretation worksheet

After `deltaforge bench`, note the median, p95, fixture size, and output destination, then answer:

1. Is the fixture large enough for sequential I/O to dominate process startup and file-open cost?
2. What would you expect to happen as the existing log grows if the command truly appends?
3. Would a faster result prove stronger durability? Why are latency and durability separate claims?
4. Which environmental factors—filesystem cache, storage device, background I/O—make two machines difficult to compare?

### Reflection

Explain why retaining old records is simultaneously the feature that enables recovery and the cost that later requires compaction.

## Non-goals

- Reading values back or deduplicating older records.
- Crash-recovery guarantees such as `fsync`.
- File locking, transactions, or concurrent writers.
