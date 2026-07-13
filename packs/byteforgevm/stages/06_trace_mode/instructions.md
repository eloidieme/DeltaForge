# Stage 11 — Watch the machine execute

## Goal

Add `byteforgevm trace <program-file>`, which shows the instruction pointer, opcode, and value stack immediately before every executed instruction.

## Background

Final output tells you what a program produced, but often not how it got there. A trace turns execution into a sequence you can inspect. Long before interactive graphical debuggers, programmers used printed traces and machine logs to reconstruct a fault one state change at a time.

The word “before” is important. On the line for `ADD`, the trace should show the two operands that `ADD` is about to consume. A post-instruction snapshot would show the result instead. Both views can be useful, but mixing their timing makes a trace difficult to read.

Trace mode should observe the same interpreter as `run`. If it implements a second execution loop, the debugger and the normal runner can quietly disagree about the language.

## Requirements

Execute with exactly the same behavior and errors as `run`. Immediately before dispatching each instruction, print:

```text
ip=<address> op=<OPCODE> stack=[<values>]
```

Use decimal addresses. Show stack values from bottom to top, separated by a comma and one space; show an empty stack as `[]`. Preserve opcode spelling from the program.

Only executed instructions receive lines. A taken jump or call makes the next trace line show its target. `HALT` receives a line, but instructions after it do not. If `PRINT` executes, its ordinary output follows that instruction's trace line on standard output. Errors remain on standard error.

## Example

```text
ip=0 op=PUSH stack=[]
ip=1 op=PUSH stack=[2]
ip=2 op=ADD stack=[2, 5]
ip=3 op=PRINT stack=[7]
7
ip=4 op=HALT stack=[]
```

## Edge cases

- Arithmetic snapshots show operands before they are consumed.
- A taken jump's next line is its target.
- `HALT` is traced and later instructions are not.
- An invalid opcode is traced, then fails with the existing runtime error.
- Calls and returns appear only when their instructions actually execute.

## Success criteria

All tests pass, trace order is deterministic, and `deltaforge bench` can measure the supplied trace workload.

### Benchmark interpretation worksheet

After running `deltaforge bench`, record the median and p95, then consider:

1. How much work belongs to instruction dispatch, and how much to formatting and writing text?
2. Would an arithmetic-only program measure the same bottleneck as one containing many `PRINT` instructions?
3. If redirecting standard output changes the result, what does that reveal about the workload?
4. Why is trace-after-optimization a fairer comparison than trace versus silent `run`?

### Reflection

Choose one test trace and reconstruct why every instruction pointer follows the previous one. Which line most clearly proves that the snapshot happens before execution?

## Non-goals

- Breakpoints, stepping, or an interactive debugger.
- Timestamps, host addresses, or call-stack tracing.
- A machine-readable trace format.
