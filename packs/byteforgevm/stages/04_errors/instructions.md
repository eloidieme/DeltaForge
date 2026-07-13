# Stage 07 — Keep guest mistakes inside the VM

## Goal

Turn unknown instructions and value-stack underflow into clear runtime errors instead of host-language panics.

## Background

ByteForgeVM is the host program. The `.bvm` file is a guest program running under its rules. A guest can be wrong: it can ask for an opcode the VM has never heard of, or try to add when the stack is empty.

Those mistakes should not tear through the boundary and crash the host. An interpreter's job includes translating impossible guest actions into controlled guest errors. This is one reason virtual machines are useful isolation layers in larger systems.

Standard output also has a specific role: it belongs to values produced by `PRINT`. Diagnostics belong on standard error, so a caller can distinguish program output from a report about why execution stopped.

## Requirements

If execution encounters an unsupported opcode, exit with status 1 and write text containing `unknown opcode` to standard error.

Before `ADD`, `SUB`, or `MUL`, require two values. Before `PRINT` or `JZ`, require one. If the stack is too small, exit with status 1 and write text containing `stack underflow` to standard error. Do not panic or invent a default value. Include the current instruction address when practical.

## Example

The one-line program:

```text
ADD
```

cannot supply two operands. Running it fails with a message such as `stack underflow at 0`.

## Edge cases

- An opcode unknown to the VM fails with `unknown opcode`.
- Arithmetic on an undersized value stack fails with `stack underflow`.

## Success criteria

All tests pass, tested guest mistakes never panic, and diagnostics do not appear as successful program output.

### Reflection

1. Which operations currently remove one value, and which remove two?
2. What host panic could occur if a stack pop were assumed to succeed?
3. Why is this boundary part of interpreter correctness rather than merely nicer error handling?

## Non-goals

- Continuing after a runtime error.
- Validating instruction operands and targets.
- Static verification of the whole program.
