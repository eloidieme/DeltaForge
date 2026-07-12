# Stage 02 — Stack arithmetic

## Goal

Execute straight-line bytecode on an operand stack, supporting integer constants, arithmetic, output, and explicit termination.

## Background

Stack machines encode operations without naming registers: operands are pushed, an arithmetic instruction consumes the top values, and the result returns to the stack. The Burroughs B5000, Forth, Java bytecode, and WebAssembly all reflect this idea in different forms. Compact instructions come with a discipline: operand order and stack depth are part of the semantics.

## Requirements

Expose `byteforgevm run <program-file>`. Support `PUSH <signed-i64>`, `ADD`, `SUB`, `MUL`, `PRINT`, and `HALT`. Arithmetic pops the right operand first and the left operand second, then pushes `left op right`. `PRINT` pops and prints one value followed by `\n`. `HALT` stops immediately; reaching the end also succeeds. Output consists only of values printed by executed `PRINT` instructions.

## Example

For `PUSH 10`, `PUSH 4`, `SUB`, `PRINT`, `HALT`:

```console
$ byteforgevm run subtract.bvm
6
```

## Edge cases

- Subtraction uses left-minus-right stack order.
- Negative integer operands are accepted.
- More than one `PRINT` produces values in execution order.
- Instructions after `HALT` are not executed.

## Success criteria

All `deltaforge test` cases pass and straight-line programs have deterministic output.

### Reflection

1. Write the stack effect of every opcode you now support. Which instruction is the first whose operand order is observable?
2. Why does `PRINT` consume its value rather than merely inspect it in this machine model?
3. What invariant should be true before and after dispatching any successful straight-line instruction?

## Non-goals

- Jumps, calls, variables, or heap memory.
- Division, overflow policy beyond signed 64-bit arithmetic, or floating point.
- Runtime-error wording, which is specified in Stage 04.
