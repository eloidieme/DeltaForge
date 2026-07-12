# Stage 03 — Header fields

## Goal

Parse the header section of one HTTP request and print normalized field names with trimmed values, stopping exactly at the blank line that terminates the section.

## Background

HTTP header names are case-insensitive, a legacy of the protocol's email-like syntax, while values usually retain their case. Normalizing names gives later logic one spelling to compare. The empty line is not decoration: it is the delimiter between metadata and an optional body. Confusing body text for headers is both a correctness bug and a recurring source of request-smuggling vulnerabilities.

## Requirements

Expose `tinyhttp headers`. Read one request from stdin, skip its request line, then process each non-empty header line up to the first blank line. Each header must contain a colon. Print the field name in ASCII lowercase, followed by `: ` and the value with surrounding whitespace removed. Preserve input order and value case. Malformed header lines exit non-zero.

## Example

```console
$ printf 'GET / HTTP/1.1\r\nHost: Example.test\r\n\r\n' | tinyhttp headers
host: Example.test
```

## Edge cases

- Header names with mixed case are normalized to lowercase.
- Optional whitespace around a value is trimmed.
- The request line is never printed as a header.
- Parsing stops at the first blank line; body text containing a colon is ignored.
- A non-empty header line without `:` is invalid.

## Success criteria

All `deltaforge test` cases pass and the printed header stream is stable enough for Stage 04 to consume conceptually.

### Reflection

Explain why lowercasing the entire header line would be wrong even though field names are case-insensitive. Which blank line determines when normalization must stop?

## Non-goals

- Combining duplicate fields or parsing comma-separated field grammars.
- Obsolete folded headers, trailers, or body framing.
- Enforcing a registry of known header names.
