# Stage 04 — Keep-alive decisions

## Goal

Decide whether an HTTP/1.x connection is reusable from the request version and `Connection` header, and report that decision as one exact boolean line.

## Background

Opening a TCP connection for every object made early web pages expensive. HTTP/1.0 introduced persistent connections by convention and HTTP/1.1 made persistence the default. The defaults differ: HTTP/1.1 stays open unless asked to close, whereas HTTP/1.0 closes unless explicitly kept alive. This tiny state table illustrates why protocol versioning changes semantics, not just labels.

## Requirements

Expose `tinyhttp keep-alive` and read one request from stdin. Print exactly `keep-alive: true\n` or `keep-alive: false\n`. For HTTP/1.1, return true unless a case-insensitive `Connection` field has the case-insensitive value `close`. For HTTP/1.0, return true only when that field's value is `keep-alive`. Header-name and token comparisons are ASCII case-insensitive. A malformed request line exits non-zero.

## Example

```console
$ printf 'GET / HTTP/1.1\r\nConnection: close\r\n\r\n' | tinyhttp keep-alive
keep-alive: false
```

## Edge cases

- HTTP/1.1 without a `Connection` field defaults to true.
- HTTP/1.1 with `Connection: close` returns false.
- HTTP/1.0 without an override defaults to false.
- HTTP/1.0 with `Connection: keep-alive` returns true.
- Header name and connection token casing do not affect the result.

## Success criteria

All `deltaforge test` cases pass and every supported version/header combination yields exactly one unambiguous line.

## Non-goals

- Serving multiple requests on a socket.
- Parsing comma-separated connection options or proxy hop-by-hop behavior.
- HTTP/2 or HTTP/3 connection management.
