# Stage 05 — Read HTTP headers

## Goal

Read valid HTTP header fields, normalize their names, trim surrounding value whitespace, and print them in their original order.

The request line describes the main action. Headers carry the additional facts later stages need to make decisions.

## Background

Consider:

```http
GET / HTTP/1.1
Host: example.test
User-Agent: DeltaForge
Accept:   text/plain
```

Each header line has a field name, a colon, and a value.

Header names are case-insensitive in HTTP. `Host`, `host`, and `HOST` name the same field. TinyHTTP normalizes names to ASCII lowercase so later code has one spelling to compare.

Values are different. Their case may be meaningful to the application, so TinyHTTP preserves it. It removes only whitespace surrounding the value:

```text
Accept:   text/plain   → accept: text/plain
X-Mode: KeepThisCase   → x-mode: KeepThisCase
```

The output keeps input order. At this point the command is describing the request, not collecting repeated names into a map or applying field-specific semantics.

This stage assumes valid header lines and focuses on the transformation. The next stage defines where the header section ends and what happens to malformed lines.

## Requirements

Add `tinyhttp headers` and read one request from stdin.

Skip the first request line. For each valid header line, split at its first colon, print the field name in ASCII lowercase, then `: `, then the value with surrounding whitespace removed. Preserve value case and header input order.

## Example

Input:

```http
GET / HTTP/1.1
Host: example.test
X-Mode: KeepThisCase
```

Output:

```text
host: example.test
x-mode: KeepThisCase
```

## Edge cases

- Header-name comparison is represented by ASCII lowercase output.
- Surrounding value whitespace is removed.
- Value case is preserved.
- Several valid headers retain their input order.

## Success criteria

All `deltaforge test` cases pass and each printed line represents exactly one valid input header.

## Non-goals

- Defining the blank-line/body boundary; that is the next stage.
- Combining duplicate header fields.
- Interpreting specific values such as `Connection`.
- Full RFC field-value grammar or obsolete line folding.
