# Build inverted index

Add:

```bash
flashindex index <path>
```

Build an in-memory inverted index from tokens to source files and print one line per token:

```txt
token path1 path2
```

Paths should be relative, sorted, and de-duplicated per token.

Edge cases:

- repeated tokens in one file should list that file once
- tokens can appear in multiple files
- output should be stable between runs

Non-goals:

- persistence
- compression
- ranking
