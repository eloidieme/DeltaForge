# Stage 03 — Form a static-file response

## Goal

Turn an existing public text file into a complete `200 OK` response and a missing file into a complete empty `404 Not Found` response.

This command concentrates on response framing for normal root-relative paths.

## Background

Suppose a public directory contains:

```text
public/
└── index.html
```

If the client asks for `/index.html`, TinyHTTP needs to send more than the file text. The recipient must know whether the request succeeded and how many body bytes belong to this response.

A minimal successful response looks like:

```http
HTTP/1.1 200 OK
Content-Length: 15

hello tinyhttp
```

The first line is the status line. `200 OK` says the representation was found.

`Content-Length` describes the body in bytes. HTTP messages may travel over a connection that carries more than one response, so a recipient needs framing rather than relying on the connection closing at the right moment.

The blank line is part of that framing. It separates response metadata from body bytes.

A missing path is not a process failure in this project. The server understood the request and has an HTTP answer: `404 Not Found`. Its body is empty, so its content length is zero.

TinyHTTP initially reads UTF-8 text files. Media types and byte ranges are separate representation details.

## Requirements

Add:

```console
tinyhttp serve-file <root> <request-path>
```

For a regular UTF-8 file beneath `<root>`, print an HTTP/1.1 response containing `200 OK`, `Content-Length` equal to the body byte length, a blank line, and the exact file body.

For a missing path, exit 0 and print `404 Not Found`, `Content-Length: 0`, a blank line, and an empty body.

This contract covers safe request paths. Requests that attempt to escape the document root are outside this command's current guarantees.

## Example

```console
$ tinyhttp serve-file public /index.html
HTTP/1.1 200 OK
Content-Length: 15

hello tinyhttp
```

The visible blank line separates the header section from the file body.

## Edge cases

- An existing regular file returns status 200 and its exact body.
- `Content-Length` counts UTF-8 body bytes, not displayed characters or lines.
- A missing file returns status 404 with an empty body.
- Both responses contain the blank line required before the body.

## Success criteria

All `deltaforge test` cases pass and each response's framing agrees with the body bytes that follow it.

## Non-goals

- Defending against parent-directory traversal.
- MIME types.
- Binary response bodies.
- Network sockets, concurrency, or streaming.
