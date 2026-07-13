# Stage 07 — Decide whether the connection stays open

## Goal

Add `tinyhttp keep-alive`. It reads one request and prints whether its HTTP connection may be reused.

## Background

A web page rarely needs only one file. It may need HTML, a stylesheet, several images, and some JavaScript. Opening a new network connection for every file costs time, so HTTP learned to reuse a connection for more than one request.

The awkward part is history. HTTP/1.0 assumes the connection will close unless the request explicitly asks for `keep-alive`. HTTP/1.1 reverses that default: the connection stays open unless the request asks to `close` it. The version is therefore part of the decision, not just a label to print.

It helps to write the rule as a table before writing code:

| Version | Header absent | `close` | `keep-alive` |
|---|---:|---:|---:|
| HTTP/1.1 | true | false | true |
| HTTP/1.0 | false | false | true |

## Requirements

Read one request from standard input. Print exactly one of these lines:

```text
keep-alive: true
keep-alive: false
```

Find a `Connection` header without caring about ASCII letter case. Compare its value with `close` and `keep-alive` the same way. Apply the defaults in the table when the header is absent. A malformed request line must exit with a non-zero status.

## Example

```console
$ printf 'GET / HTTP/1.1\r\nConnection: close\r\n\r\n' | tinyhttp keep-alive
keep-alive: false
```

The same request without the `Connection` line would print `true`, because it is HTTP/1.1.

## Edge cases

- HTTP/1.1 without a `Connection` field returns `true`.
- HTTP/1.1 with `Connection: close` returns `false`.
- HTTP/1.0 without a `Connection` field returns `false`.
- HTTP/1.0 with `Connection: keep-alive` returns `true`.
- Mixed capitalization in the field name or value does not change the answer.

## Success criteria

All tests pass, and each supported version/header combination produces exactly one boolean line.

### Reflection

1. Why can the absence of a header carry meaning?
2. Which result changes if HTTP/1.0 accidentally uses the HTTP/1.1 default?
3. Why should the decision code receive parsed values rather than scan the raw request itself?

## Non-goals

- Actually serving a second request on a socket.
- Parsing a comma-separated list of connection options.
- HTTP/2 or HTTP/3 connection management.
