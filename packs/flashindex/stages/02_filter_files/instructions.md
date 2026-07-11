# Stage 02 - Filter Source Files

## Goal

Only include source-like files in scan output.

## Requirements

Your program should expose:

```bash
flashindex scan <path>
```

Print relative paths for regular files with these extensions:

```txt
.c
.cpp
.h
.hpp
.rs
.py
.glsl
.md
.txt
.cmake
```

Also include files named `CMakeLists.txt`.

Continue skipping ignored directories from Stage 01:

```txt
.git
target
build
node_modules
```

Ignore binary-looking files and unrelated assets.

## Success Criteria

`deltaforge test` should pass all Stage 02 tests.

## Non-goals

Do not tokenize files yet. This stage is only about deciding which files are source-like.
