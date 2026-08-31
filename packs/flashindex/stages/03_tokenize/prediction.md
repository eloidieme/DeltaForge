# Prediction

Tokenizing reads every byte of every corpus file, where scanning only read directory
entries. Before measuring, predict how much slower `tokenize` will be than `scan` on a
comparable tree: roughly the same, a few times slower, or an order of magnitude slower.

Say which part you expect to dominate — opening and reading the files, or classifying
each byte — and what observation would tell you which one it actually was.
