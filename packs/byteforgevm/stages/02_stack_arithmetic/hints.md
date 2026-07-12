# Hint 1

Write each opcode's stack effect on paper; for example, `SUB` transforms `[..., left, right]` into `[..., left-right]`.

# Hint 2

Keep a program counter and a value stack as separate interpreter state, dispatching one parsed instruction at a time.

# Hint 3

A `Vec<i64>` supplies push/pop behavior; pop `right` before `left`, and break the dispatch loop as soon as `HALT` executes.
