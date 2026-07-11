# DeltaForge — Product & Technical Specification

## 1. One-line description

**DeltaForge is a local CodeCrafters-style learning framework for building performance-critical software through staged projects, black-box tests, benchmarks, profiling, optimization quests, and measurable engineering progress.**

## 2. Core vision

DeltaForge exists to help programmers learn how to build serious software without falling into either of these traps:

1. **Passive tutorial hell**
   The learner follows step-by-step instructions, copies patterns, passes tests, but does not develop real engineering judgment.

2. **Unstructured suffering**
   The learner wants to build a serious project, but has no clear milestones, no tests, no benchmark targets, no sense of what to do next, and eventually loses motivation.

DeltaForge sits between those two extremes.

It gives the learner:

```txt
clear project goal
small next stage
precise behavioral tests
optional hints
performance targets
benchmark feedback
profiling guidance
progress tracking
```

But it does **not** give the full solution.

The goal is not to make projects easy. The goal is to make them **tractable, motivating, measurable, and educational**.

## 3. Product identity

### Name

**DeltaForge**

### Meaning

* **Delta**: measurable improvement, change, iteration, optimization, performance progression.
* **Forge**: building real things through effort, refinement, and craft.

### Tagline

**Build fast software, one measurable improvement at a time.**

Alternative taglines:

```txt
Learn systems programming through tests, benchmarks, and real projects.
A project-based forge for performance-minded programmers.
Train like an engineer. Build like one.
```

## 4. Core philosophy

DeltaForge is based on the idea that real learning happens through **engineering deltas**.

A project should not only ask:

```txt
Does it work?
```

It should progressively ask:

```txt
Is it correct?
Is it robust?
Is it fast?
Does it scale?
Is it observable?
Can you explain the design?
Can you improve it measurably?
```

A typical DeltaForge project follows this rhythm:

```txt
Stage 1: make a tiny version work
Stage 2: handle realistic input
Stage 3: improve correctness
Stage 4: structure the code better
Stage 5: add performance constraints
Stage 6: benchmark and profile
Stage 7: optimize
Stage 8: compare designs
Stage 9: produce a portfolio-quality artifact
```

The system should reward progress that is visible and measurable.

Example:

```txt
✓ Correctness tests passed
✓ Throughput improved from 182 MB/s to 417 MB/s
✓ Peak memory decreased by 28%
⚠ Parallel speedup plateaus after 4 threads
```

The learner should feel that they are not just completing lessons. They are becoming better at engineering.

## 5. Target user

Primary user:

```txt
A programmer who already knows the basics of programming and wants to become better at systems, performance, concurrency, data structures, storage, compilers, networking, graphics, or infrastructure.
```

More specifically:

```txt
advanced beginner → intermediate
intermediate → strong engineer
student/new grad preparing for serious SWE roles
engineer wanting to learn Rust/C++/Zig/Go through real projects
self-learner tired of tutorials but not yet ready for completely open-ended projects
```

DeltaForge is not aimed at absolute beginners.

It assumes the user can already:

```txt
use a terminal
edit code
run a compiler
understand basic data structures
debug simple failures
read error messages
```

## 6. Core use case

The user wants to build a project like:

```txt
FlashIndex — a multithreaded local code search engine
MiniKV — a persistent key-value store
TinyHTTP — an HTTP server
MiniLSM — an LSM-tree storage engine
RayTraceLab — a CPU ray tracer
ByteForgeVM — a bytecode virtual machine
```

Instead of starting from a blank page, they run:

```bash
deltaforge init flashindex --lang rust
cd flashindex
deltaforge instructions
deltaforge test
```

DeltaForge gives them the current stage.

Example:

```txt
Stage 03: Tokenize source files

Goal:
  Extract identifiers from source files and report their line/column positions.

Your program should:
  - read all files under the input directory
  - extract identifier-like tokens
  - preserve snake_case and uint32_t
  - track line and column
  - print one token occurrence per line

Example:
  flashindex tokenize ./fixtures/basic_project

Expected output:
  src/main.cpp:1:5 main
  src/main.cpp:2:10 fetch_or

Run:
  deltaforge test
```

The user implements the stage themselves.

When they pass:

```txt
✓ Stage 03 passed
Unlocked Stage 04: Exact token search
```

The loop continues.

## 7. What makes DeltaForge different

### 7.1 Not just correctness

Most learning platforms stop at correctness.

DeltaForge goes further:

```txt
correctness
edge cases
performance
memory
concurrency
scalability
profiling
design explanation
benchmark history
portfolio export
```

### 7.2 Language-agnostic

Project packs define expected behavior at the CLI boundary.

The learner can implement the same project in:

```txt
Rust
C++
Zig
Go
Java
Python
Haskell
OCaml
```

The framework does not care, as long as the program exposes the expected commands.

### 7.3 Local-first

DeltaForge is local.

No accounts required.
No cloud required.
No subscription required.
No hidden remote judge required.

Everything lives in local project files:

```txt
project manifest
fixtures
tests
benchmark configs
state file
git repository
benchmark history
```

### 7.4 AI as course designer, not solution writer

AI may help generate:

```txt
project packs
stage instructions
fixtures
test cases
hints
benchmark datasets
review prompts
rubrics
```

But the default experience should prevent the AI from simply solving the learner’s implementation.

The AI should guide the learning environment, not replace the learner’s effort.

## 8. Product principles

### Principle 1 — Small next step

Every stage should have a clear and limited goal.

Bad:

```txt
Implement a complete search engine.
```

Good:

```txt
Implement recursive source file discovery and print one path per line.
```

### Principle 2 — Tests before hints

The learner should first see the failure.

Hints should be progressive and optionally gated.

```txt
Hint 1: Think about which directories should be skipped.
Hint 2: Use a recursive traversal but prune ignored directories early.
Hint 3: In Rust, std::fs::read_dir gives you directory entries.
```

### Principle 3 — Specification, not tutorial

DeltaForge should explain **what** must be true, not exactly **how** to implement it.

It can mention concepts, constraints, and pitfalls, but it should avoid giving complete implementation recipes too early.

### Principle 4 — Performance is measured, not guessed

DeltaForge should teach:

```txt
measure first
profile
form a hypothesis
change one thing
measure again
compare
```

### Principle 5 — Correctness comes before optimization

Early stages should not force premature optimization.

A project should first become correct and understandable. Performance pressure should appear gradually.

### Principle 6 — Preserve learner ownership

At the end of a project, the learner should be able to say:

```txt
I built this.
I understand this.
I can explain the tradeoffs.
I measured the performance.
I know what I would improve next.
```

## 9. Main components

DeltaForge has four major layers:

```txt
1. DeltaForge CLI
2. DeltaForge Core Engine
3. Project Packs
4. User Project Repositories
```

### 9.1 DeltaForge CLI

The command-line interface used by the learner.

Example commands:

```bash
deltaforge list
deltaforge init flashindex --lang rust
deltaforge instructions
deltaforge test
deltaforge next
deltaforge hint
deltaforge bench
deltaforge profile
deltaforge status
deltaforge commit
deltaforge report
deltaforge portfolio
```

### 9.2 DeltaForge Core Engine

The engine handles:

```txt
reading project manifests
copying starter templates
tracking stage progress
running tests
running benchmarks
capturing stdout/stderr
managing fixtures
handling timeouts
recording benchmark history
generating reports
interacting with git
```

### 9.3 Project Packs

A project pack defines one learning project.

Example:

```txt
flashindex/
  project.yaml
  README.md
  templates/
    rust/
    cpp/
  stages/
    01_scan_files/
    02_filter_files/
    03_tokenize/
    04_exact_search/
    05_inverted_index/
    06_persist_index/
    07_parallel_indexing/
    08_ranked_search/
    09_benchmark/
    10_html_report/
```

Each stage contains:

```txt
instructions.md
tests.yaml
fixtures/
hints.md
design_prompt.md
benchmarks.yaml
notes.md
```

### 9.4 User Project Repository

When the user runs:

```bash
deltaforge init flashindex --lang rust
```

DeltaForge creates a local repository:

```txt
flashindex-rust/
  .git/
  .deltaforge/
    state.json
    benchmark_history.json
  Cargo.toml
  src/
    main.rs
  README.md
```

The user owns this repository.

DeltaForge should not mutate user source code after initialization unless explicitly asked.

## 10. CLI specification

### 10.1 `deltaforge list`

Lists available project packs.

Example:

```bash
deltaforge list
```

Output:

```txt
Available projects:

flashindex
  Multithreaded local source-code search engine
  Topics: concurrency, indexing, data structures, performance

minikv
  Persistent key-value store
  Topics: storage, serialization, indexing, crash safety

tinyhttp
  HTTP server from scratch
  Topics: networking, parsing, concurrency, protocol design
```

### 10.2 `deltaforge init`

Creates a new user repository from a project pack.

Example:

```bash
deltaforge init flashindex --lang rust
```

Options:

```txt
--lang <language>
--name <directory-name>
--no-git
--stage <stage-id>
```

Responsibilities:

```txt
copy language template
create .deltaforge/state.json
initialize git repo unless disabled
write README with project overview
set current stage to first stage
```

### 10.3 `deltaforge instructions`

Shows the current stage instructions.

Example:

```bash
deltaforge instructions
```

Output:

```txt
Project: FlashIndex
Stage 03: Tokenize source files

Goal:
  Extract source-code tokens with line and column information.

Requirements:
  ...
```

Options:

```txt
--stage <stage-id>
--all
```

### 10.4 `deltaforge test`

Runs the current stage tests.

Example:

```bash
deltaforge test
```

Options:

```txt
--stage <stage-id>
--all
--verbose
--keep-temp
--json
```

Behavior:

```txt
load stage tests
prepare fixture temp directories
build user program if build command exists
run commands
capture stdout/stderr/exit code
compare expected results
print pass/fail summary
unlock stage if all tests pass
```

### 10.5 `deltaforge next`

Moves to next stage if current stage passed.

Example:

```bash
deltaforge next
```

If current stage is not passed:

```txt
Current stage has not passed yet.
Run: deltaforge test
```

If passed:

```txt
Unlocked Stage 04: Exact token search
```

### 10.6 `deltaforge hint`

Shows progressive hints.

Example:

```bash
deltaforge hint
```

Options:

```txt
--level 1
--level 2
--all
```

Possible gating behavior:

```txt
You have not run tests yet.
Run deltaforge test before requesting a hint.
```

### 10.7 `deltaforge bench`

Runs benchmarks for the current stage or project.

Example:

```bash
deltaforge bench
```

Options:

```txt
--stage <stage-id>
--all
--threads 1,2,4,8
--iterations 10
--warmup 2
--json
--save
```

Output:

```txt
Benchmark: index_medium_project

threads   median    p95      throughput
1         9.21s     9.42s    218 MB/s
2         5.13s     5.28s    391 MB/s
4         3.01s     3.08s    667 MB/s
8         2.54s     2.71s    790 MB/s
```

### 10.8 `deltaforge status`

Shows project progress.

Example:

```bash
deltaforge status
```

Output:

```txt
Project: FlashIndex
Language: Rust
Current stage: 07_parallel_indexing

Completed:
  ✓ 01_scan_files
  ✓ 02_filter_files
  ✓ 03_tokenize
  ✓ 04_exact_search
  ✓ 05_inverted_index
  ✓ 06_persist_index

Current:
  → 07_parallel_indexing

Optional quests:
  ○ reduce allocations
  ○ improve 8-thread scaling
```

### 10.9 `deltaforge commit`

Creates a git commit for the completed stage.

Example:

```bash
deltaforge commit
```

Generated commit message:

```txt
Complete Stage 07: Parallel indexing
```

Optional tag:

```txt
flashindex-stage-07
```

### 10.10 `deltaforge report`

Generates a local report for the project.

Example:

```bash
deltaforge report --html report.html
```

Report contents:

```txt
stage progress
benchmark history
performance graphs
memory trends
completed quests
architecture notes
```

### 10.11 `deltaforge portfolio`

Generates a portfolio-ready summary.

Example:

```bash
deltaforge portfolio
```

Output:

```md
# FlashIndex

Built a multithreaded source-code indexing engine in Rust.

Features:
- recursive file discovery
- source tokenizer
- inverted index
- persisted index format
- parallel indexing
- ranked search
- benchmark suite

Performance:
- indexed 1.2 GB in 3.8s
- peak throughput: 620 MB/s
- 6.4x speedup from 1 to 8 threads
```

## 11. Project pack format

Each project pack has a manifest.

Recommended format: YAML for readability.

Example:

```yaml
id: flashindex
name: FlashIndex
version: 0.1.0
description: Multithreaded local source-code search engine

topics:
  - concurrency
  - data structures
  - indexing
  - performance
  - cli

languages:
  rust:
    template: templates/rust
    build:
      command: ["cargo", "build", "--release"]
    run:
      command: ["cargo", "run", "--release", "--"]
  cpp:
    template: templates/cpp
    build:
      command: ["cmake", "--build", "build", "--config", "Release"]
    run:
      command: ["./build/flashindex"]

ignored_paths:
  - .git
  - target
  - build
  - node_modules

stages:
  - id: 01_scan_files
    title: Scan files
    path: stages/01_scan_files

  - id: 02_filter_files
    title: Filter source files
    path: stages/02_filter_files

  - id: 03_tokenize
    title: Tokenize source files
    path: stages/03_tokenize
```

## 12. Stage format

Each stage directory:

```txt
stages/03_tokenize/
  instructions.md
  tests.yaml
  hints.md
  fixtures/
    basic_project/
    weird_identifiers/
    empty_project/
  benchmarks.yaml
  design_prompt.md
```

### 12.1 `instructions.md`

Human-readable stage specification.

Should contain:

```txt
goal
background
requirements
examples
edge cases
commands
success criteria
non-goals
```

Example:

````md
# Stage 03 — Tokenize source files

## Goal

Extract identifier-like tokens from source files and report their file, line, and column.

## Requirements

Your program should expose:

```bash
flashindex tokenize <path>
````

For every token occurrence, print:

```txt
relative/path:line:column token
```

Tokens consist of:

* ASCII letters
* digits
* underscore

A token may not start with a digit.

## Non-goals

* full C++ parsing
* Unicode identifiers
* comment removal
* string literal parsing

````

### 12.2 `tests.yaml`

Defines black-box tests.

Example:

```yaml
tests:
  - name: tokenizes a simple C++ file
    fixture: basic_project
    command: ["tokenize", "{fixture_path}"]
    expect:
      exit_code: 0
      stdout_contains:
        - "src/main.cpp:1:5 main"
        - "src/main.cpp:2:10 fetch_or"

  - name: ignores punctuation
    fixture: punctuation_project
    command: ["tokenize", "{fixture_path}"]
    expect:
      exit_code: 0
      stdout_contains:
        - "src/flags.cpp:3:14 memory_order_relaxed"
      stdout_not_contains:
        - "::"
        - ";"
````

### 12.3 Supported test expectations

Initial MVP expectations:

```txt
exit_code
stdout_exact
stdout_contains
stdout_not_contains
stderr_contains
file_exists
file_not_exists
timeout_ms
```

Later expectations:

```txt
json_equals
csv_equals
snapshot_match
regex_match
max_runtime_ms
max_memory_mb
min_throughput_mb_s
```

## 13. Test runner design

The test runner should execute user programs as black boxes.

Flow:

```txt
1. Load current project and stage.
2. Build user project if build command exists.
3. Copy fixture to temporary directory.
4. Expand variables like {fixture_path}.
5. Run command with timeout.
6. Capture stdout, stderr, exit code.
7. Compare output against expectations.
8. Print clear diagnostics.
9. Save result to .deltaforge/state.json.
```

Example failure output:

```txt
Stage 03: Tokenize source files

✓ tokenizes simple identifiers
✓ ignores punctuation
✗ reports line and column

Expected stdout to contain:
  src/main.cpp:3:14 fetch_or

Actual stdout:
  src/main.cpp:3 fetch_or

Hint:
  Column numbers are required in this stage.
```

## 14. Benchmark system

The benchmark system is a first-class feature.

Benchmark definitions live in:

```txt
benchmarks.yaml
```

Example:

```yaml
benchmarks:
  - name: index_small_project
    fixture: generated_small_codebase
    command: ["build", "{fixture_path}", "--out", "{temp_dir}/index.fi"]
    iterations: 5
    warmup: 1
    metrics:
      - runtime_ms
      - peak_memory_mb
      - throughput_mb_s

  - name: index_with_threads
    fixture: generated_medium_codebase
    matrix:
      threads: [1, 2, 4, 8]
    command: ["build", "{fixture_path}", "--threads", "{threads}", "--out", "{temp_dir}/index.fi"]
    iterations: 7
    warmup: 2
```

### Benchmark output

```txt
Benchmark: index_with_threads

threads   median     p95       throughput
1         8.92s      9.11s     224 MB/s
2         4.97s      5.10s     402 MB/s
4         2.88s      3.02s     694 MB/s
8         2.21s      2.34s     905 MB/s

Speedup:
1 → 8 threads: 4.04x
```

### Benchmark history

Stored in:

```txt
.deltaforge/benchmark_history.json
```

Example:

```json
{
  "runs": [
    {
      "project": "flashindex",
      "stage": "07_parallel_indexing",
      "commit": "a13f9c2",
      "timestamp": "2026-07-09T13:00:00+02:00",
      "benchmark": "index_with_threads",
      "language": "rust",
      "results": [
        {
          "threads": 1,
          "median_ms": 8920,
          "throughput_mb_s": 224
        },
        {
          "threads": 8,
          "median_ms": 2210,
          "throughput_mb_s": 905
        }
      ]
    }
  ]
}
```

## 15. Performance gates

Some stages may define performance requirements.

Example:

```yaml
performance_gates:
  - name: tokenizer throughput
    benchmark: tokenize_medium_project
    metric: throughput_mb_s
    min: 150

  - name: memory limit
    benchmark: index_medium_project
    metric: peak_memory_mb
    max: 800

  - name: parallel speedup
    benchmark: index_with_threads
    metric: speedup_1_to_8
    min: 2.5
```

Performance gates should be used carefully.

Rules:

```txt
early stages should avoid strict performance gates
performance gates should be realistic
local machine noise should be considered
benchmarks should report median over several iterations
performance failures should be educational, not punitive
```

A performance failure should look like:

```txt
Correctness: passed
Performance: not yet

Required throughput: 150 MB/s
Your throughput:      91 MB/s

Likely areas to investigate:
  - excessive string allocation
  - token insertion into hash map
  - repeated path normalization
```

## 16. Optimization quests

Optimization quests are optional challenges unlocked after correctness.

Example:

```yaml
quests:
  - id: reduce_allocations
    title: Reduce allocations
    description: Reduce total allocations during indexing by at least 50%.
    benchmark: index_medium_project
    metric: allocation_count
    target_reduction_percent: 50

  - id: improve_parallel_scaling
    title: Improve parallel scaling
    description: Achieve at least 4x speedup from 1 to 8 threads.
    benchmark: index_with_threads
    metric: speedup_1_to_8
    min: 4.0

  - id: reduce_peak_memory
    title: Reduce peak memory
    description: Keep peak memory below 500 MB on the medium dataset.
    benchmark: index_medium_project
    metric: peak_memory_mb
    max: 500
```

Quests are not required to complete the main path.

They exist for deeper learning.

## 17. Design prompts

Before major stages, DeltaForge can ask the learner to write a small design note.

Example:

```md
# Design Prompt — Parallel Indexing

Before implementing this stage, answer:

1. What work can be done independently per file?
2. What data structure is shared?
3. What data structure is thread-local?
4. Where might contention happen?
5. What speedup do you expect from 1 to 8 threads?
6. What might prevent perfect scaling?
```

Stored in:

```txt
.deltaforge/design_notes/07_parallel_indexing.md
```

Later, after benchmarks, DeltaForge can ask:

```txt
You expected tokenization to dominate.
Actual benchmark: merge took 41% of total time.

What changed your understanding?
```

This teaches engineering reflection.

## 18. Hint system

Hints should be progressive.

Example `hints.md`:

```md
# Hint 1

Try to separate file discovery from file processing.

# Hint 2

Each worker should be able to process files without synchronizing on every token.

# Hint 3

Consider giving each worker a local index, then merging local indexes at the end.
```

Possible behavior:

```txt
deltaforge hint
```

Output:

```txt
Hint 1/3:
Try to separate file discovery from file processing.
```

Running again:

```txt
Hint 2/3:
Each worker should be able to process files without synchronizing on every token.
```

Optional policy:

```txt
hints unavailable until at least one failed test run
higher-level hints require multiple failed attempts
solution notes hidden unless explicitly unlocked
```

## 19. AI integration

AI is optional but powerful.

### 19.1 Allowed AI roles

The AI can help create and improve the learning framework:

```txt
generate project pack skeletons
write stage instructions
generate tests
generate fixtures
generate edge cases
generate benchmark ideas
write hints
review stage quality
summarize profiling output
ask design questions
generate portfolio summaries
```

### 19.2 Discouraged AI roles

By default, AI should not:

```txt
write the learner’s full implementation
solve current stage directly
replace debugging effort
hide the struggle that produces learning
```

### 19.3 AI modes

Potential modes:

```txt
Course Designer Mode
  Helps create or refine project packs.

Reviewer Mode
  Reviews the learner’s code after a stage passes.

Bottleneck Analyst Mode
  Explains profiling and benchmark output.

Socratic Hint Mode
  Asks guiding questions without giving full code.

Portfolio Mode
  Helps summarize completed work.
```

### 19.4 AI review example

```txt
Concurrency review — Stage 07

Observation:
  Your worker threads push token occurrences into a shared vector protected by a mutex.

Why this matters:
  This may serialize much of the indexing work.

Suggested experiment:
  Compare this approach with one vector per worker followed by a merge phase.

Do not rewrite yet:
  First measure how much time is spent waiting on the mutex.
```

## 20. Profiling integration

Long-term command:

```bash
deltaforge profile
```

Potential integrations:

```txt
Linux: perf, valgrind, heaptrack, flamegraph
macOS: Instruments, sample
Windows: Visual Studio Profiler, ETW
Rust: cargo flamegraph
C++: Tracy, VTune, perf
```

Initial MVP can avoid real profiler integration and rely on timing breakdowns.

Endgame profiling output:

```txt
Hotspots:
  42% tokenize_file
  23% hash map insertion
  11% allocation during string creation
  8% path normalization

Likely bottleneck:
  Token insertion dominates indexing time.

Suggested experiments:
  - intern strings
  - batch postings per worker
  - use integer token IDs
  - reserve hash map capacity
```

## 21. Cross-language comparisons

DeltaForge should eventually support comparing the same project across languages.

Example:

```bash
deltaforge compare --project flashindex --langs rust,cpp,zig,go
```

Output:

```txt
Language   Runtime   Memory   LOC    Build time
Rust       2.81s     420 MB   1420   4.2s
C++        2.54s     390 MB   1610   6.8s
Zig        2.63s     360 MB   1530   3.1s
Go         4.70s     690 MB   980    1.2s
```

This teaches through evidence instead of language wars.

## 22. Implementation variants

A learner may create variants of one implementation.

Example:

```bash
deltaforge variant create naive
deltaforge variant create arena-index
deltaforge variant create sharded-index
deltaforge variant compare
```

Output:

```txt
Variant         Runtime   Memory   Notes
naive           8.4s      900 MB   unordered_map<string, vector<Posting>>
arena-index     5.1s      520 MB   string interning
sharded-index   3.7s      610 MB   parallel merge
```

This is extremely useful for performance education because it makes tradeoffs visible.

## 23. Boss fights

Each project should end with a final challenge.

For FlashIndex:

```txt
Boss Fight: Index a 2 GB generated codebase

Requirements:
  - correctness tests pass
  - index build completes under target time
  - peak memory under limit
  - query latency under threshold
  - benchmark report generated
  - portfolio summary generated
```

The boss fight should feel like the final exam of the project.

It should produce artifacts:

```txt
benchmark report
architecture summary
performance graphs
README section
CV bullets
```

## 24. Portfolio export

DeltaForge should help turn completed projects into portfolio material.

Example command:

```bash
deltaforge portfolio --format markdown
```

Output:

```md
# FlashIndex

Built a multithreaded local code search engine in Rust.

## Features

- Recursive file discovery
- Source tokenizer with line/column tracking
- In-memory inverted index
- Persisted index format
- Parallel indexing
- Ranked multi-token search
- Benchmark suite

## Performance

- Indexed 1.2 GB of generated source files in 3.8 seconds
- Achieved 620 MB/s throughput
- Reached 6.4x speedup from 1 to 8 threads
- Reduced peak memory by 31% after string interning

## Engineering highlights

- Used worker-local indexes to avoid shared-state contention
- Interned token strings into integer IDs
- Stored posting lists contiguously for cache locality
- Benchmarked thread scaling and merge bottlenecks
```

## 25. Interview mode

DeltaForge can generate questions from completed projects.

Example:

```bash
deltaforge interview
```

Output:

```txt
FlashIndex Interview Questions

1. Why did you choose an inverted index?
2. Why not update a single global hash map from all worker threads?
3. What caused parallel speedup to plateau?
4. How did you represent posting lists in memory?
5. How would you support incremental indexing?
6. How would you reduce index size on disk?
7. What tradeoff did string interning introduce?
8. What would change if files were modified while indexing?
```

This helps the learner convert project work into interview readiness.

## 26. Local dashboard

Long-term command:

```bash
deltaforge dashboard
```

Opens a local web dashboard.

Sections:

```txt
project progress
stage timeline
benchmark graphs
memory graphs
flamegraphs
quest progress
design notes
portfolio export
```

This is not MVP.

It becomes valuable after the CLI experience is stable.

## 27. Security and safety

DeltaForge runs local code, often arbitrary user code.

Important constraints:

```txt
tests should run in temporary directories
fixtures should be copied, not modified in-place
commands should have timeouts
destructive tests should be explicit
project packs should be trusted or reviewed before execution
environment variables should be controlled
network access should be disabled by default for tests when possible
```

Potential future sandboxing:

```txt
Docker
Firejail
Windows sandbox
WASI
process-level restrictions
```

MVP can be local and trust-based, but the architecture should not make sandboxing impossible later.

## 28. Configuration files

### 28.1 User project state

`.deltaforge/state.json`

Example:

```json
{
  "project": "flashindex",
  "language": "rust",
  "current_stage": "03_tokenize",
  "completed_stages": [
    "01_scan_files",
    "02_filter_files"
  ],
  "hint_state": {
    "03_tokenize": 1
  },
  "created_at": "2026-07-09T13:00:00+02:00"
}
```

### 28.2 Local settings

`.deltaforge/config.toml`

Example:

```toml
[runner]
timeout_ms = 5000
keep_temp = false

[bench]
iterations = 7
warmup = 2

[git]
auto_commit = false
auto_tag = true
```

## 29. Suggested implementation language

Recommended language for DeltaForge itself: **Rust**.

Reasons:

```txt
excellent CLI ecosystem
fast startup and execution
strong filesystem support
good error handling
cross-platform builds
TOML/YAML/JSON libraries
fits the performance-oriented identity
```

Potential crates:

```txt
clap       CLI parsing
serde      serialization
serde_yaml YAML
toml       TOML
anyhow     application errors
thiserror  library errors
duct       process execution
tempfile   temporary directories
walkdir    file traversal
indicatif  progress bars
comfy-table terminal tables
```

A Python MVP would be faster to prototype, but a Rust implementation is more coherent with the long-term identity.

## 30. Internal architecture

Suggested Rust workspace:

```txt
deltaforge/
  Cargo.toml
  crates/
    deltaforge-cli/
    deltaforge-core/
    deltaforge-runner/
    deltaforge-pack/
    deltaforge-report/
  packs/
    flashindex/
```

### 30.1 `deltaforge-cli`

Responsibilities:

```txt
parse commands
print user-facing output
call core engine
format errors
```

### 30.2 `deltaforge-core`

Responsibilities:

```txt
load project state
load project manifests
stage progression
pack resolution
configuration
```

### 30.3 `deltaforge-runner`

Responsibilities:

```txt
prepare fixtures
run build commands
run test commands
capture output
compare expectations
run benchmarks
measure timing
```

### 30.4 `deltaforge-pack`

Responsibilities:

```txt
parse project pack schema
validate pack integrity
resolve stage paths
load tests and benchmarks
```

### 30.5 `deltaforge-report`

Responsibilities:

```txt
benchmark history
HTML reports
portfolio markdown
interview question generation
```

## 31. MVP definition

The MVP should be intentionally small.

### MVP goal

Prove this loop:

```txt
init project → show instructions → run failing tests → user implements → tests pass → next stage unlocks
```

### MVP scope

Support:

```txt
one project pack: FlashIndex
one language template: Rust
3 stages
simple YAML tests
stdout/stderr/exit-code assertions
basic state tracking
optional git initialization
```

### MVP commands

```bash
deltaforge init flashindex --lang rust
deltaforge instructions
deltaforge test
deltaforge next
deltaforge status
deltaforge hint
```

### MVP non-goals

```txt
benchmarks
profiling
dashboard
AI integration
cross-language comparison
sandboxing
portfolio export
optimization quests
```

Those come later.

## 32. First project pack: FlashIndex

### Project description

**FlashIndex is a local source-code search engine.**

The learner builds a CLI that scans source files, tokenizes them, builds an inverted index, and eventually supports parallel indexing and ranked search.

### Stage plan

#### Stage 01 — Scan files

Goal:

```txt
Recursively find files under a directory and print relative paths.
```

Requirements:

```txt
accept a path argument
walk directories recursively
skip directories like .git, target, build, node_modules
print one relative path per line
```

#### Stage 02 — Filter source files

Goal:

```txt
Only include source-like files.
```

Requirements:

```txt
include .cpp, .hpp, .h, .c, .rs, .py, .glsl, .md, .txt, .cmake
support extension filtering
ignore binary-looking files
```

#### Stage 03 — Tokenize files

Goal:

```txt
Extract identifier-like tokens with line and column.
```

Requirements:

```txt
ASCII identifiers
line and column tracking
relative path in output
stable output order
```

#### Stage 04 — Exact token search

Goal:

```txt
Search for a single token and print matching locations.
```

Requirements:

```txt
flashindex find <path> <token>
print file:line:column context
```

#### Stage 05 — Inverted index

Goal:

```txt
Build an in-memory inverted index.
```

Concepts:

```txt
token -> posting list
file IDs
posting structs
deduplication
```

#### Stage 06 — Persist index

Goal:

```txt
Save and load an index file.
```

Requirements:

```txt
flashindex build <path> --out index.fi
flashindex query index.fi <token>
```

#### Stage 07 — Parallel indexing

Goal:

```txt
Index files using multiple worker threads.
```

Requirements:

```txt
--threads N
worker-local indexes
final merge
benchmark with 1/2/4/8 threads
```

#### Stage 08 — Ranked multi-token search

Goal:

```txt
Search multiple tokens and rank results.
```

Requirements:

```txt
flashindex query index.fi "atomic relaxed"
rank by token matches and proximity
show top-k results
```

#### Stage 09 — Benchmark mode

Goal:

```txt
Measure indexing performance.
```

Requirements:

```txt
runtime
throughput
file count
total bytes
thread count
```

#### Stage 10 — HTML report

Goal:

```txt
Generate a demoable performance report.
```

Requirements:

```txt
indexing summary
thread scaling table
top tokens
largest files
query latency
```

## 33. Development roadmap

### Phase 0 — Spec and skeleton

Deliverables:

```txt
repo created
Rust workspace initialized
basic CLI skeleton
project pack directory structure
FlashIndex stage 01 written
```

### Phase 1 — Core MVP

Deliverables:

```txt
deltaforge init
deltaforge instructions
deltaforge test
deltaforge next
deltaforge status
simple YAML test runner
state file
```

### Phase 2 — FlashIndex MVP pack

Deliverables:

```txt
FlashIndex stages 01–05
Rust template
fixtures
tests
hints
```

### Phase 3 — Benchmark support

Deliverables:

```txt
deltaforge bench
benchmark YAML schema
timing measurement
benchmark history
CSV/JSON output
```

### Phase 4 — Performance education layer

Deliverables:

```txt
performance gates
optimization quests
design prompts
benchmark comparison
```

### Phase 5 — Reporting

Deliverables:

```txt
HTML reports
portfolio export
interview mode
```

### Phase 6 — AI support

Deliverables:

```txt
project pack generation assistant
hint generation
profiling summary
code review prompts
```

### Phase 7 — Multi-project ecosystem

Deliverables:

```txt
MiniKV
TinyHTTP
MiniLSM
RayTraceLab
ByteForgeVM
SoftwareRasterizer
```

## 34. Example end-to-end user experience

```bash
deltaforge init flashindex --lang rust
cd flashindex

deltaforge instructions
deltaforge test
```

Output:

```txt
Stage 01: Scan files

✗ scans nested directories
✗ skips ignored directories
✗ prints relative paths

3 failed
```

User implements.

```bash
deltaforge test
```

Output:

```txt
Stage 01: Scan files

✓ scans nested directories
✓ skips ignored directories
✓ prints relative paths

Stage passed.
Run deltaforge next to continue.
```

Then:

```bash
deltaforge next
```

Output:

```txt
Unlocked Stage 02: Filter source files
```

Later:

```bash
deltaforge bench
```

Output:

```txt
Benchmark: index_medium_project

threads   median    throughput
1         8.92s     224 MB/s
2         4.97s     402 MB/s
4         2.88s     694 MB/s
8         2.21s     905 MB/s

Saved benchmark result.
```

At the end:

```bash
deltaforge portfolio
```

Output:

```md
Built FlashIndex, a multithreaded code search engine in Rust...
```

## 35. Success criteria

DeltaForge succeeds if it makes the learner feel:

```txt
I know exactly what to build next.
The task is challenging but not overwhelming.
The tests are clear.
The benchmarks are motivating.
I can see my progress.
I am learning real engineering tradeoffs.
I am building portfolio-grade projects.
```

The ultimate success criterion:

> A learner who completes a DeltaForge project should be able to explain the architecture, performance tradeoffs, data structures, bottlenecks, and future improvements of their implementation in an interview.

## 36. Final product vision

DeltaForge should become a local performance-learning platform where programmers build real systems projects through:

```txt
staged challenges
black-box tests
benchmark targets
profiling
optimization quests
design reflection
cross-language comparison
portfolio export
AI-assisted course design
```

It should feel like:

```txt
CodeCrafters + performance lab + personal engineering gym + portfolio generator
```

But local, extensible, free, and owned by the learner.

The final promise:

> **DeltaForge helps you learn to build fast, serious software by turning every project into a sequence of measurable engineering deltas.**
