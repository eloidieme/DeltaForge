# Stage 09 — Call a routine and come back

## Goal

Add `CALL <address>` and `RET` for one complete call-and-return journey.

## Background

A jump knows where it is going but remembers nothing about where it came from. A call has to do both. `CALL 3` transfers execution to instruction 3 while saving the address of the instruction that should run afterward.

Consider this program:

```text
0  CALL 3
1  PRINT
2  HALT
3  PUSH 12
4  RET
```

The call saves address 1, then moves to 3. `RET` restores 1, so `PRINT` receives the value prepared by the routine. This saved location is called a return address.

Return addresses do not belong on the value stack. Program arithmetic should never be able to add, print, or accidentally remove the VM's control information. Give them a separate call stack, even though this stage uses only one saved address.

## Requirements

Extend `run` with:

- `CALL <address>`: validate the target, save `ip + 1` on a separate call stack, then jump to the target;
- `RET`: remove the latest saved return address and continue there.

Keep every earlier opcode unchanged. A normal call must resume at the instruction immediately after `CALL`.

## Example

The numbered program above prints:

```text
12
```

## Edge cases

- A call reaches its target and a matching return resumes immediately after the call.
- The return address remains separate from values manipulated by the guest program.
- Guest values already on the value stack survive a call and return.

## Success criteria

All tests pass, and you can trace the example's instruction pointer and both stacks by hand.

### Reflection

1. Why is the saved address `ip + 1` rather than `ip`?
2. What would happen if a return address were pushed onto the value stack?
3. In what way is a call a jump plus one additional state change?

## Non-goals

- Nested calls and call-stack failure cases; those come next.
- Arguments, local variables, or full stack frames.
- Named functions or linking.
