# Stage 12 — Measure parallel speedup

## Goal

Measure the same indexing workload at one, two, four, and eight threads and achieve a useful speedup without weakening byte-identical correctness.

Parallel code is not finished merely because it uses threads. It should demonstrate that the added coordination pays for itself on the supplied workload.

## Background

Suppose one worker indexes the benchmark corpus in 800 milliseconds and eight workers finish it in 200 milliseconds. The speedup is:

```text
800 ms ÷ 200 ms = 4×
```

That is useful improvement, but it is not the idealized 8×. Some work remains serial: discovering files, starting workers, combining partial indexes, sorting output, and printing bytes. Parallel overhead also consumes time.

This limit is often described by Amdahl's law: the portion that cannot run in parallel places a ceiling on the total speedup. The law does not tell you how to structure this program. It explains why adding workers eventually stops helping.

Contention can lower the ceiling further. If every token occurrence waits for one shared lock, the workers spend part of their time taking turns. If the merge repeats most of the original work, parallel tokenization may be lost in serial combination.

DeltaForge measures every declared thread count separately. It compares the median at one thread with the median at eight and applies the pack's performance gate. The threshold asks for evidence of useful parallelism, not an unrealistic perfect scaling curve.

Correctness remains the first gate. A fast index with missing or reordered postings is not an optimization of the same program.

## Requirements

Keep the threaded `index` command and byte-identical output for every thread count.

Run:

```console
deltaforge bench
```

The bundled matrix measures `--threads 1`, `2`, `4`, and `8`. The derived `speedup_1_to_8` must be at least `1.5x` for the supplied benchmark corpus before `deltaforge next` can advance, unless performance-gate enforcement has been explicitly disabled in project configuration.

## Example

A possible result might look like:

```text
threads=1  820.00 ms
threads=2  470.00 ms
threads=4  300.00 ms
threads=8  240.00 ms
speedup_1_to_8: 3.42x
```

Your numbers will differ. The relationship between the measurements, not these sample values, is the contract.

## Edge cases

- All measured thread counts produce the same canonical bytes.
- A fast result cannot compensate for failed correctness tests.
- Speedup is derived from matching one-thread and eight-thread median measurements.
- Measurements from different machines may not be directly comparable.

## Success criteria

All `deltaforge test` cases pass, `deltaforge bench` completes the full matrix, and the real `speedup >= 1.5` performance gate passes.

Afterward, record where time remains at eight threads: file discovery, tokenization, merging, sorting, output, or something else. The observation is more valuable than a claim that threads are always faster.

## Non-goals

- Perfect linear scaling.
- Choosing a universally optimal worker count.
- Hiding benchmark variance or weakening the corpus.
- Changing output to reduce the amount of required work.
