# Hint 1 — Observation

Run `index` on the same corpus with different thread counts, and run the same higher
thread count more than once. Compare the raw bytes each run produces, not just whether the
program finishes without error. Notice which parts of the output — token spelling order,
path order, or path separators — would have to line up exactly for every one of those runs
to count as the same result.

# Hint 2 — Concept

Tokenizing one file needs nothing from any other file, so that work can happen
independently and in any order. The index that results, though, is a single canonical
structure — sorted token spellings, each mapped to a sorted and deduplicated list of paths
— and that structure has only one correct shape for a given corpus. The real challenge is
not making the work happen concurrently; it is making sure that however the independent
pieces finish, they still assemble into that one fixed shape.

# Hint 3 — Experiment

Before writing any threading code, take a four-file corpus and split it into two groups on
paper. Build the token-to-path postings for each group separately, as if two workers had
done the work, then merge the two partial listings by hand into one sorted, deduplicated
structure — once assuming group A's results arrive first, and once assuming group B's do.
Confirm the merged result is identical either way; that is the property a real merge step
has to preserve regardless of which thread actually finishes first.

# Hint 4 — Structure

Separate three responsibilities: validating the thread count, running independent per-file
work, and merging the results into the canonical structure. Validate `--threads` first,
rejecting zero or non-numeric input before any threads are spawned. Partition the file
list among the requested number of workers, and give each worker only the read-only inputs
it needs — `std::thread::scope` lets you spawn worker closures that borrow shared,
immutable data like the file list without an `Arc`, and each worker can simply return its
own local token-to-path map when it finishes rather than writing into anything shared.
After every worker's handle has completed, union the returned partial maps into one
structure the same way the canonical single-threaded version builds it, sorting and
deduplicating paths in the same order, so the merge decides the final bytes rather than
the number of workers.

# Hint 5 — Retrospective

Now that identical output is confirmed at every thread count, compare returning each
worker's result through its join handle against sending partial results over an
`mpsc::channel`. The join-handle approach ties every result to the thread that produced it
and reads naturally in the order the workers were spawned, while a channel lets a merge
loop start consuming results as soon as any worker finishes, at the cost of an extra queue
and needing to know when every sender is done. It is also worth weighing worker-local
accumulation, which briefly duplicates memory across workers before the merge, against a
single shared index protected by a lock, which uses less memory but forces every worker to
take turns writing. That contention is invisible in this stage's byte-identical tests, but
it is exactly the cost the next stage asks you to measure.
