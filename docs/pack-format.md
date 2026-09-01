# Pack Format

Packs are directories with `project.yaml`, language templates, and stage directories.

```yaml
schema_version: 1
id: flashindex
name: FlashIndex
version: 2.0.0
description: Local source-code search engine
topics: [indexing, data structures, cli, performance]
tier: flagship            # flagship | preview (default: preview)
difficulty: advanced      # introductory | intermediate | advanced
estimated_hours:
  low: 18
  high: 30
languages:
  rust:
    display_name: Rust
    requires:
      - program: cargo
        label: Rust toolchain (cargo)
        install_url: https://www.rust-lang.org/tools/install
    template: templates/rust
    build:
      command: ["cargo", "build", "--release"]
    run:
      command: ["cargo", "run", "--release", "--"]
    bench_run:
      command: ["./target/release/flashindex"]
stages:
  - id: 01_scan_files
    title: Scan files
    path: stages/01_scan_files
```

Catalog fields:

- `tier` (optional, default `preview`): how finished the pack is. `flagship` means a
  complete five-level help ladder on every stage, diagnosis metadata on every check, and
  a proven reference solution. The catalog presents the two tiers differently rather than
  as equals, so the default is deliberately the conservative one: a pack that never says
  otherwise is never presented as finished.
- `difficulty` (optional): `introductory`, `intermediate`, or `advanced`.
- `estimated_hours` (optional): an inclusive `low`/`high` range, shown on the catalog card
  so a learner can judge the commitment before starting.

Each stage requires `instructions.md`, `hints.md`, and `tests.yaml`. `benchmarks.yaml`, `design_prompt.md`, and `prediction.md` are optional.

`prediction.md` is the prompt shown before a stage's first benchmark run. Only stages that declare benchmarks should carry one; a stage without one simply offers no prediction. Its leading `#` heading is stripped, and the rest is shown verbatim. Ask for a commitment that can later be compared with a number — a magnitude, a shape of curve, which of two costs dominates — rather than a yes-or-no.

Learner-facing instructions use seven sections in order: `Goal`, `Background`, `Requirements`, `Example`, `Edge cases`, `Success criteria`, and `Non-goals`. They define observable behavior and motivation without prescribing an implementation. Every listed edge case should have a deterministic black-box test.

All six sections after `Goal` are rendered to the learner in full, as paragraphs, lists, and fenced code blocks — a section may be written as prose, as bullets, or as both, and all of it reaches the workbench. A required section that is empty is an authoring error, not a blank panel.

Hint files use at least three progressive `# Hint N` sections; the flagship uses five. A heading may append a short label after an em dash, such as `# Hint 1 — Observation`, and an unlabeled heading falls back to the literal word "Hint". The flagship ladder is Observation, Concept, Experiment, Structure, Retrospective: from making the contradiction visible, through what to try before changing code, to how to decompose the work, without ever supplying the solution. The last level unlocks only after the stage passes, so it must be a genuine retrospective — a comparison of the chosen approach against an alternative — and must not be needed to pass.

The pack `README.md` is active curriculum content: `deltaforge overview` renders it, and `deltaforge init` includes it in the learner project's generated README. Bundled overviews therefore carry cumulative glossaries, concept maps, historical field notes, and failure-analysis exercises. Stage `Success criteria` sections may contain reflection prompts and benchmark interpretation worksheets as `###` subsections; these prompts ask learners to explain evidence and invariants without changing the stage's observable contract.

`pack doctor` and `validate-pack --strict` report authoring-quality findings when `Edge cases` or `Non-goals` headings are absent, a stage has fewer than three hint headings, or a stage defines fewer than two tests. These are strict/doctor findings rather than base schema failures so incomplete work-in-progress packs remain editable.

Language spec fields:

- `template` (required): path to the language starter template, copied into the learner project.
- `build` (optional): command run before tests and benchmarks.
- `run` (required): command used to invoke the learner's program when checks run.
- `display_name` (optional): human name for the language, e.g. `Rust` for the key `rust`. Shown in the catalog and the creation flow.
- `requires` (optional): the executables this language's `build` and `run` commands need on `PATH`. Each entry has a `program`, an optional `label`, and an optional `install_url`. The creation preflight and `doctor` both read these, so adding a language does not require changing either.
- `bench_run` (optional): command used to time the learner's program after the build step. It falls back to `run` when absent, so it is optional at `schema_version: 1`. Prefer pointing it directly at the built binary (for example `./target/release/<binary>`) so benchmarks measure the program rather than build-tool startup overhead. A relative first element is resolved against the project root and receives the platform executable suffix on Windows.

A pack's `ignored_paths` are excluded (in addition to a built-in list: `.git`, `.deltaforge`, `target`, `build`, `node_modules`, `__pycache__`, `.venv`, `.DS_Store`, plus the learner's `integrity.exclude` config) when computing the learner project digest that guards stage completion.

Pack content must be self-contained: `validate-pack` reports every symbolic link or special file in a pack (base validation, not just `--strict`), and pack digesting rejects them at init/sync time. A symlinked `tests.yaml` or fixture would let pack behavior change while the recorded digest stayed the same, defeating pinning.

Stage completion proofs pin a per-stage behavioral digest covering the stage's `tests.yaml`, its `fixtures/` tree, and the language `build`/`run` commands. Gate-bearing stages additionally hash canonical parsed gate semantics: the referenced benchmark execution definition, metric/bound/selector, and `bench_run`. Tests and fixture contents are hashed as raw bytes because they are runner semantics; gates are parsed and canonicalized because YAML formatting, comments, mapping order, advice, and measurement methodology do not change progression semantics. Editing documentation (instructions, hints, README, design prompts) never invalidates completed stages; editing tests, fixtures, commands, or a gate requirement invalidates only the stages it affects.

Bundled packs:

| Pack | Stages | Tier | Notes |
|---|---:|---|---|
| `flashindex` | 14 | flagship | Five-level help ladder and diagnosis metadata throughout; parallel indexing with a thread-scaling benchmark and a speedup gate; ranked multi-token search |
| `minikv` | 10 | preview | Append log, recovery, tombstones, compaction |
| `tinyhttp` | 10 | preview | Request parsing, header framing, MIME types, safe paths |
| `byteforgevm` | 11 | preview | Stack arithmetic, control flow, calls, tracing |

All four are proven against a reference solution under `tools/reference_solutions/`.

`deltaforge pack content <pack> --stage <id>` prints exactly what a learner sees for one stage and nothing else. Use it when authoring to read a stage the way it will actually be read, and as the fixture for the content-sufficiency practice described in [product/content-sufficiency.md](product/content-sufficiency.md).
