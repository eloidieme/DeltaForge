# Stage 01 — Scan files

## Goal

Teach FlashIndex to discover the files inside a project.

At the end of this stage, a command such as:

```console
$ flashindex scan project
```

will print a stable list of the files beneath `project`.

This does not search their contents yet. It gives every later stage a dependable answer to a more basic question: which files exist?

## Background

Consider this small project:

```text
project/
├── README.md
├── src/
│   ├── main.rs
│   └── storage.rs
└── target/
    └── debug/
        └── flashindex
```

A person looking at this tree can quickly distinguish the project's files from its build output. `README.md`, `main.rs`, and `storage.rs` belong to the material we may eventually want to search. The executable inside `target` was produced by the compiler.

FlashIndex does not understand that distinction automatically. It initially sees only files and directories.

Its first task is to walk through the tree. When it encounters a directory such as `src`, it must look inside and continue. This is called recursive directory traversal: visiting a directory may reveal more directories that also need to be visited.

Simply finding the files is not enough. The paths must also be useful outside the machine on which they were discovered.

An absolute path might look like this:

```text
/Users/maya/projects/flashindex/src/main.rs
```

That path includes information about one person's computer. Another learner will have the project somewhere else. FlashIndex therefore reports paths relative to the directory it was asked to scan:

```text
src/main.rs
```

Operating systems also write paths differently. Unix-like systems use `/`, while Windows normally displays `\`. FlashIndex will use `/` in its public output on every platform so that the same project produces the same visible paths.

There is one more source of variation. A filesystem does not promise to return directory entries in alphabetical order. Even if two runs discover the same files, they may discover them in a different sequence. FlashIndex sorts the finished list before printing it.

Finally, the scanner skips four directory names: `.git`, `target`, `build`, and `node_modules`. These commonly contain version-control data, compiler output, or downloaded dependencies rather than files authored as part of the project.

This is a small policy chosen for the course. It is not a complete or universal ignore list. Keeping it fixed lets this stage focus on traversal; configurable ignore rules can remain a separate problem.

## Requirements

Add the command `flashindex scan <path>`.

It must visit every regular file below `<path>` and print one path per line. Each printed path must:

- be relative to the supplied root, never an absolute path;
- use `/` as its separator on every operating system; and
- appear in lexicographic (dictionary-like) order.

Do not print directories. Do not enter a directory named `.git`, `target`, `build`, or `node_modules`, even when that directory is nested inside another folder. A missing or unreadable root must produce a non-zero exit code.

## Example

For the project shown above, the result is:

```console
$ flashindex scan project
README.md
src/main.rs
src/storage.rs
```

The paths are relative to `project`, they use `/`, and they appear in sorted order. Nothing inside `target` is printed.

## Edge cases

- A file several folders deep is still printed relative to the supplied root.
- An ignored directory is skipped wherever it appears, not only at the root.
- An empty directory succeeds and prints nothing.
- Filesystem discovery order must not change the final sorted output.

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
