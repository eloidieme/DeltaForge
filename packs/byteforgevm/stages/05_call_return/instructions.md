# Stage 05 — Call and return

## Goal

Support reusable bytecode routines with `CALL` and `RET`, including nested calls, without mixing return addresses into the program's value stack.

## Background

Subroutines transformed programming by letting one sequence serve many callers. A call transfers control while remembering where execution should resume; a return restores the most recent address, giving calls their last-in, first-out shape. Real machines often store richer stack frames, but a separate return-address stack exposes the essential mechanism cleanly and prevents arithmetic from corrupting control state.

## Requirements

Extend `run` with `CALL <addr>` and `RET`. `CALL` validates the zero-based target, records the address immediately after the call, and transfers control to the target. `RET` removes the most recently recorded return address and resumes there. Calls may nest. `HALT` still ends the whole program immediately. Invalid call targets exit non-zero with text containing `invalid jump`; `RET` without a saved address exits non-zero with `call stack underflow`.

## Example

```text
CALL 3
PRINT
HALT
PUSH 12
RET
```

produces:

```text
12
```

## Edge cases

- A normal call resumes at the instruction after `CALL`.
- Nested calls return in last-in, first-out order.
- `RET` with an empty call stack fails clearly.
- A negative or out-of-program call target fails before changing call state.
- `HALT` inside a called routine stops execution instead of returning.

## Success criteria

All `deltaforge test` cases pass and value-stack operations cannot consume or forge return addresses.

### Reflection

1. State the relationship between a successful `CALL` and the later `RET` that matches it.
2. What state must remain unchanged when a call target is rejected?
3. Which additional facts would a real stack frame need for arguments and local variables?

## Non-goals

- Arguments, local variables, stack frames, tail calls, or recursion limits.
- Named functions or a linker.
- Exceptions or unwinding.
