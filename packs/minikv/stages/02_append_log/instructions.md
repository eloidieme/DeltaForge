# Append-only log

Implement:

```bash
minikv write-log <path> <key> <value>
```

Append one line to `<path>`:

```txt
SET key value
```

Create parent directories when needed.
