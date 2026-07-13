# Stage 03 — Run a basic stack program

## Goal

Add `byteforgevm run <program-file>` with four instructions: `PUSH`, `ADD`, `PRINT`, and `HALT`.

## Background

The disassembler built a sequence of instructions. Running that sequence requires two new pieces of state: an instruction pointer and a value stack.

The instruction pointer names what happens next. It starts at 0 and normally advances by one. The value stack holds numbers, with new values placed on top. If the stack is written left to right with its top at the right, addition works like this:

```text
[2, 5] --ADD--> [7]
```

`ADD` removes 5, removes 2, and places 7 back on the stack. `PRINT` removes the top value and writes it. `HALT` stops immediately. These few rules are enough to create a real fetch-and-execute loop.

## Requirements

Support:

- `PUSH <signed-integer>`: put the number on the value stack;
- `ADD`: remove the top two values and push their sum;
- `PRINT`: remove and print the top value followed by a newline;
- `HALT`: end execution immediately.

Reaching the end of the program also succeeds. Standard output contains only values produced by executed `PRINT` instructions.

## Example

```text
PUSH 2
PUSH 5
ADD
PRINT
HALT
```

prints:

```text
7
```

## Edge cases

- `ADD` consumes two values and leaves their sum for `PRINT`.
- Instructions after `HALT` are not executed.

## Success criteria

All tests pass, and the example's stack can be explained instruction by instruction.

### Reflection

1. What are the instruction pointer and stack immediately before `ADD` in the example?
2. Why does `PRINT` remove its value in this machine rather than only inspect it?
3. Which instruction changes the usual “advance by one” rule without choosing another address?

## Non-goals

- Subtraction, multiplication, or jumps.
- Specifying every runtime error; later stages make failures precise.
- Variables, registers, or heap memory.
