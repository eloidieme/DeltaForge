# Config

Project config lives at `.deltaforge/config.toml`.

```toml
schema_version = 1

[runner]
timeout_ms = 5000
build_timeout_ms = 120000
keep_temp = false

[bench]
iterations = 7
warmup = 2

[git]
auto_commit = false
auto_tag = true
```

Run `deltaforge config validate` after manual edits.
