# Hint 1 — Observation

Take the sample line `let retry_count2 = load_retry_count();` from the instructions and
put a finger on the exact character where one name ends and the next begins. Do the same
with `123alpha`: find where the printed token actually starts, and compare that position
with where the leading digits sit. The column your program prints is exactly the spot
your finger lands on.

# Hint 2 — Concept

Tokenizing means answering two narrow character questions at every byte: can this byte
start a name, and can it continue one already started? FlashIndex's rule only cares
about ASCII letters, digits, and underscore, and a digit answers the two questions
differently — no to starting, yes to continuing. Everything else, including punctuation
and whitespace, answers no to both and therefore ends whatever token was open.

# Hint 3 — Experiment

Before touching code, trace a short two-line snippet by hand, including at least one
line with a leading digit and one with an underscore-prefixed name. For every byte,
write down whether it starts a token, continues one, or ends one, and record the line
and column where each token you find actually begins. Then trace the same snippet again
as if it used the other line-ending convention, and check whether your column numbers
still match the first-byte rule.

# Hint 4 — Structure

Separate three responsibilities: walking the corpus one admitted file at a time, turning
one file's contents into a stream of line, column, and token facts, and formatting those
facts for printing. Track line numbers by counting line breaks as you go, and track
columns as byte offsets from the start of the current line, remembering the offset where
a token began so you can slice it out once a non-continuing byte appears.
`str::char_indices` gives you byte-indexed positions to build this from, and treating a
carriage return the same as any other non-letter, non-digit character keeps
CRLF-terminated files from needing special handling. Keep the classification rule itself
in one place so nothing else in the program develops its own private idea of what a
token is.

# Hint 5 — Retrospective

After the tests pass, compare scanning the file as a `String` with `char_indices`
against scanning it as a raw byte slice. Both agree for this stage's ASCII rule, but
they diverge the moment a comment or string literal contains non-ASCII bytes: reading as
a `String` requires the whole file to be valid UTF-8 and fails outright if it is not,
while treating the file as bytes lets every byte be classified on its own regardless of
encoding. Consider too how each choice would behave on a file too large to comfortably
read twice, and whether an occurrence should hold an owned copy of its token text or
merely a position into a buffer that outlives it.
