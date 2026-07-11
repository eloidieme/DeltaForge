# Call and return

Add `CALL <addr>` and `RET`.

`CALL` should push the return address to a call stack and jump to the target address. `RET` should return to the latest saved address.

Edge cases:

- returning with an empty call stack should fail
- invalid call targets should fail
- normal `HALT` still stops execution

Non-goals:

- local variables
- function arguments
- recursion limits
