# Stage 06 — Respect the header boundary

## Goal

Stop header parsing at the first blank line and reject non-empty header lines that contain no colon.

Field normalization applies only inside the header section. Boundary handling keeps body text out and prevents guesses around malformed metadata.

## Background

An HTTP message separates headers from an optional body with an empty line:

```http
POST /submit HTTP/1.1
Host: example.test

Body-Key: this is body text
```

`Body-Key: this is body text` happens to look like a header, but it appears after the boundary. It belongs to the body and must not be printed by the `headers` command.

The blank line is therefore not visual spacing. It is protocol framing.

Now consider a line before the boundary:

```text
Broken Header
```

Without a colon, it does not have the field-name/value shape TinyHTTP understands. Silently skipping it would hide malformed request metadata. Treating the entire line as a name or value would invent a rule not shared by the client.

Protocol parsers should distinguish a defined absence from undefined input. Reaching the blank line means the header section ended normally. Encountering a malformed non-empty line means the section could not be parsed reliably.

## Requirements

Keep `tinyhttp headers`.

Skip the request line, process non-empty header lines in order, and stop at the first empty line after removing a trailing `\r`. Do not inspect or print later body lines.

Every non-empty line before the boundary must contain a colon. Otherwise exit non-zero and include `malformed header` in stderr. Valid fields retain lowercase names, trimmed values, preserved value case, and input order.

## Example

For:

```http
POST / HTTP/1.1
Host: example.test

Body-Key: not-a-header
```

output is only:

```text
host: example.test
```

## Edge cases

- The request line is never printed as a header.
- The first blank line ends header parsing.
- Header-shaped body text is ignored.
- A non-empty pre-boundary line without a colon fails.

## Success criteria

All `deltaforge test` cases pass and body bytes can never become header output merely because they contain a colon.

## Non-goals

- Reading or interpreting the request body.
- Supporting folded legacy headers.
- Enforcing header-size limits.
- Applying connection or content semantics.
