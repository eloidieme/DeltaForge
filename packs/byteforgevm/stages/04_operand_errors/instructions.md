# Stage 08 — Validate operands and jump targets

## Goal

Reject instructions whose required numeric operand is missing, malformed, negative where an address is expected, or outside the program.

## Background

An opcode and its operand form one instruction contract. `PUSH` needs a number to place on the stack. `JMP` and `JZ` need an address that identifies a loaded instruction. A bare `PUSH` has no value to push; `JMP -1` and `JMP 99` cannot name valid instructions in a short program.

This stage separates two useful questions. Parsing asks, “Can this written token become an integer?” Target validation asks, “Does that integer identify an instruction in this program?” Keeping those questions distinct makes diagnostics and later `CALL` validation easier to share.

## Requirements

An opcode that needs an operand must fail with standard error containing `missing argument` when none is present. A present operand that is not a signed integer must fail with `invalid argument`.

Before changing the instruction pointer for `JMP` or a taken `JZ`, require the target to be non-negative and smaller than the number of loaded instructions. An invalid target exits with status 1 and standard error containing `invalid jump`. None of these failures may panic.

## Example

For a three-instruction program, valid addresses are 0, 1, and 2. Both of these are invalid:

```text
JMP -1
JMP 3
```

## Edge cases

- A target beyond the final instruction fails with `invalid jump`.
- A negative target fails with `invalid jump`.
- An instruction with no required operand fails with `missing argument`.
- A non-integer operand fails with `invalid argument`.

## Success criteria

All tests pass, and every conversion from a written number to an instruction address is checked before use.

### Reflection

1. Why must a signed target be checked before converting it to an unsigned index?
2. Which validation knows the program length?
3. What machine state should remain unchanged when a jump target is rejected?

## Non-goals

- Reporting several errors in one run.
- Labels or named jump targets.
- Calls and return addresses.
