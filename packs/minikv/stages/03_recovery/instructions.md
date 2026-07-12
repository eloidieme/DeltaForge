# Stage 03 — Recovery

## Goal

Recover the current value of a key from an append-only log, proving that durable history can be replayed into useful state after the original writer has exited.

## Background

Log replay is a foundational recovery technique. A database starts from a known point and applies records in order; later operations supersede earlier ones, so a crash does not require the in-memory map to survive. The same pattern appears in database redo logs and replicated state machines. Correctness depends on chronology: the last valid operation for a key is authoritative.

## Requirements

Expose:

```bash
minikv get <log-path> <key>
```

Read the UTF-8 log from beginning to end. A valid record has the form `SET <key> <value>`, where the value is the remainder after the second separator and may contain spaces. Print the latest value for the requested key followed by `\n`. If the key never appears, exit 0 with empty stdout. A non-empty malformed record is an error and must exit non-zero with a useful message on stderr.

## Example

Given:

```text
SET colour blue
SET size large
SET colour green
```

```console
$ minikv get store.log colour
green
```

## Edge cases

- Later `SET` records override earlier values for the same key.
- Another key's records do not affect the requested key.
- A missing key produces empty stdout and status 0.
- A non-empty malformed log line causes a non-zero exit.

## Success criteria

All `deltaforge test` cases pass and replay produces the same answer every time for unchanged log bytes.

## Non-goals

- Repairing or silently skipping malformed records.
- Deletes or tombstones; they arrive in Stage 05.
- An index file, random-access lookup, or partial replay.
