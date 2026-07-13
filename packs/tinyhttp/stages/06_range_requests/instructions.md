# Stage 09 — Return part of a file

## Goal

Add a `range` command that returns one valid, inclusive interval of file bytes.

## Background

Imagine downloading a large video and losing the connection near the end. Starting again from byte zero would waste most of the earlier transfer. A byte-range request lets a client ask for only the missing part. The same idea makes seeking through media possible.

HTTP writes a range with two endpoints, and both endpoints belong to the result. In the bytes `abcdefghij`, positions 2 through 5 are `cdef`:

```text
positions: 0 1 2 3 4 5 6 7 8 9
bytes:     a b c d e f g h i j
selected:      c d e f
```

That makes the length `end - start + 1`. The `+ 1` is easy to miss if you are accustomed to programming-language slices whose upper bound is excluded.

## Requirements

Expose this command:

```bash
tinyhttp range <root> <request-path> <start> <end>
```

For this stage, the offsets in the tests are valid decimal byte positions satisfying `0 <= start <= end < file length`. Print:

```text
HTTP/1.1 206 Partial Content
Content-Range: bytes <start>-<end>/<full-length>
Content-Length: <selected-length>

<selected bytes>
```

Preserve the document-root protection from the earlier static-file stages. Calculate lengths from bytes, not displayed characters.

## Example

For the ten-byte file `abcdefghij` and range `2 5`:

```text
HTTP/1.1 206 Partial Content
Content-Range: bytes 2-5/10
Content-Length: 4

cdef
```

## Edge cases

- A normal multi-byte interval includes both named endpoints.
- A range whose start and end are equal returns exactly one byte.

## Success criteria

All tests pass, `Content-Range` describes the whole file correctly, and `Content-Length` agrees with the bytes in the body on every operating system.

### Reflection

1. For a file of length 11, what are the first and last valid one-byte ranges?
2. Why is a range from 2 through 5 four bytes long?
3. Which response length describes the whole file, and which describes only the returned part?

## Non-goals

- Rejecting every malformed range; that boundary is the next stage.
- Parsing a real HTTP `Range` header.
- Suffix, open-ended, conditional, or multipart ranges.
