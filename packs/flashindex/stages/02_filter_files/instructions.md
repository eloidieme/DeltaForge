# Stage 02 — Choose searchable files

## Goal

Restrict `flashindex scan` to files that belong in the searchable corpus.

Directory traversal answers “which files exist?” Corpus selection asks the narrower question a search engine needs: “which of those files should we read as text?”

## Background

Consider this project:

```text
project/
├── README.md
├── assets/
│   └── logo.png
├── cmake/
│   └── toolchain.cmake
├── data/
│   └── records.bin
└── src/
    └── main.rs
```

An unrestricted scan reports all five files. That is an accurate description of the directory, but it is not a useful search corpus.

`main.rs`, `README.md`, and `toolchain.cmake` contain text a programmer may want to search. `logo.png` and `records.bin` contain other kinds of data. Attempting to treat arbitrary binary bytes as source text can produce decoding errors or meaningless tokens.

FlashIndex therefore needs a selection rule.

One possibility would be to open every file and guess its format from its contents. That requires encoding detection and file-signature rules. FlashIndex instead uses a fixed list of filename extensions.

The selected collection is called the **corpus**. A corpus is simply the body of documents a search system has agreed to search.

The allow-list covers Rust, C, C++, Python, Markdown, plain-text, and CMake files. It is a product boundary, not a judgment about every other language or format. A configurable search tool could expose the list to its user; FlashIndex keeps it fixed so all commands share one corpus definition.

## Requirements

Keep the `flashindex scan <path>` command with recursive traversal, ignored directories, root-relative `/` paths, errors, and sorted output.

Only print regular files with one of these case-sensitive extensions:

```text
.c  .cpp  .h  .hpp  .rs  .py  .md  .txt  .cmake
```

Exclude every other extension. This includes `.bin`, `.dat`, `.csv`, and common image formats even when an individual file happens to contain readable characters.

## Example

For the project above:

```console
$ flashindex scan project
README.md
cmake/toolchain.cmake
src/main.rs
```

The PNG and binary-data files still exist. They are absent only because they do not belong to the corpus FlashIndex will tokenize.

## Edge cases

- `.cmake`, `.md`, and `.txt` files are accepted alongside programming-language files.
- A readable file with a disallowed extension remains excluded because this stage examines names, not contents.
- A source file inside `.git`, `target`, `build`, or `node_modules` remains excluded because the scanner never enters that directory.

## Success criteria

All `deltaforge test` cases pass and repeated scans of the same tree produce the same selected corpus.

## Non-goals

- Detecting file types from their contents.
- Supporting every programming language or configuration format.
- User-configurable include and exclude patterns.
- Reading or tokenizing the selected files.
