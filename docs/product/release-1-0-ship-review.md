# DeltaForge 1.0 ship review

Date: 2026-09-02
Reviewed at: `f3bcb85` on `main`
Method: live execution of the product (browser + CLI) plus source audit
Suite at review time: 194 `#[test]`, `cargo test` green

The question this answers: **would this ship as an Apple or Google 1.0?**

Answer: **not yet.** Four blockers, five review-gate failures, fifteen finish defects,
nine hygiene gaps. Every one is fixable in weeks, not quarters. A rendered version of
this review is published at
<https://claude.ai/code/artifact/fdcab90a-ca4a-439f-91c1-0c411b30bac0>.

This document supersedes nothing. `release-1-0-contract.md` still owns scope;
`release-1-0-closeout.md` still records what shipped. This records what a fresh,
adversarial pass found afterwards.

---

## What is already at the bar

Stated first, because it is why the remaining work is finite. Most projects that fail a
ship review fail on the substrate. This one does not.

- **The security model is considered.** Loopback bind, per-service capability token in a
  header, Origin and Host validated per request, cross-origin mutations refused, a
  bounded pre-auth connection permit, header and body caps, a read timeout, CSP,
  `nosniff`, `no-referrer`, `X-Frame-Options: DENY`, `no-store`. The one deliberate path
  exception for creation is documented as an architecture amendment rather than hidden.
- **The failure diagnosis is the best thing in the product.** *Fix this first*, one named
  contradiction, requirement / expected / received / test input, other failures behind a
  disclosure. Thirty-one wrong implementations across fourteen stages assert the exact
  primary diagnosis.
- **Concurrency and recovery are handled seriously.** One lease across checks, benchmarks
  and the watcher; an append-only journal with bounded compaction; a 30-minute idle
  shutdown gated on both connected clients *and* active runs; atomic state writes via
  temp-and-rename; a correct, specific forward-compatibility message for state written by
  a newer build.
- **The documentation tells the truth about itself.** The closeout records the design
  deliverable as written after implementation, the dogfood as contaminated, and content
  sufficiency as five stages of fourteen. That honesty is what made this review precise.
- **Cross-platform basics hold.** Projects in paths with spaces and non-ASCII characters
  create and run correctly (verified: `ws/My Projects (2026)/café/flashindex-rust`);
  extended-length Windows paths are redacted from diagnoses; CRLF fixtures are protected.

---

## P0 — ship blockers

### P0-1. Creating a project from the browser is dead on every clean machine

The first thing a new user does, and it does not work.

**Reproduced live, on the author's own machine, with default environment.**
`~/DeltaForge` does not exist there today:

```
Location   /Users/eloidieme/DeltaForge
           CANNOT CREATE HERE
           /Users/eloidieme/DeltaForge is not an existing directory
[ Create project ]   ← disabled
```

There is no affordance to create the folder and nothing on screen says a `mkdir` fixes it.

**Mechanism.** `creation::resolve_target` creates the default workspace only on the
`None` branch:

```rust
let parent = match parent {
    Some(parent) => parent.to_path_buf(),
    None => { fs::create_dir_all(&self.default_parent)?; self.default_parent.clone() }
};
if !parent.is_dir() { bail!("{} is not an existing directory", parent.display()); }
```

The page can never reach the `None` branch. `src/ui/app.js:281` prefills
`#create-parent` from `GET /api/v1/workspace`, and `app.js:322` (preflight) and
`app.js:360` (create) both send that field's value as `parent_directory`. So the
`Some(parent)` branch always runs, and on a fresh machine it always bails.

`deltaforge init` is unaffected because it never passes a parent — which is why the CLI
has always looked fine.

**Why three review passes missed it.** All three safety nets pre-create the exact
directory whose absence is the bug:

- `tests/browser_journey.rs` passes its own temp `parent_directory` at all five creation
  calls (lines 295, 316, 484, 525, 553).
- `tools/dogfood/activation.py:12` runs `(SCRATCH / "workspace").mkdir(parents=True)`
  before it starts measuring, then passes that path explicitly.
- The one test of `resolve_target(None, …)` is `src/creation.rs:652`, a unit test on a
  synthetic `CreationPolicy` — the branch the browser cannot reach.

**Fix.** Make the default workspace self-creating from the browser, and stop treating a
missing default as user error. When the Location field is untouched and equal to
`/api/v1/workspace`'s value, send `parent_directory: null`. Have the preflight report a
non-existent *permitted* parent as *will be created* rather than as a refusal. Keep the
refusal for a parent the user typed that does not exist. Then add the regression tests
that would have caught it: a browser-journey case posting with no parent, and one posting
the default workspace path before it exists.

---

### P0-2. Nothing in CI ever executes the page a learner uses

`src/ui/app.js` is 1,166 lines and is the entire product surface under contract decision
4. It has zero tests. `tests/browser_journey.rs` is good work, but its own header states
what it is:

> Every request below is the exact HTTP exchange the workbench page makes.

That is a hand-maintained mirror, and a mirror cannot detect that the page and the mirror
have diverged. P0-1 is exactly that failure mode: the mirror sends one request shape, the
page sends another, the page's shape is broken, and the suite is green.

**Fix.** Add a headless-browser journey as a CI job on Linux, driving Chromium (CDP or
Playwright). It must run the real page through catalog → create with defaults → first
failing run → reveal a hint → pass → snapshot → benchmark with a prediction → export.
Keep `browser_journey.rs` as the fast HTTP contract test; the new job is what proves the
surface. Additionally add a Node-run unit suite over `app.js`'s pure functions (route
table, request-body construction, section rendering) — that alone would have caught P0-1
and P0-3.

---

### P0-3. The section renderer corrupts learner-facing content

The 1.0 work fixed sections rendering *empty*. It did not fix them rendering *wrong*.

`src/capability.rs:363 parse_blocks` understands paragraphs, lists and fenced code, and
nothing else. `src/capability.rs:587 strip_inline_markdown` is a blanket
`text.replace(['`', '*'], "")`.

**Content corruption.** ByteForgeVM stage 4 teaches multiplication. Verified with
`deltaforge pack content byteforgevm --stage 02_more_arithmetic`, line 32:

```
pack says     - `MUL`: pop right, pop left, then push `left * right`.
learner sees  - MUL: pop right, pop left, then push left right.
```

The operator is deleted from the stage's core instruction. The `SUB` line above it
survives only because subtraction's operator is a hyphen.

**Leaked markup.** 22 sub-headings across 21 stage files in all four packs render as
literal text — `### Reading the benchmark`, `### Reflection`,
`### Benchmark interpretation worksheet`. Visible in the workbench today on FlashIndex
step 1 under *Done when*. Distribution: FlashIndex 3 files, MiniKV 2, TinyHTTP 5,
ByteForgeVM 11 (every stage). Because the heading collapses into a paragraph, the
reflection questions beneath also lose their hierarchy and read as further completion
criteria.

**Three parsers disagree.** There is no inline-code rendering anywhere in the workbench:
stage sections strip backticks entirely, prediction prompts render them literally
(visible on the Performance page as ``is `scan` limited by…``), hints render them
literally too. In a programming curriculum, code that does not look like code is a
content defect.

**Fix.**

- Add `Heading` and `InlineCode` to `OverviewBlock`; render `<h3>` and `<code>`.
- Delete the blanket `replace`. Parse inline spans properly, or escape what cannot be
  represented — never silently drop characters from a specification.
- Route prediction prompts and hints through the same renderer so all three agree.
- Add a `validate-pack --strict` rule that fails on any markdown construct the renderer
  cannot represent, so this class cannot ship again.
- Strengthen `every_shipped_stage_fills_every_panel` from *non-empty* to *round-trips
  faithfully*: assert no rendered block contains a residual `#`, backtick or unmatched
  `*`.

---

### P0-4. Distribution has never been executed, and its first execution is irreversible

No tags, no releases, no published crate. The README's primary install instruction —
download an archive from Releases and verify its checksum — points at an empty page.

- **Ordering.** In `.github/workflows/release.yml`, `publish-crate` declares no `needs:`.
  A tag push publishes to crates.io in parallel with the binary matrix. crates.io
  publishes cannot be deleted, only yanked — so a build failure on any of the four
  targets leaves a permanent version with no binaries behind it.
- **macOS.** No codesigning, no notarization. A user who follows the README gets
  Gatekeeper's *"cannot be opened because the developer cannot be verified"*. On a
  product whose thesis is *everything runs on your machine*, an OS security refusal as
  the first interaction is not survivable.
- **Windows.** No signing, so SmartScreen warns.
- No `aarch64-unknown-linux-gnu` target.
- No build provenance or attestation on artifacts users are told to checksum.
- `rust-version = "1.85"` is a promise CI never checks; no MSRV job.
- No `cargo audit` / `cargo deny`, no Dependabot.
- The release body is generic; it does not carry the version's changelog section.

The crate itself packages correctly — 796 files, 468 KiB compressed, packs included, and
`cargo package` (with verification) succeeds. That part is sound.

**Fix.** Gate `publish-crate` on `needs: [build]`. Add `cargo publish --dry-run`, an MSRV
job pinned to 1.85, `cargo deny`, ARM Linux, and build attestations. Notarize the macOS
archives, or — if that is not worth an Apple Developer account — state the Gatekeeper
step in the README with the exact command rather than letting users discover it. Then cut
`v1.0.0-rc.1` and install from it on all three platforms before `v1.0.0` exists.

---

## P1 — what design, accessibility and energy review would return

### P1-1. The Build layout collapses badly below 1120px, hiding the primary action

`src/ui/app.css:677` sets `.build-rail { grid-column: 1 / -1; position: static; }` at
`max-width: 1120px`. The rail is first in source order, so all fourteen steps become a
full-width block *above* the content, pushing the step title, instructions and
*Run checks* below the fold.

That is the default for any window narrower than 1120px — a 13-inch laptop with the
browser not maximized, or a maximized browser at 125% zoom.

**Fix.** At the collapse, either reorder the rail after the instructions in the grid, or
replace it with a compact step selector (current step, count, disclosure for the full
list). Rule: the primary action never moves below the fold because the window narrowed.

### P1-2. The secondary text token and every control border fail WCAG AA

The palette is otherwise well built — every semantic colour passes on every ground it is
used on. Two tokens do not, and they are used everywhere.

```
--text-3       #71777f  light   on --bg         4.14   needs 4.5
                               on --surface-2   3.92   needs 4.5
--text-3       #7b838e  dark    on --bg         4.18   needs 4.5
                               on --surface-2   4.22   needs 4.5

--line-strong  #c2c2bb  light   on --surface    1.79   needs 3.0  (WCAG 1.4.11)
--line-strong  #3a414b  dark    on --surface    1.71   needs 3.0  (WCAG 1.4.11)
```

`--text-3` carries every eyebrow, every fact label on the evidence panel (*Requirement*,
*Expected*, *Received*, *Test input*), every rail summary, every timestamp and every field
hint. `--line-strong` (`app.css:231`, `app.css:259`) is the border of every secondary
button and every text input — exactly what 1.4.11 governs.

**Fix.** Darken light `--text-3` to about `#656b73`, lighten dark to about `#98a1ad`;
strengthen `--line-strong` on both grounds until control borders clear 3:1. Add a
contrast check to CI — all values live in one file.

### P1-3. The single-page app never tells assistive technology that the page changed

Fundamentals are good: `lang="en"`, landmarks, skip link, visible focus ring, no positive
tabindex, real `<details>`, the step rail as an `<ol>` with descriptive labels. What is
missing is everything specific to a client-routed app.

- `document.title` is `"DeltaForge"` on all seven routes and never changes.
- Focus is not moved on navigation. Clicking *Performance* leaves focus on `<body>`.
- Zero `aria-current` in the document; the active nav item is conveyed by an underline
  only.
- On the creation form the refusal has no `aria-invalid` / `aria-describedby` on the
  Location field, sits in a side panel outside any live region, and the submit button is
  `disabled` — so it leaves the tab order and a keyboard user has no path to discover why
  they are blocked.
- `aria-live="polite"` is set on the theme toggle *button*. A control is not a live
  region.
- The rail's label reads `"Step 1, Scan files, current, current step"` — "current" twice.

**Fix.** On every route change set `document.title` to `"{Step} — {Project} —
DeltaForge"`, move focus to the new `<h1>` with `tabindex="-1"`, set `aria-current="page"`
on the active nav item. On the form, swap `disabled` for `aria-disabled`, wire
`aria-invalid` and `aria-describedby` to an inline message under Location, and make that
message the live region. Remove `aria-live` from the theme button.

### P1-4. A background service that burns 7% of a core while the learner reads

Measured on a **release** build over 30-second windows, two registered projects, nothing
running:

```
release, browser tab open, 500 ms poll   7.0% of one core, sustained
release, no client,          2 s poll    2.0% of one core, sustained
```

The 500 ms figure is the state a learner is in for most of a session — reading
instructions, writing code in another window. `spawn_source_watcher`
(`src/workbench.rs:684`) walks *every registered project's* tree on every tick, not the
open one, so this grows linearly with the number of projects ever created.

Sustained single-digit CPU from an idle background daemon is what surfaces in macOS
energy reporting. The idle back-off to 2 s already present shows the mechanism is
understood — it is applied to the wrong condition.

**Fix.** Watch only the project a connected client is viewing. Gate the tree walk on a
cheap directory-mtime check before hashing, and back off adaptively (500 ms after a
change, stretching toward 5 s while nothing moves). Reconsider the deliberate decision to
leave `verify_pack_pin`'s 541-file stat walk uncached on every tick — cache it keyed on
the pack directory's mtime. Add an idle-CPU assertion to CI.

### P1-5. One panic anywhere permanently bricks the workbench

Thread-per-connection with shared state behind mutexes. 15 instances of
`.expect("workbench lock poisoned")` in `src/workbench.rs`, no `catch_unwind` anywhere,
no poison recovery in `src/` at all.

If any handler panics while holding one of those locks, the mutex is poisoned for the
life of the process and *every subsequent request panics*. The browser hangs with no
error surface. There is no `panic::set_hook`, so the only trace goes to a terminal the
user has probably closed — this runs in the background.

The correct pattern already exists in the codebase: `unwrap_or_else(|poisoned|
poisoned.into_inner())` in `tests/browser_journey.rs`'s helper. It was never applied to
the server.

Same shape on the page: no `window.onerror`, no `unhandledrejection`, and the
`app-events` `EventSource` (`app.js:906`) has no `onerror` at all. The run stream's
handler only sets `"Reconnecting…"` (`app.js:902`) — when the 30-minute idle shutdown
fires, that message persists forever with no explanation and no restart instruction. The
idle timeout itself (`IDLE_TIMEOUT`, `workbench.rs:33`) is documented nowhere.

**Fix.** Recover from poisoning everywhere in `src/`; wrap each connection handler in
`catch_unwind` so a panic returns a 500 instead of poisoning shared state; install a
`panic::set_hook` writing to a log file under the DeltaForge home. On the page, add global
error handlers and a real disconnected state after N failed reconnects. Document the idle
timeout.

---

## P2 — consistency and finish

Several are the defect shapes the third review already named: *a rule written twice*, and
*a strict path validating what the runtime path does not*.

**P2-1. The application's own strings tell browser users to run terminal commands.**
The closeout swept 39 of these out of pack content; the code was never swept. Six sites,
all reachable from the browser via project health: `src/context.rs:218`, `:223`, `:229`,
`:376`, `:396`, `:406`, plus `src/application.rs:2193` and `src/state.rs:595`. Verified
live: the health screen reads *"Run `deltaforge sync-pack` to re-pin…"* directly beside a
button labelled *Use updated project definition* that does exactly that.
*Fix:* give every user-facing error two forms, or write them surface-neutral and let the
UI supply the action.

**P2-2. Prose rendered in a code block, truncated mid-sentence.** That health screen puts
an 872px sentence into a 758px box with `white-space: pre; overflow-x: auto`. macOS hides
overflow scrollbars, so the sentence stops mid-clause with no indication more exists. Its
backticks render as literal characters.

**P2-3. The header reads "Loading project…" permanently on the health route.** The
project name never resolves on the one screen reached when something has gone wrong.

**P2-4. The second launch silently stops printing the URL.**
`DELTAFORGE_NO_BROWSER=1 deltaforge` prints `"DeltaForge is ready at http://…"` the first
time and just `"DeltaForge is ready."` when a service already exists. The documented
headless path stops working with no way to recover the URL short of reading
`~/.deltaforge/workbench.json`. *Fix:* always print the URL, started or found.

**P2-5. A damaged state file has no recovery path and a developer-grade error.** A
truncated `state.json` prints serde's raw message including all 24 internal field names.
No backup, no repair command. A valid-JSON-but-foreign file (`{"hello":"world"}`) is
misdiagnosed as *"from an older DeltaForge"*. Writes are atomic so this is rare, not
never. *Fix:* catch the parse error, say the saved progress is damaged, keep a
`state.json.prev` on each write, add `deltaforge doctor --repair`.

**P2-6. There is no state migration story at all.** `check_schema_version`
(`src/state.rs:588`) rejects every older schema with *"recreate the project with
`deltaforge init`"*. Every future schema bump destroys all learner progress. This already
happened once (1 → 2) and the contract's answer was "accept the state break" — defensible
before there were users, not after. *Fix:* write the migration ladder now, while there is
exactly one version to migrate from.

**P2-7. `deltaforge init` does not register the project, and its doc comment says it
does.** A CLI-created project is invisible in the browser's Projects list until the user
runs `deltaforge` from inside it. `src/commands/init.rs`'s header claims it is *"driven
through `crate::application::create_project`, so a project scripted into existence is
identical to one created from the catalog"* — it calls `creation::create` directly, and
the two differ in registration, path policy and preflight.

**P2-8. The test suite leaks a workbench service.** After a full `cargo test`, a
`deltaforge` process was still listening on `127.0.0.1:57697` more than eleven minutes
later. Not the registered service, so `deltaforge exit` cannot stop it. *Fix:* short idle
timeout on test-spawned services plus a `Drop` guard that kills the child on panic too.

**P2-9. The CLI's front door contradicts decision 6.** `deltaforge --help` lists 20
commands flat, with maintainer tooling (`pack`, `validate-pack`, `sync-pack`) ranked
alongside `test`, and `portfolio` / `design` appearing nowhere in the README. *Fix:* group
the help (Learn / Automate / Author packs) and lead with "run `deltaforge` to open the
workbench".

**P2-10. Three different product descriptions, one naming a competitor.** `Cargo.toml` —
the string shipped to crates.io — reads *"A local CodeCrafters-style learning framework
for staged systems-programming projects."* `--help` says *"Local staged project learning
framework."* The README opens with *"Build a real developer tool on your own machine, one
behavior at a time."* The third is the good one.

**P2-11. The catalog's order is arbitrary and "Preview" is never explained.** Cards render
FlashIndex, ByteForgeVM, MiniKV, TinyHTTP — not alphabetical, not by difficulty, not by
step count. The *Preview* badge sits at the same visual weight as *Flagship* with no
explanation on the surface where the decision is made.

**P2-12. The dark palette is duplicated verbatim in two CSS blocks.** 24 tokens appear
identically under `@media (prefers-color-scheme: dark)` (`app.css:63`) and under
`:root[data-theme="dark"]` (`app.css:93`). A rule written twice, waiting to drift.

**P2-13. The creation screen flashes a label with no value.** During the ~130 ms preflight
the Environment panel renders its heading and an orphaned *"WILL BE CREATED AT"* label
above empty space. There is no skeleton or loading state anywhere in the app.

**P2-14. The measured-step diamond wraps onto its own line** on steps 6 and 12, whose
titles are longer. The step rail is the signature element of the design; its marker
should not orphan.

**P2-15. Two different platform conventions for user data, in one product.** The pack
cache correctly uses `%LOCALAPPDATA%` / `$XDG_CACHE_HOME` (`src/pack.rs:560`). The project
registry and capability token use `~/.deltaforge` on all three platforms
(`src/project_registry.rs:44`). `HOME` is checked before `USERPROFILE`, so on Windows the
same user gets a different home from Git Bash than from PowerShell. The `0700` hardening
on the directory holding the capability token is `#[cfg(unix)]` only — on Windows it gets
no ACL restriction.

---

## P3 — public-project hygiene

**P3-1.** No `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, issue templates or PR template.
`SECURITY.md` is present and good, which makes the gap conspicuous.

**P3-2.** Internal working documents ship publicly: `handoffs/` (eleven session-handoff
prompts, its own README calls them historical) and `Spec.md`. `docs/product/` is
different — that is a genuine engineering record and a credit to the project. Move
`handoffs/` and `Spec.md` out, or into a clearly-marked `docs/history/`.

**P3-3.** Two `wip` commits in the published history: `3bcf74b`, `5f8cd2b`. Everything
else in 90 commits has a real imperative subject and explanatory body.

**P3-4.** Thirteen README links break on crates.io and docs.rs — `docs/` is in the
`exclude` list. Use absolute GitHub URLs in the README, or ship `docs/`.

**P3-5.** The CSP is weakened by `unsafe-inline` for both scripts and styles
(`src/workbench.rs:2340`). Both assets are embedded at compile time, so their SHA-256
hashes are computable — replace `'unsafe-inline'` with hashes.

**P3-6.** No `set_write_timeout` on the main request path. It is set on the identity probe
but not on `handle_connection`. A client that stops reading holds a thread indefinitely;
the pre-auth permit bounds unauthenticated connections, an authenticated one is unbounded.

**P3-7.** Content sufficiency stands at five stages of fourteen against a contract gate of
fourteen. The practice already found six specification holes in the five it covered.

**P3-8.** The cold dogfood has never been performed by someone who is not the author. The
record says so itself. One session with one unfamiliar programmer would have found P0-1 in
ninety seconds.

**P3-9.** Three of four catalog projects are second-tier. A legitimate, honestly-labelled
decision — but it means the catalog a new user sees is 75% preview, and P0-3's worst
content corruption is in ByteForgeVM. A defect found in the flagship must still be swept
across all four.

---

## The pattern

Every P0 exists because the software was only ever driven by the person who wrote it, on
the machine where it was written, through the paths that person already knew worked.

The creation bug survived three reviews because all three harnesses created the directory
first. The content corruption survived because the flagship stage happens not to use the
construct that breaks. The release workflow is unverified because no tag has ever been
pushed.

That is not a skill problem. It is the predictable blind spot of a solo project, and the
durable fix is to make something other than the author run the product, on every path, on
every build.
