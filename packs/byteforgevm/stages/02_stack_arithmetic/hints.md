# Hint 1

Keep an instruction pointer and a `Vec<i64>` value stack. At the top of each loop, fetch the instruction named by the pointer.

# Hint 2

Write the stack effect beside each opcode before implementing dispatch. `ADD` changes `[..., a, b]` into `[..., a+b]`.

# Hint 3

Most instructions finish with the next address, but `HALT` should leave the loop immediately. `PRINT` pops, prints, and then advances.
