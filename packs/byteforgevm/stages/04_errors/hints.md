# Hint 1

Every place that assumes an opcode, operand, stack value, or target exists is a boundary that can return a guest error.

# Hint 2

Give parsing, stack pops, and target validation fallible helpers so every execution mode shares the same error behavior.

# Hint 3

Return a `Result` from the interpreter; use `ok_or_else` for `Vec::pop` and check a signed target before converting it to `usize`.
