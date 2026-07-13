# Stage 02 — Write the first log record

## Goal

Make one key/value pair survive after MiniKV exits by writing a `SET` record to a log file.

This stage handles a new or empty log. Preserving records already in the file will be the next step.

## Background

Stage 01 loses its pair as soon as the process ends. To recover information later, the program must leave evidence outside its own memory.

For the pair:

```text
language → Rust
```

MiniKV will write:

```text
SET language Rust
```

This line is a **record**. `SET` identifies the operation, `language` is the key, and the rest of the line is the value.

The file is called a **log** because it records operations in the order they occurred. Logs are used by databases, filesystems, and event-processing systems because a sequence of explicit events can later be replayed.

For now there is only one event. The important change is that a second process can open the file after the writer has gone away and still see what happened.

The requested path may include parent directories that do not exist yet. If a user asks for `data/store.log`, MiniKV should prepare `data` rather than requiring it to be created by hand.

Printing a short success marker gives the caller evidence that the command completed, while the real durable representation remains in the file.

## Requirements

Add:

```console
minikv write-log <path> <key> <value>
```

For a missing or empty destination, write exactly one UTF-8 record:

```text
SET <key> <value>\n
```

Create the file and any missing parent directories. Preserve spaces inside the value argument. On success, exit 0 and print a line containing `wrote`. Invalid arity must exit non-zero.

The next stage extends the same command to non-empty logs.

## Example

```console
$ minikv write-log data/store.log language Rust
wrote language
```

`data/store.log` now contains:

```text
SET language Rust
```

## Edge cases

- A missing log file is created.
- Missing parent directories are created.
- A value containing spaces remains intact after the key separator.
- Success output contains `wrote` only after the record has been written.

## Success criteria

All `deltaforge test` cases pass and a later process can read the complete record from the requested path.

## Non-goals

- Preserving existing records in a non-empty log; that is the next stage.
- Reading values back.
- Flush or `fsync` guarantees after power loss.
- Locking and concurrent writers.
