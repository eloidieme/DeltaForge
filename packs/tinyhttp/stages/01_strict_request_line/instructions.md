# Stage 02 — Reject ambiguous request lines

## Goal

Make request-line parsing fail when the first line is empty or does not contain exactly three whitespace-separated fields.

The valid parser has one defined input shape. Strict parsing makes every other shape visible instead of guessing what the client meant.

## Background

Compare these lines:

```text
GET / HTTP/1.1
BROKEN
GET / HTTP/1.1 EXTRA
```

Only the first line has the shape TinyHTTP understands.

The one-word line does not identify separate method, target, and version fields. The four-word line leaves an extra field with no defined meaning. A parser could accept the first three words and ignore the fourth, but another HTTP component might interpret the bytes differently.

Disagreement between protocol components is more than untidy input handling. In real HTTP systems, ambiguous message boundaries and inconsistent parsing have contributed to request-smuggling vulnerabilities. TinyHTTP handles only a small part of the protocol, but it can still establish the right habit: reject a shape the contract does not define.

An empty input is slightly different from a malformed non-empty line. It contains no request line at all. Both cases fail, but a useful diagnostic can distinguish `missing request line` from `malformed request line`.

Failure output belongs on stderr. Stdout is reserved for the three-field description of a valid request, so callers never mistake a partial parse for success.

## Requirements

Keep both `tinyhttp parse` forms.

The first line must contain exactly three non-empty whitespace-separated fields. Reject fewer or more fields with a non-zero exit and stderr containing `malformed request line`. Reject empty input with a non-zero exit and stderr containing `missing request line`.

Do not print a success-shaped `method`, `path`, or `version` report after failure. Valid requests retain the existing three-line output.

## Example

```console
$ printf 'GET / HTTP/1.1 EXTRA\r\n\r\n' | tinyhttp parse
error: malformed request line
```

The exact surrounding diagnostic may vary, but the process fails and identifies the line as malformed.

## Edge cases

- Empty input is a missing request line.
- A line with one or two fields is malformed.
- A line with more than three fields is malformed.
- Failure does not emit a partial success report on stdout.

## Success criteria

All `deltaforge test` cases pass and every accepted request produces exactly three defined fields.

## Non-goals

- Restricting methods to a known list.
- Validating URI syntax or supported HTTP versions.
- Limiting line length.
- Parsing headers or message bodies.
