# ByteForgeVM

## What you are building

ByteForgeVM is a tiny stack-based bytecode virtual machine. By the end, you will disassemble text bytecode, execute signed-integer stack arithmetic, follow conditional and unconditional control flow, convert invalid guest behavior into runtime errors, support nested calls and returns, and trace execution state for debugging.

## Why this is useful

Virtual machines are a compact way to learn interpreters, instruction dispatch, stack discipline, control flow, runtime isolation, and diagnostic tooling. The same ideas show up in scripting languages, query engines, emulators, compilers, smart-contract runtimes, and workflow systems.

## Big picture

1. Load instructions and assign stable addresses.
2. Execute arithmetic on an operand stack.
3. Let jumps choose the next instruction.
4. Turn invalid guest programs into explicit errors.
5. Add subroutines with a separate call stack.
6. Observe pre-instruction state with deterministic tracing.

## Machine model

ByteForgeVM has three pieces of execution state:

- the **instruction pointer** (`ip`), naming the next instruction;
- the **value stack**, containing program operands and results;
- the **call stack**, containing return addresses only.

Stack effects use this notation, with the top at the right:

```text
[..., left, right] --ADD--> [..., result]
```

## Cumulative opcode reference

| Opcode | Operand | Value-stack effect | Control effect |
|---|---:|---|---|
| `PUSH n` | signed integer | `[...] → [..., n]` | next instruction |
| `ADD` | — | `[..., a, b] → [..., a+b]` | next instruction |
| `SUB` | — | `[..., a, b] → [..., a-b]` | next instruction |
| `MUL` | — | `[..., a, b] → [..., a*b]` | next instruction |
| `PRINT` | — | `[..., value] → [...]` | prints, then advances |
| `JMP addr` | address | unchanged | target instruction |
| `JZ addr` | address | `[..., condition] → [...]` | target if zero, otherwise next |
| `CALL addr` | address | unchanged | save `ip+1`, then target |
| `RET` | — | unchanged | most recent saved return address |
| `HALT` | — | unchanged | stop the program |

Every stack requirement is preconditioned on enough values being present. Every address must identify an instruction inside the loaded program.

## VM glossary

- **Bytecode:** a compact instruction language consumed by a virtual machine.
- **Disassembly:** a readable, addressed rendering of an instruction stream.
- **Operand stack:** last-in, first-out storage for program values.
- **Instruction pointer:** the address of the instruction that executes next.
- **Dispatch:** selecting behavior from the current opcode.
- **Branch:** an instruction that may replace sequential control flow.
- **Return address:** the caller location restored by `RET`.
- **Trace:** an ordered record of execution state, here captured before each instruction.

## Historical field note

Stack-oriented execution appears in the Burroughs B5000, Forth, Smalltalk implementations, the Java Virtual Machine, and WebAssembly. The designs differ greatly, but all benefit from instructions whose operands are implicit at the top of a stack. Real runtimes add verification, frames, locals, heaps, garbage collection, and optimized dispatch; ByteForgeVM keeps the state small enough to reason about line by line.

## Failure-analysis lab

Name the broken state rule in each trace or result:

1. `PUSH 10; PUSH 4; SUB` leaves `-6`. Which pop became the left operand?
2. A taken `JMP 4` executes instruction 5 next. Where was the instruction pointer advanced twice?
3. `JZ` leaves its condition on the stack when the branch is not taken. Which stack effect differs between paths?
4. Nested calls return to the outer caller first. Which last-in, first-out structure was violated?
5. A trace shows the stack after `ADD` on the `ADD` line. Is the interpreter wrong, or is the observation captured at the wrong time?

## What good looks like

Good solutions parse once, keep instruction-pointer movement precise, separate value and control state, validate every stack/address precondition, and include enough context in errors to debug broken programs. The trace and ordinary runner should be two observation modes over one interpreter semantics.

## Optional extensions

Useful next experiments include comparisons, local variables, recursion limits, static verification, labels and assembly, packed binary bytecode, or a heap. Before adding an opcode, specify its stack effect and error conditions; that small discipline prevents a surprising amount of ambiguity.
