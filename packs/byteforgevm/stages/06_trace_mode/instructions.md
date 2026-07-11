# Trace mode

Add:

```bash
byteforgevm trace <program-file>
```

Print one trace line before executing each instruction:

```txt
ip=0 op=PUSH stack=[]
```

The trace should make instruction flow and stack state visible.

Edge cases:

- trace should include jumps and calls
- runtime errors should still fail clearly
- final program output may appear after trace lines

Non-goals:

- debugger commands
- breakpoints
- interactive stepping
