# Hint 1

An append-only log records history: a new write belongs after every byte already present, never in place of them.

# Hint 2

Resolve the parent-directory case first, then open the destination with create-and-append semantics and write one complete record.

# Hint 3

`std::fs::create_dir_all`, `OpenOptions::new().create(true).append(true)`, and `writeln!` provide the needed filesystem operations.
