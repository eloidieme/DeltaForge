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
stages:
  - id: 01_scan_files
    title: Scan files
    path: stages/01_scan_files
```

Each stage requires `instructions.md`, `hints.md`, and `tests.yaml`. `benchmarks.yaml` and `design_prompt.md` are optional.

Bundled packs currently include:

- `flashindex`: 8 stages, with reference solution coverage.
- `minikv`: 6 stages, with reference solution coverage.
- `tinyhttp`: 6 stages, with reference solution coverage.
- `byteforgevm`: 6 stages, with reference solution coverage.
