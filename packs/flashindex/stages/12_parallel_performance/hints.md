# Hint 1 — Observation

Look at all four measured thread counts, not just the speedup figure. Compare how far
the time drops from one thread to two against how far it drops from four threads to
eight. If the second gap is much smaller than the first, that gap — not the thread count
itself — is telling you something worth investigating.

# Hint 2 — Concept

Total time splits into a portion that must run sequentially — discovering files, launching
workers, merging their results, sorting and printing the final index — and a portion that
genuinely runs in parallel. Adding workers can only shrink the second portion; the first
stays roughly fixed no matter how many threads are used. Amdahl's law describes the
resulting ceiling: once the serial portion is a meaningful fraction of the total, doubling
the worker count stops doubling the speed.

# Hint 3 — Experiment

Before changing anything, run the benchmark and tabulate the four medians alongside how
many times faster each is than the one-thread measurement. Then ask, for the corpus you
are using, how much of a single thread's time would go to reading files, how much to
tokenizing them, and how much to combining results — a rough hand estimate is enough. If
tokenization turns out to be a small slice of the total, no amount of worker coordination
will produce a large speedup, and that observation should come before any synchronization
code changes.

# Hint 4 — Structure

If the estimate points to coordination rather than an inherently small parallel portion,
look at what every worker touches while it runs. A single shared index guarded by one lock
— or even one lock per bucket — turns concurrent tokenization back into a queue, because
workers spend time waiting rather than working. Give each worker its own local structure
to fill, with no shared writes during the hot path, and merge the finished partial results
once, after every `thread::scope` worker has returned, rather than repeatedly touching a
shared structure. If you need to see where time actually goes while diagnosing,
`std::time::Instant` around discovery, per-worker tokenization, and the merge step will
show which phase dominates — but any such diagnostic printing has to come back out before
the byte-identical output tests run.

# Hint 5 — Retrospective

With the speedup target met, compare the per-worker granularity you chose — one file per
task, or a larger batch of files per worker — against the alternative granularity. Very
fine-grained tasks add scheduling and merge overhead per unit of work; very coarse-grained
tasks can leave some workers idle while one worker still has a large batch left. It is
also worth weighing how the merge combines partial postings: unioning already-sorted,
already-deduplicated worker-local structures costs less than re-inserting every occurrence
into a fresh shared structure one at a time. These choices matter more as the corpus
grows, since a fixed serial overhead shrinks in relative terms while contention or
re-insertion costs scale with the data.
