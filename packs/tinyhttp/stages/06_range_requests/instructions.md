# Range requests

Add:

```bash
tinyhttp range <root> <path> <start> <end>
```

Return a partial HTTP response with status `206 Partial Content`, a `Content-Range` header, and the requested inclusive byte range as the body.

Example for a 10 byte file and range `2 5`:

```txt
HTTP/1.1 206 Partial Content
Content-Range: bytes 2-5/10

cdef
```

Edge cases:

- reject ranges where start is greater than end
- reject end offsets beyond the file length
- keep path traversal protection

Non-goals:

- parsing real `Range` headers
- multipart ranges
- streaming responses
