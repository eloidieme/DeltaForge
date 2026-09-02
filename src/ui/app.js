/* DeltaForge workbench client.
 *
 * The service is authoritative: this script never derives progress, never
 * decides whether a step is complete, and never invents a next action. It
 * renders `/api/v1/state`, which already carries the primary action, and it
 * replays the project's event journal so a run started from a terminal looks
 * identical to one started from this page.
 */

const token = __TOKEN_JSON__;

let activeProject = null;
let currentView = "projects";
let state = null;      // canonical WorkbenchState
let content = null;    // CapabilityContent
let events = null;     // EventSource
let busy = false;
let catalog = null;
let createChoice = null;
let preflightTimer = null;
let live = emptyLive();

function emptyLive() {
  return { active: false, kind: null, phase: "", passed: 0, failed: 0, current: 0, total: 0, started: 0 };
}

const $ = (selector) => document.querySelector(selector);
const SCREENS = [
  "projects-screen", "catalog-screen", "create-screen", "overview-screen",
  "build-screen", "performance-screen", "runs-screen", "health-screen",
];

/* ---------------------------------------------------------------- theme */

const THEME_KEY = "deltaforge-theme";

function applyTheme(theme) {
  if (theme === "light" || theme === "dark") document.documentElement.dataset.theme = theme;
  else delete document.documentElement.dataset.theme;
  const label = theme === "light" ? "Light" : theme === "dark" ? "Dark" : "System";
  $("#theme-toggle").textContent = label;
  $("#theme-toggle").setAttribute("aria-label", `Colour theme: ${label}. Activate to change.`);
}

function readTheme() {
  try { return localStorage.getItem(THEME_KEY) || "system"; } catch { return "system"; }
}

function cycleTheme() {
  const order = ["system", "light", "dark"];
  const next = order[(order.indexOf(readTheme()) + 1) % order.length];
  try { localStorage.setItem(THEME_KEY, next); } catch { /* private browsing */ }
  applyTheme(next);
}

/* ------------------------------------------------------------ transport */

// The token never appears in a URL this function builds: it is sent as a
// header instead (see `fetchJson`/`post`) so it never reaches the address
// bar, browser history, or a "copy link address". The one exception is
// `EventSource`, which cannot set custom headers at all — `eventSourceUrl`
// below is the only place the token still travels in a query string.
function api(path, project = true) {
  const params = new URLSearchParams();
  if (project && activeProject) params.set("project", activeProject);
  const query = params.toString();
  return query ? `${path}?${query}` : path;
}

function eventSourceUrl(path, project = true) {
  const params = new URLSearchParams({ token });
  if (project && activeProject) params.set("project", activeProject);
  return `${path}?${params}`;
}

async function fetchJson(path, project = true) {
  const response = await fetch(api(path, project), { headers: { "X-DeltaForge-Token": token } });
  if (!response.ok) {
    // The service answers a failed read with the same actionable text the CLI
    // prints for it, so prefer that over a generic "unavailable".
    const detail = await response.json().catch(() => ({}));
    throw new Error(detail.error || `${path} is unavailable`);
  }
  return response.json();
}

async function post(path, body = {}, project = true) {
  const response = await fetch(api(path, project), {
    method: "POST",
    headers: { "Content-Type": "application/json", "X-DeltaForge-Token": token },
    body: JSON.stringify(body),
  });
  if (!response.ok) {
    const detail = await response.json().catch(() => ({}));
    throw new Error(detail.error || "That did not work");
  }
  return response.json();
}

/* -------------------------------------------------------------- routing */

function projectRoute(view) { return `/projects/${activeProject}/${view}`; }
// No token in the href. Every click here is intercepted and routed in-page, so
// the token bought nothing except a live capability sitting in the DOM, ready
// to be handed out by "Copy link address" or carried into a new tab's history.
function routeLink(element, path) {
  element.href = path;
  element.dataset.route = path;
}
function navigate(path) {
  // No token here: every fetch this page makes sends it as a header instead
  // (see `fetchJson`/`post`), so the address bar and the browser's history
  // never carry it.
  history.pushState({}, "", path);
  renderRoute();
}
function setScreen(id) {
  for (const name of SCREENS) $("#" + name).hidden = name !== id;
}
function closeEvents() { if (events) events.close(); events = null; }

function routeState() {
  const project = location.pathname.match(/^\/projects\/([^/]+)\/(overview|build|performance|runs)$/);
  if (project) return { project: project[1], view: project[2] };
  if (location.pathname === "/catalog") return { project: null, view: "catalog" };
  if (location.pathname === "/create") return { project: null, view: "create" };
  return { project: null, view: "projects" };
}

async function renderRoute() {
  const route = routeState();
  closeEvents();
  state = null; content = null; live = emptyLive();
  activeProject = route.project;
  currentView = route.view;

  $("#projects-nav").classList.toggle("active", currentView === "projects");
  $("#catalog-nav").classList.toggle("active", currentView === "catalog" || currentView === "create");

  if (!activeProject) {
    $("#project-shell").hidden = true;
    if (currentView === "catalog") { await loadCatalog(); return; }
    if (currentView === "create") { await renderCreate(); return; }
    await loadProjects();
    connectAppEvents();
    return;
  }
  $("#project-shell").hidden = false;
  for (const view of ["overview", "build", "performance", "runs"]) {
    const nav = $(`#${view}-nav`);
    routeLink(nav, projectRoute(view));
    nav.classList.toggle("active", view === currentView);
  }
  await loadProject();
}

/* ------------------------------------------------------------- projects */

async function loadProjects() {
  setScreen("projects-screen");
  const projects = await fetchJson("/api/v1/projects", false);
  $("#projects-empty").hidden = projects.length !== 0;
  $("#project-grid").replaceChildren(...projects.map(projectCard));
}

function projectCard(project) {
  const card = element("article", "project-card");
  const percent = project.total_steps ? Math.round((project.completed_steps / project.total_steps) * 100) : 0;
  const healthy = project.status === "healthy";

  const head = element("div", "project-card-head");
  const identity = element("div");
  identity.append(
    text(element("div", "eyebrow"), project.language || "Project"),
    text(element("h2"), project.name),
    text(element("p", null, "color:var(--text-2);font-size:.88rem;margin-top:4px"), project.description),
  );
  head.append(identity, text(element("span", `tag ${healthy ? "" : "attention"}`), healthy ? "Ready" : "Needs attention"));

  const progress = element("div");
  const label = element("div", "progress-label");
  label.append(
    text(element("span"), project.total_steps ? `${project.completed_steps} of ${project.total_steps} steps complete` : "Not started"),
    text(element("span"), `${percent}%`),
  );
  const track = element("div", "progress-track");
  const fill = element("div", "progress-fill");
  fill.style.width = `${percent}%`;
  track.append(fill);
  progress.append(label, track);

  const actions = element("div", "card-actions");
  const overview = text(element("button", "button"), "Overview");
  overview.onclick = () => navigate(`/projects/${project.id}/overview`);
  const build = text(element("button", "button primary"), healthy ? "Continue building" : "Open");
  build.onclick = () => navigate(`/projects/${project.id}/${healthy ? "build" : "overview"}`);
  actions.append(overview, build);

  card.append(head, progress, actions);
  return card;
}

/* -------------------------------------------------------------- catalog */

async function loadCatalog() {
  setScreen("catalog-screen");
  $("#catalog-error").hidden = true;
  try {
    catalog = await fetchJson("/api/v1/catalog", false);
  } catch (error) {
    $("#catalog-error").hidden = false;
    $("#catalog-error-text").textContent = error.message;
    return;
  }
  $("#catalog-grid").replaceChildren(...catalog.map(catalogCard));
}

function catalogCard(entry) {
  const card = element("article", `catalog-card ${entry.tier}`);

  const head = element("div", "catalog-head");
  const identity = element("div");
  identity.append(text(element("h2"), entry.name), text(element("p", null, "margin-top:4px"), entry.description));
  head.append(identity, text(element("span", `tag ${entry.tier === "flagship" ? "flagship" : ""}`), entry.tier_label));

  const facts = element("div", "catalog-facts");
  facts.append(text(element("span"), `${entry.step_count} steps`));
  if (entry.difficulty) facts.append(text(element("span"), entry.difficulty));
  if (entry.estimated_hours) facts.append(text(element("span"), `${entry.estimated_hours.low}–${entry.estimated_hours.high} hours`));
  const available = entry.languages.filter((language) => language.available);
  facts.append(text(element("span"), available.length ? available.map((language) => language.name).join(", ") : "Toolchain missing"));

  const topics = element("div", "catalog-meta");
  topics.append(...entry.topics.slice(0, 4).map((topic) => text(element("span", "tag"), topic)));

  const actions = element("div", "catalog-actions");
  // The flagship carries the page's one filled control; preview packs are
  // offered on equal footing but not urged.
  const start = text(element("button", `button ${entry.tier === "flagship" ? "primary" : ""}`), "Start this project");
  start.disabled = available.length === 0;
  start.onclick = () => navigate(`/create?pack=${encodeURIComponent(entry.id)}`);
  actions.append(start);
  if (available.length === 0) {
    const missing = entry.languages
      .flatMap((language) => language.tools)
      .filter((tool) => tool.required && !tool.found)
      .map((tool) => tool.label);
    actions.append(text(element("span", "activity"), `Needs ${[...new Set(missing)].join(", ")}`));
  }

  card.append(head, facts, topics, actions);
  return card;
}

/* ------------------------------------------------------------- creation */

async function renderCreate() {
  setScreen("create-screen");
  if (!catalog) {
    try { catalog = await fetchJson("/api/v1/catalog", false); } catch { catalog = []; }
  }
  const wanted = new URLSearchParams(location.search).get("pack");
  const entry = catalog.find((candidate) => candidate.id === wanted) || catalog[0];
  if (!entry) {
    $("#create-title").textContent = "No projects are available";
    $("#create-lede").textContent = "DeltaForge could not find any project packs on this machine.";
    return;
  }
  createChoice = {
    entry,
    language: (entry.languages.find((language) => language.available) || entry.languages[0] || {}).id,
  };

  $("#create-title").textContent = `Create ${entry.name}`;
  $("#create-lede").textContent = `${entry.description}. ${entry.step_count} steps${entry.estimated_hours ? `, roughly ${entry.estimated_hours.low} to ${entry.estimated_hours.high} hours` : ""}.`;
  $("#create-name").value = `${entry.id}-${createChoice.language || "project"}`;
  renderLanguageChoices();

  if (!$("#create-parent").value) {
    const workspace = await fetchJson("/api/v1/workspace", false).catch(() => ({}));
    if (workspace.default_directory) $("#create-parent").value = workspace.default_directory;
  }
  schedulePreflight(0);
}

function renderLanguageChoices() {
  const { entry } = createChoice;
  $("#create-languages").replaceChildren(...entry.languages.map((language) => {
    const missing = language.tools.filter((tool) => tool.required && !tool.found);
    const choice = element("label", `choice ${language.id === createChoice.language ? "selected" : ""} ${language.available ? "" : "unavailable"}`);
    const radio = element("input");
    radio.type = "radio";
    radio.name = "create-language";
    radio.checked = language.id === createChoice.language;
    radio.disabled = !language.available;
    radio.onchange = () => {
      createChoice.language = language.id;
      $("#create-name").value = `${entry.id}-${language.id}`;
      renderLanguageChoices();
      schedulePreflight(0);
    };
    const copy = element("div", "choice-copy");
    copy.append(text(element("strong"), language.name));
    copy.append(text(element("small"), missing.length ? `Needs ${missing.map((tool) => tool.label).join(", ")}` : "Ready on this machine"));
    choice.append(radio, copy);
    return choice;
  }));
}

function schedulePreflight(delay = 350) {
  clearTimeout(preflightTimer);
  preflightTimer = setTimeout(runPreflight, delay);
}

async function runPreflight() {
  if (!createChoice) return;
  let result;
  try {
    result = await post("/api/v1/projects/preflight", {
      pack: createChoice.entry.id,
      language: createChoice.language || "",
      parent_directory: $("#create-parent").value.trim() || null,
      name: $("#create-name").value.trim(),
    }, false);
  } catch (error) {
    $("#preflight-tools").replaceChildren();
    $("#preflight-location-label").textContent = "Cannot create here";
    $("#preflight-path").textContent = error.message;
    $("#preflight-location").className = "ruled contradiction";
    $("#create-submit").disabled = true;
    return;
  }

  $("#preflight-tools").replaceChildren(...result.tools.map((tool) => {
    const row = element("li");
    const mark = text(element("span", `preflight-mark ${tool.found ? "ok" : tool.required ? "missing" : ""}`), tool.found ? "✓" : "✗");
    const copy = element("span");
    copy.append(text(element("div"), tool.found ? `${tool.label} — ${tool.version}` : `${tool.label} not found${tool.required ? "" : " (optional)"}`));
    if (!tool.found && tool.install_url) copy.append(text(element("div", null, "color:var(--text-3);font-size:.78rem"), tool.install_url));
    row.append(mark, copy);
    return row;
  }));

  $("#preflight-location").className = `ruled ${result.location.ok ? "proven" : "contradiction"}`;
  $("#preflight-location-label").textContent = result.location.ok ? "Will be created at" : "Cannot create here";
  $("#preflight-path").textContent = result.location.ok ? result.location.target : result.location.problem;
  $("#create-submit").disabled = !result.ok;
}

async function submitCreate(event) {
  event.preventDefault();
  if (busy || !createChoice) return;
  busy = true;
  $("#create-submit").disabled = true;
  $("#create-activity").textContent = "Creating the project…";
  try {
    const created = await post("/api/v1/projects", {
      pack: createChoice.entry.id,
      language: createChoice.language || "",
      parent_directory: $("#create-parent").value.trim() || null,
      name: $("#create-name").value.trim(),
      git: $("#create-git").checked,
    }, false);
    $("#create-activity").textContent = `Created at ${created.path}`;
    navigate(`/projects/${created.project_id}/build`);
  } catch (error) {
    $("#create-activity").textContent = error.message;
    $("#create-submit").disabled = false;
  } finally {
    busy = false;
  }
}

/* ------------------------------------------------------------- a project */

async function loadProject() {
  const health = await fetchJson("/api/v1/project-health");
  if (health.status !== "healthy") { renderHealth(health); return; }
  let next, latest;
  try {
    [next, latest] = await Promise.all([fetchJson("/api/v1/capability"), fetchJson("/api/v1/state")]);
  } catch (error) {
    // Health passed but a project endpoint still could not answer. Show the
    // recovery screen with what went wrong instead of letting this reject all
    // the way out and replace the page with a bare browser error.
    renderHealth({
      status: "unhealthy",
      issue: {
        title: "DeltaForge could not open this project",
        detail: error.message,
        guidance: "Resolve the problem described below, then check again.",
      },
      actions: [],
    });
    return;
  }
  state = latest; content = next;
  $("#project-name").textContent = next.project_overview.name;
  $("#project-meta").textContent = `${state.language} · ${completedSteps()} of ${next.roadmap.length} steps complete`;
  renderCurrentView();
  connectEvents(state.event_cursor);
}

function completedSteps() {
  return content ? content.roadmap.filter((step) => step.status === "complete").length : 0;
}

function renderCurrentView() {
  if (currentView === "overview") { setScreen("overview-screen"); renderOverview(); }
  if (currentView === "build") { setScreen("build-screen"); renderBuild(); }
  if (currentView === "performance") { setScreen("performance-screen"); renderPerformance(); }
  if (currentView === "runs") { setScreen("runs-screen"); renderRuns(); }
}

function renderHealth(health) {
  setScreen("health-screen");
  const issue = health.issue || {};
  $("#health-title").textContent = issue.title || "DeltaForge cannot load this project";
  $("#health-guidance").textContent = issue.guidance || "Resolve this problem, then check again.";
  $("#health-detail").textContent = issue.detail || "The project health check failed.";
  $("#health-repin").hidden = !health.actions.some((action) => action.kind === "repin_pack");
}

/* -------------------------------------------------------------- overview */

function renderOverview() {
  const overview = content.project_overview;
  $("#overview-name").textContent = overview.name;
  $("#overview-description").textContent = overview.description;
  $("#overview-topics").replaceChildren(...(overview.topics || []).map((topic) => text(element("span", "tag"), topic)));
  const current = content.roadmap.find((step) => step.current);
  $("#overview-progress").textContent = `${completedSteps()} of ${content.roadmap.length} steps complete`;
  $("#overview-current").textContent = current
    ? (current.status === "complete" ? `Ready to continue past: ${current.title}` : `Next: ${current.title}`)
    : "Every project step is complete.";
  $("#overview-step-count").textContent = `${content.roadmap.length} project steps`;
  renderRail("#overview-rail");
  $("#overview-guide").replaceChildren(...(overview.sections || []).map(renderGuideSection));
}

function renderBlock(block) {
  if (block.kind === "code") {
    const pre = text(element("pre"), block.content);
    if (block.language) pre.dataset.language = block.language;
    return pre;
  }
  if (block.kind === "list") {
    const list = element("ul");
    list.replaceChildren(...block.items.map((item) => text(element("li"), item)));
    return list;
  }
  return text(element("p"), block.text);
}

function renderGuideSection(section) {
  const node = element("section", "guide-section");
  node.append(text(element("h2"), section.title), ...section.blocks.map(renderBlock));
  return node;
}

/* ------------------------------------------------------------- the rail */

function gateMarkerFor(stageId) {
  const markers = state && state.performance ? state.performance.roadmap : [];
  return markers.find((marker) => marker.stage_id === stageId);
}

function renderRail(selector) {
  $(selector).replaceChildren(...content.roadmap.map((step) => {
    // A passed step the learner has not yet advanced past is both complete and
    // where they are, so the rail carries both marks.
    const row = element("li", `rail-step ${step.status}${step.current ? " here" : ""}`);
    const node = text(element("span", "rail-node"), step.status === "complete" ? "✓" : step.current ? "●" : "");
    node.setAttribute("aria-hidden", "true");
    const copy = element("div", "rail-copy");
    const title = element("div", "rail-title");
    title.append(text(element("span"), `${step.position}. ${step.title}`));
    const marker = gateMarkerFor(step.id);
    if (marker && marker.has_benchmarks) {
      const gate = element("span", `rail-gate ${marker.status || ""}`);
      gate.title = marker.status
        ? `Performance target: ${marker.status.replace(/_/g, " ")}`
        : "This step is measured";
      title.append(gate);
    }
    copy.append(title, text(element("div", "rail-summary"), step.summary));
    row.append(node, copy);
    // The status word is what a screen reader gets; the node glyph is decorative.
    row.setAttribute(
      "aria-label",
      `Step ${step.position}, ${step.title}, ${step.status}${step.current ? ", current step" : ""}`,
    );
    return row;
  }));
}

/* ----------------------------------------------------------------- build */

function renderBuild() {
  renderRail("#build-rail");
  const current = content.roadmap.find((step) => step.current);
  $("#step-position").textContent = current
    ? `Step ${current.position} of ${content.roadmap.length}`
    : "Project complete";
  $("#instruction-title").textContent = content.title;
  $("#instruction-summary").textContent = content.mission;
  // Each authored section goes into the panel named for it. A pack that adds
  // a section DeltaForge has no panel for is simply not shown here.
  for (const section of content.sections) {
    const target = $(`#section-${section.key}`);
    if (target) target.replaceChildren(...section.blocks.map(renderBlock));
  }
  renderHints();
  renderState();
}

function renderHints() {
  $("#help-levels").replaceChildren(...content.revealed_help.map((hint) => {
    const panel = element("div", "help-level");
    panel.append(text(element("strong"), `${hint.level}. ${hint.label}`), text(element("p"), hint.content));
    return panel;
  }));
  const revealed = content.revealed_help.length;
  const completed = state && state.capability.completed;
  // The service decides how many levels are reachable right now and says so.
  // Re-deriving it here as `min(levels, 4)` matched the flagship's five-rung
  // ladder and was wrong for every three-rung pack, leaving an enabled button
  // that the service then refused.
  const available = content.available_help_levels;
  const reveal = $("#reveal-help");
  reveal.disabled = busy || revealed >= available;
  reveal.textContent = revealed >= available
    ? (completed ? "All help shown" : "The last level unlocks once this step passes")
    : `Show help ${revealed + 1} of ${content.help_levels}`;
}

function renderState() {
  if (!state) return;
  const running = live.active || Boolean(state.active_job);
  const action = $("#primary-action");
  action.disabled = busy || (!running && !state.primary_action.enabled);
  action.textContent = running ? `Cancel ${live.kind === "benchmarks" ? "benchmark" : "run"}` : state.primary_action.label;
  $("#activity").textContent = state.freshness === "stale"
    ? "Your source changed after the last result"
    : `Last activity · ${formatTimestamp(state.last_activity_at)}`;

  renderResumption();
  renderGateNotice();
  renderSnapshotAction();

  const failure = state.primary_failure;
  const fresh = state.freshness === "fresh";
  $("#focused-rerun").hidden = !(failure && fresh && !state.active_job);
  $("#focused-rerun").dataset.test = failure ? failure.name : "";

  if (running) {
    showResult(live.kind === "benchmarks" ? "Measuring" : "Checks are running", live.phase || "Preparing your project…", "running");
  } else if (state.freshness === "stale") {
    showResult("Code changed", "The previous result is kept, but it no longer describes the code on disk.", "attention");
  } else if (fresh && state.capability.completed) {
    showResult("Step complete", content.capability_statement, "proven");
  } else if (fresh && failure) {
    const diagnosis = failure.diagnosis;
    showResult(`Fix this first · ${diagnosis ? diagnosis.headline : failure.name}`,
      diagnosis ? diagnosis.summary : (failure.failures?.[0] || "This check failed."), "contradiction");
  } else if (fresh) {
    showResult("Checks are up to date", "These results match the source currently on disk.", "proven");
  } else {
    showResult("No checks run yet", "Run the checks to see what already works and what to tackle first.", "");
  }

  $("#run-bar").hidden = !running;
  renderDiagnosis(failure && fresh ? failure : null);
  renderOtherFailures();
  renderMeter();
}

function renderResumption() {
  const resumption = state.resumption;
  const visible = Boolean(resumption) && resumption.action_pending;
  $("#resumption").hidden = !visible;
  if (!visible) return;
  $("#resumption-title").textContent = resumption.title;
  $("#resumption-detail").textContent = resumption.detail;
}

function renderGateNotice() {
  const performance = state.performance || {};
  const notice = $("#gate-notice");
  if (!performance.gate_status) { notice.hidden = true; return; }
  notice.hidden = false;
  const blocking = performance.gate_blocks_progress;
  if (performance.gate_status === "passed") {
    notice.className = "notice proven";
    $("#gate-notice-title").textContent = "Performance target met. ";
    $("#gate-notice-detail").textContent = "This step's measurement is current and passing.";
  } else if (performance.gate_status === "not_yet") {
    notice.className = "notice error";
    $("#gate-notice-title").textContent = "Performance target not met yet. ";
    $("#gate-notice-detail").textContent = blocking
      ? "This step needs to be faster before the next step unlocks."
      : "The behavior is right; the measurement is not there yet.";
  } else {
    notice.className = "notice measure";
    $("#gate-notice-title").textContent = "This step is measured. ";
    $("#gate-notice-detail").textContent = blocking
      ? "Measure it against the current source before continuing."
      : "It carries a performance target that has not been measured against the current source.";
  }
}

async function renderSnapshotAction() {
  const button = $("#snapshot-action");
  $("#snapshot-notice").hidden = true;
  if (!state.capability.completed || state.freshness !== "fresh" || state.active_job) {
    button.hidden = true;
    return;
  }
  let preview;
  try { preview = await fetchJson("/api/v1/snapshots/preview"); } catch { button.hidden = true; return; }
  button.hidden = !preview.available;
  if (!preview.available) return;
  const count = preview.changed_files.length;
  button.textContent = `Snapshot this step (${count} ${count === 1 ? "file" : "files"})`;
  button.title = `${preview.message}\n\n${preview.changed_files.slice(0, 12).map((file) => `${file.change}: ${file.path}`).join("\n")}`;
}

function showResult(title, copy, tone) {
  $("#result-card").className = `result-card ${tone}`;
  $("#result-title").textContent = title;
  $("#result-copy").textContent = copy;
}

function renderDiagnosis(failure) {
  const diagnosis = failure && failure.diagnosis;
  $("#diagnosis").hidden = !diagnosis;
  if (!diagnosis) return;
  $("#diagnosis-requirement").textContent = diagnosis.contract || diagnosis.summary;
  $("#diagnosis-expected").textContent = diagnosis.expected || "The requirement should be satisfied.";
  $("#diagnosis-actual").textContent = diagnosis.actual || failure.failures?.join("\n") || "The check failed.";
  const fixture = [diagnosis.fixture, ...(diagnosis.fixture_entries || [])].filter(Boolean).join(" · ");
  $("#diagnosis-fixture").textContent = fixture;
  $("#diagnosis-fixture-row").hidden = !fixture;
}

function renderOtherFailures() {
  const failures = state.latest_run?.failed_tests?.slice(1) || [];
  const visible = failures.length > 0 && state.freshness === "fresh" && !state.active_job;
  $("#other-failures").hidden = !visible;
  if (!visible) return;
  $("#other-failures-title").textContent = `${failures.length} other failing check${failures.length === 1 ? "" : "s"}`;
  $("#other-failures-list").replaceChildren(...failures.map((failure) => {
    const item = element("li");
    const row = element("div", "other-failure-row");
    const button = text(element("button", "button small"), "Rerun");
    button.dataset.test = failure.name;
    row.append(text(element("span"), failure.diagnosis ? failure.diagnosis.headline : failure.name), button);
    item.append(row);
    return item;
  }));
}

function renderMeter() {
  if (!live.active) {
    const run = state && state.freshness === "fresh" ? state.latest_run : null;
    $("#run-meter").textContent = run ? `${run.passed} passed · ${run.failed} failed` : "";
    return;
  }
  const seconds = live.started ? Math.floor((performance.now() - live.started) / 1000) : 0;
  const count = live.total ? ` · ${live.current}/${live.total}` : "";
  const tally = live.kind === "benchmarks" ? "" : ` · ${live.passed} passed · ${live.failed} failed`;
  $("#run-meter").textContent = `${live.phase || "Running"}${count} · ${seconds}s${tally}`;
}

/* ----------------------------------------------------------- performance */

function renderPerformance() {
  const performance = state.performance || {};
  // The heading is the step's own name. Several step titles already begin with
  // a verb, so prefixing one here reads as a stutter.
  $("#performance-title").textContent = content.title;
  $("#performance-lede").textContent = performance.has_benchmarks
    ? "Benchmarks run against a fixed corpus on this machine. Results are saved, so every later run is compared with the one before it."
    : "This step has no benchmark. The steps that are measured are listed below.";

  const running = live.active && live.kind === "benchmarks";
  const otherRunning = Boolean(state.active_job) && !running;
  const button = $("#benchmark-action");
  button.hidden = !performance.has_benchmarks;
  button.disabled = busy || otherRunning;
  button.textContent = running ? "Cancel the benchmark" : "Run the benchmark";
  $("#benchmark-bar").hidden = !running;
  $("#benchmark-activity").textContent = running
    ? live.phase
    : otherRunning ? "Checks are running; wait for them to finish." : "";

  renderPredictionSection(performance);
  renderGates(performance);
  renderMeasurements(performance);
  renderReflectionSection(performance);
  renderPerformanceRoadmap(performance);
}

function renderPredictionSection(performance) {
  const section = $("#prediction-section");
  const prompt = performance.prediction_prompt;
  section.hidden = !prompt;
  if (!prompt) return;
  $("#prediction-prompt").textContent = prompt;
  const recorded = performance.prediction;
  const answered = Boolean(recorded);
  $("#prediction-recorded").hidden = !answered || recorded.skipped;
  if (answered && !recorded.skipped) $("#prediction-text").textContent = recorded.text;
  $("#prediction-form").hidden = answered;
}

function renderReflectionSection(performance) {
  // A reflection is only worth offering once there is something to reflect on.
  const section = $("#reflection-section");
  section.hidden = !performance.has_benchmarks || performance.latest.length === 0;
  if (section.hidden) return;
  const recorded = performance.reflection;
  const answered = Boolean(recorded);
  $("#reflection-recorded").hidden = !answered || recorded.skipped;
  if (answered && !recorded.skipped) $("#reflection-text").textContent = recorded.text;
  $("#reflection-form").hidden = answered;
}

function renderGates(performance) {
  const section = $("#gates-section");
  section.hidden = performance.gates.length === 0;
  if (section.hidden) return;
  const status = performance.gate_status || "not_measured";
  const tag = $("#gate-status-tag");
  tag.textContent = status === "passed" ? "Met" : status === "not_yet" ? "Not met yet" : "Not measured";
  tag.className = `tag ${status === "passed" ? "proven" : status === "not_yet" ? "contradiction" : "measure"}`;

  $("#gates-panel").replaceChildren(...performance.gates.map((gate) => {
    const row = element("div", "gate-row");
    const head = element("div", "gate-head");
    head.append(text(element("strong"), gate.name),
      text(element("span", `tag ${gate.passed ? "proven" : status === "not_measured" ? "measure" : "contradiction"}`),
        gate.measured === null || gate.measured === undefined
          ? "not measured"
          : `measured ${formatNumber(gate.measured)}`));
    const requirement = text(element("div", "gate-requirement"),
      `${gate.metric.replace(/_/g, " ")} ${gate.comparison} ${formatNumber(gate.bound)}${gate.params_label ? ` at ${gate.params_label}` : ""}`);
    row.append(head, requirement);
    if (!gate.passed && gate.advice.length) {
      const advice = element("ul", "gate-advice");
      advice.replaceChildren(...gate.advice.map((line) => text(element("li"), line)));
      row.append(advice);
    }
    return row;
  }));
}

function renderMeasurements(performance) {
  const container = $("#measurements");
  if (!performance.latest.length) {
    const empty = element("div", "empty-state");
    empty.append(
      text(element("h2"), performance.has_benchmarks ? "No measurement saved yet" : "Nothing to measure here"),
      text(element("p"), performance.has_benchmarks
        ? "Run the benchmark and the numbers appear here. Every later run is then compared against the one before it."
        : "The steps that carry a benchmark are listed below."),
    );
    container.replaceChildren(empty);
    $("#measurement-timestamp").textContent = "";
    return;
  }
  $("#measurement-timestamp").textContent = `Measured ${formatTimestamp(performance.latest[0].timestamp)}`;
  container.replaceChildren(...performance.latest.map((benchmark) => {
    const panel = element("div", "panel");
    panel.append(text(element("h3"), benchmark.name));
    const scroll = element("div", "table-scroll", "margin-top:10px");
    const table = element("table", "measure");
    const head = element("thead");
    const headRow = element("tr");
    for (const label of ["Parameters", "Median ms", "P95 ms", "MB/s", "Peak MB", "Change"]) {
      headRow.append(text(element("th"), label));
    }
    head.append(headRow);
    const body = element("tbody");
    for (const point of benchmark.points) {
      const row = element("tr");
      row.append(text(element("td"), point.params_label || "—"));
      if (!point.success) {
        const cell = text(element("td", null, "text-align:left;color:var(--contradiction)"), point.error || "This measurement failed.");
        cell.colSpan = 5;
        row.append(cell);
      } else {
        for (const value of [point.runtime_median_ms, point.runtime_p95_ms, point.throughput_mb_s, point.peak_memory_mb]) {
          row.append(text(element("td", "numeric"), value === null || value === undefined ? "—" : formatNumber(value)));
        }
        const delta = point.median_percent_delta;
        const cell = element("td", "numeric");
        if (delta === null || delta === undefined) cell.textContent = "—";
        else {
          cell.className = `numeric delta ${delta < 0 ? "faster" : "slower"}`;
          cell.textContent = `${delta > 0 ? "+" : ""}${delta.toFixed(1)}%`;
          cell.title = "Change in median runtime since the previous saved run";
        }
        row.append(cell);
      }
      body.append(row);
    }
    table.append(head, body);
    scroll.append(table);
    panel.append(scroll);
    return panel;
  }));
}

function renderPerformanceRoadmap(performance) {
  const measured = (performance.roadmap || []).filter((marker) => marker.has_benchmarks);
  const section = $("#performance-roadmap-section");
  section.hidden = measured.length === 0;
  if (section.hidden) return;
  $("#performance-roadmap").replaceChildren(...measured.map((marker) => {
    const step = content.roadmap.find((candidate) => candidate.id === marker.stage_id);
    const row = element("li", `rail-step ${step ? step.status : "upcoming"}${step && step.current ? " here" : ""}`);
    const node = text(element("span", "rail-node"), step && step.status === "complete" ? "✓" : "");
    node.setAttribute("aria-hidden", "true");
    const copy = element("div", "rail-copy");
    const title = element("div", "rail-title");
    title.append(text(element("span"), step ? `${step.position}. ${step.title}` : marker.stage_id));
    if (marker.status) title.append(text(element("span", `tag ${marker.status === "passed" ? "proven" : marker.status === "not_yet" ? "contradiction" : "measure"}`), marker.status.replace(/_/g, " ")));
    copy.append(title);
    row.append(node, copy);
    return row;
  }));
}

/* ------------------------------------------------------------------ runs */

function renderRuns() {
  const rows = (state.attempt_history || []).slice().reverse().map(runRow);
  if (!rows.length) {
    const empty = element("div", "empty-state");
    empty.append(text(element("h2"), "No runs yet"), text(element("p"), "Run the checks from the Build page and every run will be listed here."));
    rows.push(empty);
  }
  $("#runs-list").replaceChildren(...rows);
}

function runRow(run) {
  const row = element("article", `run-row ${run.status}`);
  const kind = run.kind === "benchmarks" ? "Benchmark" : "Checks";
  const headline = {
    passed: `${kind} passed`,
    failed: `${kind} failed`,
    cancelled: `${kind} cancelled`,
    interrupted: `${kind} interrupted`,
    running: `${kind} running`,
  }[run.status] || kind;

  const copy = element("div");
  copy.append(text(element("h3"), headline));
  const detail = [formatTimestamp(run.started_at), (run.stage_ids || []).join(", "), duration(run)]
    .filter(Boolean).join(" · ");
  copy.append(text(element("p"), detail));
  if (run.error) copy.append(text(element("p", null, "color:var(--contradiction);margin-top:4px"), run.error.split("\n")[0]));

  const counts = text(element("div", "run-counts"),
    run.kind === "benchmarks"
      ? `${run.passed} measured · ${run.failed} failed`
      : `${run.passed} passed · ${run.failed} failed`);
  row.append(copy, counts);
  return row;
}

function duration(run) {
  if (!run.finished_at || !run.started_at) return "";
  const elapsed = (new Date(run.finished_at) - new Date(run.started_at)) / 1000;
  return Number.isFinite(elapsed) && elapsed >= 0 ? `${elapsed.toFixed(1)}s` : "";
}

/* ---------------------------------------------------------------- events */

function connectEvents(cursor) {
  events = new EventSource(eventSourceUrl("/api/v1/events") + `&after=${cursor}`);
  events.addEventListener("state", (event) => {
    state = JSON.parse(event.data);
    if (currentView === "build") renderState();
    if (currentView === "performance") renderPerformance();
    if (currentView === "runs") renderRuns();
  });
  events.addEventListener("run", (event) => handleRun(JSON.parse(event.data)));
  events.addEventListener("stream_error", (event) => {
    const data = JSON.parse(event.data || "{}");
    if ($("#activity")) $("#activity").textContent = data.error || "The live update stream stopped.";
  });
  events.addEventListener("gap", () => {
    if ($("#activity")) $("#activity").textContent = "Earlier output was dropped; showing what's current.";
  });
  events.addEventListener("focus", (event) => {
    const data = JSON.parse(event.data || "{}");
    if (data.route && data.route !== location.pathname) navigate(data.route);
    window.focus();
  });
  events.onerror = () => { if ($("#activity")) $("#activity").textContent = "Reconnecting…"; };
}

function connectAppEvents() {
  events = new EventSource(eventSourceUrl("/api/v1/app-events", false));
  events.addEventListener("focus", (event) => {
    const data = JSON.parse(event.data || "{}");
    if (data.route && data.route !== location.pathname) navigate(data.route);
    window.focus();
  });
}

function handleRun(event) {
  switch (event.type) {
    case "job_started":
      live = { ...emptyLive(), active: true, kind: event.kind, started: performance.now(),
        phase: event.kind === "benchmarks" ? "Preparing the benchmark" : "Preparing checks" };
      break;
    case "build_started": live.active = true; live.phase = "Building"; break;
    case "build_completed": live.phase = event.passed ? "Build complete" : "Build failed"; break;
    case "test_started": live.active = true; live.phase = event.name; live.current = event.index; live.total = event.total; break;
    case "test_passed": live.passed += 1; break;
    case "test_failed":
      live.failed += 1;
      showResult(`Failing check · ${event.result.name}`, event.result.failures?.[0] || "This check failed.", "contradiction");
      break;
    case "benchmark_started":
      live.active = true; live.kind = "benchmarks"; live.phase = event.name;
      live.current = event.index; live.total = event.total;
      break;
    case "benchmark_point_started":
      live.phase = event.params_label ? `${event.name} (${event.params_label})` : event.name;
      break;
    case "benchmark_sample_recorded":
      live.phase = `${event.name}${event.params_label ? ` (${event.params_label})` : ""} · iteration ${event.iteration} of ${event.iterations}`;
      break;
    case "benchmark_point_completed": if (event.success) live.passed += 1; else live.failed += 1; break;
    case "run_completed":
    case "benchmark_run_completed":
      live.active = false;
      refreshProject();
      break;
    case "job_interrupted":
      live.active = false;
      showResult("Run interrupted", event.reason, "attention");
      refreshProject();
      break;
    case "source_changed":
    case "project_state_changed":
      refreshProject();
      break;
    default: break;
  }
  if (currentView === "build" && state) { renderState(); renderMeter(); }
  if (currentView === "performance" && state) renderPerformance();
}

let refreshPending = null;
async function refreshProject() {
  // Several events can land in one poll; collapse them into one refetch.
  if (refreshPending) return refreshPending;
  refreshPending = (async () => {
    try {
      const [latest, next] = await Promise.all([fetchJson("/api/v1/state"), fetchJson("/api/v1/capability")]);
      state = latest; content = next;
      $("#project-meta").textContent = `${state.language} · ${completedSteps()} of ${content.roadmap.length} steps complete`;
      renderCurrentView();
    } catch (error) {
      // This runs from an event listener, so an uncaught rejection here is
      // invisible: the page would simply stop updating.
      if ($("#activity")) $("#activity").textContent = error.message;
    } finally {
      refreshPending = null;
    }
  })();
  return refreshPending;
}

/* ----------------------------------------------------------------- utils */

function element(tag, className, style) {
  const node = document.createElement(tag);
  if (className) node.className = className.trim();
  if (style) node.style.cssText = style;
  return node;
}

function text(node, value) {
  node.textContent = value ?? "";
  return node;
}

function formatTimestamp(value) {
  const date = new Date(value);
  return Number.isNaN(date.getTime())
    ? value
    : new Intl.DateTimeFormat(undefined, { dateStyle: "medium", timeStyle: "short" }).format(date);
}

function formatNumber(value) {
  if (value === null || value === undefined || !Number.isFinite(value)) return "—";
  const magnitude = Math.abs(value);
  return value.toFixed(magnitude >= 100 ? 0 : magnitude >= 1 ? 2 : 3);
}

async function openProject(path) {
  try {
    const result = await post(path);
    $("#project-open-status").textContent = `Opened in ${result.application || "your system application"}.`;
  } catch (error) {
    $("#project-open-status").textContent = error.message;
    if (!$("#health-screen").hidden) $("#health-detail").textContent = error.message;
  }
}

async function guarded(work) {
  if (busy) return;
  busy = true;
  try { await work(); } finally { busy = false; }
}

/* ---------------------------------------------------------------- wiring */

$("#theme-toggle").onclick = cycleTheme;
$("#projects-empty-catalog").onclick = () => navigate("/catalog");
$("#create-form").addEventListener("submit", submitCreate);
$("#create-name").addEventListener("input", () => schedulePreflight());
$("#create-parent").addEventListener("input", () => schedulePreflight());

$("#continue-build").onclick = () => navigate(projectRoute("build"));
$("#open-editor").onclick = () => openProject("/api/v1/project/open-editor");
$("#open-folder").onclick = () => openProject("/api/v1/project/open-folder");
$("#health-editor").onclick = () => openProject("/api/v1/project/open-editor");
$("#health-folder").onclick = () => openProject("/api/v1/project/open-folder");
$("#health-recheck").onclick = () => loadProject();
$("#health-repin").onclick = async () => {
  try {
    await post("/api/v1/project/repin-pack");
  } catch (error) {
    $("#health-detail").textContent = error.message;
    return;
  }
  await loadProject();
};

$("#primary-action").onclick = () => guarded(async () => {
  if (!state) return;
  renderState();
  try {
    if (live.active || state.active_job) await post("/api/v1/runs/cancel");
    else if (state.primary_action.kind === "begin_next_capability") {
      state = await post("/api/v1/capabilities/next");
      content = await fetchJson("/api/v1/capability");
      renderBuild();
    } else await post("/api/v1/runs");
  } catch (error) {
    showResult("Could not start", error.message, "contradiction");
  }
  renderState();
});

$("#snapshot-action").onclick = () => guarded(async () => {
  try {
    const outcome = await post("/api/v1/snapshots");
    $("#snapshot-action").hidden = true;
    $("#snapshot-notice").hidden = false;
    $("#snapshot-notice-detail").textContent =
      ` ${outcome.message} — commit ${outcome.commit.slice(0, 12)}` +
      (outcome.tag ? `, tagged ${outcome.tag}.` : outcome.existing_tag ? `. The tag ${outcome.existing_tag} already existed.` : ".");
  } catch (error) {
    $("#snapshot-notice").hidden = false;
    $("#snapshot-notice").className = "notice error";
    $("#snapshot-notice-detail").textContent = ` ${error.message}`;
  }
});

$("#gate-notice-action").onclick = () => navigate(projectRoute("performance"));

$("#focused-rerun").onclick = () => {
  const test = $("#focused-rerun").dataset.test;
  if (test) post("/api/v1/runs/rerun", { test }).catch((error) => showResult("Could not rerun that check", error.message, "contradiction"));
};

$("#other-failures-list").onclick = (event) => {
  const button = event.target.closest("button[data-test]");
  if (button) post("/api/v1/runs/rerun", { test: button.dataset.test })
    .catch((error) => showResult("Could not rerun that check", error.message, "contradiction"));
};

$("#reveal-help").onclick = () => guarded(async () => {
  if (!content) return;
  $("#help-status").textContent = "";
  // `guarded` has already set `busy`, so this re-render is what disables the
  // button while the request is in flight.
  renderHints();
  try {
    content = await post("/api/v1/hints");
  } catch (error) {
    // Swallowing this left a button that looked live and did nothing.
    $("#help-status").textContent = error.message;
  }
  renderHints();
});

$("#benchmark-action").onclick = () => guarded(async () => {
  try {
    if (live.active || state.active_job) await post("/api/v1/runs/cancel");
    else await post("/api/v1/benchmarks");
  } catch (error) {
    $("#benchmark-activity").textContent = error.message;
  }
  renderPerformance();
});

$("#prediction-save").onclick = () => saveNote("/api/v1/predictions", $("#prediction-input").value, false);
$("#prediction-skip").onclick = () => saveNote("/api/v1/predictions", "", true);
$("#reflection-save").onclick = () => saveNote("/api/v1/reflections", $("#reflection-input").value, false);
$("#reflection-skip").onclick = () => saveNote("/api/v1/reflections", "", true);

function saveNote(path, value, skipped) {
  return guarded(async () => {
    try {
      state = await post(path, { text: value, skipped });
      renderPerformance();
    } catch (error) {
      $("#benchmark-activity").textContent = error.message;
    }
  });
}

$("#export-report").onclick = () => guarded(async () => {
  $("#export-status").textContent = "Writing the record…";
  try {
    const exported = await post("/api/v1/reports", { format: "markdown" });
    $("#export-status").textContent = `Written to ${exported.path}`;
  } catch (error) {
    $("#export-status").textContent = error.message;
  }
});

document.addEventListener("click", (event) => {
  const link = event.target.closest("a[data-route]");
  if (!link) return;
  event.preventDefault();
  navigate(link.dataset.route);
});
window.addEventListener("popstate", renderRoute);
setInterval(() => { if (currentView === "build") renderMeter(); }, 1000);

// The launcher's URL (and any bookmarked/reloaded one) carries the token in
// the query string, since that is the only way to authorize the request
// that fetched this page in the first place. The token is already captured
// above; strip it from what the address bar shows and what this load adds
// to history, now that nothing further needs it there.
if (location.search.includes("token=")) {
  history.replaceState(history.state, "", location.pathname);
}

applyTheme(readTheme());
renderRoute().catch((error) => {
  document.body.replaceChildren();
  const page = element("main", "page");
  page.append(text(element("h1"), "DeltaForge could not open"), text(element("p", "lede"), error.message));
  document.body.append(page);
});
