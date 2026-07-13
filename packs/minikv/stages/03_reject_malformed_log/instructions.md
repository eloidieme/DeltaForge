# Stage 05 — Reject malformed history

## Goal

Make recovery fail clearly when a non-empty log record cannot be interpreted as a valid operation.

Stage 04 defined what valid history means. This stage prevents damaged history from being mistaken for a successful missing-key lookup.

## Background

Suppose a log contains:

```text
SET colour blue
SET broken
SET size large
```

The middle line begins like a `SET` record but has no separated value. MiniKV could skip it and continue. That would produce an answer, but the answer would hide the fact that part of the durable history was not understood.

Silent guessing is dangerous in storage software. A malformed line may be a user mistake, a truncated write, a version mismatch, or damaged data. Those causes need different repairs, but all of them deserve a visible failure.

This stage therefore separates three outcomes:

```text
valid log, key found     → print the value
valid log, key absent    → print nothing and succeed
invalid non-empty record → print a diagnostic and fail
```

Blank lines are harmless in MiniKV's text format and may be ignored. A non-empty line must be one of the operations the current stage understands.

A useful diagnostic names the problem without printing a successful value. Stderr is the right channel because stdout remains reserved for the result of a valid lookup.

## Requirements

Keep `minikv get <log-path> <key>`.

Reject any non-empty line that is not a valid `SET <key> <value>` record. Reject a `SET` line missing either its key or its value. Exit non-zero and include `malformed` in stderr. Do not print a recovered value after encountering malformed history.

Blank lines may be ignored. Valid logs retain all Stage 04 behavior.

## Example

For:

```text
SET colour blue
SET broken
```

the lookup fails:

```console
$ minikv get store.log colour
error: malformed SET record
```

The precise diagnostic may contain more context, but it must identify the malformed history and return a non-zero status.

## Edge cases

- A `SET` record without a value is malformed.
- An unknown non-empty operation is malformed.
- Blank lines do not create records or errors.
- A malformed line after a valid matching record still makes the whole lookup fail.

## Success criteria

All `deltaforge test` cases pass and no damaged non-empty record is reported as an ordinary empty or successful lookup.

## Non-goals

- Repairing, truncating, or silently skipping damaged history.
- Checksums or length-prefixed binary records.
- Recovering a partial state before the error.
- Tombstones, which introduce another valid operation later.
