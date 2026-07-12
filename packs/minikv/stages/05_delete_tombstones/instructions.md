# Stage 05 — Delete tombstones

## Goal

Represent deletion as another durable log operation, so recovery can distinguish “never seen” from “explicitly removed” and compaction cannot accidentally resurrect an older value.

## Background

In an append-only store, erasing an earlier record would defeat the storage model. Instead, systems append a tombstone: a marker saying that earlier values are no longer live. Tombstones appear in log-structured databases, distributed stores, and filesystems. They are deceptively important during compaction—dropping a marker before it has cancelled the older value can bring deleted data back.

## Requirements

Expose:

```bash
minikv delete-log <path> <key>
```

Append exactly `DEL <key>\n`, creating the file and parent directories as Stage 02 does. Exit 0 and print a line containing `deleted`. Extend replay so the latest `DEL` makes `get` return empty success, while a later `SET` makes the key live again. Extend `compact` so keys whose latest operation is `DEL` are omitted.

## Example

```console
$ minikv delete-log store.log session
deleted session
$ minikv get store.log session
```

The second command prints nothing and succeeds.

## Edge cases

- Deleting a key that has never been set still appends a valid tombstone.
- Repeated `DEL` records keep the key absent.
- A `SET` after a `DEL` restores the key with its new value.
- Compaction omits a key when its latest operation is `DEL` while retaining other live keys.

## Success criteria

All `deltaforge test` cases pass and no replay or compaction path can resurrect a key whose latest operation is deletion.

## Non-goals

- Expiring keys automatically or retaining deletion timestamps.
- Transactions, snapshots, or concurrent writers.
- Garbage-collection rules for replicated tombstones.
