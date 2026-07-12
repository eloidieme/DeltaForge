# TinyHTTP

## What you are building

TinyHTTP is a small HTTP/1.x request and response engine. By the end, you will parse request lines, normalize header fields, generate safe static-file responses, apply version-specific keep-alive defaults, classify common media types, and return correctly framed inclusive byte ranges.

## Why this is useful

HTTP looks simple until you implement it. This project teaches protocol parsing, defensive path handling, response framing, header normalization, version-dependent semantics, and byte-accurate slicing. Those skills transfer directly to servers, proxies, API gateways, and network tooling.

## Big picture

1. Separate the request line into its three protocol fields.
2. Map a safe request path to a complete static response.
3. Parse the header section without crossing into the body.
4. Apply HTTP/1.0 and HTTP/1.1 persistence rules.
5. Describe representation bytes with a media type.
6. Return an inclusive byte interval with correct lengths.

## Message anatomy

```text
GET /guide HTTP/1.1        ← request line
Host: example.test         ← header field
Connection: close          ← header field
                            ← empty-line delimiter
optional body bytes        ← message body
```

The regions have different grammars. A colon in the body does not make a header, and whitespace inside a target does not create a fourth request-line field. HTTP convention uses CRLF line endings; DeltaForge's committed file fixtures use LF checkout for cross-platform byte stability, while stdin tests still exercise CRLF requests.

## Protocol reference

### Keep-alive decision table

| Version | No `Connection` field | `Connection: close` | `Connection: keep-alive` |
|---|---|---|---|
| HTTP/1.1 | keep alive | close | keep alive |
| HTTP/1.0 | close | close | keep alive |

### Representation types

| Suffix | Media type |
|---|---|
| `.html` | `text/html` |
| `.txt` | `text/plain` |
| `.json` | `application/json` |
| other or absent | `application/octet-stream` |

### Inclusive byte ranges

For a valid interval `start..=end`:

```text
selected length = end - start + 1
valid bounds    = 0 <= start <= end < complete length
```

`Content-Range` describes both the selected interval and complete representation; `Content-Length` describes only the returned body.

## HTTP glossary

- **Request target:** the path/query portion following the method in the request line.
- **Field name:** the case-insensitive name before a header colon.
- **Message framing:** the rules that determine where a message or body ends.
- **Document root:** the directory boundary from which static content may be served.
- **Media type:** a label describing how representation bytes should be interpreted.
- **Persistent connection:** a connection eligible to carry another request/response exchange.
- **Partial content:** a response containing only a selected byte interval.

## Historical field note

The earliest web protocol at CERN was intentionally tiny. HTTP/1.0 accumulated metadata fields and optional persistent connections; HTTP/1.1 made persistence the default and formalized stronger framing rules. MIME media types came from Internet mail and were reused to describe web representations. The protocol's readable text is friendly to humans, but ambiguous parsing between components has also produced an enduring class of security failures.

## Failure-analysis lab

Diagnose the broken contract in each observation:

1. The parser accepts `GET / HTTP/1.1 EXTRA`. What ambiguity has entered the request-line grammar?
2. A body line `Role: admin` appears in header output. Which message boundary was crossed?
3. HTTP/1.0 without a `Connection` field reports `keep-alive: true`. Which version default was borrowed incorrectly?
4. Serving `/../secret.txt` returns the secret. Which trust boundary failed before file access?
5. Range `4 4` returns two bytes. Which inclusive-range equation is wrong?

## What good looks like

Good solutions reject malformed requests, validate paths before opening them, keep response formatting exact, count bytes rather than displayed characters, and treat protocol defaults explicitly. A robust explanation distinguishes syntax, security boundaries, and response semantics instead of calling every failure “parsing.”

## Optional extensions

Natural next questions include `HEAD`, `416 Range Not Satisfiable`, conditional requests with ETags, open-ended ranges, duplicate-field rules, and a minimal TCP listener. Each adds protocol surface, so consult the relevant HTTP specification before treating intuition as a contract.
