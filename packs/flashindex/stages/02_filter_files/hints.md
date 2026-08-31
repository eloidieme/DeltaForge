# Hint 1 — Observation

Run `scan` on a project that mixes source files with images and other data, and compare
its current output with the shorter list this stage's example expects. The files that
must disappear are not deeper, larger, or named oddly — look at the one part of each
name that differs between the files that stay and the files that go.

# Hint 2 — Concept

Corpus selection is a single yes-or-no test applied to every regular file traversal
already finds: does this file belong to the body of text FlashIndex is willing to read?
The test is a fixed allow-list of extensions, not a judgment about a file's actual
contents, so two files with identical readable text can land on opposite sides of the
line depending only on how their names end.

# Hint 3 — Experiment

Before writing a filter, list the extensions of a handful of files from a fixture by
hand — some allowed, some not, and at least one with an uppercase or doubled suffix such
as `NOTES.RS` or `notes.rs.bak`. Check each one against the fixed list of extensions in
the requirements and mark it in or out. Notice which mismatches are about case and which
are about there being no allowed suffix at all.

# Hint 4 — Structure

Keep traversal exactly as it already works, and add one small classification step that
each discovered regular file passes through before it is collected for output: admitted,
or not. In Rust, `Path::extension` gives you the suffix as an optional OS string;
convert it to `str` and compare it against the fixed, lowercase, case-sensitive list
from the requirements rather than testing with a manual string search. Keep this
decision separate from the ignored-directory check from the previous stage — one governs
which directories are entered, the other governs which files are kept once found.

# Hint 5 — Retrospective

Once the tests pass, compare checking each extension against a short `match` or array of
literals with looking it up in a `HashSet`. For a half-dozen fixed extensions the two
perform indistinguishably, and the match keeps the allow-list visible as ordinary code.
Also consider what `Path::extension` gives back for a name like `archive.tar.gz` or a
dotfile with no suffix at all, and how that differs from splitting the file name on `.`
yourself — the standard-library method already encodes a definition of "extension" that
matches what most tools mean by the word.
