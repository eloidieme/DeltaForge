# Hint 1

Treat the command line as the entire lifetime of the pair. No collection or file is needed; the important boundary lies between the key argument and the value argument.

# Hint 2

Validate the argument count before printing. A quoted phrase has already arrived as one argument, so preserve its text rather than splitting it again.

# Hint 3

Collect `std::env::args().skip(1)` and match the exact three-item shape `memory`, key, value. Format one `key=value` line only for that valid shape.
