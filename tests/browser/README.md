# Headless browser journey

`tests/browser_journey.rs` sends the HTTP requests the workbench page is
*supposed* to send. This sends none of its own: it drives `src/ui/app.js` in a
real browser and lets the page decide what to request.

That distinction is the whole reason this exists. In 1.0 the two shapes had
drifted — the page posted a prefilled `parent_directory` where the Rust mirror
posted its own temporary one — and project creation was broken on every machine
that was not the author's while the suite stayed green. A hand-maintained
mirror cannot detect that it has stopped mirroring.

## Running it

```
cd tests/browser
npm ci
npx playwright install --with-deps chromium
cargo build --manifest-path ../../Cargo.toml
node journey.mjs
```

`node journey.mjs` builds nothing. It expects a `deltaforge` binary, and takes
one from `$DELTAFORGE_BIN` if set, otherwise `target/debug/deltaforge`.

Set `HEADED=1` to watch it, and `KEEP=1` to leave the scratch directory behind
for inspection.

## What it asserts

The journey the contract calls the product: catalog → create with the defaults
→ a first failing run with a diagnosis → reveal a hint → pass → snapshot →
benchmark with a prediction → export the record. It runs against a home
directory and a workspace that do not exist when it starts, because that is the
state a new learner's machine is in.

Any uncaught page error or `console.error` fails the run, wherever it happens.
