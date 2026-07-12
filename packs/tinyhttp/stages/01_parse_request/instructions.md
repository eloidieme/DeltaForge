# Stage 01 — Parse an HTTP request line

## Goal

Read one HTTP request and expose its method, request target, and protocol version as three deterministic lines. This is the narrow front door of TinyHTTP: turn protocol text into structured facts without attempting to serve a network connection.

## Background

HTTP began as a remarkably small text protocol at CERN; HTTP/0.9 requests were essentially a method and path. HTTP/1.x added a version and headers but kept the start line human-readable. Parsing that line is still security-sensitive: accepting ambiguous shapes can make two components disagree about where a request begins or ends. This stage therefore defines a strict three-field boundary.

## Requirements

Support both forms:

```bash
tinyhttp parse
tinyhttp parse <request-file>
```

With no file, read the request from stdin. With a file, read its UTF-8 bytes; this form exists so `deltaforge bench` can measure a committed fixture without a shell. The first line must contain exactly three whitespace-separated fields. Print `method: <METHOD>`, `path: <TARGET>`, and `version: <VERSION>`, in that order, each followed by `\n`. Additional headers and body bytes do not change this output. A missing or malformed request line exits non-zero.

## Example

```console
$ printf 'GET /index.html HTTP/1.1\r\n\r\n' | tinyhttp parse
method: GET
path: /index.html
version: HTTP/1.1
```

## Edge cases

- Methods and versions are reported exactly as received rather than rewritten.
- Header lines after the request line do not become request-line fields.
- An empty input is invalid.
- A first line with fewer or more than three fields is invalid.

## Success criteria

All `deltaforge test` cases pass, stdin and file forms agree for equal bytes, and `deltaforge bench` completes against the large request fixture.

## Non-goals

- Validating the complete HTTP method or URI grammar.
- Parsing headers, bodies, chunked framing, or TCP streams.
- Running a listening server.
