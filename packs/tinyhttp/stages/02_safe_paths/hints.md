# Hint 1

Think of the document root as a fence. Any path component asking to move to the parent side of that fence should fail before file access.

# Hint 2

Put path validation in a small function and call it before joining the request path to the root. Successful ordinary paths should still follow the old response path.

# Hint 3

Inspect `std::path::Component`. Reject parent, root, and platform-prefix components from the request-relative portion instead of searching the raw text for one suspicious substring.
