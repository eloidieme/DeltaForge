# Stage 07 — Benchmark mode

## Goal

Expose a compact machine-readable snapshot of FlashIndex's current scan workload so external tooling can record and compare measurements.

## Background

Performance claims need measurements, but a benchmark is only useful when its output can be parsed reliably. JSON provides a small interoperability boundary; monotonic elapsed time avoids wall-clock adjustments. The number is still an observation, not a guarantee—warm caches, schedulers, and machine differences all introduce noise, which DeltaForge's benchmark history is designed to contextualize.

## Requirements

Expose `flashindex bench <path>`. Scan the Stage 02 corpus and print exactly one valid JSON object followed by `\n`, with integer fields `files` and `runtime_ms`. `files` is the number of source-like files; `runtime_ms` is a non-negative elapsed duration measured with a monotonic clock. Do not emit prose on stdout. Invalid roots exit non-zero without a success-shaped JSON object.

## Example

```json
{"files":2,"runtime_ms":1}
```

## Edge cases

- The output parses as one JSON value, not JSON mixed with labels.
- Non-source assets and ignored directories do not contribute to `files`.
- `runtime_ms` is an integer greater than or equal to zero.
- An empty corpus reports `files: 0` successfully.

## Success criteria

All `deltaforge test` cases pass and repeated invocations retain the same schema even though timing values may vary.

### Reflection

Explain the difference between a stable measurement schema and stable measurement values. Which claims can one `runtime_ms` observation support, and which require repeated controlled comparisons?

## Non-goals

- Microbenchmark precision or a promised runtime threshold.
- Saving history inside FlashIndex; DeltaForge owns that concern.
- Reporting per-file timings, memory, or CPU utilization.
