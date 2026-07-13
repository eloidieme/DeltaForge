# Hint 1

Every assumed stack value is a guest-controlled boundary. Replace the assumption with a fallible pop that names the current instruction.

# Hint 2

Return a `Result` from execution and let one command-level error path print to standard error and choose exit status 1.

# Hint 3

`Vec::pop().ok_or_else(...)` handles one required value. A shared two-pop helper can protect all binary arithmetic instructions consistently.
