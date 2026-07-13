# Hint 1

Tracing should observe the existing interpreter, not copy it. Add an execution mode or optional observer around the same dispatch loop.

# Hint 2

Emit the snapshot at the very top of the loop, after fetching the current instruction but before matching and mutating state.

# Hint 3

Join stack values with `, ` and surround them with brackets. Leave the existing error return path on standard error.
