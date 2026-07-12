# Hint 1

Exact search is a filter over token occurrences, not another pass with different boundary rules.

# Hint 2

Reuse one tokenizer result and retain occurrences whose token equals the query before invoking the existing formatter.

# Hint 3

A direct `occurrence.token == query` comparison preserves case sensitivity and prevents substring matches such as `main_index`.
