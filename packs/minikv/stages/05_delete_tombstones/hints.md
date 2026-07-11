# Hint 1

Represent recovered values as `Option<String>` so a tombstone can overwrite an older value.

# Hint 2

Compaction should write only keys whose latest value is `Some`.

# Hint 3

Keep the log format line-oriented and easy to inspect.
