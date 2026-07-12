# Stage 06 — Log statistics

## Goal

Summarize both the physical history and the logical live state of a MiniKV log, making the cost of append-only storage visible before and after compaction.

## Background

Operational tools often expose two views of storage: how many records occupy the medium and how much useful state those records represent. Their difference is a simple measure of write amplification and compaction opportunity. Database administrators have relied on such counters since early file-oriented systems; good metrics are deliberately boring, stable, and unambiguous.

## Requirements

Expose:

```bash
minikv stats <log-path>
```

Parse the same `SET` and `DEL` records as recovery. Print exactly three labelled lines in this order: `entries: <N>`, `live_keys: <N>`, and `tombstones: <N>`. `entries` counts every non-empty valid record, `live_keys` counts keys whose latest operation is `SET`, and `tombstones` counts every `DEL` record, including repeated deletes. Malformed records exit non-zero.

## Example

```text
entries: 4
live_keys: 2
tombstones: 1
```

## Edge cases

- Both `SET` and `DEL` contribute to `entries`.
- Repeated operations for one key count as entries but only the latest state affects `live_keys`.
- Every `DEL` record contributes to `tombstones`, even when the key was already absent.
- A log containing only blank lines reports zero for all three counters.

## Success criteria

All `deltaforge test` cases pass and the three counters remain internally consistent with Stage 05 replay semantics.

## Non-goals

- File-size, latency, or per-key histograms.
- JSON, Prometheus, or interactive output.
- Modifying or compacting the measured log.
