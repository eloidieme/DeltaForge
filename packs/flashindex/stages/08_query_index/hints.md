# Hint 1 — Observation

Run a query for a token that is stored and a query against an index file that does not
exist at all, then compare the exit code and stdout of each. Now compare a query for
`durable` against one for `durable_token`, when only `durable_token` is actually stored:
if the matching step only checks whether the requested text appears somewhere inside a
line, would `durable` still seem to match?

# Hint 2 — Concept

Two different questions are being asked, and they must not share one answer: can this
artifact be read at all, and does the token exist inside it. The first is about the
process's ability to open and parse the file; failing it is an error. The second is about
whether a particular key is present in otherwise valid data; failing it is simply an empty
result. A related distinction sits inside matching itself: comparing a candidate token
against one complete, delimited field is different from asking whether it occurs anywhere
in the raw text of a line.

# Hint 3 — Experiment

Write out, by hand, two lines of a saved index in the exact tab-separated shape from the
previous stage, one of them for the token `durable_token`. Now simulate two different
readers against that text: one that treats a line as matched whenever the query text
appears anywhere inside it, and one that first splits the line into fields and compares
only the first field for exact equality. Check both readers against the queries `durable`,
`token`, and `durable_token`, and note where the two readers disagree.

# Hint 4 — Structure

Separate reading the artifact from searching it. One step reads the whole file, handling
the case where the path cannot be opened or read as the error path for this command,
independent of what the token search finds afterward. A second step works line by line,
using something like `str::split` or `str::split_once` on the tab byte to isolate the
token field from the remaining path fields, comparing the token field with `==` rather
than a substring check. On a match, the remaining fields are already the sorted paths from
persistence and need only to be printed one per line; on no match across every line, empty
output with a normal exit is the correct and complete result.

# Hint 5 — Retrospective

Once the tests pass, compare scanning the file line by line for the one requested token
against parsing the whole file into an in-memory map first, the way the index was built
before it was saved. A line-by-line scan needs no structure beyond the current line and
touches only as much of the file as necessary before it finds or exhausts its target. A
full parse pays an upfront cost proportional to the entire file on every single query, but
would pay off if one process needed to answer many queries against the same index without
rereading it. For a command that opens the file once and asks one question, the two
approaches cost about the same; the difference would show up in a very large index queried
very often.
