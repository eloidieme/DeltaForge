# Stage 09 — Parallel indexing

## Goal

Build the same inverted index you produced in Stage 05, but spread the work
across a configurable number of worker threads, and prove that adding workers
changes *how fast* you index without changing *what* you produce.

## Background

Indexing a codebase is what engineers call an **embarrassingly parallel**
problem: each source file can be tokenized on its own, with no knowledge of any
other file. That independence is the opening a parallel design exploits — hand
each worker its own slice of the file list and let them run at once.

The catch is the index itself. An inverted index is *shared* state: many files
contribute paths to the same token (`fn` appears everywhere). If every worker
reaches into one shared map at the same time, you have a data race, and the
classic "fixes" — wrapping the whole map in a lock, or locking per token — trade
the race for contention, and contention is exactly the thing that erases your
speedup. Amdahl's Law (Gene Amdahl, 1967) is the arithmetic behind the
disappointment: the serial fraction of a program caps its parallel speedup no
matter how many cores you add.

The standard escape is the **map-reduce** shape that dates back to the earliest
parallel-sorting work and was popularised for data centres by Dean and
Ghemawat's 2004 MapReduce paper: give each worker a **thread-local** index it
alone touches (the *map* phase), then **merge** those partial indexes into the
final result once the workers finish (the *reduce* phase). No shared mutation
during the hot loop means no locks in the hot loop.

Merging carries the whole pedagogical weight of this stage. Your Stage 05 output
is sorted — tokens in order, paths within a token in order — and it must stay
byte-for-byte identical here. Threads finish in an unpredictable order, so a
correct merge cannot depend on *when* a worker returns; it must combine partial
results into the same canonical ordering every single time. Determinism under
non-determinism is the skill on trial.

## Requirements

Extend your program with a `--threads` option on `index`:

```bash
flashindex index <path> --threads <N>
```

- `<N>` is the number of worker threads, a positive integer.
- The output must be **byte-identical** to `flashindex index <path>` (Stage 05)
  for the same `<path>`, for **every** valid value of `<N>`. One line per token,
  `token path1 path2 ...`, tokens sorted, paths per token sorted and
  de-duplicated, relative paths with `/` separators.
- The number of workers is an execution detail only. It must never appear in the
  output and must never reorder it.

## Example

```bash
$ flashindex index ./project --threads 1
alpha src/a.rs src/b.rs
beta src/a.rs
gamma src/b.rs

$ flashindex index ./project --threads 4
alpha src/a.rs src/b.rs
beta src/a.rs
gamma src/b.rs
```

Same bytes, different worker counts. That equality is the deliverable.

## Edge cases

- `--threads 1` must behave exactly like the plain `index` command.
- `--threads 4` (or any N larger than the file count) must still produce the
  full, correctly ordered index — workers with no files simply contribute
  nothing to the merge.
- `--threads 0` is invalid: exit with a non-zero status and a message on stderr.
- A non-numeric value such as `--threads abc` is invalid: exit non-zero with a
  message on stderr.

## Success criteria

- All `deltaforge test` cases for this stage pass.
- `deltaforge bench` runs the thread matrix and reports a speedup; the stage's
  performance gate expects at least a 1.5× speedup from the fewest to the most
  threads. The gate is deliberately lenient — laptops and CI runners are noisy,
  and this stage rewards a sound parallel structure, not a tuned number.

### Benchmark interpretation worksheet

For each thread count, record median runtime, speedup relative to one thread, and parallel efficiency (`speedup / threads`), then answer:

1. At which point do additional workers stop helping proportionally?
2. Which serial work—discovery, task setup, merge, ordering, or output—could explain the curve?
3. Does peak memory grow with workers, and is that consistent with worker-local indexes?
4. Would a tiny corpus and a large corpus produce the same scaling shape?
5. If one run misses the gate, what repeated evidence would distinguish noise from a structural bottleneck?

### Reflection

Compare your design note with the measurements. Name one prediction that survived and one that changed. Most importantly, confirm that every matrix point produced byte-identical index output before interpreting speedup.

## Non-goals

- A thread pool, work-stealing scheduler, or async runtime — plain OS threads
  over a partitioned file list are enough.
- Lock-free data structures or atomics.
- Changing the index format, tokenizer rules, or file-selection rules.
- Beating any particular wall-clock time; the gate only asks that parallelism
  helps at all.
