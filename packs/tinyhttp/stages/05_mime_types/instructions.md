# MIME types

Extend:

```bash
tinyhttp serve-file <root> <path>
```

Responses should include `Content-Type` for common static files:

- `.html` -> `text/html`
- `.txt` -> `text/plain`
- `.json` -> `application/json`
- anything else -> `application/octet-stream`

Non-goals:

- full MIME database
- compression
- content negotiation
