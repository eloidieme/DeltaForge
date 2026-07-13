# Hint 1

Separate “the instruction has an integer” from “the integer is a valid address.” They are different checks with different information.

# Hint 2

Validate a signed target before converting it to `usize`, and require the converted value to be smaller than the program length.

# Hint 3

A `required_arg` helper can produce `missing argument`; a `valid_target` helper can produce `invalid jump`. Reuse the same checks for every instruction with those needs.
