# TinyHTTP

## What you are building

TinyHTTP is a small HTTP request and response engine. By the end, you will parse HTTP request lines, generate static file responses, normalize headers, and make keep-alive decisions.

## Why this is useful

HTTP looks simple until you implement it. This project teaches protocol parsing, defensive path handling, response formatting, header normalization, and connection semantics. Those skills transfer directly to servers, proxies, API gateways, and network tooling.

## Big picture

The project isolates HTTP behavior before adding networking complexity:

1. Parse a request line from stdin.
2. Serve static files from a root directory.
3. Normalize and print headers.
4. Decide whether a connection should stay alive.
5. Add content types for common static files.
6. Return byte ranges for partial responses.

## What good looks like

Good solutions reject malformed requests, avoid path traversal, keep response formatting exact, and treat protocol defaults explicitly.
