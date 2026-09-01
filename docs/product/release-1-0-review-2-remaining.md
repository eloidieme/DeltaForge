# Handoff: remaining work from the second 1.0 review

This continues `docs/product/release-1-0-review-2.md`. Read that document first —
it is the full review; this file only tracks what is left after the passes below.

## What's done (context, don't redo)

Five commits on `main`:

- `f679291` — D1, D2 (+R1), D4, D10, R2 (workbench performance, launcher/token security)
- `20f449c` — D3, D5, D6, D7, D8, D9, D11, D12 (progression and Git snapshot correctness)
- `9542c09` — D13, D16, D18 (partial), R10 (remaining diagnostics/hardening)
- `d591c4f` — the Cargo.lock drift that staled completion proofs
- `7867e92` — R4, R6, R8, R9, and a documented decision on R3

Every defect got a regression test alongside the fix. `cargo test --all-targets`
(188 tests across 8 suites), `cargo clippy --all-targets --all-features -- -D warnings`,
and `cargo fmt --all -- --check` are clean as of `7867e92`. The workbench was
additionally verified live in the browser in both colour schemes after `7867e92`,
since that commit changes the connection accept path.

### The Cargo.lock drift (`d591c4f`)

Root cause: MiniKV, TinyHTTP and ByteForgeVM each shipped a hand-written stub
`Cargo.lock` containing only `version = 4`, with no `[[package]]` entry for their
own crate. Cargo completed it during the learner's first `cargo build` — the one
`deltaforge test` runs — and that write, landing inside the project tree, was
indistinguishable from a learner edit. FlashIndex was unaffected only because its
template lockfile happened to be a real generated one.

Fixed by regenerating the three lockfiles as Cargo fixpoints and fixing the pack
scaffolder (`src/authoring.rs`), which emitted the same stub into every newly
authored pack. `validate-pack` now rejects a template lockfile that does not lock
its own package, statically and without a build, so it cannot ship again.

The digest capture point in `run_tests` was deliberately left before the build:
capturing after would record source the tests never ran against, turning a
confusing error into a false completion proof.

## What's left

### 1. R3 — orphaned build children (decided, not fixed)

`terminate_process_tree` (`src/process.rs`) runs only on timeout or cancellation.
If DeltaForge itself is killed outright, the in-flight `cargo build` — in its own
process group — is reparented to init and runs to completion.

Confirmed to reproduce on macOS. Deliberately not fixed; the reasoning is written
up in `docs/safety.md` under "Build processes can outlive DeltaForge". In short:
the symptom is bounded (state recovery is correct, and Cargo's own lock on
`target/` means a later run waits rather than colliding), every real fix is
per-platform (`PR_SET_PDEATHSIG` on Linux, a job object on Windows, neither
available on macOS or the BSDs), and the portable substitute — recording the
child's pid and killing it on the next run — can name a recycled pid belonging to
an unrelated process.

**Reopen this only with a Linux and a Windows machine to verify on.** The Linux
mechanism additionally needs care: `PR_SET_PDEATHSIG` fires when the parent
*thread* dies, not the parent process, so it is only safe here because the thread
that spawns the child is the same one that waits for it — a refactor that moves
the wait elsewhere would silently start killing live builds.

### 2. D18's mixed-separator nit, unverifiable on macOS

Pack discovery warnings on Windows can render as
`…/sandbox/badpacks\broken\project.yaml` (mixed `/` and `\`) because a
test-constructed path built with forward slashes gets joined with `PathBuf::join`,
which uses the native separator. Cosmetic, arguably a test artifact rather than a
product defect, and it needs a Windows machine to confirm a fix renders correctly.
Skip unless you have Windows access.

## Process notes

- Build/verify loop: `cargo build --lib`, then `cargo clippy --all-targets
  --all-features -- -D warnings`, then `cargo fmt --all` (or `-- --check`), then
  `cargo test --all-targets` (prefix with `GIT_CONFIG_GLOBAL=/dev/null` so a
  developer's own global Git config/hooks can't leak into the hermetic tests). The
  full suite takes ~100s and will exceed a 2-minute foreground timeout — run it in
  the background and wait rather than polling.
- **Timing-based regression tests are flaky under `cargo test`'s parallel
  execution.** Compare against a same-run baseline measurement instead of an
  absolute wall-clock bound (see
  `run_journal::tests::appends_do_not_rewrite_prior_events`).
- Test temp-dir helpers need a process-wide `AtomicU64` counter as well as
  pid+nanos; pid+nanos alone can collide under parallel test threads. The helpers
  in `tests/cli_flow.rs` and `tests/workbench_flow.rs` both have one now.
- Shell-invoked tools differ across platforms in ways that bite cfg'd tests: `dd`'s
  `bs=64m` suffix is BSD-only, so a test cfg'd for both Linux and macOS must spell
  the size in bytes.
- Commit style: small logical commits (one per related cluster of findings),
  imperative subject, body explaining *why* each fix matters with the defect id,
  `Co-Authored-By:` trailer. Push to `origin/main` when done unless told otherwise.
