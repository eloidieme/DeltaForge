# Stage 09 — Compact deleted keys

## Goal

Extend compaction so a key whose latest operation is `DEL` does not reappear in the compacted live state.

Tombstones make deletion durable during replay. Compaction must protect the same meaning while old history is discarded.

## Background

Consider:

```text
SET alpha one
SET beta two
DEL alpha
```

Recovery says that `beta` is live and `alpha` is absent.

A compactor that understands only `SET` records might remember the last value it saw for `alpha` and write:

```text
SET alpha one
SET beta two
```

The compacted log is smaller, but it has resurrected a deleted key. Replaying it no longer produces state equivalent to the input.

Compaction must apply the complete operation history before choosing output records. When a key's latest state is deleted, it has no live `SET` record to write. Other live keys remain.

In MiniKV's single-log model, the compacted artifact may omit both the old value and its tombstone because no older external copy will later be merged back in. A replicated store cannot make that decision so easily; it must know that every replica has observed the deletion. That larger coordination problem is why tombstone collection deserves care in real systems.

## Requirements

Keep `minikv compact <input-log> <output-log>` with complete destination replacement, parent creation, and an unchanged input file.

Replay both `SET` and `DEL`. Write one sorted `SET` record for every key whose latest operation is `SET`. Omit every key whose latest operation is `DEL`. Preserve other live keys and never write a stale pre-deletion value.

## Example

Input:

```text
SET alpha one
SET beta two
DEL alpha
```

Compacted output:

```text
SET beta two
```

Both logs recover `alpha` as absent and `beta` as `two`.

## Edge cases

- A latest tombstone removes the key from compacted output.
- Repeated tombstones keep the key absent.
- A later `SET` after a tombstone restores the key and appears in output.
- Deleting one key does not remove other live keys.

## Success criteria

All `deltaforge test` cases pass and compaction cannot resurrect any key whose latest operation is deletion.

## Non-goals

- Retaining deletion timestamps or tombstone records in the compacted single-log artifact.
- Deciding when distributed replicas may forget tombstones.
- Incremental or background compaction.
- Transactions spanning several keys.
