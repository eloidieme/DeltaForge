# Handoff: remaining work from the second 1.0 review

This continues `docs/product/release-1-0-review-2.md`. Read that document first —
it is the full review; this file only tracks what is left after a prior pass.

## What's already done (context, don't redo)

Three commits on `main`, already pushed to `origin/main`:

- `f679291` — D1, D2 (+R1), D4, D10, R2 (workbench performance, launcher/token security)
- `20f449c` — D3, D5, D6, D7, D8, D9, D11, D12 (progression and Git snapshot correctness)
- `9542c09` — D13, D16, D18 (partial), R10 (remaining diagnostics/hardening)

Every defect above got a regression test, added alongside the fix. D14 and D15 landed
inside the D11 commit. The full suite (`cargo test --all-targets`), `cargo clippy
--all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check` are all
clean as of `9542c09`. D4 and D10 were additionally verified live in the browser (both
color themes) since they touch the security boundary and the frontend respectively.

## What's left

### 1. A live bug found while fixing D12, not yet fixed

**MiniKV's checked-in template `Cargo.lock` is rewritten by the learner's first
`cargo build`,** which changes the project digest between when `deltaforge test`
records a completion proof and when `commit`/`next` check it. Net effect: a bare
`deltaforge test` → `deltaforge commit` (no `--force`) on a *fresh* MiniKV project can
fail with `learner project changed since stage ... passed` even though nothing was
edited.

Repro (from repo root):

```bash
rm -rf /tmp/df-minikv-lockbug
./target/debug/deltaforge init minikv --lang rust --name /tmp/df-minikv-lockbug
cd /tmp/df-minikv-lockbug
git config user.email a@b.com && git config user.name t
cp <repo>/tools/reference_solutions/minikv_rust/src/main.rs src/main.rs
<repo>/target/debug/deltaforge test
<repo>/target/debug/deltaforge commit
# error: learner project changed since stage 01_memory_commands passed; run `deltaforge test` again
```

Confirm with `md5`/`sha256` on `Cargo.lock` before and after `deltaforge test`: the
hash changes even though the learner touched nothing. `src/application.rs`'s
`run_tests` (`run_project_digest = context.project_digest()?`, near line 913) captures
the digest *before* the build runs, so a lockfile rewrite that happens as a build side
effect looks like a learner edit.

FlashIndex's template Cargo.lock does **not** exhibit this on the machine this was
found on (confirmed: identical `test` → `next` sequence on FlashIndex passes cleanly).
Likely cause: MiniKV's/TinyHTTP's/ByteForgeVM's checked-in lockfiles were generated
with a different cargo/rustc version than whatever generated FlashIndex's, and get
silently upgraded (e.g. lockfile format version bump) on first build.

**Before picking a fix**, reproduce with all four packs to confirm which are actually
affected — this may be toolchain-version-dependent, so check on more than one Rust
version if possible. Suggested fixes, roughly in order of robustness:

1. Regenerate `packs/{minikv,tinyhttp,byteforgevm}/templates/rust/Cargo.lock` with
   whatever process produced FlashIndex's (or check whether FlashIndex's template even
   ships a `Cargo.lock` at all — if not, that absence may be *why* it doesn't drift,
   and dropping the other three templates' lockfiles and letting `cargo build`
   generate one fresh may be the simplest fix).
2. If a checked-in lockfile is intentional, consider excluding `Cargo.lock` from
   `ProjectContext::project_digest`'s hardcoded exclusion list (`src/context.rs`) —
   but these packs have zero external dependencies, so weigh whether this is ever the
   right general answer for a pack that *does* have dependencies, where a learner's
   edit to Cargo.lock could matter.
3. Alternatively, capture `run_project_digest` in `run_tests`
   (`src/application.rs`) *after* the build completes rather than before — check
   whether the `SourceChanged` event-detection logic elsewhere in that function
   depends on the current (pre-build) capture point before changing it.

Add a regression test to `tests/cli_flow.rs` mirroring the repro above (bare `test`
then `commit`, no `--force`, on whichever packs are affected) once fixed.

A background task (`task_2fa1a304`, title "Fix MiniKV template Cargo.lock drift
staling proofs") may already exist for this in the session that found it — check
before duplicating effort, but that task's context does not survive into a fresh
session, so this write-up is the authoritative version.

### 2. Risks not yet addressed (Tier 3, explicitly lower-priority per the review)

All of these are described in full, with file/line pointers and reproduction detail,
in `docs/product/release-1-0-review-2.md` §3 ("Risks") — read the relevant subsection
there before starting each one; this list is just an index plus current-repo pointers
so you don't have to grep for them.

- **R3 — orphaned build children.** `terminate_process_tree` (`src/process.rs`) is
  only called on timeout/cancellation; if the workbench service itself is killed, an
  in-flight `cargo build` (in its own process group on Unix,
  `configure_process_group`) keeps running with no parent. State recovery is already
  correct (verified in the review); this is about the orphan continuing to burn CPU
  and write into `target/`. Suggested: Unix `prctl(PR_SET_PDEATHSIG)` — note this is
  **Linux-only**, not available on macOS/BSD, so it needs `#[cfg(target_os = "linux")]`
  and a documented no-op elsewhere, or a different mechanism for other Unixes; Windows
  needs a job object. Cannot be fully verified on macOS alone — test on Linux and
  Windows if at all possible before considering this done.
- **R4 — unbounded pre-authentication connections.** `handle_connection`
  (`src/workbench.rs`) spawns a thread per accepted TCP connection before any token
  check, with no cap. Not observed to be a practical problem (600 concurrent stalled
  connections handled fine in the review), but has no bound. A small semaphore-bounded
  worker pool would close it.
- **R6 — benchmark peak-memory fidelity.** `peak_rss_bytes` (`src/process.rs`) polls
  every 1ms; on macOS this samples *current* RSS, not a high-water mark, so a short
  spike between polls is invisible, and a process that exits before the first poll
  reports `None`. Either label the number as approximate in the UI/report, or use
  `getrusage(RUSAGE_CHILDREN)`'s `ru_maxrss` after `wait` on Unix for an exact
  high-water mark.
- **R8 — unbounded growth of two files.** `benchmarks::append_history`
  (`src/benchmarks.rs`) rewrites `.deltaforge/benchmark_history.json` with no cap.
  `run_journal::read_entries_unlocked`'s corrupt-journal quarantine (now
  `workbench-events.corrupt-<nanos>.json`, post-D2 fix) also has no limit on how many
  accumulate. Both need an easy cap (e.g. keep only the N most recent quarantined
  files; cap benchmark history entries the same way the event journal is now capped).
- **R9 — pack MCP path containment.** `src/bin/deltaforge-pack-mcp.rs`'s `pack_dir`
  and friends accept arbitrary filesystem paths from tool arguments with no
  containment (this is consistent with its framing as maintainer tooling driven by a
  local agent at the same privilege level — not a bug on its own). Just needs an
  explicit statement in `docs/authoring-packs.md` that this server should never be
  exposed to a less-trusted client.

### 3. One cosmetic nit, unverifiable on macOS

**D18's mixed-separator warning.** Pack discovery warnings on Windows can render as
`…/sandbox/badpacks\broken\project.yaml` (mixed `/` and `\`) because a test-constructed
path built with forward slashes gets joined with `PathBuf::join`, which uses the native
separator. Low value, purely cosmetic, and requires a Windows machine to verify a fix
actually renders correctly — skip unless you have Windows access.

## Process notes carried over from the previous pass

- Build/verify loop: `cargo build --lib`, then `cargo clippy --all-targets
  --all-features -- -D warnings`, then `cargo fmt --all` (or `-- --check`), then
  `cargo test --all-targets` (prefix with `GIT_CONFIG_GLOBAL=/dev/null` so a
  developer's own global Git config/hooks can't leak into the hermetic tests). The
  full suite takes ~80-90s; it will often exceed a 2-minute foreground timeout — run
  it in the background and wait for the notification rather than polling.
- **Timing-based regression tests are flaky under `cargo test`'s parallel execution.**
  Compare against a same-run baseline measurement instead of an absolute wall-clock
  bound (see `run_journal::tests::appends_do_not_rewrite_prior_events` for the
  pattern this pass converged on, after an absolute-threshold version flaked once
  under load).
- **Test temp-dir helpers using only pid+nanos for uniqueness can collide** under
  parallel test threads on some clocks. If you add a new `temp_root()`/
  `temp_project_path()`-style helper, give it a process-wide `AtomicU64` counter too
  (existing helpers in `tests/cli_flow.rs` and `tests/workbench_flow.rs` already do,
  or have been fixed to).
- A `--project-dir`-free flow that runs `cargo build` inside a **real** git project
  (not `--no-git`) needs the Cargo.lock drift above worked around (settle it with a
  manual `cargo build` before the first `deltaforge test`) until item 1 is actually
  fixed.
- Commit style used this pass: small logical commits (one per related cluster of
  findings, not strictly one per finding — several findings shared files too heavily
  to split cleanly), imperative subject, body explaining *why* each fix matters with
  the defect id, `Co-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>` trailer.
  Push to `origin/main` when done unless told otherwise.
