# Hint 1

Write down the two version defaults first: 1.1 is opt-out, while 1.0 is opt-in.

# Hint 2

Parse the version and scan headers for one case-insensitive field, then apply the version's default only after the scan.

# Hint 3

`eq_ignore_ascii_case` handles both `Connection` and its token; an `Option` can distinguish an absent field from an explicit value.
