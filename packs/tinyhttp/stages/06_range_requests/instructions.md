# Stage 06 — Byte-range responses

## Goal

Return one inclusive slice of a file as a correctly framed `206 Partial Content` response, while preserving the document-root security boundary.

## Background

Range requests let a client resume downloads and seek through large media without transferring the whole representation. HTTP defines ranges in bytes, and the end offset is inclusive—an easy off-by-one trap. A response must identify both the selected interval and the complete representation length so a client can place the fragment correctly.

## Requirements

Expose:

```bash
tinyhttp range <root> <request-path> <start> <end>
```

`start` and `end` are decimal byte offsets. For `0 <= start <= end < file_length`, print status `HTTP/1.1 206 Partial Content`, `Content-Range: bytes <start>-<end>/<file_length>`, `Content-Length: <end-start+1>`, a blank line, and exactly those inclusive bytes. Invalid numbers, reversed or out-of-bounds ranges, missing files, and unsafe paths exit non-zero with an explanatory stderr message.

## Example

For the 10-byte file `abcdefghij` and range `2 5`:

```text
HTTP/1.1 206 Partial Content
Content-Range: bytes 2-5/10
Content-Length: 4

cdef
```

## Edge cases

- A one-byte range where `start == end` succeeds with length 1.
- A reversed range is rejected.
- An end offset equal to or beyond the file length is rejected.
- Non-numeric offsets are rejected.
- A parent-directory traversal is rejected before outside bytes are read.

## Success criteria

All `deltaforge test` cases pass, byte counts remain correct with LF fixtures on every CI operating system, and range failures never emit a misleading partial response.

### Reflection

1. For a file of length 11, list the first and last valid one-byte ranges.
2. Explain why `end == file_length` is invalid even though an exclusive slice often permits that endpoint.
3. Which validation must happen before any outside path could be opened?
4. What additional response semantics would be needed if this CLI modeled HTTP's `416` response instead of treating invalid ranges as command errors?

## Non-goals

- Parsing an HTTP `Range` header or returning `416 Range Not Satisfiable`.
- Suffix, open-ended, conditional, or multipart ranges.
- Streaming, sockets, caching, or content encoding.
