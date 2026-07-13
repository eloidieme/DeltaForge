# Stage 08 — Describe the kind of file

## Goal

Add a `Content-Type` header to successful static-file responses.

## Background

A response body is only a sequence of bytes. The bytes do not announce whether they are a web page, a note, a picture, or something else. HTTP uses a media type to supply that missing context.

TinyHTTP will use the filename ending as a practical clue. This is not proof of a file's contents—a file can be named badly—but it is enough to learn the classification step. The supported list is intentionally small, and anything unknown receives an honest generic label instead of a guess.

The name MIME still appears around media types because the vocabulary began in Internet email, where messages needed a way to describe attachments. HTTP reused the same idea for web representations.

## Requirements

Extend `tinyhttp serve-file <root> <request-path>`. Every `200 OK` response must contain exactly one of these headers:

| Filename ending | Header |
|---|---|
| `.html` | `Content-Type: text/html` |
| `.txt` | `Content-Type: text/plain` |
| `.json` | `Content-Type: application/json` |
| unknown or absent | `Content-Type: application/octet-stream` |

Keep the existing status, length, body, and safe-path behavior unchanged. A `404` response must not claim the type suggested by the missing filename.

## Example

For a successful request for `data.json`, the response headers include:

```text
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 12
```

## Edge cases

- HTML, text, and JSON files receive their listed types.
- An unknown filename ending receives `application/octet-stream`.
- A filename with no extension also receives `application/octet-stream`.
- A missing `.json` file remains a `404` and does not receive the JSON type.

## Success criteria

All tests pass, and adding media types does not change response bodies, lengths, or path validation.

### Reflection

1. Why is `application/octet-stream` safer than guessing a familiar type?
2. What can a filename extension tell you, and what can it not prove?
3. Why should a missing `.json` path not produce `Content-Type: application/json`?

## Non-goals

- A complete operating-system media-type database.
- Character sets, content sniffing, or `Accept` negotiation.
- Compression or transfer encoding.
