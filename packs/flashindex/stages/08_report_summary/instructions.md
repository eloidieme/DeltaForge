# Report summary

Add:

```bash
flashindex summary <path>
```

Print a human-readable summary with file count, token count, and unique token count.

Example:

```txt
files: 2
tokens: 10
unique_tokens: 8
```

Edge cases:

- count only source-like files
- token count includes repeated tokens
- unique token count de-duplicates by token text

Non-goals:

- Markdown output
- charts
- benchmark history
