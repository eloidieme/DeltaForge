# Hint 1

Tokenization is a state transition between “outside a token” and “inside a token”; positions are easiest to capture at the transition in.

# Hint 2

Scan each line in byte-index order, flush a token on a separator, and treat leading digits as separators until a legal starting character appears.

# Hint 3

`char_indices` supplies byte columns; `is_ascii_alphabetic`, `is_ascii_digit`, and an optional start index are enough for this grammar.
