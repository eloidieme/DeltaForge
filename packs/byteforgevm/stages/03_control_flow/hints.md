# Hint 1

Stop thinking of `ip + 1` as an automatic end-of-loop action. Each opcode should decide its own next instruction.

# Hint 2

For ordinary instructions, set or advance the pointer to the following address. For `JMP`, replace it with the target and do nothing else.

# Hint 3

Use the operand as the zero-based index into the parsed program. Keep target conversion in one place so bounds checks can share the same rule.
