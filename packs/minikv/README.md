# MiniKV

## What you are building

MiniKV is a tiny persistent key-value store. You begin with the command boundary for one key/value pair, then make updates durable in an append-only log, recover current state after restart, compact obsolete history, represent deletion with tombstones, and report the difference between physical records and live data.

## Why this is useful

Key-value stores are the foundation of caches, databases, metadata services, queues, and embedded storage engines. This pack introduces their core storage ideas without hiding them behind a framework. Its small text format keeps the evidence visible: you can open the log and see exactly which history recovery must interpret.

## Big picture

1. Establish an exact command and output contract.
2. Append durable `SET` history without rewriting older bytes.
3. Replay that history so the latest operation wins.
4. Compact obsolete history into an equivalent live set.
5. Add `DEL` tombstones without resurrecting older values.
6. Compare physical log records with logical live state.

## Concept map

| Stage | New idea | Invariant to protect |
|---|---|---|
| 01 | Key/value command boundary | Successful output represents exactly the supplied pair. |
| 02 | Append-only persistence | A new record never destroys an earlier record. |
| 03 | Recovery by replay | The latest valid operation determines a key's state. |
| 04 | Compaction | Replaying input and output yields equivalent live state. |
| 05 | Tombstones | A deleted key stays absent until a later `SET`. |
| 06 | Operational statistics | Physical record counts and logical key counts are not confused. |

## Storage glossary

- **Append-only log:** a sequence extended at the end rather than edited in place.
- **Replay:** applying records in chronological order to reconstruct current state.
- **Live value:** the value selected by the latest operation for a key.
- **Stale record:** an older operation that no longer affects current state.
- **Tombstone:** a durable deletion marker that supersedes an older value.
- **Compaction:** producing a smaller representation with the same logical state.
- **Crash consistency:** the property that interruption leaves storage in a recoverable state.
- **Durability:** the promise that acknowledged data survives the failures included in the system's model.

### A durability ladder

“The write call succeeded” is not the strongest possible durability claim. Data may have reached a language buffer, the operating-system page cache, the device cache, or stable media. MiniKV deliberately stops before specifying flush and `fsync` behavior, but the distinction matters: append-only layout simplifies recovery; it does not make every completed call power-loss durable by itself.

## Historical field note

Database write-ahead logs, event-sourced applications, and log-structured stores all exploit sequential history. Bitcask is a particularly approachable relative of MiniKV: it uses an append-only data file and an in-memory key directory, then merges old files to reclaim space. Production systems add record framing, checksums, synchronization, locking, and careful replacement protocols; this pack isolates the semantic core those mechanisms protect.

## Failure-analysis lab

For each observation, name the violated invariant before imagining a code change:

1. After `SET colour blue` and `SET colour green`, `get colour` prints `blue`. Is the defect in append order or replay order?
2. A compacted file contains both values for `colour`. Is the state wrong, or only the purpose of compaction unmet?
3. `SET session live`, `DEL session`, then compaction produces `SET session live`. Which record was incorrectly forgotten?
4. A log with five records for one key reports five live keys. Which physical and logical quantities were confused?
5. An output log ends with correct records followed by bytes from an older, longer output. What replacement property failed?

## What good looks like

Good solutions make file writes explicit, reject malformed history instead of guessing, keep output stable, and treat recovery as a first-class behavior rather than a side effect. The strongest explanations state the replay invariant first and use it to justify reads, deletes, statistics, and compaction.

## Optional extensions

After completing the required stages, useful thought experiments include length-prefixed records, checksums, atomic output replacement, snapshots, batch writes, and single-writer locking. Each extension should begin by naming the new failure it addresses; complexity without a failure model is merely decoration.
