# Hint 1

Identify the largest piece of work that one file can perform without reading or mutating another file's state.

# Hint 2

Let workers return partial results, then combine them after they finish. The final merge must enforce the same ordered sets and maps as the canonical single-threaded index.

# Hint 3

Partition the sorted file list, spawn at most the requested positive worker count, join every handle, and union worker-local postings into the canonical structure before formatting.
