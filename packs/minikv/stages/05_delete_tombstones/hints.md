# Hint 1

A tombstone is state: while replaying, “deleted” must overwrite “present” just as a newer value would.

# Hint 2

Represent each key's latest state so it can express either a live value or deletion, and let every log operation replace that state.

# Hint 3

`Option<String>` inside the recovery map is enough: `Some(value)` for `SET`, `None` for `DEL`; compaction serializes only `Some` entries.
