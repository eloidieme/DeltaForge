# Hint 1

A call has two destinations: the callee target and the caller's resume address. Write both down before mutating execution state.

# Hint 2

Keep return addresses in a separate `Vec<usize>`. The value stack remains exclusively available to guest arithmetic and output.

# Hint 3

After validating the target, push `ip + 1` onto the call stack and set `ip` to the target. `RET` pops the saved address back into `ip`.
