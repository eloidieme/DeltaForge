# Hint 1

Start with the four-row table in the instructions. If your condition cannot be checked against that table easily, it is probably doing too much at once.

# Hint 2

Represent the `Connection` value as optional. “No header” is meaningful and should remain distinguishable from an explicit token.

# Hint 3

Use `eq_ignore_ascii_case` for both the field name and its value. Apply the version default after the header scan, not before it.
