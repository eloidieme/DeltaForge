# Hint 1

Each header line has two roles separated by its first colon. Later colons can be part of the value.

# Hint 2

Normalize only what the contract asks for: lowercase the field name and trim surrounding whitespace from the value. Preserve the value's letter case.

# Hint 3

`split_once(':')`, `trim`, and `to_ascii_lowercase` are enough. Emit each normalized pair immediately to preserve input order.
