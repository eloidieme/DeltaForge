# DeltaForge handoff prompts

One prompt per implementation task for the incomplete feature work. To dispatch a task, give the agent `00-shared-context.md` followed by the task file. Assumed already done (separate prompt, delivered earlier): the foundation-repair pass (pack pinning/`sync-pack`, per-user pack cache, digest robustness, `bench_run`, discovery error tolerance, failure output, small fixes).

## Priority order and dependencies

| # | Task | File | Effort | Depends on |
|---|------|------|--------|------------|
| 1 | Benchmark engine v2 (matrix, peak memory, speedup, `--compare`) | `01-benchmark-engine-v2.md` | M–L | foundation pass |
| 2 | Performance gates | `02-performance-gates.md` | M | 1 |
| 3 | FlashIndex performance stages (parallel indexing, ranked search) | `03-flashindex-performance-stages.md` | M | 1, 2 |
| 4 | Content depth pass (all packs) | `04-content-depth-pass.md` | L | best after 1–2 |
| 5 | Data-driven report & portfolio | `05-report-portfolio-quality.md` | M | best after 1 |
| 6 | Optimization quests | `06-optimization-quests.md` | M | 1 (reuses 2) |
| 7 | MCP read/grounding tools | `07-mcp-read-tools.md` | S–M | — |
| 8 | Design reflection loop | `08-design-reflection-loop.md` | S | 1 (lightly) |
| 9 | Second language: Go + multi-file references | `09-second-language-go.md` | M–L | — |
| 10 | Interview mode | `10-interview-mode.md` | S–M | — |

Rationale: 1→2→3 is the critical path that makes the "measurable performance" identity real; 4 and 5 make the existing surface worth using; 6–10 are independent value-adds. 7, 9, 10 can run in parallel with the critical path.

## Explicitly deferred (no prompts)

- **Boss fights** (Spec §23): compose from gates + quests + report once tasks 2, 5, 6 land — it's a final stage with gates plus required artifacts, not a new engine feature.
- **Implementation variants** (§22) and **cross-language compare** (§21): need multi-checkout orchestration; revisit after 9 proves multi-language.
- **Profiling integration** (§20) and allocation-count metrics: platform-heavy; revisit after 1–3 establish what learners actually need.
- **Local web dashboard** (§26): Spec itself says post-CLI-stability.
- **Hint gating policy** (§18): product decision needed first (current honor-system access is intentional until decided).
