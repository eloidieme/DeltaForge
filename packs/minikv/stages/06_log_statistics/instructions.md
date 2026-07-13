# Stage 10 — Count history and live state

## Goal

Report how many physical operation records the log contains, how many keys are currently live, and how many deletion tombstones were written.

The command makes the cost of append-only history visible without changing the log.

## Background

Consider:

```text
SET alpha one
SET alpha two
SET beta three
DEL beta
SET gamma four
```

Five records occupy the file. Only `alpha` and `gamma` are currently live. One tombstone was written.

These are different views of the same history:

```text
entries: 5
live_keys: 2
tombstones: 1
```

`entries` is a physical count. Every valid `SET` and `DEL` contributes, including stale or repeated operations.

`live_keys` is a logical count. It depends only on the latest operation for each key.

`tombstones` counts deletion activity, not currently absent keys. Two `DEL` records for the same key contribute two tombstones even though they describe one absent key.

Putting these numbers together helps a learner see why compaction exists. A large gap between entries and live keys suggests accumulated history. It does not automatically prove that compaction should run now; file size, workload, and operational cost would matter in a production policy.

## Requirements

Add:

```console
minikv stats <log-path>
```

Parse the same valid `SET` and `DEL` records as recovery. Print exactly:

```text
entries: <N>
live_keys: <N>
tombstones: <N>
```

in that order. `entries` counts every non-empty valid record. `live_keys` counts keys whose latest operation is `SET`. `tombstones` counts every `DEL` record. Malformed records exit non-zero.

## Example

For the five-record log above:

```console
$ minikv stats store.log
entries: 5
live_keys: 2
tombstones: 1
```

## Edge cases

- Repeated operations increase `entries` while only the latest affects `live_keys`.
- Every `DEL` increases `tombstones`, including repeated deletion of one key.
- A later `SET` restores a live key without reducing the historical tombstone count.
- A blank log reports zero for all counters.
- Malformed non-empty records fail instead of being omitted from statistics.

## Success criteria

All `deltaforge test` cases pass and each counter agrees with Stage 09 replay semantics for the same bytes.

## Non-goals

- Reporting file size, latency, or per-key statistics.
- JSON or interactive output.
- Automatically compacting the measured log.
- Choosing a production compaction threshold.
