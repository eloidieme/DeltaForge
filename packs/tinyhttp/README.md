# TinyHTTP

Open a web page and, for a moment, ignore everything visual. Underneath the text, images, and buttons, a much plainer exchange is taking place. One program sends a short message asking for something; another program sends back a status, a few facts about the reply, and some bytes.

TinyHTTP is a small laboratory for that exchange. You will not build a production web server or open a network socket. Instead, you will work directly with HTTP-shaped messages and static files. Keeping the project this small makes the important rules visible: where one part of a message ends, how a path can escape its intended directory, why a length is measured in bytes, and how two protocol versions can give the same header different meaning.

## Begin with the shape of a message

Here is a request asking for `/guide.html`:

```text
GET /guide.html HTTP/1.1
Host: example.test
Connection: close

```

The first line has exactly three jobs. It names an action (`GET`), a request target (`/guide.html`), and a protocol version (`HTTP/1.1`). The following lines are header fields: small pieces of metadata written as a name, a colon, and a value. An empty line ends the header section. If a body follows, it belongs to a different region of the message.

Those boundaries matter. A line containing `Color: blue` is a header before the empty line and ordinary body text after it. A parser that keeps reading headers into the body has misunderstood the message, even if its output looks plausible on a simple example.

## What the project grows into

The stages follow one request through increasingly precise questions:

1. Read a well-formed request line and print its three fields.
2. Reject request lines that do not have exactly three fields.
3. Turn a file into a complete `200` response, or a missing file into `404`.
4. Keep request paths inside a chosen document root.
5. Read and normalize header fields.
6. Stop header parsing at the right boundaries and reject malformed fields.
7. Decide whether HTTP/1.0 and HTTP/1.1 connections stay open.
8. Label common file formats with a media type.
9. Return one valid inclusive range of file bytes.
10. Reject invalid ranges before producing a partial response.

Each step adds one idea. Later stages still rely on earlier behavior, but they no longer ask you to learn parsing, security, framing, and range arithmetic all at once.

## From a path to a response

Suppose the document root is a directory named `public`, and it contains `public/guide.html`. A request for `/guide.html` may safely refer to that file. A request for `/../private.txt` is different: `..` asks the filesystem to walk upward, outside the directory the server promised to expose.

This is not merely an awkward filename. It is a trust-boundary problem. The safe order is:

1. interpret and validate the request path;
2. decide which file, if any, is inside the document root;
3. read its bytes;
4. describe those bytes in the response.

A successful response has a status line, headers, an empty line, and then its body:

```text
HTTP/1.1 200 OK
Content-Length: 5

hello
```

`Content-Length` is 5 because the body contains five bytes. It is not the number of lines, characters as a reader imagines them, or cells shown by a terminal. Protocol framing works with bytes.

## Small rules with large consequences

HTTP/1.0 and HTTP/1.1 look almost identical in a request line, but they begin with opposite assumptions about connection reuse:

| Version | No `Connection` field | `Connection: close` | `Connection: keep-alive` |
|---|---:|---:|---:|
| HTTP/1.1 | keep open | close | keep open |
| HTTP/1.0 | close | close | keep open |

Media types are another kind of interpretation. This project deliberately supports only a small table:

| Filename ending | Media type |
|---|---|
| `.html` | `text/html` |
| `.txt` | `text/plain` |
| `.json` | `application/json` |
| anything else | `application/octet-stream` |

The fallback does not pretend to know more than the filename tells us. `application/octet-stream` simply means “these are arbitrary bytes.”

Finally, byte ranges allow a client to ask for part of a file. Their end position is inclusive. In the ten bytes `abcdefghij`, the range `2-5` is `cdef`, four bytes rather than three:

```text
length = end - start + 1
```

That extra `+ 1` is a small piece of arithmetic, but getting it wrong means the response headers and body disagree.

## A little history

The earliest HTTP exchanges were intentionally tiny. As the web grew, HTTP/1.0 added richer metadata and HTTP/1.1 standardized persistent connections and stricter message framing. Media types came from MIME, a system first developed so Internet email could describe attachments that were not plain text.

HTTP remains unusually readable for a network protocol. That readability is useful, but it can also make loose parsing feel safe. Many real security bugs have begun when two programs disagreed about a message boundary, a length, or the meaning of a path. TinyHTTP treats these details as the main subject rather than as cleanup around the edges.

## Words you will meet

- **Request target:** the path-like part after the method in a request line.
- **Header field:** a name and value carrying metadata about a message.
- **Message framing:** the rules that say where headers and bodies begin and end.
- **Document root:** the directory a static-file server has agreed to expose.
- **Media type:** a label describing how the response bytes should be interpreted.
- **Persistent connection:** a connection that may carry another request and response.
- **Partial content:** a response containing only a selected interval of a larger file.

## What a strong solution looks like

A strong solution keeps each boundary explicit. It rejects malformed input instead of guessing, validates a path before opening a file, formats status and header lines exactly, and calculates lengths from the bytes that will actually be written. Its parts are small enough that you can explain why each rule exists.

Once the core project is complete, natural extensions include `HEAD`, `416 Range Not Satisfiable`, open-ended ranges, ETags, and a minimal TCP listener. They are useful next steps, but each introduces new protocol rules. The aim here is first to make the small foundation trustworthy.
