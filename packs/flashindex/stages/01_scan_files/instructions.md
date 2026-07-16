# Stage 01 — Scan files

## Goal

Build `flashindex scan <path>` so it recursively discovers every regular file beneath
the supplied root and prints one deterministic, portable path per line.

At the end of this stage, a command such as:

```console
$ flashindex scan project
```

will print a stable list of the files beneath `project`.

This does not search their contents. It gives the rest of the search pipeline a dependable answer to a more basic question: which files exist?

## Background

Every later FlashIndex capability depends on a trustworthy corpus. If discovery misses
a nested file, includes generated output, leaks machine-specific absolute paths, or
changes order between runs, indexing and search results cannot be reproduced.

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

That path includes information about one person's computer. The same project can live somewhere else on another machine. FlashIndex therefore reports paths relative to the directory it was asked to scan:

```text
src/main.rs
```

Operating systems also write paths differently. Unix-like systems use `/`, while Windows normally displays `\`. FlashIndex will use `/` in its public output on every platform so that the same project produces the same visible paths.

There is one more source of variation. A filesystem does not promise to return directory entries in alphabetical order. Even if two runs discover the same files, they may discover them in a different sequence. FlashIndex sorts the finished list before printing it.

Finally, the scanner skips four directory names: `.git`, `target`, `build`, and `node_modules`. These commonly contain version-control data, compiler output, or downloaded dependencies rather than files authored as part of the project.

This is FlashIndex's default ignore policy, not a complete or universal list. It covers several common sources of internal, generated, and downloaded files. Configurable ignore rules would be a separate feature.

## Requirements

- Accept exactly the command shape `flashindex scan <path>`.
- Visit every regular file below `<path>`, including files in nested directories.
- Print one root-relative path per line and never print an absolute path.
- Use `/` as the visible separator on every operating system.
- Sort the complete output lexicographically before printing it.
- Do not print directory paths.
- Never enter a directory named `.git`, `target`, `build`, or `node_modules`, wherever it appears in the tree.
- Succeed with empty output when the root contains no included regular files.
- Return a non-zero exit code when the supplied root is missing or unreadable.

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

- Every defined behavioral check passes.
- Repeating a scan of the same tree produces byte-for-byte identical output.
- The output contains only root-relative `/`-separated regular-file paths.
- The existing scan benchmark runs successfully.

### Reading the benchmark

After `deltaforge bench`, write down the file count, fixture size, median time, and p95 time. Then consider:

1. Is the fixture large enough to measure scanning rather than mostly measuring program startup?
2. How might a warm filesystem cache change a second run?
3. Why would reading every file's contents measure different work?
4. How could you make a larger but still repeatable fixture?

## Non-goals

- Deciding which file extensions are searchable.
- Reading or understanding file contents.
- Watching for later changes or scanning remote storage.
- Making the ignore list configurable.

## Capability acquired

Your program can now turn an arbitrary local project tree into a deterministic stream
of portable file identities—the stable input required by every later indexing stage.
