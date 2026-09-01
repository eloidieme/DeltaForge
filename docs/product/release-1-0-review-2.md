# DeltaForge 1.0 — independent review (second pass)

Reviewed on 2026-09-01 against the working tree at `cf930c7` (plus the uncommitted
modifications present in that tree). This review was performed from scratch: the code,
the tests, the documentation, and the running product were inspected directly, and no
prior review or handoff was consulted. Release, packaging, tagging, registry, and
deployment concerns are deliberately out of scope and are not assessed here.

Every finding below was reproduced on the machine under review (Windows 11, Rust 1.97.0)
unless it is filed under *Risks*, where the mechanism is established from the code but the
symptom was not directly observed.

---

## 1. Build and test health

### What was run

| Command | Result |
|---|---|
| `cargo build --all-targets` | clean, exit 0 |
| `cargo fmt --all -- --check` | clean, exit 0 |
| `cargo clippy --all-targets --all-features -- -D warnings` | clean, exit 0 |
| `cargo test --all-targets` | **152 passed, 0 failed**, ~3 min wall |
| `cargo test --doc` | 0 tests |
| `cargo test --test browser_journey` with `GIT_CONFIG_GLOBAL=/dev/null` | 3 passed |

Breakdown: 84 unit tests in the library, 3 in `browser_journey`, 47 in `cli_flow`,
4 in `mcp_standard_client`, 4 in `phase1_failure_corpus`, 10 in `workbench_flow`.
No test was skipped or ignored. Two consecutive full runs produced identical results —
no flakiness was observed in this environment.

The suite is genuinely behavioral: `reference_solutions_pass_all_bundled_packs` and
`learner_can_pass_all_mvp_stages_and_unlock_progress` drive real builds and real
learner binaries, and `phase1_failure_corpus` asserts the *primary diagnosis* for every
FlashIndex stage. That is a strong foundation. The gaps are not in rigour but in
coverage of the specific paths this review found broken (§6).

CI (`.github/workflows/ci.yml`) runs fmt, `cargo check --release`, clippy with
`-D warnings`, `cargo test`, and `validate-pack --strict` on Linux, macOS, and Windows.
That is the right matrix.

---

## 2. Confirmed defects

### D1 — Blocker/High · The workbench burns 20–45 % of a CPU core while completely idle

**Severity: High**

**Files:** `src/workbench.rs:573-593`, `src/application.rs:795-820`,
`src/context.rs:100-128`, `src/integrity.rs:91-160`

`spawn_source_watcher` loops every `EVENT_POLL_INTERVAL` (500 ms,
`src/workbench.rs:24`) and calls `application::observe_source_changes` for **every**
registered project. That function unconditionally:

1. calls `ProjectContext::load`, which calls `verify_pack_pin` →
   `digest_pack_tree(pack.root)` — a full read and hash of the entire pack tree; then
2. calls `initial.project_digest()` — a full read and hash of the entire learner project
   tree.

`integrity::collect_tree` accumulates a `Vec<TreeEntry>` that holds **the complete
contents of every file** before hashing (`src/integrity.rs:99`, `143-147`). There is no
mtime/size pre-check, no streaming, and no size cap.

**Reproduction** (release binary, service idle, no browser attached, no run in flight;
CPU measured with `Get-Process().TotalProcessorTime`):

| Registered projects | CPU over the sample | Peak working set |
|---|---|---|
| 0 | **0.00 s / 10 s** | — |
| 2 small projects (3.7 MB pack tree) | **2.09 s / 10 s** (≈21 % of a core) | 10.8 MB |
| same + a 129 MB corpus in one project | **6.38 s / 15 s** (≈43 % of a core) | **141.7 MB** |

The zero-project control isolates the cause precisely: this is entirely the source
watcher.

**Impact.** The service idles out only after 30 minutes (`IDLE_TIMEOUT`,
`src/workbench.rs:25`), and `last_activity` is refreshed whenever the watcher observes a
change — so a learner who leaves the workbench open pays a fifth to nearly half a core,
continuously, plus roughly 250 MB/s of sustained disk reads on the 129 MB case. On a
laptop this is a fan-and-battery problem. It scales linearly with the number of
registered projects and with project size. FlashIndex is a *source-code search engine*;
a learner keeping a test corpus inside the repository is the expected case, not an
exotic one.

**Recommended fix.** In order of value:
1. Pre-check with `stat` only — walk the tree collecting `(path, len, mtime)` and hash
   that; read contents only when the cheap fingerprint changed. This alone removes
   almost all of the cost.
2. Stream each file into the hash (`update_hash` in a read loop) instead of buffering
   every file's contents in a `Vec<TreeEntry>` — this removes the memory spike
   regardless.
3. Cache the pack digest per `(pack_root, pack mtime)` instead of recomputing it on every
   `ProjectContext::load`.
4. Back the poll interval off (e.g. 500 ms → 5 s) when `shared.clients == 0`.

---

### D2 — High · Every run event rewrites the whole event journal, making `deltaforge test` several times slower than the build it wraps

**Severity: High**

**Files:** `src/run_journal.rs:33-52`, `src/application.rs:1909-1918`

`run_journal::append` reads the entire journal, deserialises it, appends one event,
serialises the whole thing **twice** (once at line 46 for the size check inside a `while`
loop, once at line 49 to write), then does an atomic write with `sync_all` on both the
file and its parent directory. Every `BuildOutput` chunk goes through this.

**Reproduction** (release binary, MiniKV project, identical rebuild each time):

| Build stderr | plain `cargo build --release` | `deltaforge test` | journal appends | journal size |
|---|---|---|---|---|
| ~1 KB | — | **1.18 s** | ~10 | 4 KB |
| 837 KB | 0.73 s | **3.13 s** | 285 | 936 KB |
| 2.55 MB | 1.39 s | **8.94 s** | 703 | 1.12 MB |

Reproduce by writing a `src/main.rs` containing 4 000 (then 12 000) unused `let`
bindings, `touch`ing it, and timing `cargo build --release` against `deltaforge test`.

At 703 appends the journal has stabilised near its 256-event cap (~1.1 MB), so the run
rewrites and fsyncs roughly 700 MB — about **11 ms of pure journal overhead per event**.

**Impact.** The moment DeltaForge is slowest is precisely the moment a learner is stuck:
a Rust project full of compile errors emits a large `stderr`, and DeltaForge turns a
1.4-second build into a 9-second wait. This affects the terminal and the browser equally,
since both go through `JournalSink`.

**Recommended fix.** Make the journal append-only: one JSON object per line, `O_APPEND`
write, no read-modify-write; compact on a size trigger under the existing lock rather
than on every append. Failing that: keep the journal in memory in the service and
persist on a timer; coalesce `BuildOutput` chunks (they are already 4 KiB reads — batch
them to one event per ~100 ms); and drop the `to_vec` inside the `while` condition in
favour of a running byte counter.

---

### D3 — High · `deltaforge report` and `deltaforge portfolio` invalidate the completion proof and block progression

**Severity: High**

**Files:** `src/context.rs:110-116`, `src/cli.rs:255`, `src/cli.rs:262`,
`src/reporting.rs:36-38`; docs at `docs/commands.md:39-40`, `docs/quickstart.md:75`

`ProjectContext::project_digest` excludes exactly three generated names:

```rust
"deltaforge-report.md",
"deltaforge-report.html",
"deltaforge-report.json",
```

Those are the names the **browser** export writes (`ReportFormat::export_file_name`).
The **CLI** defaults are different: `--output` defaults to `report.md`
(`src/cli.rs:255`) and `PORTFOLIO.md` (`src/cli.rs:262`). Neither is excluded, so both
change the project digest and stale every completion proof.

**Reproduction:**

```bash
deltaforge test          # stage passes
deltaforge report        # writes ./report.md
deltaforge next
# error: learner project changed since stage 02_filter_files passed; run `deltaforge test` again
```

The browser path is unaffected — `POST /api/v1/reports` writes `deltaforge-report.md`
and `deltaforge next` still works afterwards. Verified both ways.

**Impact.** A documented, promoted command silently breaks progression. It is worse than
a nuisance because `docs/commands.md:39` explicitly promises the opposite ("a name
excluded from the project digest so exporting cannot stale the record it describes") and
`docs/quickstart.md:75` teaches the exact command that breaks it
(`deltaforge report --format markdown --output report.md`). The recovery — re-running
the checks — works but must be repeated after every export.

**Recommended fix.** Route both CLI commands through `export_file_name()` so all four
surfaces write `deltaforge-report.*`, or exclude the resolved `--output` path from the
digest for that invocation. A `--output` pointing anywhere inside the project should be
excluded whatever its name; that is the general fix.

---

### D4 — High · The launcher trusts the discovery record's port without authenticating the listener: the capability token is disclosed and the learner's browser can be sent to an impostor

**Severity: High** (local attacker required)

**Files:** `src/workbench.rs:365-391` (`read_compatible_record`),
`src/workbench.rs:393-408` (`probe`), `src/workbench.rs:217-263` (`launch`)

`probe` sends the capability token, in the URL, to whatever process is listening on the
port named in `~/.deltaforge/workbench.json`:

```rust
let path = format!("/api/{API_VERSION}/health?token={}", record.token);
let body = http_get(record.port, &path)?;
```

Identity is then decided purely from the JSON body that listener chose to return —
`service == "deltaforge"`, plus a `version`, a `pid`, and a `clients` count. Nothing
proves the listener is DeltaForge's own service. Worse, `read_compatible_record`
*overwrites* the recorded PID with whatever the response claims
(`src/workbench.rs:386-389`), so the one field that could have been a check becomes a
record of the attacker's assertion.

**Reproduction 1 — token disclosure.** A trivial local TCP server was started on an
ephemeral port; `~/.deltaforge/workbench.json` was left naming that port (the state a
crashed or `kill -9`'d service leaves behind). Running `deltaforge exit` produced, in the
foreign process's log:

```
GET /api/v1/health?token=SUPER-SECRET-CAPABILITY-TOKEN-abc123 HTTP/1.1
Host: 127.0.0.1:55777
```

The record is only removed *after* the token has been transmitted.

**Reproduction 2 — impostor.** The same server was changed to answer
`/api/v1/health` with `{"service":"deltaforge","api":"v1","version":"1.0.0-app3","pid":4242,"clients":0}`
and to serve `<h1>impostor</h1>` for everything else. Running bare `deltaforge`:

```
DeltaForge is ready at http://127.0.0.1:50441/projects/fi-.../overview?token=SUPER-SECRET-...
```

and the discovery record was rewritten as
`{"port":50441,"pid":4242,"token":"SUPER-SECRET-CAPABILITY-TOKEN-abc123",...}`.
With `DELTAFORGE_NO_BROWSER` unset — the normal path — DeltaForge would have **opened the
learner's browser at the attacker's page**, branded as DeltaForge, carrying the token in
the URL.

**Preconditions and honest scoping.** The attacker needs (a) a discovery record naming a
port DeltaForge's own service no longer holds — the state left by any unclean shutdown,
which the codebase itself anticipates (`lifecycle_recovers_stale_metadata_...`) — and
(b) a local process able to bind that port. On Unix the record is `0600`, so the port
number is not readable by another user and must be guessed or sprayed; on Windows only
the profile ACL applies. This is not a remote issue: the socket is loopback-only.

**Impact.** Capability-token disclosure to an arbitrary local process, and — more
seriously — a supported path for pointing the learner's browser at attacker-controlled
content that presents itself as their DeltaForge workbench. `SECURITY.md:11` names the
capability token and the Host/Origin checks as the areas of concern; this defeats them
from the client side.

**Recommended fix.** Authenticate the service *before* trusting it and before sending the
token. Cheapest correct version: store a second value `probe_id` in the record; expose it
from an unauthenticated `/api/v1/identity`; only send the capability token once the
listener has echoed the right `probe_id`. Stronger: challenge-response — the client sends
a nonce, the service returns `HMAC(token, nonce)`, the client verifies before ever
transmitting the token and refuses to open a browser at that port otherwise. In both
cases stop overwriting `record.pid` from the response; verify it instead, or drop the
field.

---

### D5 — Medium · A failed `git commit` leaves the learner's index staged

**Severity: Medium**

**Files:** `src/snapshot.rs:133-160` (`take`), specifically `add -A` at line 136 before
`commit` at line 143

`snapshot::take` runs `git add -A`, then `git rm --cached … run.lock`, then
`git commit`. If the commit fails, nothing is unwound.

**Reproduction:**

```bash
# a pre-commit hook that exits 1
git config core.hooksPath /path/with/failing/pre-commit
echo "// edit" >> src/main.rs
git status --porcelain      #  M src/main.rs   (unstaged)
deltaforge commit --force
# error: git commit -m Complete Stage 02: ... failed: policy: commits blocked
git status --porcelain      # M  src/main.rs   (now staged)
```

**Impact.** DeltaForge mutates the learner's Git index as a side effect of an operation
that failed, destroying any staged/unstaged split they had arranged. Commit failure is
not exotic: a missing `user.name`/`user.email` (common on Linux and in containers, where
Git refuses rather than auto-deriving), a `pre-commit` hook, a GPG signing failure, or a
full disk all reach this path. Note also that even on **success**, `git add -A` silently
overrides a deliberate partial staging without saying so.

**Recommended fix.** Capture `git rev-parse HEAD` and the index state first and restore
on failure (`git reset` back to the prior index), or — simpler and safer — build the
commit without touching the working index using `git stash create`-style plumbing, or
`git commit --only <paths>`. At minimum, tell the learner in the error message that the
index was left staged and how to undo it.

---

### D6 — Medium · Snapshotting a clean tree fails with an empty error message

**Severity: Medium**

**Files:** `src/snapshot.rs:133-160`, `src/snapshot.rs:166-180` (`git_output`),
`src/application.rs:1196-1232` (`create_stage_snapshot`), `src/workbench.rs:973-1005`

`git_output` reports only `output.stderr`:

```rust
bail!("git {} failed: {}", args.join(" "), String::from_utf8_lossy(&output.stderr).trim());
```

Git writes `nothing to commit, working tree clean` to **stdout**, and exits 1. There is
also no "nothing to commit" guard in `take`, even though `preview_stage_snapshot`
computes exactly that condition.

**Reproduction** (hermetic, hooks disabled):

```bash
git status --porcelain      # (clean)
deltaforge commit
# error: git commit -m Complete Stage 04: Find an exact token failed:
# EXIT=1
```

The reason is blank. The browser is worse, because it contradicts itself in the same
session:

```
GET  /api/v1/snapshots/preview → {"available":false,
   "blocked_reason":"Every change is already recorded; there is nothing new to snapshot."}
POST /api/v1/snapshots         → 409 {"error":"git commit -m Complete Stage 04 … failed: "}
```

With any global Git hook installed, the blank reason is replaced by an unrelated hook
warning line, so the learner is told the wrong cause entirely (observed:
``` `.pre-commit-config.yaml` config file not found. ```).

**Impact.** A learner who commits by hand and then asks DeltaForge for the step snapshot
gets an unexplained failure — and no tag, because tagging happens only after the commit
succeeds. `POST /api/v1/snapshots` ignores its own preview logic.

**Recommended fix.** Two independent changes: (1) have `create_stage_snapshot` consult
the same "nothing to commit" condition the preview uses, and in that case create the tag
on `HEAD` and report `existing_tag`/`changed_files: 0` honestly instead of failing;
(2) include stdout in `git_output`'s error text, and fall back to
`"git exited with status N"` when both streams are empty.

---

### D7 — Medium · With `git.auto_commit = true`, a fully passing `deltaforge test` exits 1

**Severity: Medium**

**Files:** `src/commands/test.rs:94-99`, `src/commands/commit.rs:17-19`

After a newly completed stage, `test` calls `commit::run_automatic`, whose error
propagates out of the command.

**Reproduction:**

```
PASS  missing token argument is an error
PASS  unreadable root is an error

8 passed, 0 failed
error: git commit -m Complete Stage 04: Find an exact token failed: policy: commits blocked
EXIT=1
```

The stage *was* recorded as complete; only the snapshot failed.

**Impact.** The exit status contradicts the printed result. Any script, CI job, or
learner shell prompt that keys off `deltaforge test`'s exit code sees a green run as a
failure. Combined with D5, the same invocation also leaves the index staged.

**Recommended fix.** Treat an automatic snapshot as best-effort: warn on stderr, keep
exit 0 when the tests passed. Reserve non-zero for test failure and for execution errors.

---

### D8 — Medium · The help ladder promises a retrospective that does not exist, on three of the four bundled packs

**Severity: Medium**

**Files:** `src/application.rs:843-856`; docs at `docs/pack-format.md:56`,
`docs/quickstart.md:30`

`reveal_next_hint` gates on a hardcoded level number, not on "the last level":

```rust
let maximum = if context.state.is_completed(&stage_id) { help.len() } else { help.len().min(4) };
…
if current >= maximum {
    if context.state.is_completed(&stage_id) { bail!("all help levels are already revealed"); }
    bail!("the retrospective unlocks after this capability is acquired");
}
```

MiniKV, TinyHTTP, and ByteForgeVM ship **three** hints per stage, so `maximum == 3` and
nothing is actually gated — yet the refusal still claims a retrospective is waiting.

**Reproduction** (MiniKV, stage 01, not yet passed):

```
Hint 1/3:
Hint 2/3:
Hint 3/3:
error: the retrospective unlocks after this capability is acquired
```

There is no level 4. The learner is told to keep working to unlock content that will
never appear.

**Impact.** Two problems at once. The message is false on 3 of 4 packs, and the
documentation is false in the other direction: `docs/pack-format.md:56` and
`docs/quickstart.md:30` both state that "the last level unlocks only after the step
passes", which is true only for a pack with five or more hints. A pack with six hints
would have levels 5 *and* 6 gated, not just the last.

**Recommended fix.** Either express the rule as "the final level is the retrospective"
(`maximum = help.len().saturating_sub(1)` when incomplete, with a floor so a 1-hint pack
still shows something), or keep the level-4 rule and correct both documents plus the
refusal message — emitting "all help levels are already revealed" whenever
`help.len() <= 4`.

---

### D9 — Medium · Extended-length Windows paths (`\\?\C:\…`) are shown to learners across both surfaces

**Severity: Medium** (Windows only)

**Files:** `src/context.rs:329-362` (`locate_project_root` canonicalises),
`src/creation.rs:162-164`, `src/application.rs:589-601` and `1117-1120`,
`src/ui/app.js:323` and `:341` and `:1065`, `src/commands/design.rs`,
`src/commands/config.rs`

`sanitize_project_text` (`src/application.rs:2058-2086`) redacts these prefixes in
diagnoses, but nothing redacts them in the paths DeltaForge shows as *information*.

**Reproduction:**

```
$ deltaforge config show
Config: \\?\C:\Users\…\ws\fi\.deltaforge\config.toml

$ deltaforge design
Design notes: \\?\C:\Users\…\ws\fi\.deltaforge\design_notes\02_filter_files.md

POST /api/v1/projects/preflight →
  "location":{"target":"\\\\?\\C:\\Users\\…\\ws\\kv1", …}          → rendered as "Will be created at …"
POST /api/v1/projects →
  "path":"\\\\?\\C:\\Users\\…\\ws\\kv1"                            → rendered as "Created at …"
POST /api/v1/reports →
  "path":"\\\\?\\C:\\Users\\…\\deltaforge-report.md"               → rendered as "Written to …"
```

Every error message that names the project root shows it too (`not inside a DeltaForge
project: …`, `unsupported state schema_version 1 in \\?\C:\…`).

**Impact.** The very first path a Windows learner sees — the "Will be created at" line in
the creation preflight — is an unfamiliar spelling that does not paste cleanly into most
tools. `deltaforge init` gets this right ("Target directory: fi"); the browser and the
other CLI commands do not.

**Recommended fix.** Add a single `display_path()` helper that strips `\\?\` and
`\\?\UNC\` (the logic already exists in `project_root_spellings`,
`src/application.rs:2072-2086`) and use it at every point a path is rendered for a human
— CLI `println!`s, `LocationStatus.target`, `CreatedProject.path`, `ExportedReport.path`,
and `ProjectSummary.path`.

---

### D10 — Medium · The capability token is carried in the URL and written into browser history on every navigation

**Severity: Medium**

**Files:** `src/workbench.rs:1264-1272` (auth reads the token from the query string),
`src/workbench.rs:243-246` (launch URL), `src/ui/app.js:91`

```js
history.pushState({}, "", `${path}?token=${encodeURIComponent(token)}`);
```

The page deliberately re-appends the token to the URL on **every** in-app navigation, so
each visited step lands in the browser's history with the live capability token in it. The
token is also printed to stdout in full under `DELTAFORGE_NO_BROWSER=1`, where it enters
shell scrollback and history files.

`Referrer-Policy: no-referrer` is set on every response, and there are no external
subresources, so cross-origin referer leakage is correctly closed. What remains is
history persistence, address-bar visibility during screen shares, and readability by any
browser extension with tab access.

**Impact.** The token authorises a service that executes pack-defined build and run
commands. Keeping it in the most persistent, most widely-readable place in the browser
undercuts the care taken everywhere else in the boundary. `docs/safety.md:15-17`
describes the token's generation, comparison, and at-rest storage in detail but never
mentions that it travels in the URL — a reader would reasonably assume a header or
cookie.

**Recommended fix.** Have the page `history.replaceState` the token out of the URL on
first load and keep it in a module-scoped variable, sending it as a request header (the
`Origin` + `Host` checks already block cross-origin use, and a custom header is not
sendable cross-origin without preflight). Keep query-string acceptance only for the very
first navigation. Then document the mechanism in `docs/safety.md`.

---

### D11 — Medium · `explain-failure` presents diagnoses for code that no longer exists

**Severity: Medium**

**Files:** `src/commands/explain_failure.rs:8-33`; compare
`src/application.rs:1546-1549`

The workbench computes freshness:

```rust
Some(run) if run.project_digest == context.project_digest()? => ResultFreshness::Fresh,
```

`LastTestRunSummary` carries `project_digest` for exactly this purpose. The terminal
command never reads it.

**Reproduction:**

```bash
# stage 03, starter code in place
deltaforge test                      # 0 passed, 9 failed
cp reference_solution.rs src/main.rs # fix it, do not re-run
deltaforge explain-failure
# Stage 03_tokenize: Recognize tokens
# Last run: 0 passed, 9 failed at 2026-09-01T13:28:13Z
# Failed: tokenizes simple Rust identifiers
# Fix this first: tokenizes simple Rust identifiers
```

The command confidently describes a failure that the current source no longer produces,
with nothing to indicate the result is stale. `deltaforge status` does not flag it either.

**Impact.** This directly contradicts `docs/commands.md:3-5` ("every command below is the
same operation reached from the terminal instead"). A learner in the terminal can spend
time fixing a contradiction they have already fixed.

**Recommended fix.** Compare `run.project_digest` with `context.project_digest()?` in
`explain_failure::run` and prefix the output with a line such as *"This result is from
before your last edit — run `deltaforge test` again."* Add the same marker to
`deltaforge status`, and expose the flag in `--json`.

---

### D12 — Medium · Three of the four bundled packs have duplicate step numbers

**Severity: Medium**

**Files:** `packs/minikv/project.yaml`, `packs/tinyhttp/project.yaml`,
`packs/byteforgevm/project.yaml`; consumed by `src/snapshot.rs:69-71`,
`src/snapshot.rs:183-187`

FlashIndex was renumbered to a coherent `01`–`14`. The previews were not:

```
minikv      01, 02, 02, 03, 03, 04, 04, 05, 05, 06
tinyhttp    01, 01, 02, 02, 03, 03, 04, 05, 06, 06
byteforgevm 01, 01, 02, 02, 03, 03, 04, 04, 05, 05, 06
```

**Impact.** `snapshot_message` derives the number from the id prefix, so a learner
finishing MiniKV ends up with two commits titled `Complete Stage 02: …` for two different
steps, and two `Complete Stage 03: …`, and so on — an ambiguous Git history for the exact
artefact the product asks them to keep. `deltaforge status` renders the raw ids, so the
roadmap reads as though numbers were skipped or repeated. The workbench roadmap uses the
1-based index instead, so the two surfaces disagree about what step you are on.

**Recommended fix.** Renumber the three preview packs to consecutive prefixes the way
FlashIndex was. If renumbering is not acceptable, derive the display number from the
stage's position in the manifest rather than from its id prefix, in both
`snapshot_message` and the terminal roadmap, so the two surfaces at least agree.

---

### D13 — Medium · Test and benchmark scratch directories are created in the shared temp directory with default permissions

**Severity: Medium** (Unix multi-user hosts)

**Files:** `src/runner.rs:1261-1277`, `src/benchmarks.rs:1083-1097`

```rust
let path = std::env::temp_dir().join(name);   // /tmp/deltaforge-<pid>-<nanos>-<stage>-<test>
fs::create_dir_all(&path)?;
```

Two properties matter. `create_dir_all` **succeeds when the path already exists**,
including when it exists as a symlink to a directory, so a pre-created path is silently
adopted rather than refused. And the directory gets the process umask (typically `0755`),
so on a shared Unix host every local user can read the fixture copies placed there and
anything the learner's program writes into `{temp_dir}`.

`docs/safety.md:5` says only "Fixtures are copied to temporary directories before
execution" — it does not claim isolation, but neither does it warn.

**Impact.** On a single-user laptop, nil. On a shared Linux host, CI runner, or dev
container with more than one account: disclosure of fixture content and of whatever the
learner's program emits, plus a race window in which a watcher of `/tmp` can pre-create
the directory and substitute fixture content between the copy and the run.

**Recommended fix.** Use `std::fs::DirBuilder` with `.mode(0o700)` on Unix, use
`create_dir` (not `create_dir_all`) so an existing path is an error rather than a silent
adoption, and add 128 bits from `getrandom` to the directory name so it cannot be
predicted from the pid and the clock.

---

### D14 — Low · `explain-failure` prints the same test name twice

**Severity: Low**

**File:** `src/commands/explain_failure.rs:58-59`

```rust
println!("Failed: {}", failed_test.name);
println!("Fix this first: {}", failed_test.name);
```

Observed output:

```
Failed: scans files in a basic project
Fix this first: scans files in a basic project
```

**Impact.** The "fix this first" line is the product's signature move — naming the single
thing to work on — and it is spent restating the line above it instead of adding
information.

**Recommended fix.** Drop the `Failed:` line, or make `Fix this first:` carry the
diagnosis headline (which is printed two lines further down anyway).

---

### D15 — Low · Test output shows the same string both redacted and unredacted

**Severity: Low**

**Files:** `src/commands/test.rs:39-47`, `src/runner.rs:582-583`

`result.failures` are sanitised to `{fixture_path}`, but the `actual stdout:` block two
lines below prints the raw `result.stdout`. The sanitised `report_stdout` /
`report_stderr` fields exist and are unused by the CLI.

Observed:

```
FAIL  prints paths in stable lexical order
  expected stdout exactly "crates/core/src/index.rs\n…", got "FlashIndex starter: … \"{fixture_path}\"\n"
  actual stdout:
    FlashIndex starter: implement command ["scan", "C:\Users\ede15\AppData\Local\Temp\deltaforge-25292-…\fixture"]
```

**Impact.** Confusing (the learner sees two different renderings of one string) and it
leaks absolute paths into terminal scrollback.

**Recommended fix.** Print `report_stdout` under `actual stdout:` and keep the raw
streams for `--verbose`, or drop the redaction from `failures` so one convention holds.

---

### D16 — Low · State-schema errors give no direction

**Severity: Low**

**File:** `src/state.rs:296-315`

```
$ deltaforge status     # state written by an older DeltaForge
error: unsupported state schema_version 1 in …\state.json; expected 2

$ deltaforge status     # state written by a newer DeltaForge
error: unsupported state schema_version 3 in …\state.json; expected 2
```

Both cases produce the same message and neither says what to do, even though
`docs/safety.md:11` already knows the answer for v1 ("recreate them") and the answer for
v3 is "upgrade DeltaForge".

Separately, `#[serde(deny_unknown_fields)]` on `ProjectState` means a state file written
by a future version fails *before* the version check runs, producing a 24-field serde
dump:

```
error: failed to parse state file …: unknown field `brand_new_field`, expected one of
`schema_version`, `project`, `language`, `pack_version`, … at line 141 column 19
```

**Recommended fix.** Branch on `< expected` vs `> expected` and give the corresponding
sentence. Read `schema_version` out of the JSON first (a two-field probe struct) and
check it before the strict deserialisation, so a forward-version file reports "written by
a newer DeltaForge" rather than a field list.

---

### D17 — Low · Documentation gaps and one inaccuracy about the creation boundary

**Severity: Low**

- **`VISUAL` / `EDITOR` are undocumented.** They drive the workbench's *Open editor*
  button (`src/workbench.rs:1642-1645`) and `deltaforge design --edit`, and when neither
  is set the workbench falls back to a hardcoded editor list and can refuse with *"no
  supported graphical editor was found"*. `docs/config.md:45-52` lists six other
  environment variables but not these two.
- **`DELTAFORGE_WORKSPACE` widens a security boundary, and the docs say otherwise.**
  `CreationPolicy::from_environment` (`src/creation.rs:106-124`) adds `$DELTAFORGE_WORKSPACE`
  to `permitted_roots`, so a browser-supplied parent directory is accepted anywhere under
  it. `docs/safety.md:21` states without qualification that the parent "must already
  exist and canonicalize inside the learner's home directory", and the refusal text
  (`src/creation.rs:172-175`) says "DeltaForge only creates projects inside your home
  directory". Both are inaccurate whenever the variable is set. `docs/config.md:50`
  describes only the default-parent effect.
- **Pack search order.** `docs/config.md:42` says the builtin packs directory applies
  "when running from a source checkout". In fact `builtin_packs_dir()`
  (`src/pack.rs:497-499`) is `env!("CARGO_MANIFEST_DIR")/packs` — the build machine's
  absolute path, baked into every binary and probed *before* the embedded cache on any
  machine where that path happens to exist.

**Recommended fix.** Document `VISUAL`/`EDITOR`; amend `docs/safety.md:21`,
`docs/config.md:50`, and the refusal message to say "your home directory, or the
directory named by `DELTAFORGE_WORKSPACE`"; reword `docs/config.md:42` to describe what
the path actually is.

---

### D18 — Low · Small output-quality defects

**Severity: Low**

| Where | Symptom |
|---|---|
| `src/reporting.rs:418-421` | `- 1 step snapshots recorded in Git history.` — no singular form |
| `src/pack.rs` discovery warnings | mixed separators: `…/sandbox/badpacks\broken\project.yaml` |
| `src/context.rs:341-352` | anyhow context ordering puts the cause after the guidance: `not inside a DeltaForge project: searched upward from …\nRun \`deltaforge init …\`.: could not find .deltaforge/state.json` |
| `src/terminal.rs:193-212` | `wrap()` measures `String::len()` (bytes), so any non-ASCII content wraps early |
| `src/workbench.rs:2097` | `capability_token(_root: &Path)` ignores its only parameter |

---

## 3. Risks (mechanism established, symptom not observed here)

### R1 — Event loss in the SSE stream, currently masked by D2

**Files:** `src/run_journal.rs:42-48`, `src/workbench.rs:1893-1900`

`append` trims to the newest 256 events; `serve_events` polls every 500 ms and asks for
`entries_after(cursor)`. Any event appended and trimmed inside one poll interval is
delivered to nobody, and the loop does not notice the resulting `id` jump.

I tried to provoke this with a 2.55 MB build (703 events) and observed **705 ids
delivered with zero gaps** — because D2 makes each append slow enough (~11 ms) that the
producer cannot outrun a 500 ms poll. Fixing D2 removes that accidental protection.
Address both together: make `serve_events` detect `entry.id != cursor + 1` and emit an
explicit `event: gap` the page can render as *"earlier output was dropped"*.

### R2 — `browser_journey.rs` depends on an ambient Git identity

**Files:** `tests/browser_journey.rs:76-80` and `:429`, versus `tests/cli_flow.rs:1539`,
`:1543`, `:2068`, `:2072`

`cli_flow` configures a repo-local `user.name`/`user.email` before its snapshot tests.
`browser_journey` performs a real `POST /api/v1/snapshots` (a real `git commit`) and does
not. On Windows and on the current CI runners Git auto-derives an identity, so the test
passes; in a minimal container Git refuses with *"unable to auto-detect email address"*
and the test would fail for a reason unrelated to DeltaForge. Set the identity explicitly,
as `cli_flow` already does.

### R3 — Build children are orphaned when the service is killed

**Files:** `src/process.rs:407-434`

`terminate_process_tree` is called only on timeout or cancellation. On Unix the child is
put in its own process group (`process_group(0)`), so if the workbench service is killed
the in-flight `cargo build` keeps running with no parent. State recovery is correct (the
lease is released and the next run marks the job `interrupted` — verified), but the
orphan continues to consume CPU and to write into `target/` alongside whatever runs next.
Consider a Unix `prctl(PR_SET_PDEATHSIG)` / Windows job object so the child dies with the
service.

### R4 — Unbounded pre-authentication connection handling

**File:** `src/workbench.rs:561-569`

`handle_connection` spawns a thread per accepted connection before any token check, with
no cap. 600 concurrent stalled connections were absorbed with no measurable degradation
(health responses stayed instant), so this is not a practical problem today — but the
design has no bound and the 5-second read timeout is the only limit. A small
semaphore-bounded worker pool would close it.

### R5 — FNV-1a-64 is used for everything called an "integrity digest"

**File:** `src/integrity.rs:6-7, 47-58, 204-221`

Pack pinning, stage behavioral digests, and completion proofs all use a 64-bit
non-cryptographic hash. Accidental collision is negligible; deliberate collision is
trivial. `docs/commands.md:63` and `docs/pack-format.md:75` are already honest about this
("it detects change; it is not tamper-evidence"), so this is a naming risk rather than a
broken promise — but `src/integrity.rs`'s own module name and the error text "cannot
create an integrity digest" invite the stronger reading. Either rename to
`staleness`/`fingerprint` throughout, or move to BLAKE3/SHA-256, which would also be
faster than the byte-at-a-time FNV loop.

### R6 — Benchmark peak-memory fidelity

**File:** `src/process.rs:203-238, 274-357`

`peak_rss_bytes` is sampled from a 1 ms poll loop. On macOS the sample is *current*
resident size, not a high-water mark, so a short allocation spike between polls is
invisible; a process that exits before the first poll reports `None`. The workbench and
the CLI table present the number as "peak mem" without qualification. Either label it as
approximate in the UI, or use `getrusage(RUSAGE_CHILDREN)`'s `ru_maxrss` after `wait` on
Unix, which is an exact high-water mark.

### R7 — `is_bundled_pack_root` accepts a name prefix

**File:** `src/pack.rs:633-645`

Any path containing a component starting with `deltaforge-embedded-packs` is treated as
bundled, so an external `--packs-dir /tmp/deltaforge-embedded-packs-x` would be pinned as
`"bundled"`. The pack digest check still applies, so this is not a bypass on its own —
but the legacy-compatibility heuristic is broader than it needs to be. Narrow it to the
exact legacy directory shape.

### R8 — Unbounded growth of two files

`benchmarks::append_history` (`src/benchmarks.rs:1005-1014`) reads, extends, and rewrites
`.deltaforge/benchmark_history.json` with no cap. `run_journal::read_unlocked`
(`src/run_journal.rs:126-142`) renames a corrupt journal to
`workbench-events.corrupt-<nanos>.json` with no limit on how many accumulate. Neither is
urgent; both are easy caps.

### R9 — The pack MCP server accepts arbitrary filesystem paths

**File:** `src/bin/deltaforge-pack-mcp.rs:983-991`

`pack_dir` and friends come straight from tool arguments with no containment (relative
paths *inside* a pack are checked via `is_safe_relative_path`, but the pack root is not).
This is consistent with its framing as maintainer tooling driven by a local agent at the
same privilege level, and `README.md` presents it that way — but it is worth stating
explicitly in `docs/authoring-packs.md` so nobody exposes it to a less-trusted client.

### R10 — Pager EPIPE on early quit

**File:** `src/terminal.rs:151-183`

`page()` writes the whole document into the pager's stdin before waiting.
`deltaforge instructions --all` produces ~49 KB; quitting `less` before it drains the
pipe turns a successful command into `failed to write output to pager less`. Ignore a
`BrokenPipe` from `write_all` and treat it as a normal early exit.

---

## 4. Coverage gaps

These are places where the suite looks like it covers something it does not.

1. **`legacy_schema_v1_state_loads_but_requires_a_fresh_completion_proof`**
   (`tests/cli_flow.rs:2191`) never sets `schema_version` to 1. It removes optional
   fields from a `schema_version: 2` file. The behaviour asserted by
   `docs/safety.md:11` ("Projects on state schema 1 do not load in 1.0") is real
   (`src/state.rs:296-304`) but untested, and the test's name actively misleads.
2. **No test covers D3.** Nothing asserts that a generated report does not stale the
   completion proof, in either direction. A test that runs `report`, then `next`, would
   have caught it.
3. **No test exercises a failing `git commit`.** All snapshot tests are on the happy path,
   which is why D5, D6, and D7 all survive.
4. **No test exercises the clean-tree snapshot.** `preview_stage_snapshot` returns
   `blocked_reason` for it, but no test asserts what `create_stage_snapshot` does in the
   same state.
5. **No test asserts hint gating on a three-hint pack.**
   `capability.rs::every_shipped_stage_fills_every_panel` checks that preview packs have
   ≥3 hints, and `the_flagship_help_ladder_is_labelled_end_to_end` checks FlashIndex's
   five labels — but nothing exercises `reveal_next_hint` against a three-hint pack, which
   is where D8 lives.
6. **No test asserts service identity.** `workbench_flow` covers stale records and
   incompatible versions, but nothing asserts that DeltaForge refuses a listener that is
   not its own service (D4).
7. **No performance guard.** Nothing bounds `run_journal::append` cost or the source
   watcher's per-tick work, which is why D1 and D2 could regress this far unnoticed. A
   test asserting that a run emitting N events takes < k·N ms, and one asserting the
   watcher does no content reads when nothing changed, would pin both.
8. **No doctests.** `cargo test --doc` runs zero tests. Not a defect — the codebase uses
   module-level prose rather than executable examples — but it means the many
   explanatory doc comments are never checked against reality.
9. **Windows-path rendering is untested at the presentation layer.**
   `unc_and_plain_roots_are_still_redacted` (`src/application.rs:2236`) covers diagnosis
   sanitisation only; nothing asserts that a user-visible path is free of `\\?\` (D9).

---

## 5. Optional improvements

- **`deltaforge status` is silent about an interrupted run.** After a hard kill mid-run,
  `active_job` remains set and `status` shows nothing unusual; only the next `test` (or a
  workbench launch) recovers it. Surfacing "a previous run did not finish" in `status`
  would close the loop for terminal-only users.
- **`run_lease::active` briefly takes the exclusive lock** (`src/run_lease.rs:77-90`),
  which is why `run_tests` and learner actions need bounded waits (500 ms and 750 ms
  respectively, `src/application.rs:612` and `:898`). It works, and the comments explain
  why — but a shared/read lock, or a lease file whose *content* is the liveness signal,
  would remove the need for the waits entirely. Note the asymmetry: a learner action
  waits 750 ms while a test run waits only 500 ms, so a queued *Begin next step* can win
  a race against a queued check run.
- **`atomic_write_private` silently discards its flag on Windows**
  (`src/fs_util.rs:24-25`, `let _ = private;`). `docs/safety.md:17` is honest about this
  ("Windows inherits the user's profile ACL"), but a reader of the code sees a function
  named "private" that does nothing. Either apply an explicit DACL via
  `SetNamedSecurityInfo`, or rename the Windows path to make the no-op visible.
- **`ProjectSummary.path` is served but never rendered** (`src/workbench.rs:206`,
  no consumer in `src/ui/app.js`). Dropping it would shrink the browser's exposure to the
  learner's absolute paths for no loss.
- **`changed_files` classification order** (`src/snapshot.rs:108-116`) matches
  `(b'A', _)` before `(_, b'D')`, so a file added to the index and then deleted in the
  worktree is reported as `added`. Cosmetic, but the porcelain code is `AD`.
- **CSP could be tightened.** `src/workbench.rs:2063` sets `script-src 'unsafe-inline'`
  with no `base-uri` or `form-action`. There is no HTML-injection sink in the page
  (`app.js` builds every node with `textContent`; no `innerHTML`, `eval`, or
  `document.write` anywhere), so the practical risk is nil — but adding
  `base-uri 'none'; form-action 'none'; frame-ancestors 'none'` costs nothing.

---

## 6. Prioritised remediation list

**Tier 1 — fix first**

1. **D1** — stop the source watcher reading every byte of every project twice a second
   (`stat` fingerprint first; stream file contents into the hash; cache the pack digest;
   back off when no client is connected).
2. **D2** — make the event journal append-only, and coalesce `BuildOutput` chunks.
   Do R1 in the same change, since fixing D2 exposes it.
3. **D4** — authenticate the workbench service before sending it the capability token or
   opening a browser at its port; stop rewriting `record.pid` from the response.
4. **D3** — route the CLI `report`/`portfolio` defaults through `export_file_name()`, or
   exclude any `--output` inside the project from the digest. Add the missing test.

**Tier 2 — user-visible correctness**

5. **D6** — guard the "nothing to commit" case in `create_stage_snapshot`; include stdout
   in `git_output`'s error text.
6. **D5** — do not leave the index staged when a commit fails.
7. **D7** — make an automatic snapshot best-effort so a green `deltaforge test` exits 0.
8. **D8** — fix the retrospective gate to mean "the last level", and correct
   `docs/pack-format.md:56` and `docs/quickstart.md:30`.
9. **D9** — add a single `display_path()` helper and use it everywhere a path reaches a
   human.
10. **D11** — carry the freshness check into `explain-failure` and `status`.

**Tier 3 — hardening and polish**

11. **D10** — take the capability token out of the URL; document the mechanism.
12. **D13** — `0700` scratch directories, `create_dir` not `create_dir_all`, random suffix.
13. **D12** — renumber the three preview packs (or derive the display number from
    manifest position).
14. **D16, D17, D14, D15, D18** — diagnostics and documentation corrections.
15. **R2, R3, R4, R6, R8, R10** — test hermeticity, orphaned children, connection bound,
    memory-measurement fidelity, unbounded files, pager EPIPE.
16. **Coverage gaps 1–7** — in particular, rename or repair the misleading legacy-schema
    test, and add the performance guards that would have caught D1 and D2.

---

## 7. What holds up well

Recorded because a defect list is not a picture of the whole.

- The **browser boundary** is carefully built: loopback-only, ephemeral port, 256-bit
  CSPRNG token, comparison without token-dependent early exit, `Host` validation, exact
  `Origin` match for reads and a stricter one plus `application/json` for mutations,
  bounded headers and bodies, `no-store`, `nosniff`, `no-referrer`, `X-Frame-Options:
  DENY`, and no route that serves a repository file. Every one of these was verified by
  hand against a live service, and each refused as documented. D4 and D10 are about the
  *client* side of that boundary, not the server side.
- **Project creation** is the one place a browser names a path, and the containment is
  genuinely narrow and genuinely tested — leaf validation, canonicalise-then-contain,
  hidden-component refusal, nesting refusal, existence refusal, each with its own test.
- **Concurrency control works.** A run started in the browser correctly refused a second
  browser run (409 `run_already_active`), a concurrent `deltaforge test` (*"another
  DeltaForge check run is already active"*), and `deltaforge exit` (*"DeltaForge could not
  stop while a check run is active"*) — all verified live.
- **Crash recovery works.** After `taskkill /F` of the service mid-run, the stale
  discovery record was correctly discarded on the next launch, the leaked lease was
  released by the OS, and the next `deltaforge test` recorded the abandoned job as
  `interrupted` and continued cleanly.
- **The digest-based progression contract is real.** Editing the tree after a stage passes
  blocks `next` with an accurate message; the behavioral digest correctly distinguishes
  documentation-only pack changes from changes to tests, fixtures, or commands.
- **The failure-diagnosis pipeline is the product's best feature** and it works: the
  primary diagnosis, the contract line, the expected/actual pair, and the fixture listing
  are all accurate, and `phase1_failure_corpus` pins them for all fourteen FlashIndex
  stages.
- **Windows path handling in the runner** (`src/runner.rs:663-760`) — short/long path
  equivalence, separator normalisation, prefix reconstruction — is unusually thorough.
  D9 is the gap where that same care was not applied to presentation.
