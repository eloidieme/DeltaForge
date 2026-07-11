# Task 10 — Interview mode

*(Prepend `handoffs/00-shared-context.md`. Independent; reads benchmark history if present — build against the current history schema. If Task 07 landed, extend the MCP document enum as noted below.)*

## Objective

Implement Spec §25: `deltaforge interview` turns a completed (or in-progress) project into interview preparation — questions about the architecture, tradeoffs, and measured performance of what the learner actually built. Deterministic and local: questions are authored by packs and templated from recorded data, never invented by the tool.

## Design

### Authored questions (primary source)
- New optional per-stage file `interview.md`: a flat markdown list of questions (`- Why did you choose an inverted index?`). Parse list items; ignore headings/prose around them.
- Add `LoadedPack::interview_path` helper; `validate_pack` does not require the file; `pack doctor`/`diagnose_pack` may note stages lacking one as a quality hint (not a failure).

### Generated questions (secondary, deterministic templates)
- From pack topics (`manifest.topics`): a small fixed map of topic → 1–2 generic questions (e.g. `concurrency` → "Where does your implementation synchronize, and what did it cost?"; `storage` → "What happens to your data format when the process is killed mid-write?"). Unknown topics produce nothing.
- From benchmark history, when present: templated questions using real numbers — e.g. with a threads matrix: "Your speedup from 1 to 8 threads was 4.0x. What prevented it from being 8x?"; with peak memory: "Peak memory was 412 MB on the medium fixture. What dominates it?". Only emit questions whose data exists.

### Command
- `deltaforge interview [--stage <id>] [--all] [--output <path>] [--json]`
  - Default scope: completed stages plus the current stage. `--stage` limits to one; `--all` includes locked stages' authored questions too (useful for pack authors reviewing content).
  - Human output: numbered questions grouped under stage headings (Spec §25 example), rendered via `Terminal`.
  - `--output`: write the same content as markdown (via `atomic_write`); `--json`: structured `{stage, source: authored|topic|benchmark, question}` list, JSON only on stdout.
- Ordering is deterministic: stage order, then authored → topic → benchmark within a stage (topic questions attach to the project, not a stage — put them in a final "Project-wide" group).

### Content
- Write real `interview.md` files for **flashindex** (every stage) and **minikv** (every stage) — questions should probe the tradeoff each stage teaches, in the style of Spec §25 (why this structure, what breaks it, how would you extend it). tinyhttp/byteforgevm can follow in a content pass; note it in CHANGELOG.
- Bump versions of packs that gained files.

### Integration points
- Surface in `docs/commands.md` and mention in `docs/quickstart.md` as the final step of a project.
- If Task 07 (MCP read tools) has landed, add `interview` to the `read_stage_document`/`write_stage_document` document enums (both directions) so agents can author it; if not, leave a TODO note in the code where the enum lives.

## Acceptance criteria
- Integration tests: project with one completed stage → authored questions for it appear and locked-stage questions don't (without `--all`); benchmark-derived question appears only after seeded history exists; `--json` and `--output` work; deterministic ordering (run twice, identical output).
- Packs without any `interview.md` produce topic/benchmark questions only, with no errors.
- `validate-pack --strict` clean; full quality bar passes.

## Out of scope
AI-generated question synthesis (Spec §19), spaced-repetition/answer tracking, exporting to Anki or similar.
