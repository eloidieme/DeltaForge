# DeltaForge 1.0 review

Date: 2026-09-01
Measured against: [release-1-0-contract.md](release-1-0-contract.md) and
[release-1-0-closeout.md](release-1-0-closeout.md)

An independent pass over the finished product, run after the closeout was written. It
checks the contract's claims against the code, the packs, and the running binary, and
records what it found in the order a reader should care about.

## Method

Four things, in this order.

1. **The build.** `cargo build --all-targets`, `cargo clippy --all-targets -D warnings`,
   `cargo fmt --check`, `cargo test`, `cargo package --list`.
2. **The product, driven by hand.** A fresh `deltaforge init flashindex`, a failing
   `deltaforge test`, `deltaforge explain-failure`, and a `git add -A --dry-run` against
   the resulting tree.
3. **The workbench, probed live.** The service started, then exercised with a real
   client: token present and absent, hostile `Origin` and `Host`, path traversal, and
   fourteen malformed creation requests.
4. **Four focused audits**, each reading one part of the source against one set of
   claims: the browser security boundary, the pack content claims, the documentation
   against `cli.rs` and `config.rs`, and the engine core.

## What holds

The suite is green with no failures: cli_flow 47, workbench_flow 10, the failure corpus
in four groups spanning all fourteen stages, the MCP standard-client suite 4, and the
browser journey. Reference solutions pass all four packs. `clippy -D warnings` and
`fmt --check` are clean. `cargo package` produces a correct 795-file crate carrying all
739 pack files and the UI.

**Activation, measured.** `init` takes 0.47s and the first behavioral run 4.6s on a
machine with a warm toolchain. The contract's target is five minutes.

**Every content claim in the contract and the closeout is true.** Fourteen FlashIndex
stages numbered 01-14 with manifest titles and document headings agreeing; exactly five
help levels on all fourteen and exactly three on all thirty-one preview-pack stages;
`tier: preview` on the right three packs; `prediction.md` on exactly stages 01, 03, 06,
and 12, matching the benchmark-carrying set one to one; no learner-visible file
instructing a reader to run a terminal command; no orphaned or missing stage directory
and all 176 fixture references resolving, across all forty-five stages. Ninety-six of
ninety-six FlashIndex test cases carry diagnosis metadata.

**The browser boundary is largely what `safety.md` says it is.** Loopback-only bind. An
exact `Host: 127.0.0.1:{port}` match that rejects `localhost`, an unrelated hostname, and
a rebinding name resolving to loopback — genuine DNS-rebinding resistance, tested. An
exact `Origin` plus `application/json` on all fifteen POST routes. No static-file handler
of any kind. No shell invocation anywhere in `src/`. `no-referrer`, `X-Frame-Options:
DENY`, `nosniff`, and `no-store` on responses. The creation path policy refused all
fourteen hostile inputs: traversal, absolute paths, UNC, drive-relative `C:evil`,
reserved names, hidden parents, absent parents, and nesting inside an existing project.

**The record is candid.** `content-sufficiency.md` states its own gate as met on five of
fourteen. The closeout names the design-after-implementation deviation rather than
glossing it. The handoff prompts and the gap analysis both carry status banners.

## Findings

### 1. Neither documented install path works

`README.md` offers two ways to install. Neither resolves today.

- No git tag exists in the repository, so `release.yml` has never run and the GitHub
  releases API returns an empty array. The Releases link has nothing behind it.
- The crate name `deltaforge` is not published on crates.io — the registry returns
  404 — so `cargo install deltaforge` fails. `release.yml`'s release body asserts the
  same command. The name is also unclaimed by anyone else.

The closeout calls this "untested end to end". It is stronger than untested: both
published paths are dead, and this is the first thing a new reader tries. Decision 0 says
1.0 is a public open-source product; this is the line item that makes that literal.

### 2. `explain-failure` does not show the diagnosis

`README.md` annotates the command as *"the same diagnosis, in the terminal"*, and
`architecture.md` records it as *"folded into structured failure diagnosis"*. It was not.
`src/commands/explain_failure.rs` is a separate keyword-matching heuristic that prints
raw assertion strings and five canned suggestions.

The data is already there. A CLI test run persists the full diagnosis — `priority`,
`kind`, `headline`, `summary`, `contract`, `expected`, `actual`, `fixture_entries` —
into `last_test_runs`, and `explain-failure --json` serializes all of it. Only the
human-readable branch drops it. The browser renders `Fix this first · <headline>` from
the same structure.

The consequence is that the promise in the README's opening paragraph — a diagnosis that
names the single thing to fix first — is browser-only, in the one place the contract
deliberately kept for people who prefer the terminal. `docs/commands.md` describes the
command accurately; the README does not.

### 3. Stage snapshots commit the build directory

`snapshot::take` runs `git add -A`, and nothing in the product ever writes a
`.gitignore`: not `creation::initialize_git`, not any pack template.

Measured on a fresh project after a single `deltaforge test`: `git add -A --dry-run`
stages twenty-two paths, fifteen of them under `target/`, plus `.deltaforge/state.json`
and `.deltaforge/workbench-events.json`.

`create_stage_snapshot` also holds the run lease across the commit, so
`.deltaforge/run.lock` — carrying the live PID of the process taking the snapshot — is
committed into the tag. Checking that tag out restores a lock file whose PID may belong
to a live process, at which point `run_lease::active` reports a run in progress forever
and every learner action refuses, with nothing pointing at the file.

The product tells learners their code lives in their own folder under their own version
control. A history that is mostly build artifacts is a defect in the thing being sold.

### 4. The capability token has no cryptographic randomness

`workbench::capability_token` is FNV-1a over the project path, the wall-clock nanosecond,
and the pid, formatted as `{hash:016x}{nanos:x}` — so it appends its own seed in
cleartext. A live token decoded during this review carried `1788247751268560100` as its
second half: the launch instant, to the nanosecond. There is no CSPRNG in the dependency
tree.

`architecture.md` states the requirement as an *unguessable session capability*. A
deterministic function of three locally-observable inputs does not meet it. The Host
check keeps this out of reach of a web page, so nothing is exploitable today from a
browser — but the requirement is stated, unmet, and the fix is a few lines.

### 5. The token is world-readable and passed in argv

There is no `set_permissions` call anywhere in `src/`. `fs_util::atomic_write` uses a
plain `File::create`, so on Unix `~/.deltaforge/workbench.json` — which holds the
token — lands at 0644 inside a 0755 directory. `spawn_service` additionally passes
`--token <value>` in the child's argv, readable from `/proc/<pid>/cmdline` on Linux.

On a shared host, another local user reads the token and posts to `/api/v1/runs`, which
executes pack-defined build and run commands under the first user's account. Windows is
unaffected: the record inherits the profile directory's ACL.

`safety.md`'s browser-boundary section describes the token check but not how the token is
generated, where it is stored, or with what permissions.

### 6. Smaller boundary defects

- Token comparison is a plain `!=` on `String`, not constant-time. Minor alone; it
  compounds with finding 4.
- `GET /api/v1/focus` writes `focus_target`, bumps `focus_revision`, and pushes to every
  SSE client, but is guarded by `authorized` rather than `authorized_mutation`. It is the
  one state-changing route outside the mutation rule.
- `serve()` accepts `--token ""`, after which every request authorizes with no credential
  at all, since a request with no query string also yields an empty token. Not reachable
  in normal use — the parent always passes a token — but it should be refused.

### 7. Engine correctness

None of these fire on the happy path. All were traced through the code.

**The event journal is an unlocked read-modify-write.** `run_journal::append` reads the
whole file, pushes, and rewrites it, and is reached from two paths that are not mutually
serialized: `JournalSink::emit` under the run lease, and `publish_event` from workbench
handler threads with no lease. Concurrent appends lose an event and can assign one id
twice, which breaks `entries_after` replay for a reconnecting browser. The journal is
also not append-only despite the framing, though it is crash-safe: every append rewrites
the file through `atomic_write`.

**A corrupt journal bricks the workbench.** `run_journal::read` recovers from a missing
file but bails on a parse error with no self-heal. `workbench_state` propagates that, so
every fetch hard-errors; run-event appends swallow theirs with `let _ =`, so a run
completes with the browser seeing nothing. There is no repair path in `doctor`.

**`run_lease::active()` deletes the file it inspects.** Between reading a stale record and
removing it, another process can clear the same lease and acquire a live one, which the
first process then deletes. Both then believe they hold exclusivity over `state.json`, and
`Drop` removes whatever file is at the path rather than its own. The lease record carries
only a pid, with no start time, so pid reuse makes a dead lease read as live indefinitely.

**`next`, `hint`, and `sync-pack` mutate state with no lease and no journal event.** The
browser equivalents take both. A `deltaforge next` racing a finishing browser run writes a
whole-file snapshot loaded before that run completed, losing `gate_results` and the
finished attempt. An open workbench is never told the step changed and keeps rendering the
old one. Separately, `hint --all` reveals every level including the retrospective, which
the browser gates until the stage is acquired.

**Benchmark gates rest on premises the code does not enforce.** `machine_metadata()`
records only `{os, arch}` from compile-time constants, so the "same machine" comparison is
keyed on nothing that identifies a machine and two different x86_64 Linux hosts diff as
identical. Timing wraps the whole subprocess including spawn, with 10 ms `try_wait`
quantization, which biases speedup ratios toward 1: a true 5× with a 25 ms parallel time
and 15 ms of overhead measures about 3.5×, and speedup gates block progression.
`peak_memory_mb` is a 10 ms-sampled maximum, not a peak. `percentile` returns the upper
middle value for even *n*, so `--iterations 2` reports the maximum as the median.
`warmup: 0` is never rejected.

**Digest exclusions match by name at any depth.** A `src/build/` or `src/target/`
directory in a learner's tree would be invisible to `project_digest`, so edits there
produce no `SourceChanged` event and results keep reporting `Fresh` over changed code. No
bundled pack currently collides. The tree walk is also two-pass and non-atomic, so an
editor saving atomically at the wrong instant fails `deltaforge next` or an entire
workbench state fetch with an unactionable IO error.

**Unbounded hot spin in the workbench startup lease.** The `AlreadyExists` branch
`continue`s without checking the deadline or sleeping when the lease reads as inactive, so
a `remove_file` that keeps failing spins at full CPU and never times out.
`run_lease::try_acquire` caps at two attempts; this one has no cap.

**The completion proof is forgeable** from values already present in the same writable
state file: copy `pack_digest` across and leave `behavioral_digest` empty to take the
legacy comparison branch. For a local single-user tool this is the right trade — the
threat is self-deception, not attack, and no hash helps while the file is writable. What
should change is the vocabulary. "Proof" and "integrity digest" promise tamper-evidence
the design does not attempt; it is a staleness check. The adjacent case is handled
correctly and should stay that way: `gate_status` recomputes pass and fail from the
measurement and the bound and ignores the stored flag, pinned by a test.

### 8. Documentation

- `DELTAFORGE_HOME` and `DELTAFORGE_BIN` are read by the code and documented nowhere.
- The two most-used commands hide most of their surface. `test --stage`, `--all`,
  `--filter`, `--list-tests`, `--fail-fast`, `--no-build`, `--keep-temp` and
  `bench --stage`, `--all`, `--iterations`, `--warmup` appear in no document. So do
  `init --name`, `--no-git`, `--stage`.
- `docs/test-format.md` never documents the `diagnosis` field — the one that implements
  the promise in the README's opening paragraph and the `pack-format.md` flagship bar.
- `release-1-0-gap.md` carries six broken links of the form `../../src/application.rs:24`.
  That is not a filename; GitHub needs `#L24`.
- `Spec.md` is the only superseded planning document without a status banner, while the
  handoff prompts and the gap analysis both received one. It is the largest file at the
  repository root and still presents profiling, boss fights, and optimization quests as
  product.
- There is no `SECURITY.md` for a product that ships a local HTTP service which executes
  commands.
- `workbench.rs` prints `You can run checks with: deltaforge test` on launch — the
  application doing the thing the thirty-nine-place content sweep removed from the packs.

### 9. Two observations, not defects

The preview packs carry no diagnosis metadata at all: zero of 105 test cases across
MiniKV, TinyHTTP, and ByteForgeVM, against ninety-six of ninety-six for FlashIndex. That
is exactly what `tier: preview` and decision 2 describe, and the README's project table
scopes the content bar correctly. Only the README's opening paragraph states the promise
without qualification.

`deltaforge test` prints the redacted `{fixture_path}` placeholder in its expectation line
and the real temporary path in the `actual stdout:` block directly below it. The corpus
asserts no leaked temporary path in the diagnosis fields, which holds. Showing a learner
their own temp path is harmless and arguably useful; showing both spellings of the same
string two lines apart is confusing.

## If five things were fixed

1. Publish the release: tag `v1.0.0`, and claim the crate name.
2. Print the diagnosis `explain-failure` already loads.
3. Write a `.gitignore` at creation, and exclude `.deltaforge/run.lock` from snapshots.
4. Generate the token from the OS CSPRNG, store it 0600, and keep it out of argv.
5. Key benchmark comparison on something that identifies a machine.
