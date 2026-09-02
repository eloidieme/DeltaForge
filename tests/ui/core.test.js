/* Unit tests for the workbench page's pure decisions.
 *
 * Run with `node --test tests/ui`. No browser, no bundler, no dependencies —
 * `src/ui/core.js` is a plain CommonJS-compatible script, which is what makes
 * this possible at all.
 *
 * These exist because 1.0 shipped 1,166 lines of client JavaScript that CI
 * never executed. `tests/browser_journey.rs` asserted the HTTP exchange the
 * page was *supposed* to make; nothing asserted the one it actually made.
 */

const test = require("node:test");
const assert = require("node:assert/strict");

const core = require("../../src/ui/core.js");

test("the route table maps every path the page serves", () => {
  assert.deepEqual(core.routeState("/"), { project: null, view: "projects" });
  assert.deepEqual(core.routeState("/projects"), { project: null, view: "projects" });
  assert.deepEqual(core.routeState("/catalog"), { project: null, view: "catalog" });
  assert.deepEqual(core.routeState("/create"), { project: null, view: "create" });

  for (const view of core.PROJECT_VIEWS) {
    assert.deepEqual(core.routeState(`/projects/abc123/${view}`), { project: "abc123", view });
  }
});

test("an unknown path falls back to the projects list rather than a blank screen", () => {
  assert.deepEqual(core.routeState("/projects/abc/nonsense"), { project: null, view: "projects" });
  assert.deepEqual(core.routeState("/projects/abc/build/extra"), { project: null, view: "projects" });
  assert.deepEqual(core.routeState("/nope"), { project: null, view: "projects" });
  assert.deepEqual(core.routeState(""), { project: null, view: "projects" });
  assert.deepEqual(core.routeState(undefined), { project: null, view: "projects" });
});

test("a project identifier is decoded, not passed through raw", () => {
  assert.equal(core.routeState("/projects/a%20b/build").project, "a b");
});

const FIELDS = {
  pack: "flashindex",
  language: "rust",
  parentDirectory: "/home/learner/DeltaForge",
  defaultWorkspace: "/home/learner/DeltaForge",
  name: "flashindex-rust",
  git: true,
};

/* P0-1. This is the assertion whose absence made the product unusable on every
 * machine but the author's: the page prefilled Location from the service and
 * posted the prefilled value straight back, which the service read as a
 * location the learner had typed. */
test("an untouched Location asks the service to use its own workspace", () => {
  assert.equal(core.preflightRequest(FIELDS).parent_directory, null);
});

test("an empty Location also asks the service to use its own workspace", () => {
  assert.equal(
    core.preflightRequest({ ...FIELDS, parentDirectory: "" }).parent_directory,
    null,
  );
  assert.equal(
    core.preflightRequest({ ...FIELDS, parentDirectory: "   " }).parent_directory,
    null,
  );
});

test("a Location the learner chose is sent as chosen", () => {
  assert.equal(
    core.preflightRequest({ ...FIELDS, parentDirectory: "/home/learner/code" }).parent_directory,
    "/home/learner/code",
  );
});

test("whitespace around a typed Location does not make it a different location", () => {
  assert.equal(
    core.preflightRequest({ ...FIELDS, parentDirectory: "  /home/learner/DeltaForge  " })
      .parent_directory,
    null,
  );
  assert.equal(
    core.preflightRequest({ ...FIELDS, parentDirectory: " /home/learner/code " }).parent_directory,
    "/home/learner/code",
  );
});

test("the preflight body carries exactly the fields the service reads", () => {
  assert.deepEqual(core.preflightRequest(FIELDS), {
    pack: "flashindex",
    language: "rust",
    parent_directory: null,
    name: "flashindex-rust",
  });
});

test("the create body is the preflight body plus git", () => {
  const preflight = core.preflightRequest(FIELDS);
  const create = core.createRequest(FIELDS);
  assert.deepEqual({ ...create, git: undefined }, { ...preflight, git: undefined });
  assert.equal(create.git, true);
  assert.equal(core.createRequest({ ...FIELDS, git: false }).git, false);
  assert.equal(core.createRequest({ ...FIELDS, git: undefined }).git, false);
});

test("a name is trimmed, and a missing language is sent as empty rather than undefined", () => {
  const body = core.preflightRequest({ ...FIELDS, name: "  spaced  ", language: undefined });
  assert.equal(body.name, "spaced");
  assert.equal(body.language, "");
});
