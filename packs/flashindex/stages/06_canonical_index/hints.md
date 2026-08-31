# Hint 1

The posting answers whether a file contains a token, not how many times. Repeated occurrences from one path must collapse to one membership.

# Hint 2

Canonical output needs ordering at both levels: token records and the paths inside each record. Do not rely on discovery order for either one.

# Hint 3

An ordered set such as `BTreeSet<PathBuf>` can combine deduplication and path ordering. Pairing it with an ordered token map gives the formatter a canonical iteration order.
