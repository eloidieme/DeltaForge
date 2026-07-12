# Hint 1

Compaction preserves logical state, not history: first determine what a full replay means at the end of the input.

# Hint 2

Separate the replay phase from serialization, and choose a traversal order that cannot vary between runs.

# Hint 3

A `BTreeMap` naturally retains only the latest inserted value per key and iterates keys in ascending order; `create_dir_all` handles a nested output path.
