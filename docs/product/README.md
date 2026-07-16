# DeltaForge product direction

Status: **Frozen for Phase 1**
Drafted: 2026-07-15  
Branch: `codex/product-workbench`

This directory defines the product direction for the local-web-first DeltaForge
workbench. These documents are the approved product contract for Phase 1. They
supersede older product assumptions in `Spec.md` where the two
conflict. The existing specification remains useful for engine behavior, pack formats,
and historical context.

## Documents

- [vision.md](vision.md) -- audience, promise, scope, principles, and success measures.
- [experience.md](experience.md) -- learner journey, state machine, progression, help,
  resumption, and performance-learning loop.
- [architecture.md](architecture.md) -- local-web-first architecture decision, service
  lifecycle, security boundary, application operations, and CLI disposition.
- [visual-direction.md](visual-direction.md) -- requirements for a completely new visual
  and interaction identity.
- [phase-1-vertical-slice.md](phase-1-vertical-slice.md) -- the first working slice and
  its acceptance and research protocol.
- [phase-1-observation-protocol.md](phase-1-observation-protocol.md) -- the runnable
  five-learner session script, recording sheets, pass thresholds, and issue rubric.
- [phase-1-release-audit.md](phase-1-release-audit.md) -- release gates, resolved audit
  defects, platform evidence, and the hosted-CI completion boundary.

## Frozen decisions

The following decisions are binding for Phase 1:

1. The browser workbench is the primary learning and orchestration surface.
2. The learner writes code in their own editor.
3. The CLI remains a deliberate power, automation, CI, accessibility, agent, and
   pack-authoring surface; it is not the normal product navigation.
4. A shared Rust application core remains authoritative for project behavior and state.
5. DeltaForge is local-first and fully usable offline without an account.
6. A local service is an invisible implementation detail managed by `deltaforge`.
7. FlashIndex is the flagship experience and the first complete product slice.
8. Progress is communicated primarily as acquired capabilities, with stage numbers as
   orientation.
9. Correctness precedes performance; optimization is taught as prediction, experiment,
   measurement, and reflection.
10. AI coaching is deferred until the deterministic experience is excellent.
11. DeltaForge 1.0 has no legacy-project, legacy-state, legacy-command-shape, or legacy
    visual-identity compatibility obligation. Compatibility may be retained when it is
    useful and cheap, but it may not distort the new product.
12. The current warm paper/ember visual language is retired. The workbench receives a
    new identity designed from first principles.

## Amendment procedure

Later direction changes require an explicit amendment that identifies the frozen
decision, explains why it changed, and records its consequences for active work.

Phase 0 contains no production implementation. Phase 1 begins with the vertical slice
defined in `phase-1-vertical-slice.md`.
