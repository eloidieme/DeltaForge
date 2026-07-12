# Hint 1

Replay means applying records in chronological order so that newer facts naturally replace older facts.

# Hint 2

Parse each non-empty line into an operation, key, and value remainder, updating an in-memory mapping as you go.

# Hint 3

An ordered or hash map both work for lookup; `strip_prefix("SET ")` plus `split_once(' ')` distinguishes the key from a value that may contain spaces.
