# Hint 1

The request line is the first protocol record; solve its boundary and three fields before thinking about the rest of the message.

# Hint 2

Make stdin and file input produce the same string, then pass that string through one request-line parser and one formatter.

# Hint 3

`Read::read_to_string`, `fs::read_to_string`, `lines().next()`, and `split_whitespace` are sufficient; accept only a slice matching exactly three parts.
