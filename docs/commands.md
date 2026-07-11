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
- `hint`: reveal progressive hints.
- `status`: show stage progress.
- `config show|validate`: inspect project config.
- `bench`: run pack benchmarks.
- `report`: generate Markdown or HTML reports.
- `portfolio`: generate a portfolio summary.
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
