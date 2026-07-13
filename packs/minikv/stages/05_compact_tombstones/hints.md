# Hint 1

Compaction must serialize the final replay state, not merely the last `SET` it remembers. A deleted final state has no live value to write.

# Hint 2

Keep tombstoned keys in the recovered map as explicit `None` values until output selection. That prevents an older `Some` value from surviving by accident.

# Hint 3

Iterate the ordered `key -> Option<value>` map and emit records only for `Some(value)`. The existing safe-destination behavior can remain unchanged.
