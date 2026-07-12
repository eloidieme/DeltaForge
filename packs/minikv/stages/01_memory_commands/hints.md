# Hint 1

Treat this stage as a contract exercise: identify the exact two inputs and the one output line before choosing any data structure.

# Hint 2

Keep argument validation separate from formatting so an incomplete command cannot fall through to the success path.

# Hint 3

Rust's `std::env::args` preserves a quoted value as one argument; format the successful pair with a single `println!`.
