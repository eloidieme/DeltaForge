# Commands

- `list`: show discovered project packs.
- `pack list|show|install`: inspect or copy discovered packs.
- `pack new`: scaffold a local pack.
- `pack add-stage`: add a scaffold stage to a pack.
- `pack doctor`: report authoring quality gaps.
- `pack check-reference`: prove a pack with a reference solution.
- `init <pack> --lang <language>`: create a learner repo.
- `overview`: show what the project is, why it matters, and the full stage roadmap.
- `instructions`: show current stage instructions.
- `test`: run black-box stage tests.
- `explain-failure`: summarize the last failed test run and suggest next steps.
- `next`: unlock the next stage after tests pass.
- `sync-pack`: re-pin the project to the currently discovered pack after a pack upgrade (updates the pinned version, source, and digest, and migrates the pack digest inside existing completion proofs). Supports `--json`.
- `hint`: reveal progressive hints. `--level N` never lowers previously recorded progress.
- `status`: show stage progress. Supports `--json` (project, language, current stage, and per-stage status on stdout only).
- `config show|validate`: inspect project config.
- `bench`: run pack benchmarks. Timing uses the pack's `bench_run` command (falling back to `run`).
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

- `overview` and `instructions` open in `$PAGER` when run in an interactive terminal, defaulting to `less -R`.
- Set `DELTAFORGE_NO_PAGER=1` to print directly.
- Piped or redirected output never uses a pager.
- When a test fails outside `--json` mode, the runner prints the program's actual stdout (and stderr if non-empty) beneath the failure, truncated to the first 30 lines / 2000 characters. Use `--verbose` for full output.
- `list`, `doctor`, and `validate-pack` tolerate a single malformed pack in a search directory: `list` warns on stderr and still lists the valid packs, `doctor` reports the broken pack, and `validate-pack` reports it and exits non-zero.

Pack pinning and upgrades:

- A learner project pins the pack it was created from. Bundled/embedded packs are pinned logically as `"bundled"`, while external `--packs-dir` packs are pinned by absolute path.
- After upgrading DeltaForge or editing a pinned pack, a pin mismatch is reported with `deltaforge sync-pack` as the remedy; running it re-pins the project without discarding progress.
