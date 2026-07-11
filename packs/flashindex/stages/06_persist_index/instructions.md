# Persist index

Add:

```bash
flashindex index <path> --out <index-file>
flashindex query <index-file> <token>
```

`index --out` should write the inverted index to disk. `query` should read that file and print matching relative paths for the requested token, one per line.

Edge cases:

- create the output file if it does not exist
- overwrite stale output safely
- query should return exit code 0 with empty output for missing tokens

Non-goals:

- binary file format
- memory mapping
- concurrent writes
