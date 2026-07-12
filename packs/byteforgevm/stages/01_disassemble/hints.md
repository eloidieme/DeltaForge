# Hint 1

An address identifies an instruction in the loaded program, so decide which source lines actually become instructions.

# Hint 2

Parse the file into a sequence first, then render that sequence with its indices; do not mix execution state into the representation.

# Hint 3

`str::lines`, `filter`, and `enumerate` provide the sequence, while a format width such as `{ip:04}` produces the address.
