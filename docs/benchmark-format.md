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
