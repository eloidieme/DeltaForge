## Problem

<!-- What user-visible or maintenance problem does this solve? -->

## Change

<!-- Summarize the approach and important tradeoffs. -->

## Verification

<!-- List commands, browser journeys, and platforms actually checked. -->

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `GIT_CONFIG_GLOBAL=/dev/null cargo test`
- [ ] `cargo run -- validate-pack --strict`
- [ ] User-visible changes are reflected in `CHANGELOG.md`
- [ ] No credentials, capability tokens, private paths, or learner code are included
