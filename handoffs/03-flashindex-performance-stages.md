# Task 03 — FlashIndex performance stages (parallel indexing + ranked search)

*(Prepend `handoffs/00-shared-context.md`. Depends on Tasks 01 and 02 — matrix benchmarks and performance gates must already be merged; verify before starting.)*

## Objective

No bundled pack currently applies any performance pressure — the product's differentiator never happens. Extend the FlashIndex pack (currently 8 stages, `packs/flashindex/`) with the Spec's marquee stages (§32, stages 07–08 there): parallel indexing with a thread-scaling benchmark and gate, and ranked multi-token search. Update the internal reference solution so CI proves both stages are passable.

## Design

### Stage 09_parallel_indexing
- Behavior: `flashindex index <path> --threads N` — output must be **byte-identical** to the existing stage 05 `index` output regardless of N (worker-local indexes merged deterministically; this is the pedagogical point: parallelism must not change observable behavior).
- Tests (correctness, deterministic): same fixture indexed with `--threads 1` and `--threads 4` produces identical expected lines; `--threads 0` or a non-numeric value exits non-zero with a message on stderr.
- Benchmark: `matrix: { threads: [1, 2, 4, 8] }` over a benchmark fixture (see below), `iterations` modest (3–5) to keep reference CI time sane.
- Gate: `metric: speedup`, `min: 1.5` (conservative — CI runners and laptops are noisy; Spec §15 says gates must be realistic). Include `advice` entries (shared mutable index, per-file locking, merge cost).
- Benchmark fixture: a committed, deterministic synthetic codebase. **Packs are embedded into the binary via `include_dir!`, so size matters**: keep the fixture ≤ ~2 MiB total (many small generated source files; write a small generator under `tools/` to produce it once, commit the output, do not run the generator at build/test time).
- `instructions.md` must follow the full template (goal, background on worker-local vs shared state, requirements, example, edge cases, success criteria, non-goals) and explicitly state the determinism requirement. Three progressive hints. Add a `design_prompt.md` (Spec §17 has the exact parallel-indexing prompt — use it).

### Stage 10_ranked_search
- Behavior: `flashindex search <index-or-path> "<token1> <token2> ..."` — rank files by number of distinct query tokens matched (desc), then total occurrences (desc), then path (asc) as the deterministic tie-break. Print `rank. path (matched X/Y tokens, Z occurrences)` one per line, top 10.
- Decide and document whether it operates on the persisted index from stage 06 or a directory (recommended: persisted index, reinforcing stage 06). Tests: multi-token ranking order, tie-break determinism, token absent from all files, empty query is an error.
- Full-template instructions, 3 hints.

### Reference solution
- Extend `tools/reference_solutions/flashindex_rust/src/main.rs` with both stages (std::thread + worker-local maps + deterministic merge; no new dependencies). The existing CI test that runs the reference against all stages (`tests/cli_flow.rs::reference_solution_passes_all_flashindex_v1_stages` and the deepened-packs test — check current names) must cover the new stages. Note: reference checks run tests, not benchmarks — gates are exercised by your integration test instead.

### Pack bookkeeping
- Append both stages to `packs/flashindex/project.yaml`; bump the pack `version`; update the pack README roadmap. Run `validate-pack --strict`.
- Mind pinning: if a `sync-pack`/re-pin mechanism exists (foundation pass), mention the pack change in CHANGELOG; existing learner projects will need it.

## Acceptance criteria
- Reference solution passes `deltaforge test --all` for flashindex including the two new stages (existing CI test extended or already covering).
- Integration test: reference project runs the stage 09 benchmark with the matrix and produces history entries per thread count; gate evaluation runs (don't assert the speedup value — assert the gate machinery evaluated and reported).
- Instructions for both stages contain the template sections; `validate-pack --strict` clean; binary size increase from the new fixture stays under ~2.5 MiB (check `target/release/deltaforge` before/after and report).
- Full quality bar passes.

## Out of scope
Other packs' content (Task 04), quests, boss-fight stage.
