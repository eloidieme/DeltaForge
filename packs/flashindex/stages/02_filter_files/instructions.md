# Stage 02 — Filter source files

## Goal

Refine scanning so FlashIndex reports files that plausibly contain searchable source or project text while excluding unrelated assets and binary-looking artifacts.

## Background

Indexing everything is rarely useful. Information-retrieval systems define a corpus before they tokenize it, and developer tools often use a conservative allow-list because decoding an arbitrary binary file as text produces noise or errors. Filenames can carry meaning too: `CMakeLists.txt` is source-like despite not ending in a conventional language suffix.

## Requirements

Keep `flashindex scan <path>` and Stage 01's recursion, ignores, relative `/` paths, and sorted output. Include regular files ending in `.c`, `.cpp`, `.h`, `.hpp`, `.rs`, `.py`, `.glsl`, `.md`, `.txt`, or `.cmake`, plus files named exactly `CMakeLists.txt`. Exclude every other suffix, including `.bin`, `.dat`, images, and `.csv`.

## Example

```console
$ flashindex scan mixed
CMakeLists.txt
README.md
include/search.hpp
src/main.rs
```

## Edge cases

- `CMakeLists.txt` is included by exact filename.
- Text-bearing allowed extensions such as `.md` and `.txt` are included.
- Binary-looking and unrelated asset extensions are excluded even when their contents are readable.
- Ignored directories remain excluded regardless of a file's extension.

## Success criteria

All `deltaforge test` cases pass and the scan output defines a deterministic corpus for every later stage.

## Non-goals

- Detecting file type from contents or supporting every programming language.
- Tokenizing, parsing, or interpreting included text.
- User-configurable include/exclude patterns.
