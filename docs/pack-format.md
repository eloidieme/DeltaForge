# Pack Format

Packs are directories with `project.yaml`, language templates, and stage directories.

```yaml
schema_version: 1
id: flashindex
name: FlashIndex
version: 0.1.0
description: Local source-code search engine
languages:
  rust:
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

Each stage requires `instructions.md`, `hints.md`, and `tests.yaml`. `benchmarks.yaml` and `design_prompt.md` are optional.

Language spec fields:

- `template` (required): path to the language starter template, copied into the learner project.
- `build` (optional): command run before tests and benchmarks.
- `run` (required): command used by `deltaforge test` to invoke the learner's program.
- `bench_run` (optional): command used by `deltaforge bench` to time the learner's program after the build step. It falls back to `run` when absent, so it is optional at `schema_version: 1`. Prefer pointing it directly at the built binary (for example `./target/release/<binary>`) so benchmarks measure the program rather than build-tool startup overhead. A relative first element is resolved against the project root and receives the platform executable suffix on Windows.

A pack's `ignored_paths` are excluded (in addition to a built-in list: `.git`, `.deltaforge`, `target`, `build`, `node_modules`, `__pycache__`, `.venv`, `.DS_Store`, plus the learner's `integrity.exclude` config) when computing the learner project digest that guards stage completion.

Pack content must be self-contained: `validate-pack` reports every symbolic link or special file in a pack (base validation, not just `--strict`), and pack digesting rejects them at init/sync time. A symlinked `tests.yaml` or fixture would let pack behavior change while the recorded digest stayed the same, defeating pinning.

Stage completion proofs pin a per-stage behavioral digest covering the stage's `tests.yaml`, its `fixtures/` tree, and the language `build`/`run` commands. Gate-bearing stages additionally hash canonical parsed gate semantics: the referenced benchmark execution definition, metric/bound/selector, and `bench_run`. Tests and fixture contents are hashed as raw bytes because they are runner semantics; gates are parsed and canonicalized because YAML formatting, comments, mapping order, advice, and measurement methodology do not change progression semantics. Editing documentation (instructions, hints, README, design prompts) never invalidates completed stages; editing tests, fixtures, commands, or a gate requirement invalidates only the stages it affects.

Bundled packs currently include:

- `flashindex`: 8 stages, with reference solution coverage.
- `minikv`: 6 stages, with reference solution coverage.
- `tinyhttp`: 6 stages, with reference solution coverage.
- `byteforgevm`: 6 stages, with reference solution coverage.
