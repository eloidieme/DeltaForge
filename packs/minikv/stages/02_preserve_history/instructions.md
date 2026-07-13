# Stage 03 — Preserve log history

## Goal

Extend `write-log` so every successful write adds one record at the end without destroying records already present.

The file now becomes a history rather than a container for only the latest command.

## Background

Suppose the log already contains:

```text
SET colour blue
```

The next command sets `size` to `large`. Replacing the file would produce:

```text
SET size large
```

The new fact survived, but the earlier one disappeared. A later recovery process could no longer reconstruct both keys.

An append-only log keeps both events:

```text
SET colour blue
SET size large
```

If `colour` changes later, MiniKV does not search backward and edit the first line. It adds another record:

```text
SET colour blue
SET size large
SET colour green
```

This creates stale history, but it also creates a complete sequence from which the latest state can be recovered. The trade is deliberate: writes remain simple, and a later compaction stage will reclaim obsolete records.

Append-only describes the layout, not the strongest possible durability guarantee. A successful write may still be buffered by the operating system or storage device. This project does not claim survival after every possible power failure.

## Requirements

Keep:

```console
minikv write-log <path> <key> <value>
```

Append exactly one `SET <key> <value>\n` record to the end of `<path>`. Preserve every existing byte before the new record. Continue to create missing files and parents, preserve spaces in the value argument, print a line containing `wrote`, and exit 0 on success.

## Example

Starting with:

```text
SET alpha one
```

run:

```console
$ minikv write-log store.log title "hello world"
wrote title
```

The file becomes:

```text
SET alpha one
SET title hello world
```

## Edge cases

- Existing records remain byte-for-byte before the new record.
- Repeated writes for the same key remain as separate historical events.
- Spaces in a newly appended value remain part of that value.
- Writing to a missing file still creates the first record successfully.

## Success criteria

All `deltaforge test` cases pass, repeated writes form a readable chronological history, and the append benchmark completes.

### Reading the benchmark

After `deltaforge bench`, record the existing fixture size, median, and p95. Then ask:

1. Does the measured operation rewrite old bytes or add only new bytes?
2. How much of a very small measurement may be process startup and file opening?
3. Why does a lower latency not prove stronger power-loss durability?
4. Which filesystem and cache differences make two machines hard to compare?

## Non-goals

- Recovering the latest value from the history.
- Removing stale records.
- Promising `fsync`, transactions, or atomic batches.
- Coordinating concurrent writers.
