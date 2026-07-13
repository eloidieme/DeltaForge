# Stage 02 — Choose searchable files

## Goal

Teach `flashindex scan` to keep files that are likely to contain source code or project notes and leave unrelated assets out of the search corpus.

Stage 01 answered “which files exist?” This stage answers the narrower question that a search engine actually needs: “which of those files should we read as text?”

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

Stage 01 reports all five files. That is an accurate description of the directory, but it is not yet a useful search corpus.

`main.rs`, `README.md`, and `toolchain.cmake` contain text a programmer may want to search. `logo.png` and `records.bin` contain other kinds of data. Attempting to treat arbitrary binary bytes as source text can produce decoding errors or meaningless tokens.

FlashIndex therefore needs a selection rule.

One possibility would be to open every file and guess its format from its contents. That can be useful, but it introduces encoding detection and file-signature rules before we have built the search engine itself. For this project, FlashIndex uses a smaller policy: it accepts a fixed list of filename extensions.

The selected collection is called the **corpus**. A corpus is simply the body of documents a search system has agreed to search.

The extension list is a teaching choice. It represents a modest project containing Rust, C, C++, Python, Markdown, plain text, and CMake files. It is not a list of every language worth searching. A production tool would usually let its user change the policy.

There is no special case for `CMakeLists.txt`. Its extension is `.txt`, so the ordinary text-file rule already admits it. A file named `toolchain.cmake`, on the other hand, needs the separate `.cmake` rule.

## Requirements

Keep the `flashindex scan <path>` command and all Stage 01 behavior: recursive traversal, ignored directories, root-relative `/` paths, errors, and sorted output.

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

- `CMakeLists.txt` is accepted through its ordinary `.txt` extension.
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
