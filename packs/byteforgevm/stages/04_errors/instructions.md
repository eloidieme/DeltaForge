# Stage 04 — Runtime errors

## Goal

Fail malformed or impossible executions predictably, with non-zero status and diagnostic text that identifies the class of virtual-machine error.

## Background

An interpreter mediates between untrusted program bytes and its host process. A bad opcode or empty stack should become a guest-program error, not a host panic. Mature VMs distinguish verification and runtime failures for exactly this reason. Clear instruction addresses make diagnostics actionable while keeping stdout reserved for successful program output.

## Requirements

Both `run` and later execution modes must reject unknown opcodes with stderr containing `unknown opcode`, insufficient operands with `stack underflow`, and any negative or out-of-program jump with `invalid jump`. Missing required operands and non-integer operands must also exit non-zero. Runtime failures exit status 1, do not panic, and write diagnostics to stderr. Include the current instruction address where it is meaningful.

## Example

```console
$ byteforgevm run broken.bvm
stack underflow at 0
$ echo $?
1
```

## Edge cases

- An opcode unknown to the VM fails clearly.
- Arithmetic and `PRINT` on an undersized stack fail rather than inventing values.
- Negative and too-large jump targets are invalid.
- An opcode requiring an operand fails when the operand is missing or malformed.

## Success criteria

All `deltaforge test` cases pass, no tested invalid program panics, and successful program stdout remains uncontaminated by diagnostics.

## Non-goals

- Recovering and continuing after an error.
- A static verifier or multiple-error report.
- Source filenames, line mappings, or exception handling inside bytecode.
