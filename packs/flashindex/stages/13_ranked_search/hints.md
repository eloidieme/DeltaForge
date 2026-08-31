# Hint 1 — Observation

Run `rank` with a three-token query against files that match different numbers of those
tokens, and look at the two numbers printed next to each file. Find a pair of files with
the same matched-token count but different occurrence counts, and see which one is listed
first. Then find a file that repeats one query word many times but touches only that
single token, and see where it lands relative to a file that touches more distinct tokens
with fewer total occurrences.

# Hint 2 — Concept

A multi-token query raises two separate questions about a candidate file: how much of the
question does it answer, and how much evidence does it offer for the parts it answers?
Coverage counts distinct query tokens found in the file, treating a duplicated word in the
query as asking one thing rather than two. Occurrence count adds up every matching
appearance in the file, including repeats, and only comes into play once coverage has
decided the top-level order.

# Hint 3 — Experiment

Before writing any scoring logic, pick a query with a repeated word — the same token twice
— and compare two files where one contains that word once and the other contains it three
times. By hand, decide what the coverage number should be for each file, and what the
occurrence number should be. Write down the expected rank order for the two files, then
check that it matches what should happen once the query has been deduplicated before
counting.

# Hint 4 — Structure

Split the query on whitespace and collect the pieces into a set, which gives both the
deduplicated query tokens and the denominator for the `matched X/Y` text. While scanning
the corpus's token occurrences, accumulate a record per candidate file — a map keyed by
path to a pair of counters works well — where the occurrence counter increments on every
matching appearance, but the coverage counter only increments the first time a given query
token is seen for that file, which itself needs a small per-file set of already-counted
tokens. Once every file's record is complete, collect the map into a vector and sort it by
the two descending numeric fields, without a third tie-breaker yet since exact ties are
unspecified at this stage. Keep the scoring pass and the formatting pass separate, so the
printed string is only ever assembled from numbers that are already finished.

# Hint 5 — Retrospective

Having the ranked list working, compare keeping a small per-file set of matched query
tokens against an alternative that tracks coverage without one, such as checking each
deduplicated query token against a file's occurrence map directly instead of visiting the
raw occurrence stream. The per-file-set approach touches one entry per occurrence but
allocates a little bookkeeping for every candidate file, while the per-token-lookup
approach revisits the query for every file but needs no extra per-file set. For a short
query and a modest number of matching files, as in this stage's corpus, the difference is
negligible; it becomes worth reconsidering only once either the query length or the number
of candidate files grows by orders of magnitude.
