# ByteForgeVM

## What you are building

A computer processor follows a stream of small instructions: move a value, combine two values, jump somewhere else, stop. Programming languages hide most of that machinery, which is usually helpful. ByteForgeVM makes a tiny version visible again.

You will build a virtual machine: a program that reads and executes instructions for an imaginary computer. Its language is written as plain text, so there is no binary encoding to decipher first. A complete program can be as small as this:

```text
PUSH 2
PUSH 5
ADD
PRINT
HALT
```

The program places 2 and 5 on a stack, replaces them with their sum, prints 7, and stops. Nothing in that description requires registers, variables, or a compiler. That smallness is what makes the project useful: every change to the machine state can be followed by hand.

## Start with a stack

Imagine a stack of trays. You can place a new tray on top and remove only the top tray. A value stack behaves the same way. After `PUSH 2` and `PUSH 5`, it looks like this, with the top at the right:

```text
[2, 5]
```

`ADD` removes the top two values and pushes their sum:

```text
[2, 5] --ADD--> [7]
```

For subtraction, removal order matters. In `[10, 4]`, the top value 4 is the right operand and 10 is the left operand, so `SUB` produces 6. Writing stack effects this way is a useful habit because it turns an informal idea into a checkable rule.

## The three pieces of machine state

As the project grows, the virtual machine keeps track of three things:

- the **instruction pointer**, which names the instruction that executes next;
- the **value stack**, which holds numbers used by the program;
- the **call stack**, which remembers where a called routine should return.

The instruction pointer begins at address 0. Most instructions move it to the following address, but jumps and calls can replace that normal next step. The two stacks are separate because a number such as 12 and a return address such as 12 may look identical while meaning completely different things.

## From instruction text to execution trace

1. Load a program and print a numbered instruction listing.
2. Report files and numeric operands that cannot be loaded.
3. Execute a small program with `PUSH`, `ADD`, `PRINT`, and `HALT`.
4. Add subtraction, multiplication, negative values, and repeated output.
5. Make `JMP` choose the next instruction directly.
6. Make `JZ` choose between two paths using a stack value.
7. Turn unknown instructions and empty-stack operations into guest errors.
8. Validate required operands and jump targets.
9. Add one level of `CALL` and `RET`.
10. Support nested calls and protect the call stack's boundaries.
11. Print a deterministic trace of the machine before each instruction.

The capabilities depend on one another. Execution needs a loaded instruction sequence; branches need an explicit instruction pointer; calls need validated branch targets and a separate place for return addresses; tracing observes the same state transitions as ordinary execution.

## Instruction reference

The complete machine understands these instructions:

| Instruction | Meaning | Value-stack effect |
|---|---|---|
| `PUSH n` | place signed integer `n` on top | `[...] → [..., n]` |
| `ADD` | add the top two values | `[..., a, b] → [..., a+b]` |
| `SUB` | subtract right from left | `[..., a, b] → [..., a-b]` |
| `MUL` | multiply the top two values | `[..., a, b] → [..., a*b]` |
| `PRINT` | print and remove the top value | `[..., value] → [...]` |
| `JMP addr` | continue at `addr` | unchanged |
| `JZ addr` | pop a condition; jump if it is zero | `[..., condition] → [...]` |
| `CALL addr` | remember the following address, then jump | unchanged |
| `RET` | resume at the latest remembered address | unchanged |
| `HALT` | stop immediately | unchanged |

An instruction that needs values must find enough of them on the value stack. An instruction that names an address must name an instruction that actually exists. These are rules of the guest machine. A broken guest program should receive a clear error; it should not crash the host program that implements the VM.

## Control flow, one step at a time

Without jumps, execution is a straight line: 0, 1, 2, 3. `JMP 6` changes the answer to “what runs next?” from the following instruction to instruction 6.

`JZ` adds a choice. It removes one value from the stack. If that value is zero, execution moves to the target; otherwise it continues to the following instruction. The condition is removed in both cases. Keeping both paths' stack effects the same makes later reasoning far less surprising.

Calls add another kind of movement. `CALL 8` must remember the address after the call, then move to 8. A later `RET` restores the most recently remembered return address. When calls nest, “most recent” is exactly the last-in, first-out behavior of a stack.

## A little history

Stack-oriented machines are not just classroom abstractions. The Burroughs B5000 family used stack-based execution in the early 1960s. Forth made a visible data stack central to its programming model. Smalltalk implementations, the Java Virtual Machine, and WebAssembly all use stack ideas in different forms.

Real runtimes add local variables, stack frames, heaps, garbage collection, verification, and optimized instruction dispatch. ByteForgeVM leaves those out so the essential loop remains readable: fetch the instruction at the current pointer, inspect the required state, perform one rule, and decide what comes next.

## Tracing the machine

When an interpreter gives the wrong answer, final output often says too little. A trace records the state before each instruction:

```text
ip=0 op=PUSH stack=[]
ip=1 op=PUSH stack=[2]
ip=2 op=ADD stack=[2, 5]
ip=3 op=PRINT stack=[7]
7
```

The `ADD` line shows `[2, 5]` because the snapshot is taken before `ADD` executes. That timing makes the trace a record of each instruction's inputs. The trace should observe the same interpreter used by normal execution, so debugging mode cannot quietly acquire different semantics.

## What a strong solution looks like

A strong solution parses the program once, keeps instruction-pointer changes explicit, and gives value data and return addresses separate homes. Every stack pop, required operand, and target address is checked. Errors go to standard error; successful program output stays on standard output. Most importantly, you can take a short bytecode program and explain every state change without guessing.

Possible extensions include comparisons, labels, local variables, recursion limits, static verification, binary bytecode, and heap objects. Before adding any instruction, first write down its operands, stack effect, next-instruction rule, and failure cases. That modest ritual prevents a great deal of ambiguity.
