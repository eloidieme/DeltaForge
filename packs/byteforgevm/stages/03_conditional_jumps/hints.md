# Hint 1

`JZ` always pops one condition. Only the instruction-pointer update differs between zero and non-zero values.

# Hint 2

Write both branches explicitly: zero selects the target; non-zero selects `ip + 1`. This makes double advancement easier to spot.

# Hint 3

Pop before testing with `value == 0`. Do not push the condition back on the fall-through path.
