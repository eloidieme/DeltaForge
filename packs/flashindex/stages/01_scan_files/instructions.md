# Stage 01 - Scan Files

## Goal

Recursively find files under a directory and print their relative paths.

## Requirements

Your program should expose:

```bash
flashindex scan <path>
```

For every regular file under `<path>`, print one relative path per line:

```txt
src/main.rs
README.md
```

Skip these directories wherever they appear:

```txt
.git
target
build
node_modules
```

Use forward slashes in output, even on platforms whose native separator differs.

## Success Criteria

`deltaforge test` should pass all Stage 01 tests.

## Non-goals

Do not filter by file extension yet. Stage 01 should print all regular files except files inside ignored directories.
