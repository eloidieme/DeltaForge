# Stage 04 — Add more arithmetic

## Goal

Extend straight-line execution with signed values, `SUB`, and `MUL`, while preserving operand order and output order.

## Background

Addition hides a detail because `2 + 5` and `5 + 2` have the same result. Subtraction does not. When the stack is `[10, 4]`, the VM removes the top value 4 first, but 4 is the *right* operand. It removes 10 second as the left operand:

```text
[10, 4] --SUB--> [6]
```

This “pop right, then pop left” rule is common in stack machines. Naming the values as you remove them is much safer than hoping the order will be obvious inside one expression.

Signed integers also let the stack contain negative values. They do not change the stack rules; they simply widen the examples the machine can represent.

## Requirements

Keep all Stage 03 instructions and add:

- `SUB`: pop right, pop left, then push `left - right`;
- `MUL`: pop right, pop left, then push `left * right`.

`PUSH` accepts signed integers, including negative values. More than one executed `PRINT` writes one line per value in execution order.

## Example

```text
PUSH 10
PUSH 4
SUB
PRINT
HALT
```

prints `6`, not `-6`.

## Edge cases

- Multiplication combines the top two stack values.
- Subtraction uses left-minus-right order.
- Negative integers are accepted, and multiple prints remain in program order.

## Success criteria

All tests pass, and each arithmetic instruction has a stack effect you can state without referring to implementation details.

### Reflection

1. Why can an addition-only test fail to reveal reversed operands?
2. Write the stack after each instruction in the subtraction example.
3. Which parts of the execution loop remain unchanged when a new arithmetic opcode is added?

## Non-goals

- Division, floating point, or a new overflow policy.
- Jumps and calls.
- Variables or memory beyond the value stack.
