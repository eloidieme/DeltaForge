# Stage 03 - Tokenize Files

## Goal

Extract identifier-like tokens from source files and report their positions.

## Requirements

Your program should expose:

```bash
flashindex tokenize <path>
```

For every token occurrence in source-like files, print:

```txt
relative/path:line:column token
```

Token rules:

- Tokens contain ASCII letters, ASCII digits, and underscores.
- Tokens may not start with a digit.
- Preserve underscores inside identifiers, such as `fetch_or`.
- Use 1-based line and column numbers.
- Print output in stable path order, then source order within each file.

## Success Criteria

`deltaforge test` should pass all Stage 03 tests.

## Non-goals

Do not implement a full parser. Do not remove comments or string literals yet. Do not support Unicode identifiers.
