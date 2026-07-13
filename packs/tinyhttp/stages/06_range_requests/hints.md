# Hint 1

Draw the bytes with their zero-based positions. The selected body contains the byte at `start`, the byte at `end`, and everything between them.

# Hint 2

Once you have the selected byte slice, its own length is the safest source for `Content-Length`. The whole byte vector supplies the denominator in `Content-Range`.

# Hint 3

Rust's inclusive slice syntax is `start..=end`. Keep using byte-oriented file reading so offsets and lengths refer to the same unit.
