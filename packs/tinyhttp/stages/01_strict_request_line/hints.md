# Hint 1

Count fields before assigning meanings. Fewer than three and more than three are both malformed under TinyHTTP's request-line grammar.

# Hint 2

Treat empty input as its own boundary case so you can report `missing request line` rather than indexing a line that is not there.

# Hint 3

Collect the whitespace-separated pieces or match a slice. Accept only the exact three-item shape `[method, path, version]`.
