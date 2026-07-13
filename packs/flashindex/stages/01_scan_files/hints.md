# Hint 1

Treat the supplied directory as the one fixed root. Every file you discover should eventually be described relative to that root, not relative to the directory you happen to be visiting.

# Hint 2

Separate discovery from presentation. Recursion can collect regular-file paths first; portable conversion and lexicographic sorting can happen after the walk is complete.

# Hint 3

The Rust standard library pieces to examine are `fs::read_dir`, `Path::is_dir`, `Path::is_file`, `Path::strip_prefix`, and `Vec::sort`. Convert the displayed separator without changing the real path used for I/O.
