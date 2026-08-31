# Hint 1 — Observation

Run `rank` against a query where two files score identically on both existing numbers,
several times if needed, and watch whether the relative order of those two lines ever
changes between runs. Separately, run a query that matches more than ten files and count
how many lines actually print. Notice which of these two behaviors is currently left to
chance and which is a fixed count that has nothing to do with chance.

# Hint 2 — Concept

Coverage and occurrence count are real measurements, but two different files can
legitimately produce the same pair of numbers, and a sort that stops there leaves those
files in whatever order happened to reach it. A complete ordering needs one more
comparison that can never itself tie for two distinct files — an ordering over their paths
works, because no two files share a path. Separately, a maximum result count is a decision
about how much of an already fully decided order to show, which is a different question
from how that order gets decided.

# Hint 3 — Experiment

Before changing the comparison logic, take two files that tie on both existing scores and
write their relative portable paths side by side. Decide by hand which should come first
under ascending order, and confirm that decision does not depend on which file happened to
be discovered first. Separately, take a list of eleven or twelve hand-scored candidates
where the weakest scores do not belong to the alphabetically last entries, sort the full
list by hand using all three rules, and only then cross off everything past the tenth line
— check that the surviving ten differ from what you would get by cutting the list to ten
before sorting by path.

# Hint 4 — Structure

Express the ordering as one comparison that chains three rules with `Ordering::then_with`:
descending matched-token count, then descending occurrence count, then ascending path.
`PathBuf`'s own `Ord` implementation already compares paths component by component, which
lines up with a portable, unambiguous path ordering. Reversing the first two comparisons,
so a larger number sorts earlier, is exactly what `std::cmp::Reverse` or calling
`.reverse()` on the resulting `Ordering` provides, without changing how the path
comparison reads. Sort the complete candidate list with this one comparator using
`slice::sort_by`, and only after that full sort is finished should the first ten entries
be taken and numbered starting at one; truncating or calling `Iterator::take` any earlier
would let the limit interfere with a rule it has no say over.

# Hint 5 — Retrospective

With every pair of candidates now distinguishable, compare the tuple-based comparator you
likely wrote against sorting by a derived key such as `(Reverse(matched),
Reverse(occurrences), path.clone())`. A key-based sort can be easier to read but pays for
cloning or copying each key, while chaining comparisons with `then_with` inspects the
fields in place and only computes the occurrence and path comparisons when an earlier one
ties. It is also worth noticing that, because the three-part comparison is now a genuine
total order, `slice::sort_unstable_by` would produce the same result as a stable sort —
the earlier need for stability existed only while ties were still possible. For a result
set with thousands of candidates rather than a handful, that difference in comparator cost
and algorithm choice would start to matter more than it does here.
