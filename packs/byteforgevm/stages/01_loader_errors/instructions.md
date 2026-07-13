# Stage 02 — Report programs that cannot be loaded

## Goal

Make the disassembler fail clearly when it cannot read the program or when a written operand is not an integer.

## Background

The happy path in Stage 01 assumed the file existed and every operand looked like `5` or `-12`. Those assumptions sit at the boundary between outside text and the machine's internal instruction list.

A useful loader does not let an operating-system error or a failed number conversion turn into a panic. It returns the problem to the person running the tool. Notice that this stage checks the *shape* of a numeric operand, not whether a particular opcode needs one. `PUSH many` cannot become an integer at all; deeper instruction rules come later.

## Requirements

Keep the valid disassembly behavior from Stage 01. If the program file cannot be opened or read, exit non-zero and write an explanation to standard error. If a present operand is not a signed integer, exit non-zero and include `invalid argument` in standard error.

Do not print a partial disassembly as though loading had succeeded. Parse the complete instruction list before rendering it.

## Example

This is not a loadable instruction:

```text
PUSH many
```

Running `disasm` on it exits with status 1 and reports an invalid argument.

## Edge cases

- A path naming no readable program exits non-zero.
- A written operand that is not an integer exits non-zero with `invalid argument`.

## Success criteria

All tests pass, invalid input does not panic, and valid programs still produce the Stage 01 listing.

### Reflection

1. Which failures come from the filesystem, and which come from the program text?
2. Why is `PUSH many` different from `PUSH` with no operand at this point?
3. What would be misleading about printing the first few instructions before discovering a bad operand later in the file?

## Non-goals

- Checking which opcodes require operands.
- Checking whether the VM supports an opcode.
- Recovering several loader errors in one run.
