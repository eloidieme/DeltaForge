# Stage 06 — Trace mode

## Goal

Expose a deterministic execution trace that shows the instruction pointer, opcode, and value stack immediately before every executed instruction.

## Background

Tracing is one of computing's oldest debugging techniques: record the changing machine state, then reconstruct why control reached a surprising point. Unlike an interactive debugger, a trace is reproducible and easy to attach to a bug report. Its timing matters—a pre-execution snapshot explains the inputs to an instruction, while a post-execution snapshot answers a different question.

## Requirements

Expose `byteforgevm trace <program-file>`. Execute with exactly the same semantics and errors as `run`, but before each instruction print `ip=<address> op=<OPCODE> stack=[<values>]`. Use decimal addresses, opcode spelling from the program, comma-and-space separation from bottom to top, and `[]` for empty. Jumps and calls appear only when actually executed. `PRINT` program output follows its trace line on stdout; runtime diagnostics remain on stderr.

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

- The snapshot is emitted before the instruction mutates the stack.
- Taken jumps and calls make the next trace line show their target.
- `HALT` receives a trace line and no later instruction does.
- An invalid opcode is traced, then still fails with the Stage 04 diagnostic.

## Success criteria

All `deltaforge test` cases pass, trace ordering is stable, and `deltaforge bench` can measure the trace workload.

### Benchmark interpretation worksheet

After running `deltaforge bench`, record the median and p95, then answer:

1. How much of the measured work is instruction dispatch, and how much is formatting or writing trace lines?
2. Would a longer arithmetic-only program measure the same bottleneck as a program containing many `PRINT` instructions?
3. If redirecting stdout changes the result, what does that reveal about the workload rather than the VM semantics?
4. Which comparison would be fair: trace versus trace after an optimization, or trace versus silent `run`? Explain.

### Reflection

Pick one trace from the tests and reconstruct why each instruction pointer follows the previous one. Which line gives the strongest evidence that tracing observes state before execution?

## Non-goals

- Breakpoints, watches, stepping, or an interactive debugger.
- Tracing the call stack, timestamps, or host memory addresses.
- A machine-readable trace format.
