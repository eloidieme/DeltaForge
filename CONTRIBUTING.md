# Contributing to DeltaForge

Thank you for helping make DeltaForge clearer, safer, or more useful to learners.

## Before opening a change

- Use an issue to discuss a substantial product or format change before implementing it.
- Use GitHub's private vulnerability reporting form for security issues; do not open a
  public issue. See [SECURITY.md](SECURITY.md).
- Keep the 1.0 constraints in mind: local and offline, Rust projects, no accounts or
  telemetry, and no AI in the learner experience.

## Development setup

Install Rust 1.88 or newer and Node.js 22. Then run:

```bash
cargo build --lib
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
GIT_CONFIG_GLOBAL=/dev/null cargo test
node --test "tests/ui/**/*.test.js"
cargo run -- validate-pack --strict
```

If you change a bundled pack, also run `cargo run -- pack check-reference`. If you change
the browser surface, run the Playwright journey in `tests/browser/` and inspect both
themes at desktop and narrow widths.

## Pull requests

Keep each pull request focused. Include the user-visible problem, the approach, tests,
and any platform-specific validation. Add a changelog entry for a user-visible change.
Do not commit generated `target/` contents, local service state, credentials, or learner
projects.

All contributions must follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). By submitting a
contribution, you agree that it is licensed under this repository's MIT license.
