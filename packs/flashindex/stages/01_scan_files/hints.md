# Hint 1

Think of traversal as producing a set of paths relative to one fixed root; sorting comes after discovery, not from directory iteration.

# Hint 2

At each directory entry, prune ignored directory names before recursion and retain only regular files for final output.

# Hint 3

`fs::read_dir`, `Path::strip_prefix`, `Vec::sort`, and replacing `\\` with `/` cover the observable contract without extra crates.
