# Hint 1

The instruction pointer is the answer to “what executes next”; a jump replaces that answer instead of merely modifying the stack.

# Hint 2

Let each opcode choose either a target or the following address, so the loop does not apply a second unconditional increment.

# Hint 3

Store the instruction pointer as a `usize`; for `JZ`, pop once, assign the target when the value is zero, otherwise add one.
