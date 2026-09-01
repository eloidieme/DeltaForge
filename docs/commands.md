# Commands

The browser workbench is the canonical learner surface; every command below is the same
operation reached from the terminal instead. A run started from either surface is
indistinguishable to project state and to the event stream.

- `list`: show discovered project packs.
- `pack list|show|install`: inspect or copy discovered packs.
- `pack new`: scaffold a local pack.
- `pack add-stage`: add a scaffold stage to a pack.
- `pack doctor`: report authoring quality gaps, including missing edge-case/non-goal sections, fewer than three hints, and fewer than two tests.
- `pack check-reference`: prove a pack with a reference solution.
- `pack content <pack> [--stage <id>]`: print exactly the content a learner can see for a stage — goal, background, requirements, example, expected behavior, edge cases, exclusions, and the help ladder. No tests, no fixtures, no reference solution. This is the fixture for the content-sufficiency practice in [product/content-sufficiency.md](product/content-sufficiency.md). Supports `--json`.
- `init <pack> --lang <language>`: create a learner repo. This is the automation form of the browser creation flow; both share one engine, so a scripted project and a created one are identical. Unlike the browser, `init` may write anywhere the shell can reach.
- bare `deltaforge`: start or focus the local project workbench, the canonical learner entry.
- `overview`: print the big picture, progress, and full stage roadmap as a terminal diagnostic. Supports `--json`.
- `instructions`: print the current stage instructions, a stage selected with `--stage`, or every stage with `--all`.
- `test`: run black-box stage tests and stream bounded human-readable evidence in the terminal. Supports `--json` for automation.
- `explain-failure`: summarize the last failed test run and suggest next steps.
- `next`: unlock the next stage after tests pass. Gate-bearing stages also require a current passing performance record, unless `[gates] enforce = false`; the latter prints `performance gates skipped: gates.enforce = false` without claiming a pass. The refusal names what is missing rather than a command, because the measurement is reachable from either surface.
- `sync-pack`: adopt the currently discovered pack after a pack upgrade. Updates only the project-level pin (version, source, digest); completion proofs keep the digests of what actually passed. Reports each completed stage as valid or needing revalidation. Supports `--json`.
- `hint`: reveal progressive hints. `--level N` never lowers previously recorded progress.
- `status`: show stage progress. Completed stages whose tests, fixtures, or commands changed since they passed are marked `!` (needs revalidation). Gate-bearing stages include performance `passed`, `not yet`, or `not measured`. Supports `--json` (project, language, current stage, and per-stage status on stdout only).
- `config show|validate`: inspect project config.
- `bench`: run pack benchmarks. Timing uses the pack's `bench_run` command (falling back to `run`). Human output is an aligned table per benchmark, one row per matrix point, e.g.:

  ```
  01_scan_files / scan_parallel
    params      median        p95   throughput  peak mem
    threads=1  812.40 ms  845.10 ms   61.52 MB/s  118.2 MB
    threads=8  102.75 ms  110.90 ms  486.40 MB/s  131.7 MB
    speedup_1_to_8: 7.91x
  ```

  The `speedup_<min>_to_<max>` line is derived at display time when a numeric `threads` matrix parameter is the only varying parameter (median at min threads / median at max threads); `--json` attaches it per record as a `derived` object and emits JSON only. `--save` appends to `.deltaforge/benchmark_history.json`; derived metrics are never persisted.

  `--compare` compares each point from the new run with the most recent prior saved point having the same project, language, stage, benchmark, and exact parameters. It reports median, throughput, and peak-memory deltas with improved/regressed wording. Run once with `--save`, then run again with `--compare` (optionally also `--save`). The comparison uses the history as it existed before the new run is appended, and warns when no matching prior point exists. If the saved and current OS or architecture differ, the output notes that the measurements may not be directly comparable. With `--json`, comparison details are attached to each output record under `comparison` and no human text is written to stdout; comparisons are never persisted.
- `report`: write the engineering record. Every line traces to a completion proof, a saved measurement, a gate result, a commit, or a note the learner wrote; a project with no measurements makes no performance claim. `--format markdown|html|json`, `--output` defaults to `report.md`. The browser export writes the same document to `deltaforge-report.<ext>` in the project, a name excluded from the project digest so exporting cannot invalidate the proof it describes.
- `portfolio`: write a short narrative from the same recorded evidence. `--output` defaults to `PORTFOLIO.md`.
- `design`: show prompts or edit design notes.
- `commit`: snapshot the current step — a commit, and a tag when `[git] auto_tag` is on. A tag that already exists is reported and left alone rather than treated as an error, so a second snapshot of the same step is a legitimate action. Refuses unless the step has passed, or `--force`.
- `validate-pack`: validate pack structure. Use `--strict` for authoring quality checks.
- `doctor`: check local tools, discovered packs, and optional project context. The tools it checks come from what the discovered packs declare in `languages.<id>.requires`, plus Git, which DeltaForge itself uses for snapshots. It also reports where the browser creation flow puts new projects.

Global flags:

- `--project-dir <path>` selects a learner project explicitly.
- `--packs-dir <path>` overrides pack discovery for one invocation.

Display behavior:

- Bare `deltaforge` opens the workbench. Set `DELTAFORGE_NO_BROWSER=1` to print its local URL instead of launching the system browser.
- `overview`, `instructions`, and `test` remain explicit terminal diagnostics and never generate or open HTML pages. `overview --json` and `test --json` remain machine-readable.
- The terminal renderer may open `$PAGER`, defaulting to `less -R`. Set `DELTAFORGE_NO_PAGER=1` to print that view directly.
- Human-readable `test` output keeps the first contradiction actionable with bounded actual stdout; use `--verbose` for complete captured streams. The same run state and structured evidence remain available in the workbench without a generated report or manual refresh.
- `list`, `doctor`, and `validate-pack` tolerate a single malformed pack in a search directory: `list` warns on stderr and still lists the valid packs, `doctor` reports the broken pack, and `validate-pack` reports it and exits non-zero.

Pack pinning and upgrades:

- A learner project pins the pack it was created from. Bundled/embedded packs are pinned logically as `"bundled"`, while external `--packs-dir` packs are pinned by absolute path.
- After upgrading DeltaForge or editing a pinned pack, a pin mismatch is reported with `deltaforge sync-pack` as the remedy; running it re-pins the project without discarding progress.
- Each completion proof records a per-stage behavioral digest: the stage's `tests.yaml`, its fixtures, and the language build/run commands. Documentation-only pack updates (instructions, hints, README, design prompts) therefore never invalidate completed stages; changes to tests, fixtures, or commands invalidate only the affected stages, and `next`/`commit` require re-running `deltaforge test` for them.
- Proofs recorded by older DeltaForge versions carry no behavioral digest. `sync-pack` upgrades them automatically when the pack is bit-identical to the one that passed; otherwise the stage must be revalidated.
