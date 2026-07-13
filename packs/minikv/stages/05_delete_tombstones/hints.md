# Hint 1

Deletion is another chronological operation. The recovered state for a key must represent either a live value or an explicit absence.

# Hint 2

Reuse the append path for `DEL`, then extend the replay operation type. A later record still replaces the earlier state, regardless of whether it is `SET` or `DEL`.

# Hint 3

An `Option<String>` per key expresses the two states: `Some(value)` after `SET`, `None` after `DEL`. Store it in the recovery map rather than removing all evidence of the key immediately.
