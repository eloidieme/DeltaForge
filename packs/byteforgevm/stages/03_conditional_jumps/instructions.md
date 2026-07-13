# Stage 06 — Choose a path from a stack value

## Goal

Add `JZ <address>`, which jumps when the value on top of the stack is zero and otherwise continues normally.

## Background

An unconditional jump always chooses its target. Programs become more expressive when the choice can depend on a computed value.

`JZ` means “jump if zero.” It removes one condition value from the stack. If that value is zero, the target becomes the next instruction. If it is non-zero, execution falls through to the instruction after `JZ`.

The condition is consumed on both paths:

```text
[..., 0] --JZ target--> [...]  and jump
[..., 3] --JZ target--> [...]  and continue
```

Keeping the stack effect the same on both branches prevents the stack from depending on which path happened to run.

## Requirements

Extend `run` with `JZ <address>`. Pop one value from the stack. For zero, set the instruction pointer directly to the target. For any non-zero value, advance to the following instruction. Every existing instruction retains its behavior.

## Example

```text
PUSH 0
JZ 5
PUSH 2
PRINT
HALT
PUSH 1
PRINT
HALT
```

prints `1` because the zero condition takes the branch to instruction 5.

## Edge cases

- A condition equal to zero takes the branch.
- A non-zero condition falls through.

## Success criteria

All tests pass, and both branch paths make one deliberate instruction-pointer update.

### Reflection

1. What changes in the example if the first instruction becomes `PUSH 3`?
2. Why should the condition be removed even when the branch is not taken?
3. How could you test that a taken jump does not skip its target?

## Non-goals

- Comparisons or boolean opcodes.
- Invalid-address and stack-underflow wording.
- Labels, functions, or calls.
