# Hint 1 — Observation

Using the three names `main`, `main_loop`, and `domain`, look at how many lines a plain
search for the four letters `main` would turn up, versus how many lines have `main` as
the complete printed token in tokenize's output. Those two counts are not the same, and
the difference between them is exactly what an exact-token search must remove.

# Hint 2 — Concept

Exact search is a filter over occurrences that already exist, not a new way of reading
source text: for each occurrence tokenize would produce, it asks whether that
occurrence's complete token is identical to the requested word. Because the question is
about the whole token rather than any part of it, and because tokenize and search must
never disagree about where one name ends and the next begins, both commands need to rely
on the same boundary rule and the same notion of case.

# Hint 3 — Experiment

Take a small file containing `main`, `main_loop`, and `domain`, and write out, by hand,
every occurrence tokenize would report for it. Cross out every occurrence whose token
text is not exactly the four letters `main`, and see what remains. Repeat the exercise
querying for `Main` instead, and notice what the case difference does to your remaining
list.

# Hint 4 — Structure

Let `search` reuse the same occurrence-producing logic as `tokenize` rather than writing
a second reader. Add one narrow step in between: keep only the occurrences whose token
field is equal, byte for byte, to the requested argument, using ordinary `==` comparison
on the token text. Reuse tokenize's line-formatting function so the two commands can
never print an occurrence differently, and treat a missing token argument or an
unreadable root as an argument-parsing error that happens before any file is touched.

# Hint 5 — Retrospective

After your tests pass, compare producing every occurrence in the corpus and discarding
the ones that do not match against checking each token against the query the moment it
is recognized, keeping only matches. The first approach keeps tokenizing and filtering
as two separately testable steps; the second never holds an occurrence in memory that it
is about to throw away. For a small project the two cost nearly the same amount of work;
for a corpus with millions of occurrences and only a handful of matches, materializing
every occurrence before filtering is the more wasteful habit.
