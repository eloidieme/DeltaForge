# Stage 05 — Media types

## Goal

Add a `Content-Type` field to successful static-file responses so clients know how to interpret the bytes they receive.

## Background

MIME was created for email attachments and later adopted by HTTP as the vocabulary for representation formats. A browser should render HTML, display text, and treat unknown bytes cautiously; the filename suffix is only a practical hint, not proof. This stage uses a deliberately small mapping and a safe generic fallback, mirroring the table-driven classification found in real servers.

## Requirements

Extend `tinyhttp serve-file <root> <request-path>`. Every `200 OK` response must contain one of these exact fields:

- `.html` → `Content-Type: text/html`
- `.txt` → `Content-Type: text/plain`
- `.json` → `Content-Type: application/json`
- any other or missing extension → `Content-Type: application/octet-stream`

Keep Stage 02 status, length, body, and path-safety behavior unchanged. Match the listed extensions exactly and deterministically.

## Example

```text
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 12

{"ok":true}
```

## Edge cases

- HTML, plain-text, and JSON files receive their specified types.
- An unknown extension uses `application/octet-stream` rather than guessing.
- A file without an extension uses `application/octet-stream`.
- Missing files remain `404` responses and do not acquire a successful body type.

## Success criteria

All `deltaforge test` cases pass and adding media types does not weaken response framing or traversal protection.

## Non-goals

- A complete operating-system MIME database.
- Character-set parameters, content sniffing, or negotiation via `Accept`.
- Compression or transfer encodings.
