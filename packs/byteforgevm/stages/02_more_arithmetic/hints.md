# Hint 1

Name the first popped value `right` and the second `left`. That makes the subtraction expression read like its rule.

# Hint 2

One helper can pop two operands for `ADD`, `SUB`, and `MUL`; each opcode then differs only in the operation used to create the result.

# Hint 3

The loader already uses signed integers, so negative `PUSH` values need no special stack representation. Preserve one newline for each executed `PRINT`.
