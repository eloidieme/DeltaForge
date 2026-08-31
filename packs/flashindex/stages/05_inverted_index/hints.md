# Hint 1 — Observation

Run `tokenize` on a small project and look at its raw list of occurrences. Pick one
token that appears in two different files and find every line mentioning it — notice
that those lines are scattered across the output in file order, not grouped together
anywhere, even though they all describe the same name.

# Hint 2 — Concept

Every occurrence tokenize reports is a small fact: this token appears in this file. An
inverted index answers a different question from the same facts — for a given token,
which files contain it — by collecting all the facts that share a token under that
token, instead of leaving them grouped by the file that produced them. The relationship
itself does not change; only which side of it is used as the key does.

# Hint 3 — Experiment

Take the two-file example from the instructions, or a fixture where one token appears
twice in the same file, and write out every token-and-file fact by hand from its
occurrences. Group those facts by token on paper and write the line each group would
produce. When a token appears twice in the same file, decide by hand whether its file
should be written once or twice, and check that decision against how the instructions
describe the relationship — files that contain a token, not occurrences of it.

# Hint 4 — Structure

Separate collecting the token-and-file facts from grouping them and from printing the
result. The facts themselves come straight from occurrences you already know how to
produce for each file, so this stage needs no second rule for recognizing a token.
Choose a map keyed by token text whose value collects the containing file paths, and
make sure that value cannot record the same file twice for one token even when the token
occurs there several times — a `BTreeMap<String, BTreeSet<String>>` is one way to get a
stable key order and per-token file uniqueness together. Keep this grouping step
separate from formatting the final token-and-paths line.

# Hint 5 — Retrospective

Once the tests pass, compare a `BTreeMap` of `BTreeSet`s with a `HashMap` of `Vec`s that
gets sorted and deduplicated once, right before printing. The tree-based version keeps
every insertion in order as it happens, at a small per-insertion cost; the hash-based
version is cheaper to build but only produces the same output twice if the final
sort-and-dedup step is never skipped. Consider, too, what it costs to hold every token's
complete file list in memory at once, and whether that would still be reasonable for a
corpus with millions of distinct tokens rather than a handful.
