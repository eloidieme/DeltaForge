# DeltaForge — current state (July 2026)

*(Snapshot prompt: read this before any new task for the up-to-date picture. `00-shared-context.md` still describes the core principles and source layout — this file records what has shipped since and what is open. Updated 2026-07-16.)*

DeltaForge is a local, CodeCrafters-style learning framework written in Rust (~14k lines,
lean deps: clap/serde/anyhow/regex/cap-std, no async runtime in the main binary). Learners
init a project from a "pack", implement a CLI program stage by stage against black-box
tests, and progress with `deltaforge next`. Everything runs locally; bundled packs are
embedded in the binary via include_dir. Repo: github.com/eloidieme/DeltaForge (single
author, direct commits to main).

## CLI surface

Bare `deltaforge` opens the workbench. Diagnostic and authoring commands include list,
pack (list/show/new/add-stage/doctor/check-reference/install), init, validate-pack,
instructions, overview, test, next, sync-pack, status, hint, config, bench, report,
portfolio, design, commit, doctor, and explain-failure. Plus `deltaforge-pack-mcp`, a
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
Example/Edge cases/Success criteria/Non-goals. FlashIndex Stage 1 now has the Phase 1
five-level progressive-help ladder; untouched catalog stages retain their existing hints.

## Workbench experience

Bare `deltaforge` is the canonical learner entry. It starts or focuses one hidden,
token-protected loopback workbench service and returns the prompt. The application core
owns mission content, run coordination, live events, diagnosis, freshness, resumption,
recovery, help, and capability progression. `overview`, `instructions`, and `test` are
terminal-only diagnostics; they never generate or open HTML. The retired generated
learning/test-report pages, live viewer, `serve` command, and warm paper/ember theme were
removed from the product path. `DELTAFORGE_NO_BROWSER=1` keeps bare launch usable by
printing the workbench URL and CLI fallback.

## Verification workflow

Use `docs/product/phase-1-checkpoint.md` as the current verification authority. Phase 1
workbench changes require release check, Clippy, library tests, the failure corpus, the
real-service workbench suite, and browser/keyboard inspection proportional to the change.

## Known open threads (prioritized candidates)

1. FlashIndex stage numbering incoherence — the main remaining "professional quality"
   blocker: directory ids run 01–10 with four doubled pairs (05_inverted/05_canonical,
   06_persist/06_query, 09_indexing/09_performance, 10_ranked/10_stable) while doc
   headings number 01–14 and project.yaml titles diverge from doc titles for ~9 stages;
   the CLI prints "Stage {id}: {yaml title}" above a doc that calls itself a different
   number/name, and commit.rs strips the id prefix so two stages produce "Complete Stage
   05". Fix = renumber ids to 01–14 (or headings to ids) + sync yaml titles; mind
   state/proof migration and the stage-id tags.
2. Phase 1 closeout: native macOS gates and Linux/Windows cross-target release checks
   pass, stale legacy assertions are replaced, and the observation protocol is ready.
   Commit/push authorization and passing native GitHub CI jobs on macOS, Linux, and
   Windows are the only remaining engineering evidence; see
   `docs/product/phase-1-release-audit.md`.
