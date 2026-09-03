# DeltaForge

Build a real developer tool on your own machine, one behavior at a time.

DeltaForge gives you a written contract for each step and black-box checks that hold you
to it. The flagship FlashIndex pack adds a diagnosis that names the single thing to fix
first and a five-level help ladder,
and — where a step is about speed rather than correctness — benchmarks with a target to
hit. Your code lives in your own folder, in your own editor, under your own version
control. Nothing leaves the machine.

```bash
deltaforge
```

That opens a local workbench in your browser. Everything else — choosing a project,
creating it, running checks, reading failures, revealing help, measuring performance,
snapshotting a completed step, exporting the record — happens there. The terminal is for
writing code.

## Projects

| Project | What you build | Steps | Tier |
|---|---|---:|---|
| **FlashIndex** | A source-code search engine: directory scanning, tokenizing, an inverted index, a persisted artifact, parallel indexing with a real speedup target, and ranked search | 14 | Flagship |
| MiniKV | A persistent key-value store with an append log, recovery, tombstones, and compaction | 10 | Preview |
| TinyHTTP | An HTTP parser and static response engine with header framing, MIME types, and safe paths | 10 | Preview |
| ByteForgeVM | A stack-based bytecode virtual machine with control flow, calls, and tracing | 11 | Preview |

FlashIndex is held to the 1.0 content bar: every step has a five-level help ladder,
every check carries the diagnosis a stuck learner is shown first, and the whole pack is
proven against a reference solution on every build. The other three are complete and
playable, and are presented as preview rather than as equals.

Rust is the implementation language in 1.0.

## Install

Download a published release archive for macOS, Linux (x86-64 or ARM64), or Windows from
[Releases](https://github.com/eloidieme/DeltaForge/releases), verify its checksum, and
put the binary on your `PATH`. Before the first tagged release is published, install the
current source directly from the repository:

```bash
cargo install --git https://github.com/eloidieme/DeltaForge
```

Release archives also carry GitHub build-provenance attestations. With the GitHub CLI,
verify one with `gh attestation verify <archive> --repo eloidieme/DeltaForge`.

### macOS will refuse to run it the first time

The macOS archives are not signed or notarized, because the project has no Apple
Developer ID. **Expect macOS to block the binary and say it may be malware.** This is
what Gatekeeper does to any unsigned binary you downloaded; it is not a claim about
DeltaForge, and it happens before DeltaForge runs a single line.

Verify the checksum and provenance above first. Then allow it, either way:

**In System Settings** — the path Apple intends:

1. Run `deltaforge` once. macOS refuses and closes it.
2. Open **System Settings → Privacy & Security**, scroll to the Security section, and
   next to the message about `deltaforge` choose **Allow Anyway**.
3. Run it again and choose **Open Anyway** in the confirmation.

**Or in the terminal**, which does the same thing in one step:

```bash
xattr -d com.apple.quarantine /absolute/path/to/deltaforge
```

Do not apply that command recursively, and not to a binary whose checksum you have not
checked. Apple's [Gatekeeper guidance](https://support.apple.com/en-us/102445) explains
what overriding this protection means. A future release removes the step once Developer
ID signing and notarization are in place.

If you would rather not do any of this, `cargo install deltaforge` builds from source and
Gatekeeper never applies.

You need a working Rust toolchain (`cargo`) to build the projects, and Git if you want
DeltaForge to snapshot completed steps.

## Getting started

```bash
deltaforge
```

Pick a project from the catalog, choose where it goes, and DeltaForge creates the
repository and opens the workbench on the first step. A clean machine reaches its first
behavioral run in a few seconds of machine time.

`DELTAFORGE_NO_BROWSER=1` prints the local URL instead of opening a tab.
`DELTAFORGE_WORKSPACE` changes where new projects are created; the default is
`~/DeltaForge`.

The workbench runs in the background. Stop it cleanly from any directory with
`deltaforge exit`.

## The terminal, if you prefer it

Every browser action has a command behind it, and the two are the same operation: a run
started from either surface is indistinguishable to project state and to the event
stream. See [the command reference](https://github.com/eloidieme/DeltaForge/blob/main/docs/commands.md)
for the full list, and [the configuration reference](https://github.com/eloidieme/DeltaForge/blob/main/docs/config.md)
for per-project settings.

```bash
deltaforge init flashindex --lang rust   # scripting and CI
deltaforge test                          # run the current step's checks
deltaforge explain-failure               # the same diagnosis, in the terminal
deltaforge bench --save --compare        # measure, and compare with history
deltaforge status --json                 # machine-readable progress
deltaforge doctor                        # toolchain, packs, project health
```

## Safety

DeltaForge runs learner commands directly, never through a shell. Fixtures are copied to
temporary directories for checks and benchmarks and are treated as immutable inputs.
Project state lives under `.deltaforge/`.

The project list and private workbench discovery record live in the platform's
per-user application-data directory: `$XDG_DATA_HOME/deltaforge` (or
`~/.local/share/deltaforge`) on Linux and other Unix systems,
`~/Library/Application Support/DeltaForge` on macOS, and
`%LOCALAPPDATA%\DeltaForge` on Windows. `DELTAFORGE_HOME` overrides this path.

The workbench binds only to loopback, requires a per-service capability token, validates
Origin and Host on every request, refuses cross-origin mutations, and never serves
repository files. Browser requests name projects by opaque registry identifier and never
carry a filesystem path — with one deliberate, tightly bounded exception for project
creation, which is documented in
[the architecture decision](https://github.com/eloidieme/DeltaForge/blob/main/docs/product/architecture.md).
See [the safety model](https://github.com/eloidieme/DeltaForge/blob/main/docs/safety.md).

## Authoring packs

A project pack is a manifest plus a directory per step: a guide, black-box tests,
fixtures, a help ladder, and optionally benchmarks and a prediction prompt. See
[the pack format](https://github.com/eloidieme/DeltaForge/blob/main/docs/pack-format.md),
[the test format](https://github.com/eloidieme/DeltaForge/blob/main/docs/test-format.md),
[the content style guide](https://github.com/eloidieme/DeltaForge/blob/main/docs/content-style.md),
and [the pack-authoring guide](https://github.com/eloidieme/DeltaForge/blob/main/docs/authoring-packs.md).

Pack authoring is maintainer tooling in 1.0. It is documented, not promoted.

```bash
deltaforge pack new example --name "Example" --description "..." --dest packs
deltaforge pack add-stage --pack-dir packs/example 02_next --title "Next behavior"
deltaforge pack doctor example
deltaforge pack content example --stage 02_next   # exactly what a learner sees
deltaforge validate-pack example --strict
```

`deltaforge-pack-mcp` is a stdio MCP server exposing the same operations to an AI agent.
It returns structured `ok`/`blocked` reports with problems and next actions, so an agent
creates packs through scaffolding and constrained edits rather than guessing the format.
It is maintainer tooling that runs at your own privilege level and does not confine pack
paths, so never expose it to a client you trust less than your own shell — see the trust
boundary in [the pack-authoring guide](https://github.com/eloidieme/DeltaForge/blob/main/docs/authoring-packs.md).

Reference solutions under `tools/reference_solutions/` prove the bundled packs are
passable. They are never copied into a learner's project.

## Documentation

- [Quickstart](https://github.com/eloidieme/DeltaForge/blob/main/docs/quickstart.md) — the first ten minutes
- [Commands](https://github.com/eloidieme/DeltaForge/blob/main/docs/commands.md) — every command
- [Configuration](https://github.com/eloidieme/DeltaForge/blob/main/docs/config.md) — per-project configuration
- [Curriculum map](https://github.com/eloidieme/DeltaForge/blob/main/docs/curriculum-map.md) — what each project teaches
- [Product record](https://github.com/eloidieme/DeltaForge/tree/main/docs/product) — the 1.0 contract, the architecture decision, and the
  validation record

## License

MIT. See [LICENSE](https://github.com/eloidieme/DeltaForge/blob/main/LICENSE).
