# Hint 1

List the checks in the order that makes unsafe data inaccessible: path first, numeric parsing next, then bounds once the file length is known.

# Hint 2

Do not slice until you have proved both `start <= end` and `end < bytes.len()`. A failed proof follows one error path and writes no response.

# Hint 3

Parse offsets into `usize`, map parsing failures to `invalid range`, and keep the earlier `unsafe path` result distinct so each boundary is visible.
