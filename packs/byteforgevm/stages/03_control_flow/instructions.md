# Stage 05 — Jump to another instruction

## Goal

Add `JMP <address>`, an instruction that chooses the next instruction directly.

## Background

Until now, the instruction pointer followed a simple rhythm: after instruction 0 came 1, then 2, then 3. `HALT` could stop that rhythm, but nothing could continue somewhere else.

`JMP` makes the instruction pointer visible as state. When the VM executes `JMP 4`, the next instruction is exactly address 4. It is not 5, and the VM must not first advance past the jump and then add the target. Physical processors and other interpreters face the same distinction between “fall through to the next instruction” and “replace the program counter.”

Start with an unconditional jump so there is only one new question: where does execution continue?

## Requirements

Extend `run` with `JMP <address>`. Addresses are the zero-based instruction positions shown by `disasm`.

When `JMP` executes, set the instruction pointer to its target. Do not also perform the ordinary one-instruction advance. Keep every earlier opcode unchanged.

## Example

```text
JMP 2
PUSH 99
PUSH 9
PRINT
HALT
```

prints `9`. Instruction 1 is never executed.

## Edge cases

- A jump skips every instruction between itself and its target.
- The target instruction itself executes; the pointer is not advanced past it.
- A jump may land directly on `HALT`, producing no later output.

## Success criteria

All tests pass, and a jump has exactly one clearly defined next address.

### Reflection

1. What is the instruction-pointer sequence in the example?
2. Why is a shared unconditional `ip += 1` at the end of the loop risky now?
3. How is `HALT` different from a jump beyond the program?

## Non-goals

- Conditional jumps or loops based on stack values.
- Defining invalid-target diagnostics; Stage 08 does that.
- Labels or symbolic addresses.
