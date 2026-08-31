# Hint 1 — Observation

Look at the stale fixture the tests run against, the one already containing an older
index, and read what is actually stored there before the command touches it. Then look at
the tab-separated line shown in this stage's example: token, then paths, each field
divided from the next in exactly one place. After the command finishes on that stale
fixture, should any trace of what was just read still remain in the file?

# Hint 2 — Concept

A saved index is read by a different process, later, possibly by different code
entirely, so the file itself has to carry every fact — token boundaries and path
boundaries alike — with no memory of how it was written. Writing to that file is also not
the same operation as writing to a fresh one: a destination path may already exist and
already hold complete, valid content from a previous, larger run. A correct write leaves
exactly one complete, current file behind, never the current data layered over whatever
was already there.

# Hint 3 — Experiment

Before writing any code, take the in-memory canonical shape from the previous stage —
one token, several sorted paths — and write out by hand, character by character, what one
line of the file should contain, marking where the token field ends and where each path
field ends. Then do the reverse: given only that line of text, could it be split back into
the same token and the same list of paths without guessing? Finally, write out what the
file should look like for a corpus with no tokens at all — is an empty file a complete,
valid answer, or does something still need to exist?

# Hint 4 — Structure

Separate three responsibilities: turning the canonical map into exact bytes, preparing
the destination location, and performing the write itself. For the first, decide once on
a field separator and a record terminator and apply it uniformly, whether a record has one
path or several. For the second, `Path::parent` combined with `fs::create_dir_all` handles
a destination whose containing directories do not exist yet. For the third, `fs::write`
replaces a file's entire contents in one call, which matters directly for the stale-file
case; building the content incrementally instead, `fs::File::create` paired with
`io::BufWriter` and `Write::write_all` gives the same replace-not-append guarantee.

# Hint 5 — Retrospective

Once the tests pass, compare assembling the entire file contents as one `String` before
a single `fs::write` against streaming records one at a time through a buffered writer.
Building the whole string first is simple and easy to reason about, but it means holding a
second full copy of the index in memory at the moment of writing, alongside the map that
produced it. Streaming keeps memory bounded regardless of corpus size, at the cost of a
little more bookkeeping and a write path where a mid-stream error leaves a harder question
about what was already flushed. Neither choice gives crash-safety on its own; that is a
separate guarantee this stage does not promise.
