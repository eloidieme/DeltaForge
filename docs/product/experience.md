# Learner experience contract

## Entry behavior

The bare command `deltaforge` is the universal entrance.

### Inside a DeltaForge project

Open that project's workbench directly.

### Outside a DeltaForge project

- On first launch, open the welcome and project catalog.
- When projects are known, open the local project library.
- When the workbench is already open, reuse or focus the existing application rather
  than opening duplicate tabs.

The local service starts invisibly and the terminal prompt returns immediately. The
normal experience never mentions `serve`, a port, a shutdown token, or a generated HTML
file.

## First-use journey

```text
Launch DeltaForge
  -> choose a serious project
  -> choose an installed language
  -> choose or confirm a project location
  -> pass environment preflight
  -> create a normal learner-owned Git repository
  -> open the workbench
  -> run the initial behavioral checks
  -> understand the first contradiction
  -> implement in the external editor
  -> see that source changed and the result is stale
  -> run again
  -> acquire the first capability
  -> deliberately begin the next capability
```

## Canonical learner state machine

The application core, not the frontend, determines the canonical learner state and
available actions.

| State | Learner question | Primary action | Important transitions |
|---|---|---|---|
| Welcome | What is DeltaForge? | Choose a project | Project selected |
| Project selection | What should I build? | Start selected project | Configuration |
| Configuration | Can my machine run this? | Create project | Preflight error or ready |
| Ready for first run | What does success require? | Run initial checks | Running |
| Running checks | What is happening? | Cancel run | Failed, passed, or interrupted |
| Diagnosing failure | What fact is wrong? | Work on the primary failure | Source changed or rerun |
| Implementing | What should I change? | Open project in editor | Source changed |
| Result stale | Is my change ready to verify? | Run checks | Running |
| Stage passed | What can my program do now? | Continue | Next capability ready |
| Capability ready | What am I building next? | Begin capability | Ready for first run |
| Prediction needed | What do I expect? | Record prediction | Baseline ready |
| Benchmark ready | What should I measure? | Run benchmark | Benchmarking |
| Benchmarking | How is the experiment progressing? | Cancel benchmark | Results or interrupted |
| Reflection needed | What did the evidence teach me? | Record reflection | Experiment complete |
| Final challenge ready | Is the complete system ready? | Begin final challenge | Final run |
| Project complete | What did I build and learn? | Open engineering story | Export, interview, or new project |
| Interrupted | What stopped, and what remains valid? | Retry or recover | Previous active state |
| Project unhealthy | Why can DeltaForge not continue? | Perform recovery action | Healthy or diagnostic export |

## Action hierarchy

Every screen exposes:

1. one primary action associated with the current state;
2. a small set of contextual secondary actions;
3. deeper details and raw evidence on demand;
4. global project navigation that never competes with the current action.

The interface must not present a grid of equivalent command buttons.

## Project selection

Project cards answer:

- What will I build?
- What will it be able to do?
- What engineering ideas will I practice?
- How demanding is it?
- How long might it take?
- Which languages can I use on this machine?
- What is the final challenge?

DeltaForge recommends a first project but does not require a placement quiz.

## Project creation and editor

The library proposes an editable location such as
`~/DeltaForge/flashindex-rust` and remembers the chosen parent directory.

DeltaForge detects common editors and remembers the learner's choice. It offers:

- Open in editor
- Reveal project folder
- Copy project path
- Copy a terminal command

It does not silently modify editor configuration. Optional generated tasks or launch
configuration require a later explicit action.

## Mission presentation

The current capability is presented progressively:

1. Mission
2. Why it matters
3. Success conditions
4. Example
5. Complete requirements
6. Edge cases
7. Non-goals

The full behavioral specification remains accessible. The default view emphasizes the
smallest useful next goal rather than presenting a document wall.

Future capability names and concise previews are visible. Detailed future
specifications remain locked until available. Completed capabilities remain revisitable.

## First run

The primary initial action for a capability is to run its checks before implementation.
This is strongly encouraged but not enforced by hiding the specification or editor.
The initial run:

- proves the build and execution loop;
- grounds the contract in observable evidence;
- establishes a baseline for later attempts.

## File changes and test execution

DeltaForge watches relevant project files. When source or build configuration changes,
it marks the previous result stale and offers a new run.

Tests do not run automatically on save by default. The learner may run checks from the
workbench or CLI; both paths produce the same state and live events.

## Failure experience

A failed run initially presents:

- one prioritized `Start here` contradiction;
- overall passed, failed, and pending counts;
- the smallest relevant input;
- expected and actual behavior;
- the associated contract;
- a focused rerun action;
- optional progressive help.

Other failures remain available but collapsed. Build failures, crashes, and foundational
contract failures take precedence over derivative mismatches. Pack authors may provide
priority information, but the engine must have sensible deterministic defaults.

Raw commands, fixtures, output, and contracts remain inspectable.

## Help ladder

Help is manual, progressive, local, private, and unpenalized.

1. **Observation:** make the current contradiction easier to see.
2. **Concept:** name the relevant engineering idea or invariant.
3. **Experiment:** suggest a small diagnostic action.
4. **Structure:** suggest an implementation shape without supplying the solution.
5. **Retrospective:** after completion, compare valid approaches and tradeoffs.

After repeated failures or prolonged inactivity, DeltaForge may quietly offer
`Want a nudge?`; it does not interrupt or reveal help automatically.

## Passing and progression

Passing a stage automatically makes the next capability available. It does not
automatically navigate away or begin the next task.

The pass moment communicates:

- capability acquired;
- verified behaviors;
- relevant code changes;
- attempts and elapsed work where meaningful;
- hint use privately;
- optional stage snapshot;
- preview of the next capability.

The learner explicitly selects `Begin next capability`. `deltaforge next` is not part
of the normal product journey.

The UI emphasizes capabilities while retaining stage numbers for orientation:

```text
Stage 4 of 14
Capability: Complete identifier retrieval
```

## Git behavior

New standalone projects initialize a Git repository after clear disclosure. DeltaForge
does not create a nested repository without consent.

When a stage passes, DeltaForge shows the relevant changes and offers a stage snapshot.
It does not auto-commit until the learner explicitly enables automatic snapshots. It
never rewrites, cleans, or discards learner changes.

## Resumption

A returning learner sees:

- project and current capability;
- last activity time;
- latest run and primary remaining failure;
- whether relevant files changed afterward;
- interrupted jobs or health problems;
- one primary next action.

The resumption target is useful work within thirty seconds.

## Performance-learning loop

Correctness unlocks the performance lab when the curriculum calls for it.

Before the first benchmark, DeltaForge asks for a short prediction. Recording one is
the primary path but remains skippable.

Each experiment contains structured core fields and optional prose:

```text
Question
Baseline
Prediction
Hypothesis
Change or variant
Measured result
Comparison
Interpretation
Next experiment
```

Required performance gates appear in the relevant capability contract. Optional quests
appear after correctness. The product distinguishes gates, quests, and learner-created
experiments rather than merging them into a single score.

## Completion tone

Completion is restrained, specific, and earned. DeltaForge may use thoughtful motion
and ceremony, but not confetti, coins, XP, streaks, or leaderboards.

The final state centers the learner's verified system, measurements, decisions, failed
experiments, and future questions.
