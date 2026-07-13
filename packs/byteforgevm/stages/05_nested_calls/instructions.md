# Stage 10 — Let routines call other routines

## Goal

Support nested calls and define what happens when `CALL`, `RET`, or `HALT` reaches a call-stack boundary.

## Background

One saved return address could live in a single variable. That stops working as soon as a routine calls another routine. The inner call needs a new return address without erasing the outer one.

A stack has exactly the required order. If routine A calls B and B calls C, C must return to B before B can return to A. The most recently saved return address is the first one restored.

Nested calls also expose three boundaries. `RET` cannot work when no call is active. `CALL` must not save a return address if its target is invalid. `HALT` ends the entire VM immediately, even if execution is currently inside a routine; it is not an implicit `RET`.

## Requirements

Allow `CALL` instructions to nest using the separate return-address stack. `RET` pops the most recent return address.

- `RET` with an empty call stack exits non-zero with `call stack underflow`.
- A negative or out-of-program `CALL` target exits non-zero with `invalid jump` before call state changes.
- `HALT` stops the whole program immediately, including from inside a called routine.

## Example

If A calls B and B calls C, the call stack grows like this:

```text
[] → [return-to-A] → [return-to-A, return-to-B]
```

The first `RET` restores `return-to-B`; the second restores `return-to-A`.

## Edge cases

- Nested calls return in last-in, first-out order.
- `RET` with no saved address fails with `call stack underflow`.
- An invalid call target fails with `invalid jump`.
- `HALT` inside a called routine stops rather than returning.

## Success criteria

All tests pass, and value-stack operations cannot consume or forge return addresses.

### Reflection

1. Draw the call stack before and after each return in a two-level nested call.
2. Why should target validation happen before pushing the return address?
3. What extra information would a real function's stack frame need?

## Non-goals

- Arguments, locals, recursion limits, or tail-call optimization.
- Exceptions and stack unwinding.
- Named routines or a linker.
