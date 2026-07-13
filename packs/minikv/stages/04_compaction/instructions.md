# Stage 06 — Compact stale history

## Goal

Turn a valid historical log into a smaller equivalent log containing one current `SET` record for each key.

This stage concentrates on the meaning of compaction: remove operations that no longer affect recovery while preserving the same live state.

## Background

Consider:

```text
SET colour blue
SET size large
SET colour green
```

Recovery needs all three records only while it is reconstructing the changes. Once we know the final state, `SET colour blue` can no longer affect a future lookup. It is stale history.

A compacted representation is:

```text
SET colour green
SET size large
```

The important claim is not merely that the output is shorter. It is that replaying the original and replaying the compacted log produce the same current key/value map.

This is **compaction**. Append-only systems periodically perform some version of it because simple writes accumulate records that have been superseded. Log-structured databases, message logs, and storage engines vary greatly in their formats, but they all need a clear definition of what information may be discarded.

MiniKV writes compacted records in ascending key order. Recovery does not require that order, but a canonical artifact is easier to test, compare, and reproduce.

For this stage, concentrate on which records belong in the result. Destination replacement and directory handling are tightened in the next stage.

## Requirements

Add:

```console
minikv compact <input-log> <output-log>
```

Replay the valid input using the Stage 05 grammar. Write one UTF-8 record for the latest value of each key:

```text
SET <key> <latest-value>\n
```

Order output records by ascending key. Do not include superseded values. On success, exit 0 and print a line containing `compacted`. Malformed input must fail under the same rules as recovery.

## Example

Input:

```text
SET alpha one
SET beta two
SET alpha three
```

Output:

```text
SET alpha three
SET beta two
```

Both logs recover `alpha` as `three` and `beta` as `two`.

## Edge cases

- Several values for one key collapse to the latest value.
- Every live key appears exactly once.
- Output records use ascending key order regardless of input order.
- Malformed input fails instead of producing a partial compacted state.

## Success criteria

All `deltaforge test` cases pass, replaying input and output yields equal live state, and the compaction benchmark completes.

### Reading the benchmark

Record input bytes, output bytes, median, and p95. Then ask:

1. How much of the input is stale, and how does that affect the usefulness of compaction?
2. Which time belongs to parsing, rebuilding state, sorting, and writing?
3. Does a smaller result necessarily imply a faster compaction run?
4. What second fixture would separate parsing cost from output cost?

## Non-goals

- Safe replacement details for an existing destination; the next stage focuses on them.
- Modifying the input log in place.
- Deletion records.
- Background or concurrent compaction.
