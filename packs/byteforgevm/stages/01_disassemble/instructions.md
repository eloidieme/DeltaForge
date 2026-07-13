# Stage 01 — Give every instruction an address

## Goal

Add `byteforgevm disasm <program-file>`, which prints a clean, numbered listing of a bytecode program without executing it.

## Background

Before a virtual machine can run a program, it has to decide what the program contains. ByteForgeVM's source format is simple: one instruction per non-empty line. Loading those lines into a sequence gives each instruction a stable address.

Addresses let people and instructions refer to the same place. A debugger can say “instruction 3 failed,” and a later jump can say “continue at instruction 3.” This first stage concentrates on that shared map before the machine has any execution state.

A readable instruction listing is called a disassembly. Tools such as `objdump` disassemble packed machine code; yours starts from text, but the purpose is the same.

## Requirements

Read the UTF-8 program file. Ignore empty lines. For every remaining line:

1. separate the opcode from its optional integer operand;
2. normalize spacing between them to one space;
3. print its zero-based address as four digits, followed by the instruction.

The required shape is `0000 OP` or `0000 OP ARG`. Preserve the opcode's spelling. This stage describes instructions; it does not yet decide whether an opcode can execute.

## Example

Given:

```text
PUSH 2
PUSH 5
ADD
HALT
```

the command prints:

```text
0000 PUSH 2
0001 PUSH 5
0002 ADD
0003 HALT
```

## Edge cases

- The first instruction has address `0000`.
- An instruction without an operand, such as `HALT`, has no trailing operand.
- Blank lines do not consume addresses or leave gaps in the listing.

## Success criteria

All tests pass, and the same source program always receives the same addressed listing.

### Reflection

1. Why is an address based on instruction position rather than source byte position here?
2. What does the disassembler know about `ADD`, and what does it deliberately not know yet?
3. Why is it useful to finish loading before rendering the listing?

## Non-goals

- Executing or validating opcodes.
- Labels, comments, symbols, or packed binary bytecode.
- Following control flow.
