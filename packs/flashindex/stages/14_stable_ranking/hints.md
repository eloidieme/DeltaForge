# Hint 1

Write the comparison as a three-part sentence: better coverage first, then more occurrences, then the smaller path.

# Hint 2

Apply the result limit after the complete sort. Limiting candidates earlier can discard a file that should have ranked above the retained prefix.

# Hint 3

Compose comparisons in order: descending matched count, descending occurrence count, ascending `PathBuf`. Enumerate only the first ten sorted records so displayed ranks begin at one and stay consecutive.
