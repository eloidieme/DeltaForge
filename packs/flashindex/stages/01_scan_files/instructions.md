# Stage 01 — Scan files

## Goal

Teach FlashIndex to look through a project and list its files. This is the smallest useful piece of a search tool: before it can search words, it needs to know which files exist.

## Background

Think of a project as a tree. The project folder is the trunk, its folders are branches, and files are the leaves. A **directory scan** walks that tree and collects the leaves.

The operating system does not promise to hand directory entries to a program in the same order every time. Windows also writes paths with `\`, while Unix-like systems use `/`. FlashIndex therefore sorts the finished list and displays every path with `/`. That makes its output predictable for people, tests, and scripts on any supported system.

Real projects contain folders that are not part of the code a person wrote. For this course, FlashIndex skips four names:

- `.git` stores version-control history and internal data.
- `target` is the usual Rust build-output folder.
- `build` is a common build-output folder in C and C++ projects.
- `node_modules` contains downloaded JavaScript packages and can hold thousands of files.

This is a deliberately small **project policy**, not a universal law. A production search tool would usually let people change the ignore list. Here, a fixed list lets us learn traversal without also building a configuration system.

## Requirements

Add the command `flashindex scan <path>`.

It must visit every regular file below `<path>` and print one path per line. Each printed path must:

- be relative to the supplied root, never an absolute path;
- use `/` as its separator on every operating system; and
- appear in lexicographic (dictionary-like) order.

Do not print directories. Do not enter a directory named `.git`, `target`, `build`, or `node_modules`, even when that directory is nested inside another folder. A missing or unreadable root must produce a non-zero exit code.

## Example

Suppose `project` looks like this:

```text
project/
├── README.md
├── src/
│   └── main.rs
└── target/
    └── debug/app
```

Then the command prints only the two project files:

```console
$ flashindex scan project
README.md
src/main.rs
```

The `target` file is absent because the whole directory was ignored.

## Edge cases

- A file several folders deep is still printed relative to the supplied root.
- An ignored directory is skipped wherever it appears, not only at the root.
- An empty directory succeeds and prints nothing.
- Filesystem discovery order must not change the final sorted output.
- Symbolic links and special files are not regular files and are not followed.

## Success criteria

All `deltaforge test` cases pass, every printed path is portable and ordered, and the existing scan benchmark runs successfully.

### Reading the benchmark

After `deltaforge bench`, write down the file count, fixture size, median time, and p95 time. Then consider:

1. Is the fixture large enough to measure scanning rather than mostly measuring program startup?
2. How might a warm filesystem cache change a second run?
3. Why would reading every file's contents measure different work?
4. How could you make a larger but still repeatable fixture?

## Non-goals

- Deciding which file extensions are searchable. That comes next.
- Reading or understanding file contents.
- Watching for later changes or scanning remote storage.
- Making the ignore list configurable.
