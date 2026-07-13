# Hint 1

Think of the file as a durable transcript. One successful command contributes one complete line with an operation, key, and value.

# Hint 2

Prepare the parent directory before opening the file. Keep construction of the `SET` record separate from filesystem setup and success output.

# Hint 3

`Path::parent`, `fs::create_dir_all`, and an `OpenOptions` file opened with creation enabled can establish the first log. `writeln!` supplies the required terminating newline.
