# Hint 1

Absence is a result of valid replay. Malformation means replay could not establish a trustworthy result; do not let the two share the same return path.

# Hint 2

Ignore only truly blank lines. Every other line must have a recognized operation and all fields required by that operation.

# Hint 3

Have record parsing return `Result` before it mutates recovery state. Propagate an error containing `malformed` to `main`, write it to stderr, and use a failing exit status.
