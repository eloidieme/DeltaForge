# Hint 1

The work splits cleanly along files: each file's tokens can be found without
looking at any other file. Decide up front which data is private to a worker and
which data is shared, and arrange for nothing shared to be written while workers
are running.

# Hint 2

Compute the sorted file list once, partition it into roughly equal slices, and
give each thread its own slice and its own local index. When the threads finish,
fold their local indexes into one. Because the final structure is sorted, the
order in which threads complete cannot affect the result.

# Hint 3

`std::thread::spawn` with `join()` on each handle is enough; a
`BTreeMap<String, BTreeSet<PathBuf>>` per worker merges into a final
`BTreeMap<String, BTreeSet<PathBuf>>` by unioning the sets. Parse `--threads`
with `str::parse::<usize>()` and reject `0` and parse errors before spawning
anything.
