# Hint 1

Let both file reading and operand conversion return errors to the command boundary. Neither failure needs to panic.

# Hint 2

Build the complete instruction vector before printing it. Then a bad later operand cannot leave a believable partial listing behind.

# Hint 3

Use `read_to_string` for the file and `parse::<i64>()` for a present operand, mapping the conversion failure to text containing `invalid argument`.
