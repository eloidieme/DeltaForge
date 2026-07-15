# DeltaForge — current state (July 2026)

*(Snapshot prompt: read this before any new task for the up-to-date picture. `00-shared-context.md` still describes the core principles and source layout — this file records what has shipped since and what is open. Written 2026-07-15.)*

DeltaForge is a local, CodeCrafters-style learning framework written in Rust (~14k lines,
lean deps: clap/serde/anyhow/regex/cap-std, no async runtime in the main binary). Learners
init a project from a "pack", implement a CLI program stage by stage against black-box
tests, and progress with `deltaforge next`. Everything runs locally; bundled packs are
embedded in the binary via include_dir. Repo: github.com/eloidieme/DeltaForge (single
author, direct commits to main).

## CLI surface

list, pack (list/show/new/add-stage/doctor/check-reference/install), init, validate-pack,
instructions, overview, test, next, sync-pack, status, hint, config, bench, report,
portfolio, design, commit, doctor, explain-failure, serve. Plus `deltaforge-pack-mcp`, a
stdio MCP server for AI-assisted pack authoring (structured ok/blocked reports).

## Core mechanics

- Tests: per-stage `tests.yaml` (exit_code, stdout_exact/contains/not_contains,
  stderr_contains, regex_match, file_exists/contains/not_contains, json_equals, stdin,
  env, timeout). Fixtures are copied to temp dirs; commands run without a shell.
- Completion proofs pin a per-stage behavioral digest (tests.yaml + fixtures + build/run
  commands); changing tests intentionally stales proofs; `sync-pack` re-pins and reports
  which stages need revalidation. `commit` creates "Complete Stage NN: Title" commits and
  `deltaforge-<stage_id>` tags; git.auto_commit/auto_tag in config.
- Benchmarks: schema-v2 history, parameter matrices (e.g. threads [1,2,4,8]), median/p95,
  throughput, best-effort peak memory, derived speedup, digest-pinned performance gates
  (flashindex requires speedup_1_to_8 >= 1.5), bench --compare.
- Pack authoring proof: `pack doctor` (structure/quality) and `pack check-reference`
  (runs an internal reference solution through every stage). IMPORTANT: check-reference
  runs the FINAL solution against ALL stages, so early-stage tests must be
  forward-compatible with later behavior (e.g. stage-01 scan fixtures may only contain
  corpus-extension files because the final binary filters).

## Packs (all bundled, "V2" editorial baseline)

FlashIndex (flagship, v1.1.0, 14 stages): local source-code search engine — scan, filter,
tokenize, exact search, inverted index, canonical ordering, persistence, query, bench
JSON, summary, parallel indexing (byte-identical across thread counts), parallel speedup
gate, ranked search (coverage → occurrences), stable ranking (path tie-break, top ten).
94 black-box tests after an exhaustiveness pass: every Requirements/Edge-cases bullet in
the instructions maps to a test (missing roots, all nine extensions case-sensitively, EOF
without newline, CRLF, byte-counted columns vs chars, posting dedup/byte order, tab
records, prefix/suffix query non-matches, limit-after-full-sort, etc.). Proven with
check-reference against tools/reference_solutions/flashindex_rust. CRLF fixture protected
by .gitattributes -text. Also: MiniKV (10 stages), TinyHTTP (10), ByteForgeVM (11), each
with reference solutions.
Content style: docs/content-style.md; each stage doc = Goal/Background/Requirements/
Example/Edge cases/Success criteria/Non-goals + exactly three hints.

## Web experience (recent major work)

Architecture (deliberate decision): CLI commands stay authoritative and write static HTML
to .deltaforge/ui/ (learning.html by learning_web.rs, test-report.html by test_web.rs);
`deltaforge serve` (src/viewer.rs) is a dependency-free loopback HTTP server with an SSE
/events stream. One persistent tab follows the terminal: `test` regenerates both pages on
every human-readable run and bumps a version marker (tab reloads/navigates);
`instructions`/`overview` navigate the tab. Interactive commands auto-spawn the viewer
(auto-spawned ones exit after 30 idle minutes); serve --stop/--restart use a token-guarded
/shutdown endpoint (token in .deltaforge/ui/viewer.json). Stable per-project port. The
request reader routes on the request line and drains huge cookie headers (127.0.0.1
cookies are shared across all local ports); idle speculative connections close silently.
Not-yet-generated pages get a self-updating placeholder. file:// opening remains the
fallback; --json/--terminal/CI/DELTAFORGE_NO_BROWSER never touch the server.
Design: src/web_theme.rs is the shared theme both pages embed — warm paper/ember palette,
light+dark via prefers-color-scheme, shared Learn/Test-report pill nav, serif display
headings, card shadows/radius, motion (entrance stagger, hover lift, failure-dot pulse,
cross-document view transitions) gated by prefers-reduced-motion.

## Verification workflow

cargo test --release (62 lib + 51 cli_flow integration + 4 MCP), clippy clean. cli_flow
hard-codes per-stage test counts and embeds a passing_flashindex_source() implementation
that must satisfy pack contracts (it was fixed to count byte columns). UI changes get
visual verification in both color schemes.

## Known open threads (prioritized candidates)

1. FlashIndex stage numbering incoherence — the main remaining "professional quality"
   blocker: directory ids run 01–10 with four doubled pairs (05_inverted/05_canonical,
   06_persist/06_query, 09_indexing/09_performance, 10_ranked/10_stable) while doc
   headings number 01–14 and project.yaml titles diverge from doc titles for ~9 stages;
   the CLI prints "Stage {id}: {yaml title}" above a doc that calls itself a different
   number/name, and commit.rs strips the id prefix so two stages produce "Complete Stage
   05". Fix = renumber ids to 01–14 (or headings to ids) + sync yaml titles; mind
   state/proof migration and the stage-id tags.
2. Runner/report refinements: word-level diffs for stdout_exact failures, per-test
   timings in the report, parallel `test --all`, fold explain-failure into failure cards.
3. Git timeline in the viewer (stage commits/tags, diff-since-last-stage) rendered into
   learning.html at generation time.
4. Optional manual light/dark toggle (currently system-preference only).
