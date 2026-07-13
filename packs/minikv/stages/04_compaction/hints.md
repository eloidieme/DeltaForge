# Hint 1

State the equivalence first: replaying the input and replaying the output must produce the same key/value map.

# Hint 2

Recover the complete latest-value map before deciding which records to write. Compaction operates on logical state, not adjacent duplicate lines.

# Hint 3

An ordered map such as `BTreeMap<String, String>` can represent latest values and provide ascending key iteration for the compacted records.
