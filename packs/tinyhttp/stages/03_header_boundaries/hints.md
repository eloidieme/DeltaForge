# Hint 1

Label the message regions on paper: request line, headers, blank line, body. Only one of those regions contains header fields.

# Hint 2

Skip the first line, then stop at the first empty line after removing a possible carriage return. Do not resume when body lines appear afterward.

# Hint 3

While you are inside the header region, require every non-empty line to contain a colon. A line without one is malformed rather than ignorable.
