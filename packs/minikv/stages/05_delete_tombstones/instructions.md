# Stage 08 — Record deletion tombstones

## Goal

Represent deletion as a durable `DEL` record and make recovery treat the latest operation as either an assignment or a removal.

The tombstone makes deletion survive a restart without erasing earlier bytes.

## Background

Suppose the log contains:

```text
SET session active
```

Removing `session` from an in-memory map would make it disappear for the rest of the current process. The durable log would still say that the key is active. After restart, recovery would bring it back.

An append-only store cannot erase the earlier record without abandoning its storage model. Instead, it appends another operation:

```text
SET session active
DEL session
```

`DEL session` is a **tombstone**: durable evidence that earlier values for `session` are no longer live.

Recovery still follows one rule—the latest operation wins—but the reconstructed state now needs to distinguish a live value from an explicit deletion.

A later `SET` can legitimately restore the key:

```text
SET session active
DEL session
SET session renewed
```

The latest operation is again a value, so `get session` prints `renewed`.

Deleting a key that has never been set is also valid. The tombstone records the requested operation. This becomes important in larger or replicated stores where another part of the system may have older knowledge, even though MiniKV remains a single local log.

## Requirements

Add:

```console
minikv delete-log <path> <key>
```

Append exactly `DEL <key>\n`, creating the file and parents under the same rules as `write-log`. Exit 0 and print a line containing `deleted`.

Extend valid recovery grammar to include `DEL <key>`. If the latest operation for the requested key is `DEL`, `get` exits 0 with empty stdout. A later `SET` restores the key and its new value.

## Example

```console
$ minikv delete-log store.log session
deleted session
$ minikv get store.log session
```

The second command succeeds without printing a value.

## Edge cases

- Deleting a key that was never set still appends a valid tombstone.
- Repeated `DEL` records keep the key absent.
- A `SET` after a `DEL` restores the key.

## Success criteria

All `deltaforge test` cases pass and restart recovery never returns a value whose latest operation is `DEL`.

## Non-goals

- Removing tombstones during compaction.
- Expiration times or deletion timestamps.
- Replicated tombstone garbage collection.
- Transactions or concurrent writers.
