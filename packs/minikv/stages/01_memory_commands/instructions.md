# Stage 01 — In-memory commands

## Goal

Accept one key and one value, keep that pair for the lifetime of the command, and echo it in a stable textual form. This deliberately small first stage establishes MiniKV's command-line contract before files, recovery, and compaction make the storage model more demanding.

## Background

A key-value store presents data as an associative map: a key names a value, much as a dictionary headword names its definition. The idea predates modern databases—early symbol tables and indexed files used the same lookup model—and remains the core abstraction behind caches and systems such as Redis. Here the process is intentionally short-lived. The lesson is the boundary between command arguments and observable output, not persistence yet.

## Requirements

Expose this command:

```bash
minikv memory <key> <value>
```

`<key>` and `<value>` are each one command-line argument. On success, print exactly one line in the form `<key>=<value>` and exit with status 0. Preserve the argument text, including spaces that the caller placed inside a quoted value. If either argument is absent, exit non-zero; do not print a plausible key/value result.

## Example

```console
$ minikv memory greeting "hello world"
greeting=hello world
```

## Edge cases

- A value containing spaces remains one value and is printed unchanged.
- A missing value is invalid and exits non-zero.
- Each invocation prints only the pair supplied to that invocation; no state survives into another process.

## Success criteria

All `deltaforge test` cases pass, successful output is byte-stable, and invalid arity cannot be mistaken for a stored value.

## Non-goals

- Persisting values after the process exits.
- Supporting multiple operations in one invocation.
- Deletes, recovery, compaction, or concurrency.
