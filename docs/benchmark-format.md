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

Each run carries one entry in `points` per measured configuration; benchmarks without a parameter matrix produce a single point with empty `params`. `peak_memory_mb` is reserved for best-effort peak-memory capture and is `null` until measured. Legacy history files (a bare JSON array with a flat `results` object per record, written before the file was versioned) are converted automatically on read; saving rewrites the file in the current format. Files with a newer `schema_version` are rejected rather than silently misread.
