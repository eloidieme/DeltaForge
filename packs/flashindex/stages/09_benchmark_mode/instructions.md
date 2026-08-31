# Stage 09 — Describe a scan as data

## Goal

Add a command that reports the size and elapsed time of one corpus scan as a small JSON object.

DeltaForge can time any command from the outside. This stage teaches FlashIndex to describe the work it performed from inside the process as well.

## Background

Suppose two indexing runs finish in different amounts of time. Before comparing them, we need to know whether they performed the same work.

A run over two source files and a run over twenty thousand files are not competing measurements. Elapsed time without workload context can be technically correct and still misleading.

FlashIndex will therefore report two facts together:

```json
{"files": 37, "runtime_ms": 4}
```

`files` describes the selected corpus. `runtime_ms` describes how long this scan took inside the process.

The output is JSON because another program can parse it without relying on decoration intended for a person. That creates a strict stdout boundary: a helpful sentence before the object would make the stream invalid JSON.

Elapsed duration should come from a monotonic clock. A wall clock tells us the time of day and can jump when the system clock is corrected. A monotonic clock is designed to measure an interval that only moves forward.

One number is still only one observation. Filesystem caches, other processes, power settings, and storage devices all affect timing. This command makes measurement possible; it does not turn one run into a universal performance claim.

## Requirements

Add:

```console
flashindex bench <path>
```

Scan the searchable corpus and print exactly one valid JSON object followed by `\n`. The object must contain two non-negative integer fields:

```json
{"files": <N>, "runtime_ms": <N>}
```

`files` is the number of selected source-like files. `runtime_ms` is the elapsed scan duration measured with a monotonic clock. Do not print prose on stdout. An invalid or unreadable root must exit non-zero without printing a success-shaped object.

## Example

```console
$ flashindex bench project
{"files":3,"runtime_ms":0}
```

Zero milliseconds is valid for a very small or very fast workload; it does not mean that no work occurred.

## Edge cases

- Non-source assets do not contribute to `files`.
- An empty corpus reports `files` as zero.
- `runtime_ms` is always a non-negative integer.
- Stdout contains one JSON object and no surrounding explanation.
- An unreadable root fails instead of reporting zero files.

## Success criteria

All `deltaforge test` cases pass and the complete stdout stream parses as the required JSON object.

## Non-goals

- Replacing DeltaForge's repeated benchmark measurements.
- Claiming that one observed duration is stable across machines.
- Reporting CPU time, memory, or per-file timings.
- Optimizing the scanner merely to reduce this number.
