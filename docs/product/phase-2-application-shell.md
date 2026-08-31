# Phase 2 milestone: application shell and project library

Status: **Complete — implemented and release-verified**  
Approved: 2026-07-18

## Product problem

The Phase 1 workbench proved the local browser execution loop, but its interface still
looked and behaved like a page served from one repository. It opened directly on the
current task, had no global navigation, compressed the project explanation into the
task page, and used internal learning-system terms in ordinary interface copy.

This milestone makes DeltaForge read as a standalone local application.

## User-facing language

The application uses familiar terms:

| Internal/domain term | Interface term |
|---|---|
| capability | step |
| mission | instructions |
| evidence or proof | test result |
| contradiction | failing check |
| contract | requirements |
| progressive help | hints |
| capability acquired | step complete |
| chronicle | history |

Internal Rust types, persisted fields, and versioned API paths may keep established
domain names where changing them would add migration risk without improving the
learner experience.

## Routes

The local application has stable, reloadable routes:

```text
/projects
/projects/<project-id>/overview
/projects/<project-id>/build
/projects/<project-id>/runs
```

The global header always returns to Projects. Project navigation always exposes
Overview, Build, and Test results.

## Projects hub

The service owns a user-level registry under the DeltaForge application home. A
registered project contains a stable opaque identifier, canonical local path, optional
pack search directory, and last-opened time.

Browser requests select projects only by registered identifiers. They cannot submit a
filesystem path or command. Missing project directories are removed from the registry
when it is read.

Bare `deltaforge` behavior is now:

- inside a project: register it, start or reuse the shared service, and open its
  Overview route;
- outside a project: start or reuse the shared service and open Projects;
- while another project is already open: reuse the same service and focus the requested
  application route.

## Dedicated overview

Overview is a full project page, not a disclosure inside the current step. It renders:

- the project name, description, topics, and progress;
- all authored explanatory README sections in their original order;
- paragraphs, examples, code blocks, and lists;
- the complete build plan with current and completed steps;
- one Continue building action.

## Build and results

Build contains the current instructions, why they matter, expected behavior,
requirements, example, edge cases, exclusions, checks, focused failure, and hints.
Test results contains the bounded persisted run history.

The application core remains authoritative. The browser only renders returned state
and invokes fixed operations.

## Acceptance

This milestone is complete when:

1. two projects register into and are served by one process;
2. the hub lists both and each opaque identifier resolves to the correct project;
3. launching outside a project opens the hub;
4. all four routes reload directly;
5. Overview includes authored explanations and examples;
6. no core action requires the learner to understand the local service or repository
   serving model;
7. visible workflow terms use the interface vocabulary above;
8. desktop, mobile, keyboard, restart, security, and existing run flows pass.
