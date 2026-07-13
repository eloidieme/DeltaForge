# Hint 1

Search should consume the same token occurrences as `tokenize`. A second character-scanning rule is an opportunity for the two commands to disagree.

# Hint 2

Filter occurrences by complete token equality only after their boundaries and positions have been established.

# Hint 3

If your tokenizer already returns a structured occurrence containing path, line, column, and token, `search` can retain matching records and reuse the same formatter.
