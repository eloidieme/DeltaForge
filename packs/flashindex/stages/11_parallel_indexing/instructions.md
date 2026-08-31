# Stage 11 — Build the index with several workers

## Goal

Allow the caller to choose a positive number of worker threads while keeping the finished index byte-for-byte identical to the canonical single-threaded result.

The first requirement of parallel indexing is correctness: scheduling may change, but the index may not.

## Background

Imagine a corpus with four source files. Tokenizing one file does not require the contents of the other three, so different workers can perform that part of the job independently.

The difficulty appears when their results meet.

Suppose two workers both find `retry`:

```text
worker A → retry in src/main.rs
worker B → retry in src/network.rs
```

The final index needs one canonical posting:

```text
retry src/main.rs src/network.rs
```

Workers do not finish in a predictable order. If output depends on whichever worker returns first, paths may change order between runs. If every worker mutates one shared index during tokenization, access must be coordinated, and that coordination can make the program harder to reason about.

A useful way to think about the problem is to separate independent work from combination. Each worker may produce partial evidence. A later merge produces the same sorted, deduplicated structure as canonical indexing. The exact design remains yours; the observable requirement is that scheduling never changes the bytes.

More threads than files is not an error. Some workers may simply receive no file. Zero threads, however, describes no possible workforce and must be rejected clearly.

## Requirements

Extend `index` with:

```console
flashindex index <path> --threads <N>
```

`<N>` must be a positive integer. For every valid value, stdout must be byte-identical to `flashindex index <path>` for the same corpus: the same token lines, sorted and deduplicated paths, portable separators, and final newline behavior.

The worker count is an execution choice only. Do not print it in the index. Reject zero and non-numeric values with a non-zero exit and an error containing `positive integer`.

## Example

These commands must print exactly the same bytes:

```console
$ flashindex index project
$ flashindex index project --threads 1
$ flashindex index project --threads 4
```

Only the way the work is scheduled may differ.

## Edge cases

- One worker matches the canonical non-threaded command.
- Several workers finish in unpredictable order without changing output order.
- More workers than files still indexes every file exactly once.
- Zero workers is rejected.
- A non-numeric worker count is rejected.

## Success criteria

All `deltaforge test` cases pass and repeated runs across several valid thread counts remain byte-identical.

## Non-goals

- Meeting a speed target.
- Printing worker logs or timing information in index output.
- Requiring a particular work queue, locking strategy, or merge algorithm.
- Changing tokenization or index semantics.
