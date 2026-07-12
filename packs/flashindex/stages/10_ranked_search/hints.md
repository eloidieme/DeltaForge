# Hint 1

Relevance here is a small tuple, not a single number. For each candidate file
you need three facts: how many distinct query tokens it contains, how many total
query-token occurrences it has, and its path. Ranking is just sorting files by
that tuple with the right directions.

# Hint 2

Walk the query-token occurrences per file and accumulate a set of matched tokens
(its size is the coverage) and a running occurrence count. Split the query on
whitespace into a set so duplicate query tokens collapse and give you `Y`.

# Hint 3

Sort by `(distinct_matched desc, occurrences desc, path asc)`. In Rust,
`slice::sort_by` is stable, so starting from a path-sorted collection (iterate a
`BTreeMap` keyed by path) and comparing only the first two keys already leaves
ties in ascending-path order; comparing the path last makes it explicit. Take
the first 10 with `iter().take(10)`.
