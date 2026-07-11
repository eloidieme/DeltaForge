# Log statistics

Add:

```bash
minikv stats <log-path>
```

Print a small line-oriented summary:

```txt
entries: 4
live_keys: 2
tombstones: 1
```

Edge cases:

- count both `SET` and `DEL` as entries
- `live_keys` should count keys with a latest live value
- `tombstones` should count `DEL` records

Non-goals:

- histograms
- JSON output
- file-size accounting
