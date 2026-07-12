# Hint 1

Corpus selection is a predicate over the already discovered relative paths; keep it separate from recursion.

# Hint 2

Check the exceptional exact filename first, then compare the optional extension against the fixed allow-list.

# Hint 3

`Path::file_name` and `Path::extension().and_then(OsStr::to_str)` let a small `matches!` expression describe the rules.
