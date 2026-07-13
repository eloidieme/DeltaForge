# Stage 01 — Read a request line

## Goal

Read one valid HTTP request and print the three parts of its first line: method, request target, and protocol version.

TinyHTTP will not open a network socket yet. Beginning with protocol text lets us understand what a server receives before connection handling obscures it.

## Background

A browser asking for a page may begin its request with:

```http
GET /index.html HTTP/1.1
```

The line contains three fields.

`GET` is the **method**. It describes the kind of action the client requests.

`/index.html` is the **request target**. In this project it will later identify a file beneath a public directory.

`HTTP/1.1` is the **version**. The version matters because later connection rules differ between HTTP/1.0 and HTTP/1.1.

This first line is commonly called the request line or start line. Headers may follow it, then a blank line, then an optional body. This stage reads only the first line and reports its fields. The additional bytes remain present but do not change the three facts being requested.

HTTP began with an even smaller request format, but HTTP/1.x retained human-readable lines. That readability is helpful for learning and debugging. It should not tempt a parser to be vague: each field still needs an exact boundary.

The command accepts stdin for ordinary use and a file for deterministic benchmarking. Both forms represent the same request bytes and must produce the same parsed result.

## Requirements

Add both forms:

```console
tinyhttp parse
tinyhttp parse <request-file>
```

With no file argument, read the request from stdin. With a file, read its UTF-8 contents.

For a valid three-field first line, print exactly:

```text
method: <METHOD>
path: <TARGET>
version: <VERSION>
```

Each line ends with `\n`. Headers and body bytes after the first line do not affect this output.

## Example

```console
$ printf 'GET /index.html HTTP/1.1\r\n\r\n' | tinyhttp parse
method: GET
path: /index.html
version: HTTP/1.1
```

The command describes the request. It does not serve the file yet.

## Edge cases

- `GET` and `POST` are preserved as received rather than normalized.
- HTTP/1.0 and HTTP/1.1 version text is printed unchanged.
- A request read from a file is parsed by the same rules as stdin.
- Additional headers do not change the three output lines.

## Success criteria

All `deltaforge test` cases pass and the request-file benchmark completes with the same parsing behavior.

### Reading the benchmark

Record request bytes, header count, median, and p95. Then ask:

1. How much of a tiny parse is process startup and file I/O?
2. Does the command read only the first line or the complete fixture before parsing it?
3. Which larger fixture would still measure the same contract?
4. Which exact output check must remain unchanged after optimization?

## Non-goals

- Rejecting every malformed request-line shape; the next stage tightens that boundary.
- Validating the method, target, or version vocabulary.
- Parsing headers or the body.
- Opening a network listener.
