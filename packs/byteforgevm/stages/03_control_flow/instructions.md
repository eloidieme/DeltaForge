# Stage 03 — Control flow

## Goal

Let bytecode choose its next instruction with unconditional and zero-tested jumps, turning the straight-line evaluator into a genuine interpreter for branches and loops.

## Background

Control flow is the point where an instruction pointer becomes visible state. Physical CPUs branch by replacing their program counter; stack VMs do the same with instruction addresses. `JZ` also demonstrates a common stack-machine pattern: a condition is data until the branch consumes it. Small off-by-one errors here can skip targets or execute both paths.

## Requirements

Extend `run` with `JMP <addr>` and `JZ <addr>`, where addresses are zero-based instruction indices from Stage 01. `JMP` sets the next instruction directly to its target. `JZ` pops one value: if it is zero, jump to the target; otherwise continue with the following instruction. The condition is consumed in both cases. All prior opcodes retain their behavior.

## Example

```text
PUSH 0
JZ 4
PUSH 99
PRINT
PUSH 1
PRINT
HALT
```

prints:

```text
1
```

## Edge cases

- An unconditional jump skips intervening instructions.
- `JZ` takes the branch for exactly zero.
- A non-zero `JZ` condition falls through and is still removed from the stack.
- A jump target is executed directly rather than incremented past.

## Success criteria

All `deltaforge test` cases pass and both branch paths produce the specified output without double-advancing the instruction pointer.

## Non-goals

- Named labels or assembly-time target resolution.
- Comparison opcodes, structured loops, or functions.
- Defining invalid-target errors before Stage 04.
