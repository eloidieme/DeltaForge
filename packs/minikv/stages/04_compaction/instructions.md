# Compact stale entries

Implement:

```bash
minikv compact <input-log> <output-log>
```

Write one `SET` line per key to the output log, keeping only the latest value.
