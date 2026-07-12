# Hint 1

The inversion changes the lookup direction: each occurrence contributes its file to the collection owned by its token.

# Hint 2

Choose ordered collections for both levels so deduplication and canonical output do not require a separate cleanup pass.

# Hint 3

A `BTreeMap<String, BTreeSet<PathBuf>>` naturally prints tokens and paths in order while collapsing repeated occurrences.
