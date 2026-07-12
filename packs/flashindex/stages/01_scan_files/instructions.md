# Stage 01 — Scan files

## Goal

Walk a directory tree and print every regular file as a deterministic relative path while pruning directories that should never enter a source index.

## Background

Search begins with discovery. Unix tools such as `find` made recursive traversal a standard building block, but developer trees add noise: version-control metadata, downloaded dependencies, and compiler outputs can dwarf authored source. A scanner also crosses an operating-system boundary—Windows and Unix represent separators differently—so a portable tool needs one public path convention.

## Requirements

Expose `flashindex scan <path>`. Recursively visit regular files beneath `<path>`, print one root-relative path per line, sort all output lexicographically, and use `/` separators on every platform. Do not print absolute paths or directories. Skip any directory named `.git`, `target`, `build`, or `node_modules`, wherever it appears. An unreadable or missing root exits non-zero.

## Example

```console
$ flashindex scan project
README.md
crates/core/src/lib.rs
src/main.rs
```

## Edge cases

- Files nested several directories deep are reported relative to the supplied root.
- Ignored directory names are pruned at any depth and none of their files appear.
- Output ordering is stable even when filesystem enumeration order differs.
- An empty directory succeeds with empty stdout.

## Success criteria

All `deltaforge test` cases pass, paths are portable and ordered, and the existing scan benchmark runs successfully.

### Benchmark interpretation worksheet

After `deltaforge bench`, record file count, fixture bytes, median, and p95, then answer:

1. Is this workload large enough to distinguish traversal cost from process startup?
2. Which filesystem effects—directory cache, metadata cache, storage medium—could change a rerun?
3. Why would adding file-content reads make the benchmark answer a different question?
4. What larger fixture would remain deterministic without becoming expensive to embed and copy?

## Non-goals

- Filtering by extension or reading file contents.
- Following symbolic links or indexing special files.
- Watching for changes or traversing remote storage.
