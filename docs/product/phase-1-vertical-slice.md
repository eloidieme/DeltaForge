# Phase 1 vertical slice: FlashIndex Stage 1

Status: **Approved by frozen Phase 0 baseline**

## Purpose

Phase 1 must prove the new product architecture through one complete learner loop before
DeltaForge migrates every command or builds the full project library.

The slice uses FlashIndex Stage 1, deterministic file discovery. It is production code
where it establishes the new application boundary, but it remains deliberately narrow
in product breadth.

## Decisive product test

> A learner completes FlashIndex Stage 1 through the workbench, runs at least one check
> through the CLI, closes everything, resumes successfully, and never has to reason
> about the local service.

## Included journey

```text
Run bare deltaforge inside a prepared FlashIndex learner project
  -> local service starts invisibly
  -> browser opens the current mission
  -> learner runs the initial checks from the browser
  -> live build and test progress appears
  -> one failure is prioritized and explained
  -> learner opens the repository in an external editor
  -> source changes mark the previous result stale
  -> learner runs checks from the CLI
  -> the browser follows the same run live
  -> learner fixes the implementation
  -> checks pass
  -> capability-acquired state appears
  -> next capability becomes available
  -> learner closes terminal and browser
  -> a later bare deltaforge resumes the correct state
```

## Required product states

1. Current mission before any run
2. Build and checks running
3. Checks failed with one `Start here` diagnosis
4. Source changed and result stale
5. CLI-started run visible in the browser
6. Capability acquired
7. Returning/resumption
8. Interrupted or crashed job
9. Project unhealthy with an actionable recovery

## Application-core tasks

### 1. Extract typed Stage 1 operations

Create application services for:

- locating and loading the prepared project;
- computing the canonical workbench state;
- loading current capability content;
- starting a Stage 1 test run;
- targeting one test for rerun;
- revealing the next hint;
- recording completion and making Stage 2 available;
- producing a stage-change summary;
- obtaining project health and resumption information.

No operation prints terminal UI, opens a browser, or renders HTML.

### 2. Define new 1.0 state

Create the minimum new state and persistence needed for:

- current capability;
- completion evidence;
- latest run summary;
- bounded attempt history;
- revealed hint level;
- source freshness marker;
- activity timestamp;
- interrupted-job recovery.

Do not implement legacy state migration.

### 3. Define structured events

At minimum:

```text
JobStarted
BuildStarted
BuildOutput
BuildCompleted
TestStarted
TestPassed
TestFailed
RunCompleted
SourceChanged
ProjectStateChanged
JobInterrupted
```

## Local-service tasks

### 4. Implement service discovery and lifecycle

- Start from bare `deltaforge`.
- Reuse a compatible running service.
- Replace an incompatible or stale service safely.
- Return the terminal prompt after opening/focusing the workbench.
- Exit after idle time when no tabs or jobs remain.
- Recover cleanly from stale lifecycle metadata.

### 5. Implement the minimum versioned API

The API needs:

- project/workbench state;
- capability content;
- start/cancel/rerun test job;
- hint reveal;
- begin-next-capability action;
- editor/folder opening action;
- live event stream;
- health and version probing.

### 6. Implement the security boundary

- Loopback binding
- Capability token
- Host and origin validation
- Defined operations only
- Prepared project restricted to its registered root
- Bounded inputs and output
- No repository file-serving endpoint
- Tests for drive-by cross-origin and traversal attempts

### 7. Observe source freshness

Watch only relevant project and build inputs. On change:

- mark the last result stale;
- emit `SourceChanged`;
- update the primary action to `Run checks`;
- do not run automatically.

## CLI tasks

### 8. Redefine bare invocation

Inside the prepared project, `deltaforge` starts/focuses the workbench and returns.

### 9. Connect `deltaforge test`

The CLI uses the same Stage 1 application operation and event model. Its run must appear
live in the open workbench.

No legacy text or JSON compatibility is required. The new CLI output should remain
clear and scriptable, with a new structured format specified before broader expansion.

### 10. Add diagnostic fallback

If the browser cannot open, print the local URL and explain the terminal test path. The
prepared project must remain testable without the browser.

## Frontend tasks

### 11. Build the new application shell

Create a genuinely new visual direction. Do not port the current learning and report
pages into a single-page router unchanged.

### 12. Render canonical workbench state

The frontend receives the primary action and state from the application model. It does
not recreate progression logic.

### 13. Build live-run presentation

Show:

- current build/test phase;
- completed, active, and pending checks;
- elapsed time;
- cancellation;
- partial failures as they arrive;
- clear interruption behavior.

### 14. Build the first diagnosis

For at least one real Stage 1 failure, display:

- concise contradiction headline;
- minimal relevant fixture/input;
- expected and actual result;
- related contract;
- focused rerun;
- first hint action;
- other failures collapsed.

### 15. Build completion and resumption

Completion displays the acquired deterministic-file-discovery capability and makes the
next capability available. Resumption reconstructs the latest run, freshness, and next
action without rerunning anything.

## Content tasks

### 16. Rewrite Stage 1 for mission presentation

Provide:

- mission;
- why it matters;
- success conditions;
- example;
- complete requirements;
- edge cases;
- non-goals;
- capability completion statement;
- Stage 2 preview.

### 17. Author the help ladder

Write and review all five help levels, even if the retrospective level is not exposed in
the first slice.

### 18. Create the Phase 1 failure corpus

Include at least:

- no output;
- missing nested files;
- absolute instead of relative paths;
- unstable ordering;
- unexpected files;
- build failure;
- timeout or crash.

Specify the expected primary diagnosis for each.

## Acceptance scenarios

### Browser-controlled run

Given a prepared Stage 1 project with an incomplete implementation, when the learner
starts checks in the workbench, the browser streams the build and tests and finishes in
the correct failed state without a page reload.

### CLI-controlled run

Given an open workbench, when the learner runs `deltaforge test`, the same job and events
appear in the browser and both surfaces agree on the result.

### Source freshness

Given a completed run, when a relevant source file changes, the workbench marks the
result stale without automatically starting a new run.

### Focused diagnosis

Given multiple failures, the workbench presents one deterministic primary contradiction
and keeps the remaining failures available but collapsed.

### Completion

Given a passing implementation, the workbench records completion evidence, presents the
acquired capability, and makes Stage 2 available without requiring `deltaforge next`.

### Resumption

Given a previous failed or passed run, when all DeltaForge processes and tabs have been
closed and the learner later runs bare `deltaforge`, the workbench restores the correct
state and next action without rerunning tests.

### Invisible lifecycle

During the journey, the learner never needs to start, stop, restart, or identify a
service or port.

### Browser unavailable

When the browser cannot be opened, the CLI remains usable and prints an actionable
fallback rather than failing the project workflow.

### Security

Unrelated web origins cannot invoke project operations, arbitrary commands cannot be
submitted, and repository files cannot be retrieved through guessed paths.

## Product research protocol

Until external participants are available, the product owner dogfoods the slice using a
fresh project and an interrupted/resumed session. The test protocol is prepared for at
least five target learners before expansion beyond FlashIndex's early stages.

Observe without teaching the interface. Record:

- time to current mission;
- time to initial run;
- any question about server, terminal, browser, or refresh behavior;
- whether the learner can state the primary failure;
- whether they know where to write code;
- whether they notice stale state after editing;
- whether CLI/browser agreement is understood;
- whether they can explain the acquired capability;
- time to resume after a later launch.

The slice passes product validation when users complete the loop without service-model
questions and can correctly identify the current mission, evidence, and next action.

## Out of scope

- Full project library and catalog
- Project creation UI
- Multi-project workbench
- Complete FlashIndex progression
- Performance lab
- Chronicle and final challenge
- AI coaching
- Legacy project migration
- Legacy generated-page fallback
- Final design system
- Native desktop shell
- Cloud synchronization

## Definition of Phase 1 complete

Phase 1 is complete only when:

- the decisive product test passes end to end;
- browser and CLI share typed operations and events;
- the service lifecycle is invisible in normal use;
- the new visual direction is credible in all required slice states;
- the security tests pass;
- the owner has completed an interrupted and resumed dogfood run;
- the prepared research protocol is ready for external participants;
- the implementation has not introduced abstractions solely for legacy compatibility.
