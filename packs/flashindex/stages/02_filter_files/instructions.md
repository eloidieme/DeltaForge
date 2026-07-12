# Stage 02 — Choose searchable files

## Goal

Narrow the file list so FlashIndex keeps likely source code and project notes, while leaving out images, generated data, and other files that would add noise to a text search.

## Background

Stage 01 treated every regular file equally. That was useful for learning to walk a directory tree, but a search index needs a **corpus**: the chosen collection of documents it will search.

For example, a PNG image is a perfectly valid file, but feeding its bytes to a text tokenizer would produce nonsense. We could inspect every file's contents, guess its format, and handle text encodings. That is valuable work, but it would make this early stage much larger. FlashIndex starts with a simpler rule: keep files whose extensions are on a small allow-list.

The exact list below is a teaching choice. It represents a modest Rust/C/C++/Python project with Markdown notes and CMake configuration; it is not a claim that these are the only useful file types. `.glsl`, for example, is valid source code for graphics shaders, but it is too specialized to include without introducing that context. A real tool would normally make this list configurable.

There is no special rule for `CMakeLists.txt`. Its extension is `.txt`, so the ordinary `.txt` rule already includes it. By contrast, a file such as `toolchain.cmake` demonstrates why `.cmake` appears separately.

## Requirements

Keep the `flashindex scan <path>` command and all Stage 01 behavior: recursive scanning, ignored directories, root-relative `/` paths, errors, and sorted output.

Only print regular files with one of these extensions:

```text
.c  .cpp  .h  .hpp  .rs  .py  .md  .txt  .cmake
```

Exclude every other extension. This includes `.bin`, `.dat`, `.csv`, and common image formats, even if a particular file happens to contain readable characters.

## Example

Given this small project:

```text
mixed/
├── CMakeLists.txt
├── assets/logo.png
├── cmake/toolchain.cmake
├── include/search.hpp
└── src/main.rs
```

FlashIndex prints:

```console
$ flashindex scan mixed
CMakeLists.txt
cmake/toolchain.cmake
include/search.hpp
src/main.rs
```

`logo.png` is a real file, but it is outside the corpus we chose.

## Edge cases

- `CMakeLists.txt` is included through the normal `.txt` rule, not a filename exception.
- `.cmake`, `.md`, and `.txt` files are included just like programming-language files.
- A readable file with a disallowed extension is still excluded; this stage checks names, not contents.
- A source file inside `.git`, `target`, `build`, or `node_modules` remains excluded because Stage 01 prunes that directory first.
- Matching is case-sensitive: `NOTES.TXT` is outside this deliberately simple policy.

## Success criteria

All `deltaforge test` cases pass, including the `.cmake` example, and `scan` produces a stable corpus that every later stage can reuse.

## Non-goals

- Detecting file types by reading their contents.
- Supporting every programming language or configuration format.
- User-configurable include and exclude patterns.
- Tokenizing or interpreting the files we selected.
