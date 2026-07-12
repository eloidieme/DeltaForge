# Hint 1

Treat the document root as a security boundary: decide whether a request path is safe before joining or opening anything.

# Hint 2

Separate path validation, file lookup, and response formatting so the 404 path cannot accidentally reuse body data.

# Hint 3

Inspect `std::path::Component` for parent/root/prefix components, then use the body byte slice's length when writing `Content-Length`.
