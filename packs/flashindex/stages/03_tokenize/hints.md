# Hint 1

On paper, mark each transition from “outside a token” to “inside a token.” The byte position of that transition is the column you eventually print.

# Hint 2

Use two character questions: can this byte begin a token, and can it continue one? A digit answers no to the first and yes to the second.

# Hint 3

Processing one source line at a time makes one-based line numbers natural. Byte indices from `str::char_indices` can become one-based columns, while a remembered start index lets you slice the completed ASCII token.
