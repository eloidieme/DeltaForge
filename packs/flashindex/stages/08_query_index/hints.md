# Hint 1

Start from the format agreement you made as the writer. The reader should reverse that agreement into token postings without consulting the source tree.

# Hint 2

Keep “valid artifact, token absent” separate from “artifact could not be read.” Only the first case is empty success.

# Hint 3

Parse the artifact into the same logical token-to-path shape used before persistence, then perform exact lookup and print the already canonical paths one per line.
