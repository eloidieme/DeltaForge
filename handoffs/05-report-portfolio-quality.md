# Task 05 — Data-driven reports and portfolio

*(Prepend `handoffs/00-shared-context.md`. Benefits from Task 01's richer benchmark history (matrix points, peak memory) — check `git log` and build against whatever history schema is current.)*

## Objective

`deltaforge portfolio` currently emits canned boilerplate (static "Architecture Highlights" strings in `src/commands/portfolio.rs`) and the HTML report is escaped markdown inside a `<pre>` tag (`src/commands/report.rs::render_html`). Make both artifacts earn their place: everything they claim must come from real recorded data, and the HTML report should look like something a learner would actually show someone (Spec §10.10, §24).

## Requirements

### Portfolio (`src/commands/portfolio.rs`)
- Remove all canned filler. Derive every section from data:
  - **Summary**: pack name/description, language, stages completed X/Y with date range (from `completed_stage_timestamps`).
  - **Features**: completed stage titles, phrased from the stage list (as today, but only this — no static architecture claims).
  - **Performance**: per benchmark (grouped by name), first recorded run vs best run: "reduced median from 812 ms to 143 ms (−82%)", throughput and peak-memory equivalents when present, speedup line when a threads matrix exists. Omit the section entirely when history is empty rather than printing placeholder text.
  - **Design notes**: for each stage with a note in `.deltaforge/design_notes/`, include the first paragraph as an excerpt.
- Add `--json` (structured version of the same data; JSON only on stdout).

### Report (`src/commands/report.rs`)
- **Markdown**: add a benchmark-trend section per benchmark name: runs count, first/best/latest median, delta %; keep existing sections.
- **HTML**: real rendering, not `<pre>`. Requirements:
  - Self-contained single file: inline CSS, no external requests, no JS frameworks (vanilla JS optional but not required).
  - Proper headings, the stage-progress list, test-result and benchmark tables as real `<table>`s.
  - Benchmark history charts as **hand-rolled inline SVG** (one small line/bar chart per benchmark showing median over runs; axis labels and values; no chart library). Keep it simple and legible in both light and dark contexts (neutral colors).
  - HTML-escape all interpolated strings (an `html_escape` helper already exists — use it everywhere user/pack content is interpolated).
- **JSON**: extend additively only (existing keys keep their shape).

### Shared
- Factor the "group history by benchmark, compute first/best/latest/delta" logic into one place used by both commands (a small module, e.g. `src/commands/bench.rs` helper or a new `src/report_data.rs` — your call, keep it tidy).
- Defaults for `--output` may already exist from the foundation pass (`report.md` / `PORTFOLIO.md`); if not, add them.

## Acceptance criteria
- Integration test: init a temp project, complete a stage, save two benchmark runs, generate portfolio + all three report formats; assert the portfolio contains a computed delta line and no canned "Architecture Highlights" text; assert the HTML contains `<svg` and a benchmark table and no `<pre>`-wrapped markdown; assert JSON report still parses with the previously existing keys.
- A generated HTML report opened locally renders sanely (verify manually once; describe what you saw in the final report).
- Empty-data cases (no history, no notes, no completed stages) produce clean output with no placeholder filler and no crashes.
- Full quality bar passes.

## Out of scope
Quest/gate/interview sections in reports (their tasks extend these artifacts when they land), the local web dashboard (Spec §26 — explicitly deferred).
