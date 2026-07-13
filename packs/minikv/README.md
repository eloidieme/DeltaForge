# MiniKV

## What you are building

Consider a program that needs to remember three small facts:

```text
language → Rust
theme    → dark
timeout  → 30
```

While the program is running, it can keep these pairs in memory. The problem appears when the process exits. Its memory is released, and the next process begins without the values.

MiniKV will become a small persistent key-value store: keys name values, writes survive a restart, and later commands can recover the current state.

Before it can do that, it needs to answer several questions.

First: what should a durable write look like?

MiniKV will record each change as a line in a file:

```text
SET language Rust
SET theme dark
SET timeout 30
```

Instead of finding an old record and editing it, every write is added to the end. This creates an append-only log: a chronological history of operations.

Second: what happens when a key changes?

```text
SET theme dark
SET theme light
```

Both events remain in the file, but only the latest one determines the current value. Recovery reads the history in order and reconstructs the state a previous process left behind.

Third: what should happen to history that can no longer affect a read?

As updates accumulate, the log grows. MiniKV will compact it into one current record per live key. Compaction must preserve the result of replay, not merely make the file shorter.

Deletion adds another difficulty. Removing a key from memory is not durable because an earlier `SET` remains in the log. MiniKV records a `DEL` tombstone so recovery knows that the old value is no longer live. Compaction must respect that marker or a deleted key can reappear.

Finally, MiniKV will count both sides of the storage model: physical records in the file and logical keys still alive. The difference makes the cost of retained history visible.

## Why this is useful

The log is plain text and the commands are small, but the questions are the same ones larger storage engines must answer.

What evidence survives a process restart? Which operation wins when several records mention one key? When is old history safe to remove? How should damaged history fail? What does deletion mean when old copies still exist?

These ideas appear in database recovery logs, event-sourced applications, message systems, filesystems, and log-structured stores. Larger systems add checksums, record framing, synchronization, atomic replacement, and concurrency control. MiniKV keeps the state model visible by leaving those mechanisms outside its boundary.

The project also shows the difference between layout and guarantee. Appending is a useful storage layout, but it does not automatically promise survival after a power failure. Replacing an output file is different from making that replacement crash-atomic. MiniKV names only the guarantees it actually provides.

## Big picture

```text
Accept one key and one value
    ↓
Write the first durable SET record
    ↓
Append later records without destroying history
    ↓
Replay valid history so the latest operation wins
    ↓
Reject history that cannot be interpreted safely
    ↓
Compact stale values into an equivalent live state
    ↓
Replace the compacted artifact completely
    ↓
Record and recover DEL tombstones
    ↓
Compact without resurrecting deleted keys
    ↓
Compare physical history with logical live state
```

The arrows are dependencies rather than a list of unrelated commands. Recovery depends on complete append history; compaction depends on recovery semantics; tombstones change both replay and compaction; statistics must interpret the same records as every other command.

## Storage terms, when you need them

- **Key:** the name used to retrieve a value.
- **Record:** one complete operation stored in the log.
- **Append-only log:** a history extended at the end rather than edited in place.
- **Replay:** applying records in chronological order to rebuild current state.
- **Stale record:** an older operation that no longer changes the recovered result.
- **Compaction:** producing a smaller representation with the same logical state.
- **Tombstone:** a durable deletion marker that supersedes an older value.
- **Durability:** the promise that acknowledged data survives the failures named by the system's model.

## A note about durable writes

When a file-writing function succeeds, the bytes may have reached the language runtime, the operating-system page cache, the storage device's cache, or stable media. Those are not identical guarantees.

MiniKV does not require `fsync` or define recovery after power loss. Its append-only history makes the logic of recovery easy to inspect, but its durability claim stops when the file-writing operation succeeds.

## When something goes wrong

Before changing code, identify which promise failed:

1. `SET colour blue` followed by `SET colour green` recovers `blue`. Was the error in append order or replay order?
2. A compacted file contains both values. Is the live state wrong, or did compaction fail to remove stale history?
3. `SET session active`, then `DEL session`, then compaction produces `SET session active`. Which operation was forgotten?
4. Five records for one key are reported as five live keys. Which physical and logical quantities were confused?
5. A shorter compacted result is followed by lines from yesterday's output. Did state selection fail, or did destination replacement fail?

Naming the broken promise usually narrows the problem more effectively than starting from a suspected function.

## What good looks like

A good MiniKV solution has one shared interpretation of log records. Writes create complete operations, recovery applies them in order, statistics count the same operations, and compaction serializes the same final state.

Malformed history fails visibly instead of being repaired by guesswork. Output remains deterministic. Claims about durability and safe replacement stop at the guarantees described in the command contract.

## Optional directions

From this foundation, useful experiments include length-prefixed records, checksums, crash-atomic replacement through a temporary sibling, snapshots, batch writes, and single-writer locking.

Begin any extension by naming the failure it addresses. A checksum helps detect corrupted records; it does not make a write atomic. A lock coordinates writers; it does not make buffered bytes survive power loss. Keeping those distinctions clear is part of storage design.
