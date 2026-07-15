# Changelog

## Unreleased

### Live viewer server

- Added `deltaforge serve`: a dependency-free local HTTP server on `127.0.0.1` that serves the generated learning and test-report pages and pushes a server-sent event whenever a command regenerates them. One browser tab now follows the terminal — `deltaforge test` refreshes the report in place and `deltaforge instructions`/`overview` navigate the connected tab — instead of a new tab opening on every failed run.
- Interactive commands that would open a browser now start the viewer automatically when none is running (auto-started viewers shut themselves down after thirty idle minutes) and reuse an already-connected tab instead of opening another. When no viewer can be started, the previous file-based opening remains the fallback.
- `deltaforge test` now regenerates the browser report after every human-readable run, not only failing ones, so a connected tab always shows the latest result. `--json`, `--terminal`, `--list-tests`, non-interactive output, and `DELTAFORGE_NO_BROWSER=1` behave exactly as before and never start a server.
- The viewer binds only to the loopback interface, prefers a stable per-project port, serves only known page names inside `.deltaforge/ui/`, and the generated HTML files remain valid standalone documents — the live-reload hook is injected only when a page travels through the viewer.
- The request reader routes on the request line alone and drains large header blocks, because cookies scoped to `127.0.0.1` are shared across every local port and real browser requests routinely exceed a small buffer. Idle speculative browser connections are closed silently instead of receiving an unsolicited error response.
- Added a shared Learn / Test report navigation to the top of both generated pages, so either page reaches the other without returning to the terminal. In the viewer, a known page that has not been generated yet shows a placeholder that names the command that creates it and reloads itself the moment the page exists.
- Added `deltaforge serve --stop` and `deltaforge serve --restart`. Shutdown goes through a loopback endpoint guarded by a random token recorded in `.deltaforge/ui/viewer.json`, so local commands can stop the viewer but a drive-by web page cannot. Restart reuses the project's stable port, so a connected tab reconnects to the fresh server by itself — useful after reinstalling deltaforge while an older viewer is still running.
- Unified the learning page and test report under one shared theme (`web_theme`): a single warm paper/ember palette with matching light and dark variants that both pages resolve from the system preference — the report previously shipped light-only and clashed with the dark learning page. The shared layer also unifies the header, pill navigation, serif display headings, and card treatment, and adds motion: entrance transitions, staggered card reveals, hover lift, a pulsing failure indicator, and cross-document view transitions between the two pages, all disabled under `prefers-reduced-motion`.
- Fixed invisible command text in the learning page's console examples under the light theme: an inline-code background rule also matched code inside dark code blocks.

### Exhaustive FlashIndex contracts (FlashIndex 1.1.0)

- Expanded FlashIndex black-box coverage from 61 to 94 tests so every requirement and edge-case bullet in the stage guides has a corresponding deterministic case: missing/unreadable roots for `scan`, `search`, `summary`, and persisted `index --out`; all nine corpus extensions with case-sensitive suffix matching; tokens at end-of-file without a trailing newline; CRLF line endings; byte-counted columns around multi-byte characters; lone-underscore tokens; posting dedup, bytewise token order, and per-posting path order; tab-separated multi-path persisted records; prefix/suffix non-matches against a saved index; negative and two-thread worker counts; whitespace-only ranked queries; and rank limits applied only after the complete sort.
- Corrected two Stage 05 (inverted index) tests that pinned path order inside a posting even though canonical ordering is explicitly deferred to the next stage; membership is now asserted order-insensitively.
- Bumped FlashIndex to `1.1.0`. Existing projects should run `deltaforge sync-pack`; the strengthened tests intentionally require affected completed stages to be revalidated.

### Browser test reports

- Added a self-contained `.deltaforge/ui/test-report.html` for failed human-readable test runs. Interactive failures open it automatically; redirected runs print its path without launching a browser.
- Made test failures easier to diagnose with failure-first grouping, structured expected/actual comparisons for text, exit codes, files, patterns, and JSON, separate stdout/stderr views, visible whitespace, stage-instruction links, runtimes, and copyable filtered rerun commands.
- Added a Test input view for every result, showing the sanitized command, run settings, standard input, declared environment, and a bounded pre-run snapshot of fixture directories with expandable text or hexadecimal file previews.
- Added `deltaforge test --open` to show a report after successful runs and `deltaforge test --terminal` to retain detailed terminal-only behavior. `--json`, `DELTAFORGE_NO_BROWSER=1`, and non-interactive output remain safe for automation.
- Reduced interactive terminal noise to test progress and the final summary when a browser report is available. Detailed output remains the fallback for redirected commands and terminal-only environments.

### Natural-language editorial pass (all bundled packs 1.0.1)

- Removed authoring-history language from pack overviews, guides, hints, and design prompts so the material reads as one coherent explanation rather than a commentary on curriculum revisions.
- Reframed fixed policies in terms of each tool's purpose and tradeoffs. FlashIndex's corpus examples now use ordinary `.txt`, `.md`, and `.cmake` files to demonstrate the documented extension policy directly.
- Made FlashIndex's persisted index contract explicit: UTF-8 token records, tab-separated fields, newline-separated records, and sorted deduplicated portable paths. Its persistence test now verifies the token-to-path record rather than merely looking for a token substring.
- Existing projects should run `deltaforge sync-pack`. Documentation-only changes preserve completion proofs. FlashIndex's changed Stage 02 fixture/test and Stage 07 persistence test require those completed stages to be revalidated.

### Gentler curricula and full content rewrite (all bundled packs 1.0.0)

- Split the four bundled projects from 28 broad stages into 45 smaller stages: FlashIndex now has 14, MiniKV 10, TinyHTTP 10, and ByteForgeVM 11. Original stage IDs remain available, with follow-up IDs inserted beside the concepts they separate.
- Rewrote every overview, stage guide, and hint set in the new editorial baseline: begin with a concrete situation, make the problem clear before naming the solution, introduce terminology only when it becomes useful, explain arbitrary teaching policies honestly, and keep exactly three progressive hints per stage.
- Divided tests, benchmarks, and design prompts along the new learning boundaries. Added black-box coverage for stale artifact removal, exact persisted queries, unreadable benchmark roots, case-sensitive summaries and ranking, malformed compaction input, UTF-8 byte lengths, VM loader/operand/call boundaries, and pre-error tracing.
- Added `file_not_contains` to the test format and pack-authoring MCP schema so stages can prove that stale records and deleted keys are absent from generated artifacts.
- Added `docs/content-style.md` as the pack-writing standard and `docs/curriculum-map.md` as the canonical new sequence and migration guide.
- Existing learner projects must run `deltaforge sync-pack`. Because tests and fixtures moved or changed, behavioral proofs for affected completed stages intentionally require revalidation. Original IDs remain loadable, but a fresh project is recommended to experience every inserted stage in order.

### Browser learning surface and approachable FlashIndex opening (FlashIndex 0.4.0)

- Changed interactive `overview` and `instructions` from a long terminal document to a self-contained local learning page. It opens in the system browser with stage navigation, progress state, task/example/rationale/reference tabs, neighboring-stage previews, copyable examples, responsive layout, and accessible keyboard-friendly controls. No pack content or external asset leaves the machine.
- Added `--terminal` for the original renderer, `--no-open` for generating `.deltaforge/ui/learning.html` without launching a browser, and `DELTAFORGE_NO_BROWSER=1` for terminal-only environments. Redirected and piped commands retain terminal behavior, and `overview --json` is unchanged.
- Rewrote FlashIndex stages 01–03 from first principles, using shorter explanations, concrete project trees and source examples, and explicit labels for teaching policies that are choices rather than universal rules.
- Corrected the Stage 02 corpus contract: removed an unexplained niche extension, made `.cmake` support real and tested, and removed redundant filename special cases already covered by the `.txt` and `.md` rules.
- Bumped FlashIndex to `0.4.0`. The Stage 02 fixture and tests changed, so existing FlashIndex learners should run `deltaforge sync-pack`; a previously completed Stage 02 intentionally requires revalidation. Instruction-only changes to Stages 01 and 03 do not invalidate their proofs.

### Curriculum study aids (all bundled packs 0.3.1)

- Expanded every pack overview with a cumulative concept map or protocol/opcode reference, a focused glossary, historical field notes, diagnostic failure-analysis labs, and optional extensions. Corrected the MiniKV, TinyHTTP, and ByteForgeVM summaries so their final-stage capabilities match the actual roadmaps.
- Added post-stage reflection questions at the major design boundaries: recovery and compaction, HTTP parsing/security/connection/range semantics, VM stack/control/error/call/trace semantics, and search token/index/persistence/parallel/ranking semantics.
- Added interpretation worksheets to every targeted benchmark stage. Learners now record fixture shape and observed metrics, distinguish startup/output/I/O effects from the algorithm under study, and check correctness before drawing performance conclusions.
- Bumped all bundled packs to `0.3.1`. This release changes only manifests and learner-facing documentation: existing projects should run `deltaforge sync-pack`, but completed stages retain valid behavioral proofs and require no revalidation.

### Content depth pass (all bundled packs 0.3.0)

- Rewrote all 28 stage guides across FlashIndex, MiniKV, TinyHTTP, and ByteForgeVM into the seven-section learning template, with deeper conceptual background, historical context, precise observable contracts, worked examples, explicit tested edge cases, success criteria, and bounded non-goals. Every stage now has exactly three progressive hints.
- Expanded black-box coverage for malformed inputs, empty corpora, deterministic ordering, persistence replacement, tombstone replay, HTTP framing and traversal, VM operand/control failures, tokenizer boundaries, and ranked-result limits. Reference solutions were extended to prove the strengthened contracts on every stage.
- Added targeted benchmarks for MiniKV append and compaction, TinyHTTP request parsing, ByteForgeVM tracing, and FlashIndex tokenization and inverted-index construction. Existing FlashIndex stage 09 retains its benchmark-v2 thread matrix and digest-pinned speedup gate.
- Added design prompts for MiniKV compaction and tombstones, ByteForgeVM calls, TinyHTTP ranges, and FlashIndex persistence.
- Extended `pack doctor` / `validate-pack --strict` authoring findings for missing `Edge cases` or `Non-goals` headings, fewer than three hints, and fewer than two tests.
- Bumped every bundled pack to `0.3.0`. Existing learner projects must run `deltaforge sync-pack`; documentation-only edits do not invalidate completion, but the changed tests and fixtures intentionally require affected completed stages to be revalidated with `deltaforge test`.

### FlashIndex performance stages (pack 0.2.0)

- Added FlashIndex stage `09_parallel_indexing`: `index <path> --threads N` produces byte-identical output to the single-threaded `index` for any N, with a `threads:[1,2,4,8]` benchmark matrix and a conservative `speedup >= 1.5` performance gate (with tuning advice). Ships a committed, deterministic ~1.3 MiB synthetic benchmark fixture generated by `tools/gen_flashindex_bench_fixture.py` (run by hand, never at build time).
- Added FlashIndex stage `10_ranked_search`: `rank <path> "<tokens>"` ranks files by distinct query tokens matched, then total occurrences, then ascending path as the deterministic tie-break, printing the top 10. Ranked search takes a directory (building the index in memory) rather than a persisted index file, because Stage 06's on-disk format is learner-defined and cannot serve as a shared black-box fixture. It uses the `rank` verb so it does not collide with Stage 04's occurrence-printing `search`.
- Extended the FlashIndex Rust reference solution with both stages (std::thread worker-local indexes + deterministic merge; no new dependencies).
- Bumped the FlashIndex pack to `0.2.0`. Existing learner projects on the older pack must run `deltaforge sync-pack` to adopt the two new stages.

## 0.1.0

- V1 local CLI surface for staged project packs.
- FlashIndex pack expanded to eight stages.
- Project discovery, config validation, test runner options, benchmarks, reports, portfolio summaries, design prompts, and git commits.

## V2 working tree

- Added grounded MCP pack-authoring reads for manifests, stage documents, structured tests and benchmarks, and recursively listed UTF-8 fixtures, plus confirmed fixture-file deletion with traversal, symlink, special-file, and 1 MiB protections.
- Hardened MCP fixture reads, recursive listing, and deletion against concurrent symlink swaps with no-follow directory capabilities and handle-relative unlinking; read-only pack discovery no longer extracts the embedded cache.
- Added fail-closed, digest-pinned performance gates for benchmarked stages, including authoring validation, MCP support, progression enforcement, status, JSON, and the `gates.enforce` escape hatch.
- Hardened gate reporting and validation so partial runs cannot claim individual passes, display-only gate renames preserve proofs, recorded outcomes are recomputed from measurements, and speedup matrices reject duplicate or nonnumeric thread values.

- Hardened stage completion with full-run eligibility, pinned pack identity, and learner/pack integrity proofs.
- Made pack scaffolding and installation transactional and tightened path/schema validation.
- Added bounded concurrent process capture, process-tree timeouts, isolated benchmark fixtures, and truthful benchmark exit statuses.
- Added MCP protocol negotiation and malformed-message recovery.
- Added legacy state compatibility coverage and strict Ubuntu, macOS, and Windows CI.
- Added an end-to-end stdio integration test using the official Rust MCP SDK.
- Added constrained MCP tools for pack/stage metadata, stage documents, structured tests and benchmarks, and fixture files.

- Added `pack list`, `pack show`, and local `pack install`.
- Added `doctor` and `explain-failure`.
- Added JSON report output.
- Extended tests with stdin, per-test env, and file-content assertions.
- Added MiniKV, TinyHTTP, and ByteForgeVM bundled packs.
- Added learner-facing `overview` output and richer generated project READMEs.
- Deepened MiniKV, TinyHTTP, and ByteForgeVM to 6 stages each.
- Added Rust reference solutions for MiniKV, TinyHTTP, and ByteForgeVM.
- Added integration coverage proving all bundled packs are passable by reference solutions.
- Added deterministic pack authoring commands: `pack new`, `pack add-stage`, `pack doctor`, and `pack check-reference`.
- Added `deltaforge-pack-mcp`, a stdio MCP server for AI-assisted pack creation and validation.

### Foundation repair

- Pinned bundled/embedded packs logically as `"bundled"` instead of by absolute temp path, so version bumps no longer brick existing learner projects; old pins under any embedded-cache location are treated as bundled.
- Added `deltaforge sync-pack` to adopt an updated pack by re-pinning the project's pack version, source, and digest. Supports `--json`. All pin-mismatch errors now point at it.
- Moved the embedded-pack cache to a per-user directory (`$XDG_CACHE_HOME`/`~/.cache/deltaforge` on Unix, `%LOCALAPPDATA%\deltaforge` on Windows), keyed by a content digest of the embedded tree, and made extraction atomic via extract-to-sibling-then-rename.
- Applied digest exclusions at every directory depth and expanded default project-digest exclusions (plus each pack's `ignored_paths`), so generated directories no longer invalidate completion proofs. (Symlink handling is described under "Integrity truthfulness".)
- Added an optional `bench_run` command to language specs and set it for all bundled packs so benchmarks time the built binary rather than `cargo run` startup overhead.
- Made pack discovery resilient to a single malformed pack: `list` warns, `doctor` reports it, and `validate-pack` reports and fails.
- Showed actual program stdout/stderr beneath test failures (truncated; full output with `--verbose`).
- Prevented `hint --level N` from lowering recorded progress, expanded `{fixture_path}`/`{temp_dir}` in test `stdin` and `env` values, defaulted `report`/`portfolio` `--output`, and added `--json` to `status`.

### Integrity truthfulness

- Completion proofs now pin a per-stage behavioral digest — the stage's `tests.yaml`, its fixtures, and the language `build`/`run` commands — instead of the whole-pack digest. Documentation-only pack updates no longer invalidate completed stages; changes to tests, fixtures, or commands invalidate exactly the stages they affect.
- `sync-pack` updates only the project-level pin and never rewrites what a proof claims was proven. It reports each completed stage as valid or needing revalidation, and `next`/`commit` block stale stages with a message pointing at `deltaforge test`. Legacy proofs (no behavioral digest) are upgraded automatically only when the pack is bit-identical to the one that passed.
- `status` marks completed-but-stale stages with `!` (JSON: `needs_revalidation`).
- Split integrity digests into two modes. Pack digests reject symlinks and special files outright (pack behavior must be self-contained). Learner project digests hash file symlinks as link path + target path + target contents — so editing or relinking a symlinked source file is detected — and reject directory symlinks with an actionable error.
- Added `[integrity] exclude = [...]` to `.deltaforge/config.toml`: learner-controlled names appended to the digest exclusion list (matched at any depth, like the built-ins).

### Benchmark engine v2

- Versioned `.deltaforge/benchmark_history.json` as `{"schema_version": 2, "runs": [...]}`; each run now carries per-configuration `points` (with `params` and a reserved `peak_memory_mb`) instead of a single flat `results` object. Legacy bare-array history files convert losslessly on read; newer schema versions are rejected. `bench --json` emits the new `points` shape.
- Added optional benchmark parameter matrices (`matrix: { threads: [1, 2, 4] }` in `benchmarks.yaml`): the cartesian product of all parameters is measured independently, `{name}` placeholders in command args expand per point, and each saved history entry records its `params`. Matrix declarations are validated by `validate-pack` (identifier names, non-empty scalar value lists, no undeclared command placeholders) and by the MCP `replace_stage_benchmarks` schema.
- Added best-effort peak-memory capture to `deltaforge bench`: each point's `peak_memory_mb` reports the approximate peak resident set size (max across measured iterations), sampled per-OS (Linux `VmHWM`, macOS `proc_pid_rusage`, Windows `PeakWorkingSetSize`) and `null` when unavailable — sampling never fails a benchmark. The test-runner path is unchanged.
- Reworked human `bench` output into an aligned per-benchmark table (one row per matrix point: params, median, p95, throughput, peak memory) and added a derived `speedup_<min>_to_<max>` line when a numeric `threads` matrix parameter is the only varying parameter. `bench --json` attaches the speedup as a per-record `derived` object; derived metrics are computed at display time and never written to history.
- Added `bench --compare`: each new benchmark point is compared with its most recent prior saved point using exact benchmark parameters, with median, throughput, and peak-memory deltas, improved/regressed wording, machine OS/architecture warnings, and machine-readable JSON comparison output. Comparisons are computed before saving and are never persisted.
