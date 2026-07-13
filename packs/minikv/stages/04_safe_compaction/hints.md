# Hint 1

Treat the compacted bytes as the complete destination artifact. Writing only a prefix is insufficient when yesterday's artifact was longer.

# Hint 2

Prepare parents, generate the complete output, and use replacement semantics. Never choose the input file as scratch storage.

# Hint 3

`fs::create_dir_all` plus `fs::write` naturally creates or truncates the destination. Read the input fully before writing and do not open it with write access.
