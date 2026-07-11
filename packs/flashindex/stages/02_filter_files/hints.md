# Hint 1

Keep the Stage 01 traversal behavior, then add a predicate that decides whether a file should be printed.

# Hint 2

Check both the extension and special filenames like `CMakeLists.txt`.

# Hint 3

For binary-looking files, reading a small prefix is enough for this stage. A NUL byte is a strong signal to skip.
