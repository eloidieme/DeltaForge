# Stage 02 — Static responses

## Goal

Translate a safe request path into either a complete `200 OK` response containing a file or a complete `404 Not Found` response, while keeping every lookup confined beneath a configured document root.

## Background

The earliest web servers mainly mapped URL paths to files. That simple mapping introduced a lasting security boundary: a client-controlled path must not escape the directory an operator chose to publish. Parent segments such as `..` powered classic directory-traversal bugs. Status lines and `Content-Length` also matter because HTTP peers use framing, not terminal display, to know where a response ends.

## Requirements

Expose:

```bash
tinyhttp serve-file <root> <request-path>
```

For a regular file beneath `<root>`, print an HTTP/1.1 response with status `200 OK`, a `Content-Length` equal to the UTF-8 body's byte length, a blank line, and the exact file body. For a missing path, exit 0 with `404 Not Found`, `Content-Length: 0`, and an empty body. Reject any request path containing a parent, absolute-root, or platform-prefix component; exit non-zero and never reveal the outside file's contents.

## Example

```text
HTTP/1.1 200 OK
Content-Length: 5

hello
```

## Edge cases

- An existing file returns `200` and its body.
- A missing file returns a well-formed empty `404` rather than a process error.
- A path containing `..` is rejected even if the target exists outside the root.
- `Content-Length` counts bytes, not characters or lines.

## Success criteria

All `deltaforge test` cases pass, response framing matches the body bytes, and no tested traversal exposes data outside the root.

### Reflection

1. Separate the three decisions in this stage: path safety, resource existence, and response formatting.
2. Why is a missing file an HTTP result while an unsafe path is a command failure in this contract?
3. Which assertion would catch a character-count implementation of `Content-Length`?

## Non-goals

- Directory listings, redirects, index-file discovery, or URL decoding.
- MIME classification, introduced in Stage 05.
- Sockets, concurrency, caching, or streaming large files.
