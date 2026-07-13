# Stage 04 — Recover the latest value

## Goal

Read a valid append-only log and return the latest value recorded for one key.

The log preserves history across processes. Recovery turns that history back into current state.

## Background

Consider:

```text
SET colour blue
SET size large
SET colour green
```

The file contains two records for `colour`. They are not two current values. They describe a change over time.

Replaying the log means reading records in chronological order and applying each operation to the state reconstructed so far:

```text
after line 1: colour = blue
after line 2: colour = blue, size = large
after line 3: colour = green, size = large
```

The latest operation for a key wins. This rule is the central recovery invariant MiniKV will preserve through deletion, statistics, and compaction.

The value is the remainder of the record after `SET` and the key. In `SET title hello world`, the value is `hello world`, not only `hello`. The writer preserved the argument, so the reader must preserve it too.

A key absent from a valid log is an ordinary lookup result. It should produce no value and still succeed. Damaged or ambiguous history is an error rather than another form of absence.

## Requirements

Add:

```console
minikv get <log-path> <key>
```

Read valid UTF-8 `SET <key> <value>` records from beginning to end. Print the latest value for the requested key followed by `\n`. Values may contain spaces after the key separator.

If the key never appears, exit 0 with empty stdout. Records belonging to other keys must not affect the result.

## Example

For the log above:

```console
$ minikv get store.log colour
green
```

The earlier value remains in the file but no longer determines the current state.

## Edge cases

- A later `SET` replaces an earlier value for the same key.
- Records for other keys do not affect the requested key.
- A value containing spaces is recovered in full.
- A missing key produces empty stdout and status 0.

## Success criteria

All `deltaforge test` cases pass and replaying unchanged valid bytes always produces the same current value.

## Non-goals

- Defining the complete malformed-record grammar.
- Deletion records.
- Random-access indexes or partial replay.
- Modifying the log during a read.
