# Hint 1

Design the file as a contract between two separate invocations: every distinction the reader needs must survive serialization.

# Hint 2

Build the full deterministic index first, serialize it completely, and only then replace the destination; let `query` parse the same record boundaries.

# Hint 3

A line per token with tab-separated paths is sufficient; `create_dir_all` and `fs::write` handle a new nested destination and truncate stale contents.
