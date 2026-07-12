# Hint 1

Reason in byte offsets, and remember that both endpoints belong to the requested interval.

# Hint 2

Validate path and numeric bounds before slicing; compute response length from the validated slice rather than duplicating arithmetic.

# Hint 3

Read with `fs::read`, guard `start <= end && end < bytes.len()`, and slice with the inclusive range `start..=end`.
