# MiniKV

## What you are building

MiniKV is a tiny persistent key-value store. By the end, you will have a command-line storage engine that writes updates to an append-only log, recovers the latest values after restart, and compacts stale entries.

## Why this is useful

Key-value stores are the foundation of caches, databases, metadata services, queues, and embedded storage engines. This pack introduces the core storage ideas without hiding them behind a framework.

## Big picture

The project moves from simple command behavior to durable storage:

1. Parse in-memory set/get-style commands.
2. Append updates to a log.
3. Recover the latest value for a key.
4. Compact old log entries into a smaller live set.
5. Add delete tombstones and make recovery respect them.
6. Report log statistics for entries, live keys, and tombstones.

## What good looks like

Good solutions make file writes explicit, handle missing keys cleanly, keep output stable, and treat recovery as a first-class behavior rather than a side effect.
