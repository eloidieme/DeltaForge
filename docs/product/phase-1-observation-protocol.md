# Phase 1 external learner observation protocol

Status: **Ready for participant scheduling**

Prepared: 2026-07-16

## Purpose

This protocol validates the Phase 1 FlashIndex Stage 1 learner loop with at least five
people from DeltaForge's primary audience. It operationalizes the research measures in
`vision.md` and `phase-1-vertical-slice.md`; it does not amend them.

The study asks whether a learner can:

1. enter through bare `deltaforge` and orient to the current mission;
2. run the initial behavioral checks;
3. explain the prioritized contradiction before receiving help;
4. find the learner-owned source and make progress in an external editor;
5. notice that an edit made prior evidence stale;
6. run at least one check through the CLI and understand that the browser shows the
   same run;
7. complete Stage 1 and explain the acquired capability;
8. close everything and resume useful work later;
9. complete the loop without learning about services, processes, ports, generated
   pages, or refresh mechanics.

This is a formative product study with a small sample, not a statistically powered
experiment. Exact behavior, timings, questions, and breakdowns matter more than an
average satisfaction score.

## Participants

Run five valid sessions, labeled `P01` through `P05`.

### Inclusion criteria

Each participant must:

- be comfortable using a terminal and an external code editor;
- understand basic Git usage;
- be able to build and debug an ordinary Rust program;
- have a working Rust toolchain on the study machine;
- identify as either an advanced beginner moving toward intermediate engineering or
  an intermediate engineer entering a new systems domain or language;
- have no prior exposure to the Phase 1 DeltaForge workbench or the FlashIndex Stage 1
  solution.

Aim for at least two advanced beginners and at least two intermediate engineers. The
fifth participant may be from either group. Include macOS, Windows, and Linux across the
sample when recruiting makes that practical, but do not treat five-person research as
a substitute for the supported-platform release audit.

### Exclusion criteria

Exclude:

- absolute programming beginners;
- DeltaForge contributors or anyone who has inspected this implementation branch;
- anyone who has seen the FlashIndex reference implementation or Stage 1 tests;
- sessions with a broken language toolchain or study-machine failure that prevents the
  product journey from beginning. Fix the environment and rerun that participant; do
  not count the failed setup as one of the five valid sessions.

## Ethics, privacy, and study data

Before starting, tell the participant:

- this is a product evaluation, not an evaluation of their programming ability;
- they may stop at any time and may decline screen, audio, or video recording;
- DeltaForge works locally and the study does not require uploading their source;
- notes use only the participant ID and experience band;
- verbatim quotes may be retained without identifying details;
- recordings, if separately consented to, follow the owner's stated retention policy.

Never place names, email addresses, employer names, access tokens, home-directory
names, or other personal identifiers in the study sheet or issue tracker.

## Study shape

Each participant completes two sessions.

### Session A: fresh-project loop

- Duration: 60–90 minutes.
- Start: a fresh prepared FlashIndex Rust project, no DeltaForge browser tab, and a
  terminal open in the project directory.
- End: Stage 1 is complete and the workbench offers `Begin next capability`, or the
  session reaches a genuine blocker or the 90-minute cap.

### Session B: cold resumption

- Duration: 10–15 minutes.
- Schedule: 24–72 hours after Session A.
- Start: all prior DeltaForge browser tabs and terminals closed.
- End: the participant has reopened DeltaForge and stated the restored mission,
  evidence, and next action.

Do not rehearse the relaunch command immediately before Session B.

## Facilitator setup

Use the exact candidate build being evaluated. Record its commit or worktree identity,
DeltaForge version, OS, browser, editor, terminal, and Rust versions.

For each participant:

1. verify `cargo --version` and the candidate `deltaforge --version` before the timed
   session;
2. initialize a uniquely named fresh FlashIndex Rust project with default learner
   content and no prior state;
3. verify the project contains the starter implementation, then close it without
   running checks or opening the workbench;
4. put the terminal in the project directory;
5. ensure the editor is installed but the project is not already open;
6. close every DeltaForge browser tab;
7. prepare a visible timer and a fresh copy of the recording sheet;
8. disable notifications that could expose participant or facilitator information.

Do not pre-open the mission, run `deltaforge test`, reveal help, edit learner source,
or leave a workbench service running for that project.

If the candidate cannot create a genuinely fresh project without facilitator repair,
record a product blocker instead of silently repairing learner-visible state.

## Moderator rules

### Observe without teaching

- Ask the participant to think aloud, but do not require constant narration.
- Do not point at controls, name the primary action, supply a command, explain the
  interface, interpret the diagnosis, suggest a code structure, or reveal a hint.
- Do not say `browser`, `service`, `server`, `port`, `refresh`, `state`, or `stale`
  before the participant independently encounters or mentions the concept.
- Do not celebrate a correct path or signal that a choice is wrong.
- Allow normal mistakes, backtracking, use of `--help`, and use of product-authored
  project documentation.
- Before the initial run, record any attempt to consult external documentation; do not
  prevent it. After the initial failure, normal language documentation and search are
  allowed, but record their use. Do not allow access to DeltaForge implementation,
  tests, reference solutions, or another participant's project.
- Never repair, refresh, restart, or rerun on the participant's behalf.

### Intervention ladder

Remain silent unless the participant has made no meaningful progress for three minutes
or explicitly asks the moderator for help. Use at most the next unused prompt:

1. `What are you looking for right now?`
2. `What would you try next?`
3. `Please reread the task in your own words.`

Only provide technical rescue for a confirmed study-environment failure unrelated to
the candidate. Record the exact intervention, timestamp, reason, and resulting action.
Any product-navigation or service-model explanation makes the individual session fail
the corresponding criterion even if the learner later completes the code.

### Neutral checkpoint questions

Ask only at the specified milestones:

- Before the initial run: `What are you trying to make true, and what would you do
  next?`
- After the first failed run, before any hint: `What happened, and what would you work
  on first?`
- After a source edit, only after the participant has acted on or clearly missed the
  changed-evidence presentation: `What evidence do you have for the current code?`
- After a CLI run: `How does what you see here relate to the run in the terminal?`
- At capability acquisition: `What can your program do now, and what would you do
  next?`
- During Session B: `Where are you, what evidence do you have, and what would you do
  next?`

Record the answer verbatim or as a close paraphrase before asking a follow-up.

## Participant briefing

Read this script verbatim at the start of Session A:

> This is a product evaluation, not a test of you. You have a prepared local project
> called FlashIndex. Use DeltaForge to understand and complete its first capability.
> Work as you normally would and think aloud when you can. You may use your editor,
> terminal, and the guidance DeltaForge provides. I will mostly observe and may not
> answer product questions during the task. Tell me when you believe the first
> capability is complete.

Then say:

> Begin from this terminal when you are ready.

Do not mention a command unless the participant cannot begin after three minutes and
the intervention ladder has been exhausted; such a rescue is recorded as a failed
activation.

## Session A procedure and measures

Start the timer when the participant receives control.

### 1. Entry and orientation

Observe how the participant enters DeltaForge. Record:

- time to the first visible current mission;
- whether they use bare `deltaforge` without moderator instruction;
- any duplicate tab, browser-focus, loading, or terminal-return confusion;
- their answer to the pre-run mission and next-action question.

### 2. Initial behavioral run

Record:

- time to initiating the first run;
- whether it occurs within five minutes;
- whether external documentation was consulted first;
- whether live progress is noticed;
- any attempt to refresh, restart, find a port, or start a server.

### 3. Failure comprehension

After the failed run settles and before the participant reveals help, ask the neutral
failure question. Record:

- the contradiction they state;
- whether it matches the `Start here` diagnosis;
- whether they distinguish the primary failure from the total failure count;
- which evidence they use: fixture, expected, observed, contract, or raw output;
- whether help is revealed, at which level, and why;
- whether they know where to write code without moderator direction.

Do not require the participant to use every help level. The protocol evaluates whether
the ladder supports productive struggle, not whether people maximize hint usage.

### 4. Editing and freshness

Allow the participant to open the project and edit normally. After the first relevant
save, record:

- whether they notice that the prior result is no longer current;
- time from save to noticing the change;
- what they believe was preserved;
- whether they expect checks to run automatically;
- any manual refresh or service question.

If the participant starts a browser run immediately, let it proceed and record the
behavior. The CLI observation task can be introduced after the next relevant edit.

### 5. CLI/browser agreement task

At the first natural rerun point after a source edit, provide this task card verbatim:

> For this check only, start the run from the terminal. Use the product's available
> guidance if you need it.

Do not provide the command. Record:

- time to find and start the CLI check;
- command-discovery path;
- whether the workbench shows the CLI-started job without reload;
- the participant's answer to the CLI/browser relationship question;
- any belief that the CLI and browser have separate results or progression models.

The participant may use either surface for later runs.

### 6. Completion

Continue until Stage 1 passes or the session stops. Record:

- total time to completion;
- number of full and focused runs if observable;
- help levels revealed;
- moderator interventions;
- whether `Capability acquired` is noticed;
- the participant's explanation of the acquired capability;
- whether `Begin next capability` is identified as the next action;
- any automatic-navigation expectation or use of `deltaforge next`.

At completion, ask the participant to close the DeltaForge browser tab and terminal as
they normally would. Do not ask them to stop a service. Record any attempt or question
about stopping one.

If the participant has not completed after 90 minutes, stop the task, record the exact
state and blocker, and continue with the post-session questions. Schedule Session B
only when there is meaningful state to resume; mark the end-to-end criterion failed.

## Session B procedure and measures

Read this script verbatim:

> Please continue the FlashIndex work where you left it. Work as you normally would
> and tell me when you know what to do next.

Start the timer when the participant receives control. Record:

- command or action used to return;
- time until the restored workbench is visible;
- time until the participant states a useful next action;
- whether useful work is resumed within thirty seconds;
- whether the latest evidence and its freshness are understood;
- whether the current capability and completion are understood;
- whether a run starts automatically;
- any refresh, server, service, process, port, static-page, or shutdown question;
- the answer to the Session B orientation question.

End after the participant identifies and, if they wish, begins the correct next action.
Do not require Stage 2 work.

## Post-session questions

Ask after Session B so the questions do not prime the main journey:

1. `At the beginning, what did you think DeltaForge was responsible for?`
2. `Which part of the first failure was most useful?`
3. `Was any evidence unclear or missing?`
4. `How did you decide where to make the code change?`
5. `What did the changed-result state mean to you?`
6. `Did the terminal run and workbench feel like one system or two? Why?`
7. `What capability did you acquire?`
8. `When you returned, what helped you continue?`
9. `What, if anything, did you expect to refresh, start, or stop?`
10. `What was the most confusing moment?`

Do not ask for a numeric satisfaction score. If desired, end with `What would you
change first?`

## Per-participant recording sheet

Copy this section once for each participant.

### Participant and environment

| Field | Value |
|---|---|
| Participant ID | P0_ |
| Experience band | Advanced beginner / Intermediate |
| Relevant language experience |  |
| OS and version |  |
| Browser and version |  |
| Editor and version |  |
| Terminal |  |
| Rust version |  |
| DeltaForge candidate identity |  |
| Session A date |  |
| Session B date |  |
| Screen/audio recording consent | Yes / No |

### Milestone measures

Use `MM:SS` from the start of the relevant session. Use `N/O` for not observed and
`N/A` only when the milestone genuinely does not apply.

| Measure | Result | Pass | Evidence or quote |
|---|---:|:---:|---|
| Current mission visible |  |  |  |
| Initial run initiated |  |  |  |
| Initial run within 05:00 without external docs |  | Yes / No |  |
| Mission stated correctly before initial run |  | Yes / No |  |
| Next action stated correctly before initial run |  | Yes / No |  |
| Primary contradiction explained before hint |  | Yes / No |  |
| Source location found without moderator direction |  | Yes / No |  |
| Stale evidence noticed after save |  | Yes / No |  |
| Time from save to stale-state recognition |  |  |  |
| CLI check started without supplied command |  | Yes / No |  |
| CLI/browser shared-run model understood |  | Yes / No |  |
| Stage 1 completed |  | Yes / No |  |
| Acquired capability explained correctly |  | Yes / No |  |
| Next capability action identified |  | Yes / No |  |
| Useful work resumed in Session B |  |  |  |
| Resume within 00:30 |  | Yes / No |  |
| Mission/evidence/next action correct on return |  | Yes / No |  |
| No service-model question or explanation required |  | Yes / No |  |
| No manual page refresh required |  | Yes / No |  |
| End-to-end individual protocol pass |  | Yes / No |  |

### Event log

| Time | Product state | Learner action or quote | Moderator intervention | Issue ID |
|---:|---|---|---|---|
|  |  |  |  |  |

### Help and run summary

| Field | Result |
|---|---|
| Full runs observed |  |
| Focused runs observed |  |
| Highest help level revealed before pass |  |
| Retrospective opened after pass | Yes / No |
| Product-navigation interventions |  |
| Environment-only interventions |  |
| External documentation used |  |
| Completion blocker, if any |  |

### Session assessment

- Strongest evidence the product model was understood:
- Strongest evidence of confusion:
- Exact service/process/port/refresh question, if any:
- Primary issue severity:
- Individual pass/fail rationale:

## Issue log and severity

Create one issue per distinct product problem. Link every observation by participant ID
and timestamp; do not merge different root causes because they look visually similar.

| Severity | Definition | Examples | Gate effect |
|---|---|---|---|
| S0 — Safety | Data loss, execution outside the project boundary, privacy breach, or security-boundary failure. | Learner files overwritten unexpectedly; repository content exposed through the service. | Stop the study and block release. |
| S1 — Blocker | A target learner cannot complete or resume the core loop, or must understand the service model or manually repair product state. | Workbench cannot open; run never settles; evidence is lost; participant must find a port, restart a service, or refresh to continue. | One occurrence blocks Phase 1. |
| S2 — Major | The loop completes only with moderator product guidance, or a core state is materially misunderstood. | Primary contradiction is unreadable; source location cannot be found; CLI/browser results appear separate; capability or next action is wrong. | Repeated by two participants blocks Phase 1; one requires fix or explicit retest decision. |
| S3 — Friction | Recoverable confusion or inefficiency that does not corrupt the product model or require moderator guidance. | A label causes hesitation; secondary evidence is hard to find; a task takes an unnecessary extra step. | Triage before release; cluster repeated patterns. |
| S4 — Polish | Cosmetic or preference-level feedback with no observed task impact. | Spacing, wording preference, non-blocking animation criticism. | Does not block Phase 1. |

Classify the observed impact, not the facilitator's guess about implementation effort.
Environment failures and participant-specific coding mistakes are not product issues
unless DeltaForge caused, obscured, or failed to recover from them.

## Individual and aggregate pass criteria

### Individual protocol pass

A participant passes only when they:

- initiate the first behavioral run within five minutes without external documentation
  or moderator command instruction;
- correctly state the current mission and primary contradiction before a hint;
- find where to edit without moderator product guidance;
- understand that edited source makes prior evidence stale;
- start the required CLI check without being supplied the command and understand that
  the browser and CLI show one shared run;
- complete Stage 1 and correctly explain the acquired capability and next action;
- resume useful work within thirty seconds during Session B;
- correctly state the restored mission, evidence, and next action;
- never require a service/process/port explanation or a manual refresh.

### Phase 1 research gate

The observation gate passes only when:

- five valid participants complete both sessions;
- all five complete the end-to-end loop;
- all five avoid service-model explanations and manual-refresh recovery;
- all five correctly identify mission, evidence, and next action on return;
- at least four of five meet every other individual timing and comprehension criterion;
- no S0 or S1 issue occurs;
- no S2 issue is observed in two or more participants;
- every S2 and repeated S3 has an owner and a written fix, retest, or defer decision.

Do not average away a blocker. If the gate fails, fix the smallest coherent product
cause and rerun affected scenarios with new target participants until five valid
post-fix sessions satisfy the gate.

## Aggregate synthesis sheet

| Measure | P01 | P02 | P03 | P04 | P05 | Gate |
|---|---:|---:|---:|---:|---:|---|
| Initial run time |  |  |  |  |  | At least 4/5 <= 05:00 |
| Primary failure understood |  |  |  |  |  | At least 4/5 |
| Source location found |  |  |  |  |  | At least 4/5 |
| Stale state noticed |  |  |  |  |  | At least 4/5 |
| Shared CLI/browser run understood |  |  |  |  |  | At least 4/5 |
| Capability explained |  |  |  |  |  | At least 4/5 |
| End-to-end completion |  |  |  |  |  | 5/5 |
| Resume time |  |  |  |  |  | At least 4/5 <= 00:30 |
| Return orientation correct |  |  |  |  |  | 5/5 |
| Service-model question/explanation |  |  |  |  |  | 0/5 |
| Manual refresh required |  |  |  |  |  | 0/5 |
| Individual protocol pass |  |  |  |  |  | At least 4/5 |

Report medians and ranges for time measures, plus each individual result. With five
participants, do not report percentages without the underlying count and do not claim
statistical significance.

## Final research report outline

1. Candidate identity and study dates
2. Participant composition and environment matrix
3. Gate result: pass or fail
4. Milestone results with individual values, median, and range
5. Exact service-model and refresh observations
6. Mission, failure, stale-state, CLI/browser, capability, and resumption comprehension
7. Issue table by severity and participant
8. Product changes made between rounds
9. Retest evidence
10. Decision: complete Phase 1 observation gate or continue iteration

The report must distinguish observed behavior, participant interpretation, moderator
inference, and implementation diagnosis.
