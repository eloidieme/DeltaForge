# Product vision

## North star

DeltaForge is a local engineering studio where programmers build serious software,
prove its behavior, improve it measurably, and learn to explain the decisions behind
it.

Its concise promise is:

> **Build it. Prove it. Improve it. Explain it.**

DeltaForge does not aim to make difficult projects easy. It keeps the learner in
productive struggle while continuously making four things clear:

1. Where am I?
2. What am I trying to make true?
3. What evidence do I currently have?
4. What is the single best next action?

## Primary user

The primary DeltaForge 1.0 user is a programmer who:

- is already comfortable using an editor and terminal;
- understands basic Git usage;
- can build and debug ordinary programs in at least one language;
- wants to become stronger at systems, storage, networking, concurrency, performance,
  compilers, or infrastructure;
- wants guidance and evidence without a tutorial that supplies the implementation;
- values locally owned code and an environment that works offline.

The initial product is especially appropriate for advanced beginners moving toward
intermediate engineering and intermediate engineers entering a new systems domain or
language.

## Learner outcome

A learner who completes a DeltaForge project should be able to:

- explain the system's observable behavior and architecture;
- reproduce and diagnose important failures;
- identify the data structures and invariants that matter;
- form a performance hypothesis before changing code;
- measure runtime, throughput, memory, and scaling where relevant;
- explain why an optimization helped and what it cost;
- describe rejected or failed approaches;
- identify the next meaningful engineering improvement;
- present credible, evidence-backed work without relying on canned portfolio claims.

## Product positioning

DeltaForge is not primarily a course catalog or coding-test platform. It is a guided
engineering environment centered on a learner-owned repository.

Its distinctive loop is:

```text
behavior -> evidence -> diagnosis -> decision -> measurement -> understanding
```

The final repository is important, but the deeper product is the evolution of the
learner's engineering judgment.

## Surface contract

### Browser workbench

The browser is the primary surface for:

- selecting and creating projects;
- understanding the current mission;
- initiating and following test and benchmark runs;
- diagnosing failures;
- revealing help;
- beginning the next capability;
- recording predictions and reflections;
- reviewing the project chronicle;
- completing final challenges and exporting the engineering story.

### External editor

The learner writes and owns source code in their chosen editor. DeltaForge may open or
reveal the project but does not provide a built-in toy editor or silently change editor
configuration.

### CLI

The CLI remains first-class for:

- terminal-preferring learners;
- targeted tests and benchmarks;
- automation and CI;
- JSON and agent workflows;
- accessibility fallback;
- pack authoring and validation;
- diagnosis when a browser is unavailable.

It invokes the same application operations as the workbench. It is not a second,
independent product model.

## Product principles

### One obvious next action

Every learner state has one visually and semantically dominant action. Secondary
actions remain available without competing for attention.

### Evidence before explanation

The learner first sees what the program did. DeltaForge explains the contradiction and
relevant contract before offering implementation guidance.

### Productive struggle without punishment

Hints are progressive and private. There is no score penalty, purity metric, or shame
for asking for help.

### Capabilities, not administrative progress

Stage numbers orient the learner. Capability statements communicate what the learner's
program can now do.

### Correctness before performance

Optimization does not distract from an unstable behavioral foundation. Performance
pressure appears when the implementation is ready to support meaningful experiments.

### Measurement as an engineering conversation

A benchmark is not merely a number. The product connects a prediction, change,
measurement, tradeoff, interpretation, and next experiment.

### Learner ownership

No account, cloud workspace, mandatory upload, or AI dependency is required. Code,
history, measurements, reflections, and exports remain local unless the learner
explicitly sends or publishes them.

### Calm seriousness

DeltaForge respects deep work. It may provide ceremony and satisfaction, but it does
not use streak pressure, currencies, leaderboards, cartoon rewards, or noisy
gamification.

## Scope for 1.0

DeltaForge 1.0 serves a single local learner and includes:

- local project library and project creation;
- browser-first correctness learning loop;
- external-editor workflow;
- structured testing, diagnosis, and progressive hints;
- capability progression and resumption;
- performance experiments and reflection;
- project chronicle;
- at least one complete flagship finale;
- evidence-backed reports and exports;
- full offline operation;
- supported Windows, macOS, and Linux installation.

## Explicit non-goals

DeltaForge 1.0 does not target:

- absolute programming beginners;
- classrooms, cohorts, or team administration;
- cloud development environments;
- mandatory accounts or synchronization;
- community feeds, peer review, or social profiles;
- competitive leaderboards;
- certifications;
- daily streaks or retention mechanics;
- general-purpose algorithm drills;
- a built-in code editor;
- backward compatibility as a product requirement;
- AI as a prerequisite for learning.

## Flagship

FlashIndex is the flagship product experience. It is the first pack to receive the
complete workbench, feedback, progression, performance-lab, chronicle, and finale
treatment. Other packs follow only after the core FlashIndex loop is convincing.

## Privacy position

- No account is required.
- No mandatory telemetry is collected.
- Source and project data are not uploaded by default.
- Diagnostic export is explicit and reviewable.
- Future AI interactions disclose the provider and exact context being sent.
- Source code requires explicit consent for each external AI interaction unless the
  learner establishes a narrower persistent permission.
- Structured history is retained locally; raw run output is bounded and clearable.

## Product success measures

### Activation

A new user with a working language toolchain reaches the first behavioral run within
five minutes without consulting external documentation.

### Orientation

At any observed point, the learner can correctly state the current mission and next
action.

### Failure comprehension

After a failed run, the learner can explain the primary contradiction before revealing
a hint.

### Resumption

A returning learner resumes useful work within thirty seconds.

### System invisibility

The learner never needs to understand local-service processes, ports, static-page
generation, or refresh mechanics during normal use.

### Learning quality

After a performance experiment, the learner can state the original prediction, actual
result, important tradeoff, and next experiment.

### Completion credibility

A completed project produces an engineering story whose factual claims trace to tests,
measurements, commits, or learner-authored reasoning.
