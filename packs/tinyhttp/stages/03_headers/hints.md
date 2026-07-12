# Hint 1

HTTP divides a message into regions: request line, header fields, blank delimiter, then optional body. Keep those boundaries explicit.

# Hint 2

Iterate after the first line, stop on the first empty line after removing a possible carriage return, and split each remaining line once.

# Hint 3

`split_once(':')`, `trim`, and `to_ascii_lowercase` express the required normalization without changing the value's internal characters.
