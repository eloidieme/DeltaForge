# Phase 1 implementation checkpoint

Status: **Active implementation checkpoint — Phase 1 is not complete**

Updated on: 2026-07-16

This document records the exact boundary reached after the first fifteen Phase 1
implementation slices. It supplements, and does not replace, the approved scope and
definition of done in `phase-1-vertical-slice.md`.

## Product outcome currently available

DeltaForge now has the beginning of the product architecture described in Phase 0:

- `deltaforge test` is an adapter over a shared typed application operation instead of
  owning execution and persistence itself;
- test runs produce structured build, test, completion, and state-change events;
- run state persists source evidence, a bounded attempt history, and active-job
  information;
- an abandoned active job is recovered as interrupted on the next load;
- the canonical workbench state reports the current capability, next capability,
  completion, hint level, last activity, and whether the latest result is fresh;
- bare `deltaforge` inside a prepared project starts or reuses one hidden loopback
  service, opens the workbench, and returns the terminal prompt;
- browser-unavailable mode prints the local URL and the `deltaforge test` fallback;
- the service exposes a versioned health/state/event API and a new
  workbench shell;
- browser and CLI check runs now share one cross-process run lease and one bounded,
  persisted event journal;
- the workbench can start, focus-rerun, follow, and cancel checks without a reload;
- Stage 1 mission content and its five-level help ladder are loaded through the
  application core instead of being embedded in the shell;
- failed runs persist a deterministic primary diagnosis with bounded fixture, expected,
  observed, contract, and command evidence that survives service restarts;
- the workbench presents the primary contradiction, keeps other failures collapsed,
  and reveals help progressively without exposing the retrospective before completion;
- passing Stage 1 produces an application-owned `Begin next capability` action and the
  browser can advance to Stage 2 without `deltaforge next`;
- each secondary contradiction can be rerun from its collapsed disclosure without
  replacing the full-run diagnosis, and the primary contradiction remains visually
  dominant;
- bare `deltaforge` is the only taught learner entry, while `overview`, `instructions`,
  and `test` remain terminal-only diagnostics that never generate or open legacy pages;
- the service is restricted to loopback and protects every route with a per-service
  capability token, exact Host validation, Origin validation, bounded headers, and a
  closed route table that never serves repository files.

The shell establishes a new cool graphite/blue visual direction. It intentionally does
not reuse the retired warm paper/ember language or embed the old generated learning and
test-report pages.

## Frozen implementation slices

### Slice 1: shared application boundary

Committed as `d28644f` (`Extract Phase 1 application run boundary`).

Implemented:

- typed `TestRunRequest`, `TestRunOutcome`, `RunEvent`, and `EventSink` contracts;
- CLI-independent runner options;
- structured events for job start, build lifecycle/output, individual test lifecycle,
  run completion, source changes, state changes, and interruption;
- serializable fixture/input/diagnostic details for future browser diagnosis;
- canonical `WorkbenchState` and source-freshness computation;
- persisted project digest on the latest test result;
- a twenty-entry attempt-history bound;
- persisted active-job metadata and interrupted-job recovery;
- new structured CLI JSON result envelope;
- CLI rendering of the shared event stream, including terminal fallback diagnostics;
- preservation of kept temporary-directory output for the power-user CLI path.

### Slice 2: invisible service and read-only workbench foundation

Included in the checkpoint commit containing this document.

Implemented:

- optional CLI subcommand routing so bare invocation is meaningful;
- hidden internal `__workbench` process entry point;
- per-project service discovery in `.deltaforge/workbench.json`;
- stale-record cleanup, compatible-service probing, and service reuse;
- hidden Windows process launch and a four-second readiness handshake;
- thirty-minute idle shutdown when no event-stream clients remain;
- `GET /api/v1/health`;
- `GET /api/v1/state`;
- `GET /api/v1/events`, which observes persisted state and source freshness;
- a token-protected root workbench document;
- CSP, no-store, no-sniff, no-referrer, and frame-denial response headers;
- unit coverage for missing tokens, wrong Hosts, hostile Origins, guessed repository
  paths, and the new shell identity;
- a responsive three-column mission/evidence shell for the initial Stage 1 state.

### Slice 3: shared run coordination and live browser control

Implemented after the original checkpoint commit:

- a project-scoped, cross-process run lease with stale-owner cleanup;
- live-state loading that preserves a genuinely active CLI or browser job instead of
  misclassifying it as interrupted;
- cancellable build and test child processes with process-tree termination;
- a 256-entry, two-megabyte persisted event journal with per-field output bounds;
- cursor-based SSE replay that closes the state-fetch/event-stream handoff gap;
- CLI-started job, build, test, failure, completion, interruption, and state-change
  events appearing in an already-open workbench;
- bounded JSON request-body parsing and exact Origin, Host, token, and content-type
  validation for mutations;
- `POST /api/v1/runs`;
- `POST /api/v1/runs/rerun`;
- `POST /api/v1/runs/cancel`;
- one-active-run conflict responses across both CLI and browser entry points;
- an enabled primary Run/Cancel action, elapsed time, live phase and counts, the first
  failing check, and a focused-rerun action in the shell;
- canonical state fields for the active job, latest attempt, latest current-stage run,
  and event cursor;
- idle shutdown protection while any CLI or browser run remains active;
- an end-to-end loopback integration test covering CLI event delivery, a
  browser-started focused rerun, and browser cancellation.

### Slice 4: structured mission, diagnosis, help, and progression

Implemented in the current worktree:

- typed capability content assembled from pack-authored mission, why, success,
  requirements, example, edge-case, non-goal, completion, preview, and help sections;
- rewritten FlashIndex Stage 1 mission content and all five help levels;
- pack-authored deterministic diagnosis priorities, headlines, and related contracts;
- an explicit root-relative-path behavioral check whose temporary fixture path is
  expanded only for comparison and sanitized again before persistence;
- durable bounded diagnosis evidence, including fixture entries and expected/observed
  comparison text, with build and timeout failures taking foundational priority;
- application-owned primary actions for initial/rerun, cancel, begin-next, and final
  journey states;
- serialized hint and progression state mutations under the same project run lease;
- `GET /api/v1/capability`, `POST /api/v1/hints`, and
  `POST /api/v1/capabilities/next`;
- a structured mission/specification surface, primary diagnosis panel, collapsed
  secondary contradictions, and progressive-help surface in the workbench;
- an end-to-end passing Stage 1 flow covering completion, fifth-level retrospective,
  and browser progression to Stage 2;
- integration coverage proving that a later compile failure supersedes an older
  behavioral result with an actionable build diagnosis;
- an executable seven-implementation failure corpus covering no output, truncated
  traversal, absolute paths, reversed ordering, unignored generated files, compile
  failure, and timeout;
- corpus assertions for the exact primary priority, kind, headline, related contract,
  expected/observed evidence, sanitized command, fixture context, and absence of
  temporary filesystem paths across every persisted diagnosis field.

### Slice 5: durable source freshness

Implemented in the current worktree:

- a service-level source observer that runs without an open event-stream client;
- persisted observed project digest, monotonic source revision, last transition, and
  acknowledged event revision;
- restart-safe `SourceChanged` publication that reconciles a journal write completed
  before its state acknowledgement without duplicating the event;
- relevant source/build-input changes producing durable `SourceChanged` and
  `ProjectStateChanged` journal entries;
- the existing project digest exclusions preventing `.deltaforge`, `target`, `build`,
  dependency, cache, and pack/config-declared ignored paths from creating false stale
  transitions;
- test-run evidence bound to the digest captured before the build begins, so an edit
  during a run leaves its result stale instead of incorrectly proving newer source;
- canonical state exposing the current source revision and last transition, with stale
  evidence selecting the application-owned `Run checks` action;
- live workbench handling for the explicit source event and source-change timestamp;
- real-service acceptance coverage for a relevant edit, ignored build output, edits
  while the service is stopped, restart recovery, repeated edits, and no duplicate
  events after stabilization.

### Slice 6: project health, recovery, and repository actions

Implemented in the current worktree:

- prepared-project location separated from full state/configuration/pack loading, so
  bare launch and the hidden service remain available when the project is unhealthy;
- structured healthy/unhealthy application state with classified configuration,
  state, pack-pin, language, and general project-load issues;
- application-owned recovery actions and specific guidance instead of an unstructured
  HTTP failure;
- fixed-operation `GET /api/v1/project-health`, pack re-pin, editor-open, and
  folder-open routes;
- editor/folder operations restricted to the registered project root, with no
  browser-supplied path or command and exact mutation authorization still required;
- configured editor commands executed directly without a shell, plus platform-specific
  fixed fallbacks;
- a dedicated unhealthy workbench presentation that remains available when canonical
  workbench state and capability content cannot load, automatically rechecks health,
  and returns to the normal surface after recovery;
- always-available healthy-state `Open editor` and `Open folder` actions;
- real-service coverage for startup with invalid configuration, malformed state,
  rejected path injection, external repair/recheck, changed-pack diagnosis, bounded
  in-workbench pack recovery, and return to canonical state.

### Slice 7: restart-safe resumption and stage-change summaries

Implemented in the current worktree:

- a persisted workbench-session resumption point that does not overwrite the learner's
  last meaningful activity timestamp;
- typed returning summaries for interrupted work, stage changes, stale source, failed
  checks, acquired capabilities, and ready-to-continue projects;
- a structured stage-change summary retaining the previous and current capability IDs
  and titles;
- canonical state restoring the latest run, freshness, primary remaining failure, and
  next action without automatically rerunning checks;
- an explicit application-owned `Resume checks` primary action that remains pending
  only until new learner or source activity occurs;
- browser presentation of the returning summary while preserving the latest diagnosis
  and normal live-run hierarchy;
- true service-process stop/restart acceptance coverage proving an active run becomes a
  durable interrupted attempt, remains idle after restoration, and can be restarted by
  the learner;
- restart acceptance after Stage 1 progression proving the Stage 1-to-Stage 2 change is
  summarized, plus offline-source restart coverage using the returning action.

### Slice 8: live build output and diagnostic shutdown

Implemented in the current worktree:

- bounded stdout and stderr chunks delivered while the configured build child is still
  running instead of after process completion;
- bounded final process output retained independently for deterministic build-failure,
  timeout, and cancellation diagnosis;
- streamed build events flowing through the same application sink, persisted journal,
  CLI renderer, SSE replay, and browser state as every other run event;
- a live workbench build-output tail with per-chunk stream identity and text-only
  rendering;
- a bounded reader queue that applies backpressure to chatty child processes instead of
  accumulating unbounded output in memory;
- authenticated `POST /api/v1/service/shutdown`, restricted to an exact authorized JSON
  mutation and hidden from the learner journey;
- shutdown/run serialization that rejects shutdown while a run is starting or active,
  rejects new runs once shutdown begins, removes discovery metadata, acknowledges the
  caller, and terminates the service process;
- unit coverage proving the first output chunk can cancel a still-running child, plus
  real-service coverage for build-event ordering and safe diagnostic shutdown.

### Slice 9: service lifecycle and connected-tab reuse

Implemented in the current worktree:

- a per-project startup lease that serializes simultaneous bare launches and recovers
  after a launch-process crash;
- discovery parsing that removes corrupt or dead records without trusting their PID or
  contacting unrelated local processes;
- health probing that verifies service identity, authoritative process version and PID,
  and connected event-stream client count;
- live incompatible-service replacement through its authenticated shutdown operation,
  with replacement refused while the existing service has an active run;
- record ownership checks that prevent stale cleanup from deleting metadata written by
  a newer service;
- a configurable hidden idle timeout for lifecycle testing, with production retaining
  the thirty-minute default and active jobs or event-stream clients preventing exit;
- idle exit removing the service discovery record before process termination;
- a service-local focus revision delivered to connected workbench event streams, with
  the browser making a best-effort `window.focus()` request;
- bare launch requesting focus when a workbench tab is already connected and opening a
  browser only when no connected tab can be reused;
- concurrent real-service acceptance coverage for simultaneous launch, corrupt metadata,
  live incompatible replacement, idle exit, discovery cleanup, and connected-tab focus.

### Slice 10: browser sign-off and focused secondary reruns

Implemented in the current worktree:

- desktop browser inspection of returning failure, live run, capability-acquired,
  Stage 1-to-Stage 2 progression, and unhealthy-project recovery states;
- responsive inspection at 390 by 844 pixels with no horizontal overflow, compact
  project identity, and editor/folder recovery actions retained as full-size controls;
- keyboard-reachable native button and disclosure controls, explicit high-contrast
  `:focus-visible` treatment, and accessible names for every focused-rerun action;
- a low-prominence focused-rerun control for each collapsed secondary contradiction,
  preserving the deterministic primary diagnosis and the full latest-run evidence;
- real-service acceptance coverage proving a secondary check is selected by name,
  emits a one-check run, and leaves the persisted nine-check failure summary intact;
- progressive-help browser inspection through the first revealed level without
  exposing later help automatically;
- automatic transition from a classified unhealthy configuration back to the normal
  workbench after external repair;
- locale-readable activity timestamps in place of raw ISO strings;
- run totals derived from the current capability's canonical evidence, preventing the
  prior capability's pass count from appearing beside a new capability's `No run yet`
  state.

### Slice 11: stale, interruption, and CLI-live sign-off

Implemented in the current worktree:

- browser inspection of a CLI-started run while its real project build was active,
  including shared `Cancel run`, build phase, elapsed time, and live counts;
- browser inspection of source-stale evidence after a real relevant edit, with the
  previous result preserved and the current `Run checks again` action clearly separated;
- browser inspection of a service process stopped during an active build and restarted
  into the durable `Previous run interrupted` resumption state without an automatic run;
- focused-rerun controls hidden whenever the full diagnosis is stale, preventing a
  one-check action from competing with the required full proof refresh;
- the recovered-interruption marker cleared after the learner completes a subsequent
  run, restoring the normal last-activity presentation instead of retaining obsolete
  recovery copy;
- acceptance assertions covering the recovery marker before and after the resumed run,
  plus a shell regression assertion for fresh-only focused reruns;
- visual inspection of the repaired stale state confirming the one-primary hierarchy
  and absence of a stale focused-rerun control.

### Slice 12: physical keyboard sign-off and focus continuity

Implemented in the current worktree:

- an OS-level Chrome keyboard traversal covering rail actions, the primary action,
  mission disclosures, the primary diagnosis rerun, the secondary-failure disclosure,
  every accessible secondary rerun, and progressive help;
- Return-key activation verified for native disclosures and progressive help, with help
  advancing without mouse input;
- keyboard-only run start and cancellation verified against a deliberately slow real
  Rust build, with focus remaining on the primary control as it changed between `Run
  checks again` and `Cancel run`;
- keyboard-only unhealthy-project recovery verified through `Check again` after an
  external configuration repair;
- keyboard-only Stage 1 completion and progression verified from `Cancel run` to
  `Begin next capability` and then to Stage 2's `Run initial checks` action;
- focus restoration added when help and primary controls are temporarily disabled for
  an authenticated mutation, preventing focus from falling back to the document body;
- shell regression assertions covering focus restoration for both controls.

### Slice 13: legacy generated-page path removal

Implemented in the current worktree:

- removed the learner-facing `serve` command and all of its stop, restart, live-update,
  stable-port, and hidden UI-directory options;
- removed the legacy static learning-page generator, test-report generator, live viewer,
  and shared warm paper/ember page theme modules;
- removed `--open` and `--terminal` from `test`, plus `--terminal` and `--no-open` from
  `overview` and `instructions`;
- made `overview` and `instructions` explicit terminal diagnostics while preserving
  `overview --json`, stage selection, and all-stage instruction output;
- made human-readable `test` always stream bounded terminal evidence without creating,
  opening, or updating `.deltaforge/ui`;
- moved the small platform browser-launch adapter into the workbench, leaving the
  canonical loopback workbench as the only browser product surface;
- changed initialization output and generated project README guidance to teach bare
  `deltaforge`, with `test`, `status`, and `doctor` retained as terminal fallbacks;
- replaced generated-page CLI assertions with acceptance coverage proving the retired
  commands and flags are absent and a normal failed test run creates no UI directory;
- updated the README, command reference, and current-state handoff to describe the
  workbench architecture instead of the retired viewer.

### Slice 14: decisive interrupted-and-resumed Stage 1 dogfood

Validated against a fresh initialized FlashIndex learner project:

- opened the canonical Stage 1 mission in the real loopback workbench with no prior
  evidence and ran the initial checks from the browser;
- observed live build state followed by the deterministic `Start here` diagnosis,
  with the other eight contradictions retained behind one disclosure;
- revealed Observation, Concept, Experiment, and Structure progressively while the
  Retrospective remained locked before completion;
- introduced a deliberately slow Rust build, confirmed the prior nine-failure result
  became stale without being discarded, cancelled the active build, and verified the
  durable latest attempt was `cancelled` while the prior diagnosis remained available;
- started another slow build, terminated the workbench service during the active job,
  and reopened the project into `Previous run interrupted` without an automatic rerun;
- confirmed interruption recovery retained the full prior diagnosis and all four
  previously revealed help levels, then used the workbench's bounded editor action;
- applied a passing Stage 1 implementation and ran `deltaforge test` through the CLI
  while the workbench remained open; both surfaces agreed on nine passed checks;
- observed `Capability acquired`, `Begin next capability`, and the newly unlocked
  fifth-level Retrospective without a page reload;
- closed the browser and service, relaunched with bare `deltaforge`, and confirmed the
  returning workbench restored current 9/9 evidence, all five help levels, and the
  next-capability action without rerunning checks;
- completed the journey without needing to identify, start, stop, or reason about a
  service or port in the learner-facing flow.

### Slice 15: five-learner observation protocol

Prepared in `phase-1-observation-protocol.md`:

- a five-participant target profile spanning advanced beginners and intermediate
  engineers with working Rust, editor, terminal, and basic Git experience;
- explicit exclusions preventing contributors, prior workbench users, and learners who
  have seen the FlashIndex tests or solution from contaminating the study;
- a 60–90 minute fresh-project session followed by a 10–15 minute cold-resumption
  session 24–72 hours later;
- verbatim participant briefings, neutral checkpoint questions, and a constrained
  moderator intervention ladder that preserves observation without teaching;
- milestone measures for activation, mission orientation, initial run, primary-failure
  comprehension, editor discovery, source freshness, CLI/browser agreement, capability
  acquisition, and thirty-second resumption;
- per-participant environment, event, timing, help, run, intervention, quote, and
  assessment recording sheets plus a five-person aggregate synthesis table;
- binding individual and Phase 1 research-gate thresholds that do not average away
  service-model questions, manual-refresh recovery, or other blockers;
- an S0–S4 issue-severity rubric with explicit release effects, retest rules, and a
  final research-report outline;
- privacy, consent, environment validation, invalid-session, and study-data handling
  rules suitable for external participant scheduling.

## Evidence collected

The following checks passed at this checkpoint:

- `cargo check --release --offline`;
- `cargo clippy --release --offline --all-targets -- -D warnings`;
- all 68 current library tests after deleting eleven legacy page/viewer unit tests;
- all new run-coordination, journal, workbench security, request, and shell unit tests;
- targeted CLI integration tests for the starter project, test selection/JSON, and
  kept-temp/timeout configuration;
- all nine real-service workbench integration tests, including a CLI-triggered nine-test
  failure stream, deterministic persisted diagnosis, four-level pre-completion help,
  primary and secondary browser focused reruns, cancellation, build-failure recovery
  state, a passing Stage 1 run, post-completion retrospective, and progression to Stage
  2;
- durable freshness integration covering live and offline edits, ignored paths,
  monotonic revisions, restart recovery, stale primary action, and event deduplication;
- unhealthy-project integration covering invalid configuration and state, secure
  recovery operations, pack adoption, and restoration of canonical workbench state;
- stop/restart integration covering durable interruption, no automatic rerun, explicit
  resume action, learner-triggered continuation, and Stage 1-to-Stage 2 return context;
- live build integration proving build output is journaled between build start and
  completion, plus authenticated shutdown coverage that cannot interrupt an active run;
- lifecycle integration covering simultaneous launch serialization, corrupt/dead record
  recovery, authenticated incompatible replacement, idle exit, and tab-focus signaling;
- the seven-case Phase 1 failure corpus through real initialized FlashIndex projects
  and the shared application state;
- a fresh FlashIndex dogfood project initialized successfully;
- two consecutive bare launches reused the same port, PID, and capability token;
- the dogfood health endpoint returned API `v1` and the running binary version;
- the dogfood state endpoint restored FlashIndex capability `01_scan_files` with
  `never_run` freshness and Stage 2 available as the next capability;
- browser-controlled dogfood completed all nine Stage 1 checks, rendered `Capability
  acquired`, and advanced to Stage 2 without a manual refresh;
- responsive DOM inspection confirmed a 390-pixel viewport without horizontal overflow,
  with all visible buttons and disclosures remaining keyboard-focusable;
- unhealthy configuration repair, progressive help, primary diagnosis, all eight
  secondary rerun controls, returning state, live progress, and completion/progression
  were exercised against the rebuilt local service;
- a deliberately slow real Rust build confirmed that an external CLI run appears live
  in the open workbench; a source edit and a killed/restarted service confirmed stale
  and interrupted presentation without manual refresh;
- physical Chrome Tab/Return traversal completed the mission, diagnosis, help,
  cancellation, recovery, completion, and progression paths, including the repaired
  focus-continuity behavior.
- CLI surface acceptance confirmed that `serve`, `test --open`, `test --terminal`, and
  the learning-page flags are absent, while normal failed runs create no
  `.deltaforge/ui` directory;
- the decisive fresh-project dogfood journey passed initial browser failure, the full
  five-level help ladder, stale evidence, cancellation, interrupted-service recovery,
  CLI/browser agreement, Stage 1 completion, and a cold completed-project resume;
- the external observation protocol now provides a runnable two-session script,
  recording materials, pass thresholds, and issue-severity rubric for five target
  learners;

The full updated CLI integration suite is now green. Three assertions that still
expected the retired stage-heading text were replaced with the current terminal
contract. Strict validation also exposed and drove a fix for labeled progressive-help
headings. The previously noted Windows fixture-newline risk was addressed by restoring
repository-wide LF normalization while preserving the intentional FlashIndex CRLF
fixture subtree.

## Final release audit status

The detailed command and platform record is in `phase-1-release-audit.md`.

- native macOS formatting, release compilation, release Clippy, all 133 tests, strict
  validation, and all four direct reference-solution proofs pass;
- Linux and Windows release compilation and Clippy pass for all targets through
  target-specific cross-compilation;
- the CI matrix now runs the same release compilation and lint contract before the full
  test and strict-validation gates on macOS, Linux, and Windows;
- native Linux and Windows execution for this exact candidate remains pending because
  the current Phase 1 worktree has not been committed or pushed.

## Current limitations

Phase 1 is not yet complete, but the decisive Stage 1 browser learner loop, observation
preparation, native macOS audit, and Linux/Windows target audit are complete. Hosted
native Linux and Windows execution for the candidate commit remains.

- The service observes the shared persisted journal at 500 ms intervals rather than
  receiving direct inter-process notifications.
- Build output is streamed in bounded chunks, but cross-process delivery to an open
  workbench still inherits the journal's polling interval.
- The shell now covers structured mission, running progress, durable prioritized
  diagnosis, stale, interruption, completion, help, progression, and unhealthy-project
  recovery states. Returning, failure, browser-live, CLI-live, stale, interrupted,
  completion, progression, responsive, and unhealthy recovery states now have browser
  sign-off.
- Native controls, focus styling, Tab order, disclosure activation, help, cancellation,
  recovery, and progression have passed an OS-level Chrome keyboard traversal.
- Cancellation is cooperative at the run coordinator and forcefully terminates the
  active child process tree, but it does not yet carry structured partial-test results
  into the durable latest-run summary.
- Connected-tab focus is best effort because browsers and window managers may decline a
  programmatic `window.focus()` request.

## Work remaining for Phase 1

### 1. Complete the final release audit

- Commit and push the Phase 1 candidate after explicit user authorization.
- Record passing `ubuntu-latest`, `macos-latest`, and `windows-latest` matrix jobs for
  that candidate, fixing and rerunning any platform-owned failure.

## Resume point

Resume Phase 1 by committing and pushing the release candidate after user authorization,
then record the hosted three-platform CI result.

Do not expand into the full catalog, later FlashIndex stages, AI coaching, cloud
features, or a final design system until the Phase 1 release audit is complete.
