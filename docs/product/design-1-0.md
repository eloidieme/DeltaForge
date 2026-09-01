# The 1.0 visual system

The deliverable required by [visual-direction.md](visual-direction.md), recorded after
implementation rather than before it. That order is a deviation and it is stated as one:
the interface was designed and built in one pass, and this document is the design as
built, written so the next change to the frontend has something to be consistent with.

The system lives in `src/ui/app.css`; this describes why it is the way it is.

## What the direction asked for

Serious, calm, precise, encouraging, tactile, distinctive, owned. And explicitly not:
the paper/editorial metaphor, a generic administration dashboard, terminal cosplay, neon
cyberpunk, excessive cards and pills and glows, or any game currency.

## Three ideas

Everything else follows from these.

**One ground, few surfaces.** The learner's own work is the content; DeltaForge is a
quiet frame around it. There is a page ground, one surface for things that sit on it, and
two tonal steps beyond that for inset regions. Elevation is a hairline plus a small tonal
step — never a shadow stack, never a glow. This is what keeps the interface calm while
still having structure, and it is why nothing in the product floats.

**Evidence is ruled, not boxed.** A finding, a measurement, or a contradiction is marked
by a coloured rule along its leading edge rather than wrapped in its own card. A failure
diagnosis is four labelled facts under a red rule; a saved prediction is prose under a
violet rule. This is the answer to *dense when useful, quiet when not*: a region can hold
a lot without becoming a wall of cards, and the same treatment scales from one fact to a
table.

**One dominant action.** Exactly one filled control is on screen at a time; everything
else is a hairline ghost button. On the build screen that is *Run checks*, and it becomes
*Cancel run* while a run is live rather than sitting next to a second button. On the
catalog it is the flagship's *Start this project*; the preview packs are offered on equal
footing but not urged. The rule is what makes *what now* legible before *where else*.

## Colour

A near-neutral ground with a single accent and four semantic families. Every state
carried by colour is also carried by words, so nothing depends on seeing hue.

| Token | Means | Where it appears |
|---|---|---|
| `--accent` | the action to take | the primary button, the current step, focus rings, active nav |
| `--proven` | what is established | a passing result, a met target, a completed step |
| `--attention` | what has gone stale | source changed since the last result, an interrupted run |
| `--contradiction` | the thing to fix | the primary failure, a missed target, a refused location |
| `--measure` | numbers | gates, predictions, the diamond on a measured step |

The neutral ramp is a very slightly warm off-white in light and a slightly cool graphite
in dark. Neither end is pure white or pure black: a full-contrast ground makes long
reading tiring, and this is an interface people sit in for hours.

Light and dark are both first-class, defined as complete token sets rather than as
inversions. Dark is not light with the lightness flipped — the semantic hues are lifted
in chroma and lightness so they hold against a dark ground, and the soft tints are near
the ground rather than near white. The viewer's system preference is the default, and a
three-way toggle (System, Light, Dark) overrides it and persists per browser.

## Type

One sans stack for prose and one monospace stack for evidence. That split is the
typographic rule of the whole product: anything the learner's program produced, any path,
any command, any measured number is monospaced with tabular figures, and everything
DeltaForge says about it is not. A learner should be able to tell, without reading, which
words are theirs.

Headings are set slightly tight and at a modest weight. There is one eyebrow style —
small, spaced, uppercase, muted — used to label a region without adding a heading level.

## The step rail

The one deliberately distinctive element.

Steps are drawn as a connected line of nodes: a hollow ring for a step not yet reached, a
ringed accent node for the current one, a filled check for a completed one. The line runs
behind the nodes and stops at the first and last, so the journey reads as a path with
ends rather than a list that happens to be vertical.

A step that carries a measurement gets a small diamond beside its title — outlined when
unmeasured, filled when the target is met, red when it is not. This is the concrete answer
to the gap analysis's sharpest finding: before this existed, a performance gate could only
ever appear as a wall at the moment progression was refused. Now it is visible from
step one, and it is the only place in the interface where a shape rather than a colour
carries a distinct meaning, which is why it also carries a title attribute.

The rail is the product's silhouette. It should survive any future restyling.

## Motion

Two hundred milliseconds, one easing curve, and only on things that change state:
hover and active on controls, the progress fill, the sweep on a live run bar. Nothing
animates on load, nothing delays work, nothing hides data behind a transition. Every
animation and transition collapses to effectively zero under `prefers-reduced-motion`,
and the run bar becomes a full-width static bar rather than disappearing, because it is
carrying information.

## Keyboard and assistive technology

- A skip link is the first focusable element on the page.
- One visible focus ring, two pixels of accent with an offset, on everything interactive.
  It is never removed.
- The live-run meter, the resumption notice, the open-project status, and the export
  status are all `aria-live="polite"`, so a state change is announced without stealing
  focus.
- Step nodes are decorative and marked `aria-hidden`; each rail row carries an
  `aria-label` naming its position, title, and status in words.
- The language chooser in the creation flow is a real radio group with a labelled
  `role="radiogroup"`, not clickable divs.
- Every table cell that carries a delta also carries the sign in text.

## The canonical states

`visual-direction.md` lists ten moments the system must be proven against. Eight are in
1.0 scope, and each has a place in the built interface:

| Moment | Where |
|---|---|
| First-launch project selection | `/catalog`, flagship first |
| Current mission before the first run | `/build`, the instructions column |
| Checks running | the result card in its running tone, with the sweep bar and live meter |
| One actionable failure | the result card in its contradiction tone, plus four ruled facts; other failures behind a disclosure |
| Capability acquired | the result card in its proven tone, plus the snapshot offer |
| Returning after time away | the resumption notice above the instructions |
| Performance prediction and baseline | `/performance`, prediction section and measurement table |
| Experiment comparison and tradeoff | the measurement table's change column, and the gate panel with its advice |

The final challenge and the chronicle are cut from 1.0, so their moments are not
designed.

## Layout

Three columns on the build screen — rail, instructions, evidence — collapsing to two at
1120px and one at 840px, with a narrow breakpoint at 560px that drops the least useful
element from the app bar rather than wrapping it. The rail and the evidence column are
sticky on wide viewports and static once the layout collapses. Wide content — measurement
tables, code blocks, captured output — scrolls inside its own container; the page body
never scrolls horizontally.

## Implementation notes

The page is composed at compile time from `src/ui/index.html`, `src/ui/app.css`, and
`src/ui/app.js`, and served inline as one document. That keeps the page a single request
with a single capability token and no subresource that would need its own authorization,
while leaving three readable files to work in. There is no build step and no framework.

The client renders `/api/v1/state` and never derives progress, decides whether a step is
complete, or invents a next action. The primary action's kind and label come from the
service.
