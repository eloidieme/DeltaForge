# Benchmark Format

Stage benchmarks live in optional `benchmarks.yaml` files.

```yaml
benchmarks:
  - name: scan_basic_project
    fixture: basic_project
    command: ["scan", "{fixture_path}"]
    iterations: 5
    warmup: 1
    timeout_ms: 5000
```

Run `deltaforge bench --save` to append results to `.deltaforge/benchmark_history.json`.

Iterations must be greater than zero. Each warmup and measured iteration starts from a fresh copy of the fixture, and any failed benchmark makes the command exit unsuccessfully while JSON mode keeps stdout machine-readable.

## Parameter matrix

A benchmark may declare an optional `matrix`: a map from parameter name to a non-empty list of scalar values (strings, numbers, or booleans).

```yaml
benchmarks:
  - name: scan_parallel
    fixture: basic_project
    command: ["scan", "{fixture_path}", "--threads", "{threads}"]
    matrix:
      threads: [1, 2, 4, 8]
```

The cartesian product of all parameters is measured — each point independently, with its own warmup, iterations, and fixture reset. `{name}` placeholders in command args are expanded per point, alongside the built-in `{fixture_path}` and `{temp_dir}`. Parameter names must be identifiers (`[A-Za-z_][A-Za-z0-9_]*`) and must not shadow the built-ins; `validate-pack` also rejects command placeholders that reference undeclared parameters. Benchmarks without a `matrix` behave exactly as before and produce a single point.

## Performance gates

Stages may declare progression requirements. A gate selects one exact current matrix point (or the derived `speedup` for a threads-only matrix) and has exactly one finite bound.

```yaml
performance_gates:
  - name: tokenizer throughput
    benchmark: scan_parallel
    metric: throughput_mb_s
    min: 150
    params: { threads: "8" }
    advice: ["excessive string allocation"]
```

`runtime_median_ms`, `runtime_p95_ms`, `throughput_mb_s`, `peak_memory_mb`, and `speedup` are supported. Non-speedup matrix gates must select every matrix key exactly; non-matrix and speedup gates use empty `params`. Speedup benchmarks use a `threads`-only matrix containing at least two distinct unsigned-integer values with no duplicates. A missing, failed, timed-out, ambiguous, or non-finite measurement is never a pass, and an incomplete stage run never reports an individual gate as passed. `bench` evaluates new measurements only and records a complete stage result in state; `bench --save` also writes history. Saved history never satisfies a gate.

## History file

`.deltaforge/benchmark_history.json` is versioned:

```json
{
  "schema_version": 2,
  "runs": [
    {
      "project": "flashindex",
      "language": "rust",
      "stage": "01_scan_files",
      "benchmark": "scan_basic_project",
      "timestamp": "2026-01-15T12:00:00Z",
      "git_commit": null,
      "command": ["./target/release/flashindex", "scan", "..."],
      "points": [
        {
          "params": {},
          "success": true,
          "iterations": 5,
          "warmup": 1,
          "runtime_median_ms": 12.5,
          "runtime_p95_ms": 15.2,
          "throughput_mb_s": 182.4,
          "peak_memory_mb": null,
          "error": null
        }
      ],
      "machine": { "arch": "x86_64", "os": "linux" }
    }
  ]
}
```

Each run carries one entry in `points` per measured configuration; benchmarks without a parameter matrix produce a single point with empty `params`. Legacy history files (a bare JSON array with a flat `results` object per record, written before the file was versioned) are converted automatically on read; saving rewrites the file in the current format. Files with a newer `schema_version` are rejected rather than silently misread.

## Peak memory

`peak_memory_mb` is a best-effort, approximate peak resident set size of the benchmarked process, taken as the maximum across a point's measured iterations. It is sampled from the runner's 10 ms poll loop: Linux reads the kernel high-water mark (`VmHWM` in `/proc/<pid>/status`), macOS samples resident size via `proc_pid_rusage`, and Windows reads `PeakWorkingSetSize`. It is `null` on other platforms, when every sample fails, or when the process exits before the first sample lands — a failed sample never fails the benchmark. Treat it as indicative (sampling granularity, OS accounting differences), not as an exact measurement.

## Comparing runs

Run a benchmark once with `deltaforge bench --save`, then use `deltaforge bench --compare` on a later run. Each newly measured point is paired with the most recent prior saved point having the same project, language, stage, benchmark name, and exact `params` map. This point-level lookup remains truthful if a benchmark matrix gains or loses configurations between runs.

The comparison reports changes in median runtime, throughput, and peak memory. Lower runtime and memory are improvements; higher throughput is an improvement. A missing measurement is reported as unavailable rather than inferred, and a point without a matching saved predecessor is reported explicitly. Differences in machine OS or architecture are called out because measurements from different machines may not be directly comparable.

`--compare --save` is supported: comparison reads history before the new results are appended, so a run never compares against itself. In JSON mode, each record receives a `comparison` object on stdout. Comparison results and derived speedups are display-time data and are never written to the versioned history file.
