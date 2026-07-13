# Commands

- `list`: show discovered project packs.
- `pack list|show|install`: inspect or copy discovered packs.
- `pack new`: scaffold a local pack.
- `pack add-stage`: add a scaffold stage to a pack.
- `pack doctor`: report authoring quality gaps, including missing edge-case/non-goal sections, fewer than three hints, and fewer than two tests.
- `pack check-reference`: prove a pack with a reference solution.
- `init <pack> --lang <language>`: create a learner repo.
- `overview`: open the local learning page at the project overview, including the big picture, progress, and full stage roadmap.
- `instructions`: open the local learning page at the current stage (or a stage selected with `--stage`).
- `test`: run black-box stage tests. Failed human-readable runs generate `.deltaforge/ui/test-report.html`; interactive runs open it automatically.
- `explain-failure`: summarize the last failed test run and suggest next steps.
- `next`: unlock the next stage after tests pass. Gate-bearing stages also require a current passing performance record from `deltaforge bench`, unless `[gates] enforce = false`; the latter prints `performance gates skipped: gates.enforce = false` without claiming a pass.
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
- `report`: generate Markdown or HTML reports. `--output` defaults to `report.md`.
- `portfolio`: generate a portfolio summary. `--output` defaults to `PORTFOLIO.md`.
- `design`: show prompts or edit design notes.
- `commit`: create a stage-aware git commit.
- `validate-pack`: validate pack structure. Use `--strict` for authoring quality checks.
- `doctor`: check local tools, discovered packs, and optional project context.

Global flags:

- `--project-dir <path>` selects a learner project explicitly.
- `--packs-dir <path>` overrides pack discovery for one invocation.

Display behavior:

- In an interactive terminal, `overview` and `instructions` generate `.deltaforge/ui/learning.html` and open it in the system browser. The page is self-contained: it loads no scripts, fonts, styles, or learner content from the network.
- The learning page keeps the complete pack available in a stage sidebar, but presents the current task in smaller tabs: task, example, rationale, and reference. It also shows completed/current/upcoming status and previews the neighboring stages.
- Pass `--terminal` to use the original terminal renderer. Pass `--no-open` to generate the page and print its path without launching a browser; this is useful in remote environments and automated tests.
- Set `DELTAFORGE_NO_BROWSER=1` to make browser-capable commands use their terminal view by default. Piped or redirected commands also stay in the terminal so scripts do not launch a GUI. `overview --json` remains JSON-only.
- The terminal renderer may open `$PAGER`, defaulting to `less -R`. Set `DELTAFORGE_NO_PAGER=1` to print that view directly.
- In an interactive terminal, `test` keeps its live output compact and opens a failure-first browser report when the run fails. The self-contained report groups failures before passes, presents structured expected/actual comparisons, exposes stdout and stderr separately, can reveal spaces/tabs/line endings, links back to the stage instructions, and provides a copyable filtered rerun command.
- Each reported test includes a **Test input** view with the sanitized command, working directory, timeout, declared environment, standard input, and a browsable snapshot of the fresh fixture tree. Text files can be expanded in place; binary files receive a bounded hexadecimal preview. Large fixture trees and file contents are capped so reports remain responsive.
- Pass `test --open` to generate and open the report even when every test passes. Pass `test --terminal` to keep detailed diagnostics in the terminal and skip report generation. `DELTAFORGE_NO_BROWSER=1` suppresses automatic report generation and browser launches; an explicit `test --open` still generates the report and prints its path. `test --json` remains JSON-only and never launches a browser.
- When browser reporting is unavailable because output is redirected, a failed run still generates the report and prints its path while retaining detailed terminal diagnostics. Program output in the terminal is truncated to the first 30 lines / 2000 characters; use `--verbose` for full output.
- `list`, `doctor`, and `validate-pack` tolerate a single malformed pack in a search directory: `list` warns on stderr and still lists the valid packs, `doctor` reports the broken pack, and `validate-pack` reports it and exits non-zero.

Pack pinning and upgrades:

- A learner project pins the pack it was created from. Bundled/embedded packs are pinned logically as `"bundled"`, while external `--packs-dir` packs are pinned by absolute path.
- After upgrading DeltaForge or editing a pinned pack, a pin mismatch is reported with `deltaforge sync-pack` as the remedy; running it re-pins the project without discarding progress.
- Each completion proof records a per-stage behavioral digest: the stage's `tests.yaml`, its fixtures, and the language build/run commands. Documentation-only pack updates (instructions, hints, README, design prompts) therefore never invalidate completed stages; changes to tests, fixtures, or commands invalidate only the affected stages, and `next`/`commit` require re-running `deltaforge test` for them.
- Proofs recorded by older DeltaForge versions carry no behavioral digest. `sync-pack` upgrades them automatically when the pack is bit-identical to the one that passed; otherwise the stage must be revalidated.
