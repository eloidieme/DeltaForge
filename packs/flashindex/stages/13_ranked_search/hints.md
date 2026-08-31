# Hint 1

For each file, keep two different quantities: the set of distinct query tokens it matched and the total number of matching occurrences.

# Hint 2

Deduplicate the query before scoring. Coverage is the size of the matched-token set; density is the occurrence counter.

# Hint 3

A path-keyed map can accumulate one score record per candidate file. Sort by the two descending numeric keys and leave exact numeric ties unspecified.
