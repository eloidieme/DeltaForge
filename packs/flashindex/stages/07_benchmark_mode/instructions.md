# Benchmark mode

Add:

```bash
flashindex bench <path>
```

Print a compact JSON object with at least:

```json
{"files": 3, "runtime_ms": 1}
```

Edge cases:

- output must be valid JSON
- count only source-like files
- runtime must be non-negative

Non-goals:

- microbenchmark precision
- historical storage inside FlashIndex
- DeltaForge benchmark history integration
