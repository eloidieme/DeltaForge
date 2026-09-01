# Quickstart

## Start

```bash
deltaforge
```

A local workbench opens in your browser. From there:

1. **Catalog.** Pick a project. Each card says how many steps it has, roughly how long it
   takes, and whether your machine already has the toolchain it needs.
2. **Create.** Choose a language and where the project goes. The environment preflight
   tells you what it found before anything is written. The default location is
   `~/DeltaForge`; set `DELTAFORGE_WORKSPACE` to change it.
3. **Build.** The workbench opens on the first step, showing what to build, why it
   matters, the exact requirements, a worked example, and the edge cases that are
   checked.

## The loop

Write code in your own editor — the workbench has an **Open editor** button — then run
the checks.

When they fail, the workbench names one thing to fix first: the requirement, what was
expected, what your program actually produced, and the input it was given. Other failing
checks are there if you want them, but the first one is the one to work on.

When you are stuck, reveal help one level at a time. The ladder goes from *look at this*
to *here is how to decompose it*, and the last level unlocks only after the step passes —
it is a retrospective, not a hint.

When the checks pass, the workbench offers a snapshot: a commit and a tag for the
completed step, showing you what it would record before it records anything. Then
continue to the next step.

## Steps that are measured

Some steps are about speed, not just correctness. Those carry a benchmark, and the step
rail marks them with a small diamond so you can see one coming.

On the **Performance** page: commit to a prediction before you measure, run the
benchmark, and compare. Results are saved, so every later run shows how far it moved.
A step with a performance target will not let you continue until the target is met with
the current source.

Prediction and reflection are both optional and both skippable.

## When you come back

Close the tab and come back whenever. The workbench restores where you were, tells you
whether your source changed since the last result, and never invents progress you did
not make.

## Exporting what you built

**Export the record** on the Overview page writes an engineering record into the
project: which steps are complete, how many checks each one proved, what was measured on
which machine, which targets were met, the snapshots in Git history, and the notes you
wrote. Every line traces to something the project actually recorded.

## From the terminal instead

Every action above has a command, and both surfaces are the same operation:

```bash
deltaforge init flashindex --lang rust
cd flashindex-rust
deltaforge test
deltaforge explain-failure
deltaforge hint
deltaforge bench --save --compare
deltaforge next
deltaforge commit
deltaforge report --format markdown --output report.md
```

See [commands.md](commands.md) for the full list.
