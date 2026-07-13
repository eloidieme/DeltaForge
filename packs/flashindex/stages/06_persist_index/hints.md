# Hint 1

Write down what a future reader must recover: every token boundary and every path boundary. Your file format needs an unambiguous representation for both.

# Hint 2

Treat a rebuild as replacement, not append. Also decide what a valid artifact for an empty index looks like before handling the non-empty case.

# Hint 3

`fs::create_dir_all` can prepare the destination parent, and `fs::write` replaces an existing file rather than leaving stale trailing bytes. Escaping or record framing still belongs to your chosen UTF-8 format.
