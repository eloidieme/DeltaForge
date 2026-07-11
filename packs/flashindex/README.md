# FlashIndex

## What you are building

FlashIndex is a local source-code search engine. By the end, you will have a command-line tool that can scan a project, filter source files, tokenize identifiers, search for exact symbols, build an inverted index, persist it, benchmark indexing work, and summarize what was indexed.

## Why this is useful

Fast local code search is a realistic systems problem. It touches directory traversal, file filtering, parsing, data structures, persistence, CLI design, and performance measurement. These are the same building blocks behind developer tools, language servers, indexing services, and build systems.

## Big picture

The project starts with a simple scanner and gradually turns it into a small search engine:

1. Recursively scan files.
2. Filter source-like files.
3. Extract identifier-like tokens with line and column positions.
4. Search for exact token matches.
5. Build an inverted index.
6. Persist and query the index.
7. Expose benchmark output.
8. Produce a summary report.

## What good looks like

Good solutions keep output deterministic, separate scanning from tokenization, avoid reading obviously irrelevant files, and use benchmarks to guide optimization rather than guessing.
