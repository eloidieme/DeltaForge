# Hint 1 — Observation

Run the benchmark command on the same small project twice and place the two lines of
output side by side. Which field should stay identical between the two runs, and which one
is allowed to differ? Then look at exactly what stdout contains, character by character —
is there anything at all before the opening brace or after the closing one, even a blank
line?

# Hint 2 — Concept

This command reports two facts of a different nature bundled into one small object: how
much work was selected, a property of the corpus that would not change between runs, and
how long that work took, a property of this particular run on this particular machine at
this particular moment. Measuring the second kind of fact safely means using a clock built
for measuring an interval rather than one built for telling the current time, since a
clock of the second kind can jump backward or forward for reasons that have nothing to do
with how much work happened. Because another program is expected to parse this output
directly, the stream also has to be exactly one thing: valid JSON, with nothing decorative
mixed in.

# Hint 3 — Experiment

Before writing the command, take the file-selection rule already established for
indexing and count, by hand, how many files in a small fixture qualify as source-like
versus how many are assets that should be excluded — compare that number against what
`files` should report. Separately, think through where a timing interval should start and
stop: if measuring began before checking whether the given root even exists, what would
the reported duration mean for a root that turns out to be unreadable? Decide on paper
which steps belong inside the interval and which belong outside it before writing the
timing code.

# Hint 4 — Structure

Separate validating the root, measuring, and reporting into distinct steps. Resolve and
check the root first, and exit with a failure before any timing begins if it cannot be
read, so a bad path never produces a success-shaped object. Start `Instant::now()`
immediately before running the same selection logic earlier stages already built, call
`.elapsed()` immediately after it finishes, and convert that duration with `as_millis()`
to get a plain non-negative integer. With both integers in hand, build the two-field JSON
text directly — the shape is fixed and small enough not to need a general serialization
approach — and print it with a single call so nothing else reaches stdout alongside it.

# Hint 5 — Retrospective

Once the tests pass, compare formatting this fixed, two-field object by hand against
reaching for a general-purpose serialization approach. Hand formatting costs nothing
extra, is easy to verify byte for byte against the exact shape the tests expect, and asks
nothing of a dependency-free project — but it only stays simple because the shape has
exactly two required integer fields and no nesting. It is also worth separating what this
number actually measures: timing only the scan, rather than the whole command including
argument handling and printing, keeps the reported duration attached to the one operation
the object claims to describe. A future field, or a shape with optional values, would make
hand-formatting considerably more fragile than it looks here.
