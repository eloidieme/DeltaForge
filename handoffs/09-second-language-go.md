# Task 09 — Second language template (Go) + multi-file reference support

*(Prepend `handoffs/00-shared-context.md`. Independent of the performance tasks; if Task 03 added flashindex stages 09–10, the Go template and reference must cover them too — check first.)*

## Objective

DeltaForge claims to be language-agnostic (Spec §7.2) but every pack ships only a Rust template, and `check_reference` can only copy a single `src/main.rs`. Prove the claim end-to-end: add a Go template to the FlashIndex pack with a Go reference solution, and generalize reference checking to arbitrary languages/file layouts.

## Part 1 — Generalize reference checking (do this first)

- `src/authoring.rs::check_reference` currently does `fs::copy(reference, project_dir.join("src/main.rs"))`. Change: when `--reference` is a **directory**, overlay its contents recursively onto the initialized project (overwriting collisions); when it is a file, keep the current single-file Rust behavior for backward compatibility. Reject symlinks inside the overlay (reuse the existing symlink-rejection helpers).
- Remove the hard-coded `bail!("only rust templates are currently scaffolded")` constraint only where it blocks *checking* non-Rust languages; `pack new` scaffolding may stay Rust-only.
- Update the `pack check-reference` CLI help, the MCP `check_reference` schema/description, and `docs/authoring-packs.md`.

## Part 2 — Go template for FlashIndex

- `packs/flashindex/project.yaml` gains:
  ```yaml
  go:
    template: templates/go
    build:
      command: ["go", "build", "-o", "flashindex-bin", "."]
    run:
      command: ["./flashindex-bin"]
  ```
  plus `bench_run` if that field exists (foundation pass) — same binary. **Windows**: `./flashindex-bin` must resolve to `flashindex-bin.exe`; verify how the process runner resolves relative program paths on Windows and normalize (append `EXE_SUFFIX` when resolving a relative first-arg that isn't found as-is) — this is the one genuinely platform-sensitive piece; solve it in the runner, not the pack.
- `packs/flashindex/templates/go/`: `go.mod` (module name `flashindex`, a Go version comfortably available in CI), and a starter `main.go` mirroring the Rust starter's shape (usage message on no args, exit code 1). Update `ignored_paths` if Go emits build artifacts (the single binary is covered by `.gitignore` conventions — add a template `.gitignore` if the Rust template has one; check).
- Update pack README/overview to mention both languages; bump pack version.

## Part 3 — Go reference solution

- `tools/reference_solutions/flashindex_go/` implementing **all** flashindex stages (stdlib only: `filepath.WalkDir`, `bufio`, `sort`; goroutines + local maps for the parallel stage if it exists). Deterministic output identical to the Rust reference's expected behavior.
- Integration test in `tests/cli_flow.rs`: init flashindex with `--lang go`, overlay the Go reference via the directory-overlay path, run `test --all`, assert success. **Skip gracefully** (with a printed notice) when `go` is not on PATH locally, but ensure CI always runs it.
- CI (`.github/workflows/ci.yml`): add `actions/setup-go` (stable Go) to the matrix so all three OSes exercise the Go path.

## Acceptance criteria
- `deltaforge init flashindex --lang go` produces a project where `deltaforge test` runs (and fails on the starter, like Rust).
- `pack check-reference flashindex --lang go --reference tools/reference_solutions/flashindex_go` passes all stages, via CLI and via the MCP tool.
- Single-file Rust reference checking still works (existing tests untouched and green).
- `validate-pack --strict` clean; full quality bar passes on all three CI OSes (watch the Windows binary-suffix case specifically).

## Out of scope
Go templates for the other three packs (follow-up once the pattern is proven), C++/Zig templates, cross-language comparison command (Spec §21 — deferred).
