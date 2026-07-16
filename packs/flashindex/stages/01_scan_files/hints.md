# Hint 1 — Observation

Compare the path the command was given with the paths it prints. The supplied directory
is the one fixed root; every discovered file should eventually be described relative to
that same root, not relative to whichever directory is currently being visited.

# Hint 2 — Concept

Directory traversal is naturally recursive: visiting one directory can reveal regular
files and more directories that need the same treatment. Keep the root invariant fixed
while the current directory changes.

# Hint 3 — Experiment

Before changing the implementation, trace it on a three-level tree by hand. Record the
current directory, each child classification, and the path eventually added to output.
Then repeat with a nested `target` directory and check where traversal should stop.

# Hint 4 — Structure

Separate discovery from presentation. One function can recursively collect regular-file
paths while applying the directory ignore policy. A later step can strip the fixed root,
convert the displayed separator, sort the complete collection, and print it. In Rust,
the standard-library pieces to examine include `fs::read_dir`, `Path::is_dir`,
`Path::is_file`, `Path::strip_prefix`, and `Vec::sort`.

# Hint 5 — Retrospective

After the checks pass, compare collecting then sorting with maintaining sorted order
during traversal. Consider determinism, memory use, error propagation, and whether the
choice would still be appropriate for a project containing millions of files.
