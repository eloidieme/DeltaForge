# Stage 04 — Compaction

## Goal

Rewrite a historical log into a smaller equivalent log containing one current `SET` record per live key. The compacted file must recover to the same logical state while discarding values that can no longer affect any read.

## Background

Append-only designs exchange cheap writes for accumulating stale data. Log-structured merge trees, Bitcask-style stores, and Kafka all perform some form of compaction to reclaim that space. Compaction is a semantic operation, not mere compression: it must preserve the state implied by replay. Deterministic key order also makes artifacts reproducible and easy to diff.

## Requirements

Expose:

```bash
minikv compact <input-log> <output-log>
```

Replay the input using Stage 03's grammar, then write the output as UTF-8 `SET <key> <latest-value>\n` records. Emit exactly one record per key, ordered by ascending key. Create missing output parents and replace stale output contents. Leave the input file unchanged. On success exit 0 and print a line containing `compacted`; malformed input exits non-zero.

## Example

For `SET alpha one`, `SET beta two`, `SET alpha three`, the output is:

```text
SET alpha three
SET beta two
```

## Edge cases

- Several values for one key collapse to the latest value.
- Output records are ordered by ascending key, independent of input order.
- Missing output parent directories are created.
- Existing output bytes are replaced rather than left after the new compacted log.

## Success criteria

All `deltaforge test` cases pass, replaying input and output yields equal live state, and `deltaforge bench` measures the supplied stale-log workload successfully.

## Non-goals

- Mutating the input log in place.
- Tombstone semantics, introduced in Stage 05.
- Atomic crash recovery, background compaction, or concurrent access.
