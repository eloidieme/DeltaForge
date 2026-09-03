/* Pure decisions the workbench page makes.
 *
 * Everything here is a function of its arguments alone: no `document`, no
 * `location`, no `fetch`, no module-level state. That is the whole point.
 * `app.js` is 1,100 lines of DOM and could only ever be exercised by driving a
 * browser; the decisions that were wrong in 1.0 — which route a path is, what
 * goes in a creation request body — were not DOM problems at all. They live
 * here so `node --test tests/ui` can execute them directly, on every commit,
 * without a browser.
 *
 * The browser loads this as a plain script before `app.js` and reads
 * `DeltaForgeCore` off the global object. Node requires it as a CommonJS
 * module. Neither environment is special-cased beyond the export block at the
 * bottom.
 */

/* -------------------------------------------------------------- routing */

/* Routes the page answers, in the order they are tried. A single table so the
 * page and its tests cannot disagree about what a path means. */
const PROJECT_VIEWS = ["overview", "build", "performance", "runs"];

function routeState(pathname) {
  const project = /^\/projects\/([^/]+)\/([a-z]+)$/.exec(pathname || "");
  if (project && PROJECT_VIEWS.includes(project[2])) {
    return { project: decodeURIComponent(project[1]), view: project[2] };
  }
  if (pathname === "/catalog") return { project: null, view: "catalog" };
  if (pathname === "/create") return { project: null, view: "create" };
  return { project: null, view: "projects" };
}

/* A route change is a page change even though the browser does not reload.
 * Keep the naming decision executable outside the DOM so every route can be
 * checked without relying on a screen reader or a browser tab. */
function pageTitle({ view, projectName, heading }) {
  const project = (projectName || "Project").trim();
  const page = {
    projects: "Projects",
    catalog: "Project catalog",
    overview: "Overview",
    performance: "Performance",
    runs: "Runs",
  }[view] || (heading || "DeltaForge").trim();
  return projectName && !["projects", "catalog", "create"].includes(view)
    ? `${page} — ${project} — DeltaForge`
    : page === "DeltaForge" ? page : `${page} — DeltaForge`;
}

/* A current stage sometimes also has status "current". Saying both produced
 * "current, current step" in the accessible name. Preserve a distinct
 * completed state, but collapse the duplicate word. */
function railAriaLabel(step) {
  const status = String(step.status || "").replace(/_/g, " ");
  const states = [];
  if (status && !(step.current && status === "current")) states.push(status);
  if (step.current) states.push("current step");
  return [`Step ${step.position}`, step.title, ...states].filter(Boolean).join(", ");
}

/* Health details are prose, not terminal output. They only use inline code,
 * so represent that small vocabulary explicitly and leave every other
 * character untouched. */
function plainTextBlocks(value) {
  const source = String(value || "");
  if (!source) return [];
  return source.split(/\n\s*\n/).map((paragraph) => ({
    kind: "paragraph",
    spans: inlineCodeSpans(paragraph),
  }));
}

function inlineCodeSpans(value) {
  const spans = [];
  let start = 0;
  for (const match of value.matchAll(/`([^`\n]+)`/g)) {
    if (match.index > start) spans.push({ kind: "text", text: value.slice(start, match.index) });
    spans.push({ kind: "code", text: match[1] });
    start = match.index + match[0].length;
  }
  if (start < value.length) spans.push({ kind: "text", text: value.slice(start) });
  return spans.length ? spans : [{ kind: "text", text: value }];
}

/* ------------------------------------------------------------- creation */

/* The body of `POST /api/v1/projects/preflight`.
 *
 * `parent_directory` is null whenever the learner has not chosen somewhere
 * else — an empty field, or a field still holding the default workspace this
 * page prefilled from `GET /api/v1/workspace`. Null is what tells the service
 * the directory is its own to create, which on a machine that has never run
 * DeltaForge it has to be. Sending the prefilled path back verbatim is what
 * made project creation impossible on every clean machine in 1.0: it read as
 * a location the learner had typed, and a typed location that does not exist
 * is a refusal.
 */
function preflightRequest(fields) {
  const parent = (fields.parentDirectory || "").trim();
  const workspace = (fields.defaultWorkspace || "").trim();
  const chosen = parent && parent !== workspace ? parent : null;
  return {
    pack: fields.pack,
    language: fields.language || "",
    parent_directory: chosen,
    name: (fields.name || "").trim(),
  };
}

/* The body of `POST /api/v1/projects`: the preflight body plus the one field
 * that only matters once something is written. */
function createRequest(fields) {
  return { ...preflightRequest(fields), git: Boolean(fields.git) };
}

/* ---------------------------------------------------------------- export */

const DeltaForgeCore = {
  PROJECT_VIEWS,
  routeState,
  pageTitle,
  railAriaLabel,
  plainTextBlocks,
  preflightRequest,
  createRequest,
};

if (typeof module === "object" && module.exports) module.exports = DeltaForgeCore;
else globalThis.DeltaForgeCore = DeltaForgeCore;
