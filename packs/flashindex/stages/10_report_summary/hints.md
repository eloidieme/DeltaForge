# Hint 1 — Observation

Run `summary` against a small project you can also inspect by hand, then read every line
it prints. Compare the `files` count with how many source files you can actually count in
that tree, and compare `tokens` with `unique_tokens`. Notice that the second of those two
never exceeds the first, and ask yourself what has to be true about a word for it to raise
one counter without raising the other.

# Hint 2 — Concept

The three lines answer three different questions about the same corpus. `files` measures
how many documents were selected, independent of their contents. `tokens` measures the
total amount of identifier-like material, counting a word again every time it appears.
`unique_tokens` measures the size of the vocabulary: how many distinct spellings occur at
least once, no matter how often each one repeats.

# Hint 3 — Experiment

Before touching the summary command, take a short snippet with a couple of repeated words
— for example two small functions where one identifier appears in both — and list every
token in the order it would be produced. Mark each entry as either a spelling already seen
or a new one, then count the marks two ways: once counting every entry, once counting only
the new ones. Those two hand-computed numbers are exactly what `tokens` and
`unique_tokens` must report for that snippet.

# Hint 4 — Structure

Build the summary on top of the same corpus selection and tokenization the earlier stages
already established, rather than writing a second scanner. Feed the resulting stream of
token occurrences into two separate accumulators: one that increments for every
occurrence, and one backed by a set — a `HashSet<String>` works well — that only grows
when an insert reports the spelling as new. Keep this counting logic distinct from the
printing step, which does nothing more than format three already-computed integers into
the fixed three-line layout. Because the output never lists the tokens themselves, the
set's iteration order never matters, only its final size.

# Hint 5 — Retrospective

Now that the counters agree with the corpus, compare accumulating a set incrementally
against the alternative of collecting every token into a vector, sorting it, and counting
distinct runs. The incremental set touches each occurrence once and needs no extra sorting
pass, while sorting first pays an `O(n log n)` cost for an order the summary never needs
to display. The tradeoff is memory against simplicity: a hash set holds one entry per
distinct spelling, which is normally far smaller than the occurrence stream, so the extra
memory it uses buys back the time a sort would spend. On a corpus with millions of
occurrences but a modest vocabulary, that difference is the whole reason one approach
stays fast while the other does not.
