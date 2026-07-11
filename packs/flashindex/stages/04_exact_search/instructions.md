# Exact token search

Add:

```bash
flashindex search <path> <token>
```

Search source-like files under `<path>` and print one matching token occurrence per line using the same `path:line:column token` format from tokenization.

Examples:

```txt
src/main.rs:1:4 main
src/lib.rs:2:8 main_index
```

Edge cases:

- match whole tokens only
- preserve stable relative paths
- continue to ignore generated, binary-looking, and dependency directories

Non-goals:

- substring search
- ranking
- regular expressions
