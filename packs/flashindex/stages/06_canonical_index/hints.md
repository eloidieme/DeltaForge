# Hint 1 — Observation

Index a file where one token, say `retry`, occurs several times, and look at the posting
line for that token. Count how many times the containing path is listed there against how
many times the token actually occurs in the file. Then look at a corpus with tokens like
`alpha`, `Beta`, and `gamma`: is the order in which token lines appear, and the order of
paths inside each line, something the filesystem handed over, or does it look arranged?

# Hint 2 — Concept

A posting answers a yes-or-no membership question — does this file contain the token —
not a count of occurrences. The twenty places `retry` appears in one file must collapse to
a single entry, not twenty. Two further relationships need a fixed shape: which token
comes first among all discovered tokens, and which path comes first among the files
containing one token. Neither relationship should depend on the order the scanner visited
files, or the order an unordered collection happens to hand back its keys.

# Hint 3 — Experiment

Before touching the implementation, take a small corpus by hand: three files, one token
repeated across two of them and once more within a single file. Write down the raw list of
(token, path) pairs exactly as tokenization would discover them, including repeats. Then
reduce that list to the canonical form the tests expect — one entry per token, each with
its deduplicated, sorted paths — and compare how different the two lists are for a token
that repeats heavily versus one that appears only once.

# Hint 4 — Structure

Separate accumulation from formatting. One structure can map each token to the set of
paths that contain it, built up as tokenization proceeds; because it only needs membership
plus a fixed order, an ordered set such as `BTreeSet<PathBuf>` handles deduplication and
sorting in the same step, and an ordered map such as `BTreeMap<String, BTreeSet<PathBuf>>`
keeps tokens in that same fixed order. A later, separate pass walks that structure in order
and writes each token followed by its already-sorted paths. Keep this pass unaware of how
the map was populated, since its only job is to render a shape that is already canonical.

# Hint 5 — Retrospective

Once the tests pass, compare building an ordered set as tokens are discovered against
collecting every (token, path) pair into a plain vector and sorting and deduplicating it
once at the end. The ordered-set approach keeps memory proportional to the number of
distinct relationships at every point, while the collect-then-sort approach can briefly
hold one entry per raw occurrence before it shrinks. For a token that appears in a handful
of files the difference is invisible; for a corpus where a common token like `fn` occurs in
most files, or a project large enough to produce millions of occurrences, the size of that
intermediate structure — and how early duplicates get discarded — starts to matter.
