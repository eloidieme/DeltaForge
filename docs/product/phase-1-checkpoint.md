# Phase 1 implementation checkpoint

Status: **Frozen implementation checkpoint — Phase 1 is not complete**

Frozen on: 2026-07-15

This document records the exact boundary reached after the first two Phase 1
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
- the service exposes a versioned, read-only health/state/event API and a new
  workbench shell;
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

## Evidence collected

The following checks passed at this checkpoint:

- `cargo check --release --offline`;
- `cargo clippy --release --offline --all-targets -- -D warnings`;
- all 58 library tests after the application-boundary slice;
- all three new workbench security/shell unit tests;
- targeted CLI integration tests for the starter project, test selection/JSON, and
  kept-temp/timeout configuration;
- a fresh FlashIndex dogfood project initialized successfully;
- two consecutive bare launches reused the same port, PID, and capability token;
- the dogfood health endpoint returned API `v1` and the running binary version;
- the dogfood state endpoint restored FlashIndex capability `01_scan_files` with
  `never_run` freshness and Stage 2 available as the next capability.

The full legacy CLI integration suite is not the acceptance oracle for this branch:
the frozen product direction explicitly permits breaking the old text and JSON
contracts. Several remaining legacy assertions still describe the previous
CLI-generated-page product and must be replaced by Phase 1 acceptance scenarios, not
preserved through compatibility code. One pre-existing Windows fixture newline issue
in the MiniKV reference-solution test was also observed independently of the new
application boundary.

## Current limitations

This is not yet a usable end-to-end Phase 1 workbench.

- The `Run checks` button is deliberately disabled.
- The service API is read-only; it cannot start, cancel, or focus-rerun a job.
- Browser event delivery currently observes persisted state at intervals. It does not
  yet forward the in-process build/test event stream across processes.
- Build output is emitted after the build process returns, not streamed line by line.
- The shell shows only the initial mission/evidence view. Running, failure,
  interruption, unhealthy-project, completion, and returning states are not designed.
- Stage 1 text in the shell is provisional and partly hard-coded.
- The shell has not completed visual browser inspection. The in-app browser loaded a
  DeltaForge tab, but DOM/screenshot inspection was interrupted before sign-off.
- There is no hint-reveal, next-capability, editor/folder-open, or cancellation API.
- There is no service shutdown operation yet; only idle shutdown is implemented.
- The old viewer and generated pages still exist for legacy commands. They have not
  yet been removed from the normal product journey.

## Work remaining for Phase 1

### 1. Complete application operations and canonical state

- Load structured Stage 1 mission content through the application core.
- Add focused rerun, hint reveal, begin-next-capability, project health, resumption,
  stage-change summary, and editor/folder actions.
- Define the canonical primary action for every required workbench state.
- Record enough latest-run detail to reconstruct failure and completion after restart.
- Emit and persist explicit source-change and interruption transitions.

### 2. Complete the service and API

- Add bounded POST-body parsing and the defined-operation endpoints for start, cancel,
  focused rerun, hint reveal, next capability, and editor/folder opening.
- Enforce Origin validation and capability tokens on every mutating request.
- Add one-active-job concurrency control and safe cancellation.
- Bridge CLI-started and browser-started runs into one shared live event journal so an
  open workbench follows both without reload.
- Stream build and test events as they happen rather than polling only final state.
- Add service-version replacement tests, stale-metadata recovery tests, idle-shutdown
  tests, and a defined shutdown path for tests and diagnostics.

### 3. Build all required workbench states

- Current mission before any run.
- Live build/check progress with elapsed time and cancellation.
- Failed checks with one deterministic `Start here` diagnosis.
- Stale result after a relevant source edit.
- CLI-started run appearing live in the browser.
- Capability acquired with Stage 2 made available automatically.
- Returning/resumption state.
- Interrupted/crashed job state.
- Unhealthy-project state with an actionable recovery.

### 4. Complete the first diagnosis and help loop

- Choose the primary contradiction deterministically.
- Show the minimum relevant fixture/input, expected result, actual result, and related
  contract.
- Implement focused rerun and first-hint actions.
- Keep secondary failures collapsed but available.
- Author and review all five help levels.

### 5. Rewrite and validate FlashIndex Stage 1 content

- Replace provisional shell text with structured mission, why, success conditions,
  example, complete requirements, edge cases, non-goals, capability statement, and
  Stage 2 preview.
- Create the specified failure corpus: no output, missing nested files, absolute paths,
  unstable ordering, unexpected files, build failure, and timeout/crash.
- Specify and test the expected primary diagnosis for every corpus entry.

### 6. Finish product integration and remove the old normal path

- Make browser-controlled checks complete the shared learner loop.
- Make `deltaforge test` appear live in an already-open workbench.
- Make passing checks transition directly to capability acquired without
  `deltaforge next`.
- Ensure bare invocation focuses/reuses an existing workbench tab where possible.
- Remove generated learning/report pages and explicit `serve` concepts from the normal
  learner journey; retain only consciously scoped power/diagnostic behavior.

### 7. Meet the Phase 1 quality and product bar

- Complete desktop and responsive browser inspection of every required state.
- Add end-to-end acceptance tests for browser control, CLI control, freshness,
  diagnosis, completion, resumption, lifecycle invisibility, browser fallback, and
  security.
- Run the full updated suite on the supported platforms.
- Dogfood a full interrupted-and-resumed Stage 1 completion.
- Prepare the observation protocol for at least five external target learners.
- Run the decisive product test from `phase-1-vertical-slice.md` without service-model
  questions or manual refreshes.

## Resume point

Resume Phase 1 at the service mutation/event bridge:

1. add the single active-job coordinator and event journal;
2. expose `POST /api/v1/runs` and cancellation/focused-rerun operations;
3. enable the shell's primary action and render live run progress;
4. prove that a CLI-started run appears in the same open workbench.

Do not expand into the full catalog, later FlashIndex stages, AI coaching, cloud
features, or a final design system until the Phase 1 decisive product test passes.
