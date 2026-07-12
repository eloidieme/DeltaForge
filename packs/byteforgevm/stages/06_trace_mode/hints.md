# Hint 1

Tracing should observe one interpreter, not create a second one whose semantics can drift from `run`.

# Hint 2

Add an execution-mode flag and emit the snapshot at the top of the dispatch cycle, before matching the opcode.

# Hint 3

Format stack values with `iter().map(i64::to_string).collect::<Vec<_>>().join(", ")`; keep errors on stderr through the existing `Result` path.
