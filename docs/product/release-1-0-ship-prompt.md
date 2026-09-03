# Handoff prompt — take DeltaForge to an Apple/Google 1.0

Paste everything below the rule into a fresh agent session started in
`/Users/eloidieme/Projects/DeltaForge`.

---

## Mission

DeltaForge 1.0 was reviewed on 2026-09-02 against a single question: *would this ship as
an Apple or a Google 1.0?* The answer was no, for 33 specific, reproducible reasons.

**Your job is to make the answer yes.** Fix every finding, prove each fix, and leave the
project in a state where a stranger can install it and complete FlashIndex step 1 without
help.

Read this first, in full, before touching anything:

```
docs/product/release-1-0-ship-review.md
```

That document is your work order. It contains every finding with its exact file, line,
mechanism, reproduction, and recommended fix. A rendered copy is at
<https://claude.ai/code/artifact/fdcab90a-ca4a-439f-91c1-0c411b30bac0> — the local file is
authoritative.

You have **complete autonomy**. Do not stop to ask permission for ordinary engineering
decisions. Do not scope this down. Finish it.

---

## Context you must load before deciding anything

Read these, in this order. They are short and they will stop you from re-litigating
decisions that were already made deliberately.

| File | Why |
|---|---|
| `docs/product/release-1-0-ship-review.md` | **The work order.** All 33 findings. |
| `docs/product/release-1-0-contract.md` | Frozen scope + 16 decisions + 4 amendments. Scope conflicts resolve here. |
| `docs/product/release-1-0-closeout.md` | What shipped, and what the 1.0 work found. Explains several defect shapes. |
| `docs/product/release-1-0-review-2-remaining.md` | The two items deliberately left open (R3 orphaned builds, D18 Windows separator). **Do not reopen R3** — see below. |
| `docs/product/architecture.md` | The local security boundary and the creation path amendment (A3). P0-1's fix must stay inside it. |
| `docs/product/design-1-0.md` | The visual system as built. P1-1/P1-2 changes must stay inside it. |
| `docs/safety.md` | The documented decision on build processes outliving DeltaForge. |
| `docs/pack-format.md`, `docs/test-format.md`, `docs/content-style.md` | Needed for P0-3's validator rule and the content sweep. |
| `README.md`, `CHANGELOG.md` | Both will need updating at the end. |

`handoffs/` is marked historical and predates the 1.0 contract. Ignore it as guidance;
you will be relocating it as part of P3-2.

### Decisions that are already made — do not revisit

- **Rust only** in 1.0. No second language.
- **FlashIndex is the flagship; the other three ship as `tier: preview`.** Do not raise
  them to flagship. But P0-3's content sweep *does* cover all four packs.
- **No AI in the learner experience.** MCP stays authoring-only.
- **No telemetry, accounts, cloud, or anything multi-learner.**
- **R3 (orphaned build children) stays decided-not-fixed.** It needs Linux and Windows
  hardware to verify; the reasoning is in `docs/safety.md`. Leave it.
- **D18's Windows mixed-separator nit** needs a Windows machine. Leave it unless CI can
  prove a fix.

---

## Working agreement

**Autonomy.** Work continuously through the plan. Do not check in between tranches. Do
not ask whether to proceed. If a finding turns out to be wrong or already fixed, say so in
your final report with evidence and move on — do not silently drop it.

**Use subagents aggressively, driven by precise specs you write first.** This has worked
well on this codebase before. Good candidates to delegate to a lighter model:

- The P0-3 content sweep across all 45 stages in 4 packs (mechanical, high volume).
- The P2 finish list — most items are single-file, well-specified edits.
- Writing the `CONTRIBUTING.md` / `CODE_OF_CONDUCT.md` / issue templates (P3-1).
- Re-running the nine remaining content-sufficiency stages (P3-7) via
  `tools/content_sufficiency/`.
- Auditing all four packs for markdown constructs the new renderer must handle.

**Keep in-house** (do not delegate): the creation-path fix (P0-1), the browser test
harness (P0-2), the renderer grammar change (P0-3), lock-poisoning and `catch_unwind`
(P1-5), the watcher redesign (P1-4), and the state migration ladder (P2-6). These are
architectural or concurrency work.

**Write helper scripts freely.** Put throwaway ones in your scratchpad; put anything worth
re-running under `tools/`. At minimum you will want:

- `tools/a11y/contrast_check.py` — parse `src/ui/app.css` tokens, compute every
  foreground/background pair, fail on AA violations. Wire into CI (P1-2).
- `tools/perf/idle_cpu.py` — start the service, sample `ps -o time=` over 30 s with and
  without a client, assert a ceiling. Wire into CI (P1-4).
- A markdown-fidelity checker used by both `validate-pack --strict` and a test (P0-3).

**Search the web when you need current facts.** Specifically: macOS notarization for a
CLI binary distributed in a tarball (`codesign` + `notarytool` + stapling, and what
happens to a non-app-bundle binary), GitHub `actions/attest-build-provenance` usage, and
current `cargo-deny` configuration. Do not guess at these; they change.

**Use the browser.** `mcp__Claude_Browser__*` is available. Verify every UI change
visually in **both** colour schemes and at **both** 1280px and 900px widths. The review
was produced this way; reproduce its screenshots before and after.

---

## The plan — four tranches, in order

Each tranche has a gate. **Do not start the next tranche until the current gate holds.**
The ordering exists so later work cannot paper over earlier breakage.

### Tranche 1 — Make the first run work, and prove it (~2–3 days)

Nothing else matters until a stranger can create a project. Do the harness in the same
tranche, because the fix without the harness leaves the hole that produced the bug.

1. **P0-1** — Fix creation with the default workspace. Send `parent_directory: null` when
   the Location field is untouched and equals `/api/v1/workspace`'s value; have preflight
   report a missing *permitted* parent as *will be created*, not as a refusal; keep the
   refusal for a parent the user typed. Stay inside the A3 path boundary in
   `architecture.md` — creation still accepts a parent and a leaf, never a full path, and
   still resolves through one guarded function.
2. **P0-2** — Add a headless-browser journey CI job on Linux driving the real `app.js`:
   catalog → create with defaults → first failing run → reveal a hint → pass → snapshot →
   benchmark with a prediction → export. Keep `tests/browser_journey.rs` as the fast HTTP
   contract test.
3. **P0-2** — Add a Node unit suite over `app.js`'s pure functions (route table, request
   bodies, rendering). Run it in CI.
4. **Delete `~/DeltaForge` on this machine and keep it deleted** so local dogfooding
   matches a stranger's.

**Reproduction to confirm before you start:**

```bash
rm -rf ~/DeltaForge
cargo run -- --help >/dev/null    # build
DELTAFORGE_NO_BROWSER=1 cargo run
# open the printed URL, Catalog → FlashIndex → Start this project
# expect: "CANNOT CREATE HERE … is not an existing directory", Create disabled
```

**Gate:** a fresh user account with no `~/DeltaForge` and no `~/.deltaforge` reaches a
first failing check run entirely in the browser, with no terminal command but
`deltaforge`. Prove it with the new headless job, not by hand.

---

### Tranche 2 — Stop corrupting content, stop the app bricking (~3–4 days)

1. **P0-3** — Add `Heading` and `InlineCode` to `OverviewBlock` in `src/capability.rs`;
   render `<h3>` and `<code>` in `app.js`. Delete the blanket
   `text.replace(['`','*'], "")` in `strip_inline_markdown` and parse inline spans
   properly. Route prediction prompts and hints through the same renderer.
2. **P0-3** — Add a `validate-pack --strict` rule that fails on any markdown construct the
   renderer cannot represent. Strengthen `every_shipped_stage_fills_every_panel` from
   *non-empty* to *faithful*: no rendered block may contain a residual `#`, backtick, or
   unmatched `*`.
3. **P0-3 / P3-9** — Sweep all four packs. Re-run `deltaforge pack content` on all 45
   stages as the evidence. **Delegate this.**
4. **P1-5** — Recover from lock poisoning throughout `src/` (the correct pattern already
   exists in `tests/browser_journey.rs`'s helper). Wrap each connection handler in
   `catch_unwind` so a panic returns 500 instead of poisoning shared state. Install a
   `panic::set_hook` writing to a log file under the DeltaForge home.
5. **P1-5** — Client-side: `window.onerror`, `unhandledrejection`, an `onerror` on the
   `app-events` `EventSource`, and a real disconnected state after N failed reconnects
   that names the workbench as stopped and says how to restart. Document `IDLE_TIMEOUT`.
6. **P2-6** — Write the state migration ladder now, while there is exactly one version to
   migrate from. A `migrate(from, to)` chain plus a test that loads a v1 fixture and
   reaches v2.

**Reproduction to confirm before you start:**

```bash
cargo run -- pack content byteforgevm --stage 02_more_arithmetic | sed -n '32p'
# expect: "- MUL: pop right, pop left, then push left right."   (operator deleted)
grep -rn "^###" packs/*/stages/*/instructions.md | wc -l   # expect 22
```

**Gate:** no stage in any pack renders a residual `#`, backtick or unmatched `*`; a
deliberately-panicking handler returns 500 and the next request succeeds (write that
test).

---

### Tranche 3 — Pass the reviews the launch would face (~4–5 days)

1. **P1-1** — Fix the ≤1120px collapse so the primary action never falls below the fold.
2. **P1-2** — Fix `--text-3` and `--line-strong` on both grounds; add the contrast check
   to CI.
3. **P1-3** — SPA route semantics: `document.title` per route, focus move to the new `h1`
   with `tabindex="-1"`, `aria-current="page"` on the active nav item. Fix the creation
   form's error association (`aria-invalid`, `aria-describedby`, inline live message,
   `aria-disabled` instead of `disabled`). Remove `aria-live` from the theme button.
4. **P1-4** — Cut idle CPU: watch only the viewed project, gate the walk on directory
   mtime, back off adaptively (500 ms → 5 s), cache the `verify_pack_pin` walk keyed on
   the pack directory's mtime. Add the idle-CPU assertion to CI.
5. **P2-1** — Sweep terminal instructions out of application strings the way pack content
   was swept: `src/context.rs:218,223,229,376,396,406`, `src/application.rs:2193`,
   `src/state.rs:595`.
6. **P2-2 … P2-15** — Work the whole finish list. **Delegate most of these**, one subagent
   per cluster, with a precise spec per item written from the review.

**Gate:** every token pair clears AA (the CI script says so); idle CPU under an open
browser tab is below 1% of a core (the CI script says so); the whole journey works at
1280×800 and at 900px wide, in both themes, verified in the browser.

---

### Tranche 4 — Actually release it (~2–3 days plus a wait)

The release workflow is the last untested subsystem, and its first run is partly
irreversible.

1. **P0-4** — Gate `publish-crate` on `needs: [build]`. Add `cargo publish --dry-run` to
   CI.
2. **P0-4** — Add the missing CI jobs: MSRV pinned to 1.85, `cargo deny`, Dependabot,
   `aarch64-unknown-linux-gnu`, build attestations.
3. **P0-4** — Decide macOS signing. Notarize, or state the Gatekeeper step in the README
   with the exact command. **Do not leave a user to discover it.** Research this properly
   — a bare binary in a tarball behaves differently from an app bundle.
4. **P0-4** — Tag `v1.0.0-rc.1` and install from the published archive on macOS, Linux and
   Windows. This is the only way the install path stops being a claim. If you cannot reach
   Linux or Windows hardware, drive it through CI on those runners and say so explicitly.
5. **P3-1 … P3-4** — Hygiene files; relocate `handoffs/` and `Spec.md`; make README links
   absolute so they resolve on crates.io.
6. **P3-5, P3-6** — CSP hashes instead of `unsafe-inline`; `set_write_timeout` on the main
   request path.
7. **P3-7** — Run the remaining nine content-sufficiency stages via
   `tools/content_sufficiency/`. **Delegate.** Act on what they find.
8. **P3-8** — You cannot perform this one; it needs a human who is not the author. Leave a
   ready-to-run protocol and say so.

**Gate:** a binary installed from a published archive, on a machine that has never built
this project, completes FlashIndex step 1 in the browser. Then, and only then, tag
`v1.0.0`.

---

## Verification loop — run this after every change

```bash
cargo build --lib
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
GIT_CONFIG_GLOBAL=/dev/null cargo test          # ~100 s; run in background, do not poll
cargo run -- validate-pack --strict
cargo run -- pack check-reference               # if you touched packs
```

`GIT_CONFIG_GLOBAL=/dev/null` matters — a developer's global Git config or hooks otherwise
leak into the hermetic tests.

Plus, for UI work: open the workbench in the browser and check **light and dark**, at
**1280px and 900px**.

---

## Landmines — these have each cost a previous session real time

- **`tests/cli_flow.rs` and `tests/workbench_flow.rs` hard-code FlashIndex stage-01's test
  count in six places.** Adding one test case means updating all six.
- **`pack check-reference` runs the final solution against every stage**, so early-stage
  fixtures must stay forward-compatible.
- Editing `instructions.md` / `hints.md` never invalidates completion proofs; editing
  `tests.yaml`, fixtures, or commands does.
- **Learner actions take the run lease with a bounded wait**, because the source watcher
  takes it every poll. An instant-fail acquisition made browser actions flaky. Preserve
  that when you redesign the watcher for P1-4.
- **Timing-based tests are flaky under parallel `cargo test`.** Compare against a same-run
  baseline, never an absolute wall-clock bound. See
  `run_journal::tests::appends_do_not_rewrite_prior_events`.
- **Temp-dir helpers need a process-wide `AtomicU64`**; pid+nanos alone collides under
  parallel test threads.
- **Shell tools differ across platforms in cfg'd tests**: `dd`'s `bs=64m` suffix is
  BSD-only — spell sizes in bytes when a test is cfg'd for Linux too.
- **CRLF fixtures need `.gitattributes -text` protection.**
- **The digest capture in `run_tests` is deliberately before the build.** Capturing after
  would certify source the tests never ran against. Do not "fix" this.
- **`classify_project_health_error` is order-sensitive** — the `integrity digest` and
  `capability instructions are missing` branches must precede the `config.toml` substring
  check.
- **`parse_help` numbers levels by position, not by heading digits**, because `hint_state`
  is a count and `HelpLevel::level` is an index. Do not change this while touching the
  renderer.
- **Known flake:** `source_changes_are_durable_filtered_and_recovered_after_restart` failed
  once under parallel load. Its fixed `sleep(1_200)` waits are tuned to the watcher's
  cadence — **P1-4 changes that cadence, so this test will need retuning.** Expect it.

---

## Commit and delivery

- Small, logical commits — one per related cluster of findings.
- Imperative subject; body explains *why*, naming the finding id (`P0-1`, `P2-7`, …).
- End every commit message with:
  `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`
- Branch off `main` (do not commit directly to `main`), push when a tranche's gate holds.
- **Every fix gets a regression test.** A fix without a test is not done — that is the
  root cause of this entire review.

## Documents to update before you finish

- `docs/product/release-1-0-ship-review.md` — annotate each finding with its resolution
  and the commit that closed it. Do not delete findings.
- `docs/product/release-1-0-contract.md` — add amendments for anything you change that the
  contract fixed (creation defaults and gate enforcement are the likely ones).
- `CHANGELOG.md` — a real entry for whatever version this becomes.
- `README.md` — install instructions that match reality once a release exists.
- `docs/commands.md`, `docs/config.md` — the idle timeout, `doctor --repair`, grouped help.

## Final report

When the tranche-4 gate holds, write `docs/product/release-1-0-ship-closeout.md`
containing:

1. Each of the 33 findings and its disposition — fixed (with commit), not-a-defect (with
   evidence), or deferred (with the reason and what would unblock it).
2. What the work found that the review did not predict. This is the most valuable section;
   the previous closeout's version of it was the best thing in the docs.
3. The new numbers: contrast pairs, idle CPU, test count, CI job list.
4. What is still true about the blind spot named at the end of the review, and what you
   put in place so it does not recur.

Report honestly. If something is still broken, say it is still broken.
