# Visual and interaction direction

## Decision

The current warm paper/ember theme is not the baseline for the new product. DeltaForge
will receive a new visual and interaction identity designed around the workbench's
states and ambitions.

Existing CSS, typography, layout, color, and decorative choices may be discarded. They
should only be reused when independently judged to be the best solution for the new
experience.

Phase 0 freezes what the identity must accomplish, not its final colors, typefaces, or
component styling.

## Desired qualities

The product should feel:

- **Serious:** appropriate for difficult, multi-session engineering work.
- **Calm:** clear hierarchy, low noise, and no constant demand for attention.
- **Precise:** states, evidence, actions, and measurements are visually unambiguous.
- **Encouraging:** difficult work feels tractable without becoming childish.
- **Tactile:** actions and state transitions feel responsive and consequential.
- **Distinctive:** recognizable without imitating a terminal, generic SaaS dashboard,
  or existing learning platform.
- **Owned:** the learner's project and evidence dominate DeltaForge branding.

## Interaction principles

### State before navigation

The interface expresses the learner's current state before exposing the full product
map. A learner should understand `what now` before deciding `where else`.

### One dominant action

Color, placement, size, and wording reinforce one primary action. Secondary controls do
not form a dashboard of equally weighted buttons.

### Evidence has visual hierarchy

The primary contradiction, measurement, or acquired capability is prominent. Raw logs,
complete fixtures, and secondary failures are available progressively.

### Motion communicates change

Motion may connect running, failure, source-change, passing, and continuation states.
It should never delay work, conceal data, or ignore reduced-motion preferences.

### Dense when useful, quiet when not

Test comparisons and benchmark tables may be information-dense. Mission and completion
states should not inherit that density unnecessarily.

### Keyboard and assistive technology are first-class

All core actions, navigation, live status, comparisons, charts, and disclosures must be
usable without a pointer and understandable without color or animation.

## Explicitly avoid

- carrying forward the paper/editorial metaphor by default;
- generic administration-dashboard layout;
- terminal cosplay as the main visual identity;
- neon cyberpunk styling merely because the product concerns systems programming;
- excessive cards, pills, gradients, glows, and status badges;
- game currencies, XP bars, streak flames, and confetti;
- hiding important evidence behind animation or hover;
- decorative benchmark charts without an interpretive purpose;
- a permanent AI chat panel dominating the workspace.

## Canonical design moments

The eventual visual system must be proven through real content in these moments:

1. First-launch project selection
2. Current mission before the first run
3. Checks running
4. One actionable failure
5. Capability acquired
6. Returning after time away
7. Performance prediction and baseline
8. Experiment comparison and tradeoff
9. Final challenge
10. Completed engineering story

Components should be extracted from these screens after their hierarchy works. The team
should not begin with an abstract component library.

## Phase 1 visual scope

The first vertical slice requires a coherent new shell for only:

- first launch or direct project entry;
- current mission;
- live run;
- failure diagnosis;
- source-changed state;
- capability completion;
- resumption.

It does not need the final catalog, performance-lab identity, chronicle, or complete
design system. It must nevertheless be a genuine new direction rather than current HTML
restyled around API calls.

## Phase 1 design deliverable

Before production frontend work expands beyond the vertical slice, produce and review:

- a low-fidelity state-flow wireframe;
- high-fidelity designs for the seven Phase 1 states;
- light and dark behavior, if both are included in 1.0;
- keyboard and focus behavior;
- loading, interrupted, empty, and unhealthy states;
- typography and color rationale;
- motion samples for run progress, stale results, and capability completion.

The design is successful when a user can infer the current state and next action without
reading navigation documentation.
