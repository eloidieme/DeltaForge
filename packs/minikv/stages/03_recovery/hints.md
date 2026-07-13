# Hint 1

Read the log as a sequence of state changes. When the requested key appears again, its later value replaces the earlier candidate.

# Hint 2

Split a `SET` record only far enough to isolate the operation and key. The remainder belongs to the value and may contain spaces.

# Hint 3

Walking `source.lines()` from beginning to end with an `Option<String>` for the requested value is sufficient. Print only the final option after replay completes.
