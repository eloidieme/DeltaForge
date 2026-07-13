# Stage 10 — Reject ranges that cannot be served

## Goal

Make `tinyhttp range` reject malformed, reversed, out-of-bounds, and unsafe requests before it prints a partial response.

## Background

The previous stage described the happy path. A real boundary also has to say what happens when the requested interval makes no sense.

Suppose a file has 11 bytes. Its valid positions are 0 through 10. Position 11 is already one byte past the end. A range `7 2` runs backward, while `0 99` reaches beyond the file. Neither should be “fixed” by silently swapping or shortening its endpoints: that would return different bytes from the ones the caller requested.

There is also an ordering question. A path such as `/../secret.txt` must be rejected as unsafe before the program reads bytes outside the document root. Validation is not only about producing a helpful error; it protects which data the command is allowed to see.

## Requirements

Keep the successful `range` behavior from Stage 09. Before slicing or printing a response:

1. reject a request path that escapes the document root;
2. parse both offsets as decimal numbers;
3. require `start <= end`;
4. require `end < file length`.

On any failure, exit non-zero and write an explanatory message to standard error. Invalid numeric intervals must include `invalid range`; unsafe paths must include `unsafe path`. Do not print a misleading `206 Partial Content` response on failure.

## Example

For an 11-byte file, this request fails because position 99 does not exist:

```console
$ tinyhttp range public /letters.txt 0 99
invalid range
```

## Edge cases

- A start greater than the end is rejected.
- An end beyond the last byte is rejected.
- A non-numeric offset is rejected.
- A parent-directory traversal is rejected before outside bytes are read.

## Success criteria

All tests pass, every invalid request exits non-zero, and no failure emits a successful partial response.

### Reflection

1. Why is `end == file length` invalid for an inclusive range?
2. Why should the program reject rather than repair a reversed range?
3. Which checks can happen before reading the file, and which require knowing its length?

## Non-goals

- Producing an HTTP `416 Range Not Satisfiable` response.
- Supporting suffix or open-ended ranges.
- Streaming, sockets, caching, or content encoding.
