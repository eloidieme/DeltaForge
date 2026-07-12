# Stage 01 — Disassemble bytecode

## Goal

Read ByteForgeVM's textual bytecode and render a stable, addressed listing. The result should let a human point to an instruction unambiguously before the virtual machine is capable of executing anything.

## Background

A disassembler translates a machine-oriented instruction stream into a readable listing. From early mainframe monitors to tools such as `objdump`, addresses have been essential: branches refer to locations, debuggers stop at them, and error reports name them. ByteForgeVM uses text rather than packed binary, but the same separation matters—loading and describing a program is distinct from running it.

## Requirements

Expose `byteforgevm disasm <program-file>`. Treat each non-empty UTF-8 line as one instruction and number instructions from zero after blank lines are omitted. Print each as a four-digit, zero-padded address, one space, then the opcode and its optional integer argument normalized with single spaces: `0000 OP ARG`. Preserve opcode spelling. A missing file or invalid integer argument exits non-zero.

## Example

```console
$ byteforgevm disasm add.bvm
0000 PUSH 2
0001 PUSH 5
0002 ADD
0003 HALT
```

## Edge cases

- The first instruction is address `0000` and numbering advances by instruction, not source byte.
- An instruction without an operand, including `HALT`, prints no trailing operand.
- Blank lines do not create gaps in addresses.
- A missing program file exits non-zero.

## Success criteria

All `deltaforge test` cases pass and the same program always produces the same addressed listing.

## Non-goals

- Executing instructions or validating whether an opcode is supported.
- Labels, symbols, comments, or packed binary bytecode.
- Control-flow analysis.
