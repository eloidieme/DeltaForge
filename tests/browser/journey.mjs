/* The learner's journey, driven through the real page in a real browser.
 *
 * Nothing here constructs an HTTP request. Every exchange with the service is
 * one `src/ui/app.js` decided to make, which is the only way to notice that
 * the page and the Rust contract test have stopped agreeing.
 *
 * Exit code 0 means the journey completed with no page errors.
 */

import { spawn } from "node:child_process";
import { chromium } from "playwright";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.resolve(HERE, "..", "..");
const HEADED = process.env.HEADED === "1";
const KEEP = process.env.KEEP === "1";
const TOKEN = "browser-journey-token";

/* A step's budget. Compiling the starter template on a cold CI runner is the
 * slow one; everything else is far below this. */
const RUN_TIMEOUT = 300_000;
const UI_TIMEOUT = 30_000;

function binary() {
  if (process.env.DELTAFORGE_BIN) return process.env.DELTAFORGE_BIN;
  const suffix = process.platform === "win32" ? ".exe" : "";
  return path.join(REPO, "target", "debug", `deltaforge${suffix}`);
}

const log = (message) => console.log(`  ${message}`);

function check(condition, message) {
  if (!condition) throw new Error(message);
}

/* --------------------------------------------------------------- service */

async function startService(scratch) {
  const home = path.join(scratch, "home");
  const workspace = path.join(scratch, "workspace");
  fs.mkdirSync(home, { recursive: true });
  // `workspace` is deliberately NOT created. A learner's machine does not have
  // one, and every harness this project had created it first.

  const child = spawn(binary(), ["__workbench", "--token", TOKEN], {
    env: {
      ...process.env,
      DELTAFORGE_HOME: home,
      DELTAFORGE_WORKSPACE: workspace,
      // A minimal Git identity: the snapshot step runs a real `git commit`,
      // and a runner with no global identity refuses for reasons that have
      // nothing to do with this journey.
      GIT_CONFIG_COUNT: "3",
      GIT_CONFIG_KEY_0: "core.hooksPath",
      GIT_CONFIG_VALUE_0: "NUL",
      GIT_CONFIG_KEY_1: "user.name",
      GIT_CONFIG_VALUE_1: "DeltaForge Journey",
      GIT_CONFIG_KEY_2: "user.email",
      GIT_CONFIG_VALUE_2: "deltaforge@example.com",
    },
    stdio: ["ignore", "inherit", "inherit"],
  });

  const record = path.join(home, "workbench.json");
  const deadline = Date.now() + 20_000;
  while (Date.now() < deadline) {
    try {
      const parsed = JSON.parse(fs.readFileSync(record, "utf8"));
      if (parsed.port) return { child, port: parsed.port, home, workspace };
    } catch {
      /* not written yet */
    }
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
  child.kill();
  throw new Error("the workbench service did not start");
}

/* ------------------------------------------------------------------ page */

/* Wait until `read` returns something truthy, then return it. Playwright's own
 * waiters cover elements; this covers the states the page reaches after a
 * server-sent event, which no selector describes. */
async function until(page, what, read, timeout = UI_TIMEOUT) {
  const deadline = Date.now() + timeout;
  let last;
  while (Date.now() < deadline) {
    last = await read();
    if (last) return last;
    await page.waitForTimeout(250);
  }
  throw new Error(`timed out waiting for ${what}${last ? ` (last saw ${last})` : ""}`);
}

/* Wait for `document.title` to satisfy `matches`, then return it.
 *
 * A screen is unhidden as soon as the route is known, but its title is set once
 * the data that names it has arrived — `renderCreate` shows the form and then
 * fetches the catalog and the workspace. Locally that gap is a few
 * milliseconds; on a CI runner it is long enough that asserting the title the
 * instant the screen appears reads the previous route's. That is a race in this
 * harness, not in the page: the title is correct a moment later, every time. */
async function titleBecomes(page, matches, describe) {
  let last = "";
  try {
    await page.waitForFunction(
      (source) => new Function("title", `return (${source})(title)`)(document.title),
      matches.toString(),
      { timeout: UI_TIMEOUT },
    );
  } catch {
    last = await page.title();
    throw new Error(`${describe}; title stayed "${last}"`);
  }
  return page.title();
}

const textOf = (page, selector) =>
  page.$eval(selector, (node) => node.textContent.trim()).catch(() => "");

const isHidden = (page, selector) =>
  page.$eval(selector, (node) => node.hidden).catch(() => true);

/* ------------------------------------------------------------------ main */

async function journey(page, service) {
  const origin = `http://127.0.0.1:${service.port}`;

  /* 1. The catalog, entered the way the launcher enters it. */
  await page.goto(`${origin}/catalog?token=${TOKEN}`, { waitUntil: "domcontentloaded" });
  await page.waitForSelector("#catalog-grid .catalog-card", { timeout: UI_TIMEOUT });
  const cards = await page.$$eval(".catalog-card h2", (nodes) => nodes.map((n) => n.textContent));
  check(cards[0] === "FlashIndex", `the flagship is not first: ${cards.join(", ")}`);
  check(
    JSON.stringify(cards.slice(1)) === JSON.stringify(cards.slice(1).toSorted()),
    `preview projects are not alphabetical: ${cards.join(", ")}`,
  );
  check(
    (await textOf(page, ".catalog-tier-note")).includes("playable and correct"),
    "the catalog does not explain what Preview means",
  );
  await titleBecomes(page, (title) => title === "Project catalog — DeltaForge", "the catalog never titled itself");
  check(await page.$eval("#catalog-nav", (node) => node.getAttribute("aria-current") === "page"), "catalog navigation has no current-page state");
  check(await page.$eval("#catalog-screen h1", (node) => document.activeElement === node), "catalog heading did not receive focus");
  log(`catalog lists ${cards.length} projects, flagship first and previews alphabetical`);

  /* The token is captured from the query string and then removed from it, so a
   * copied address bar or a history entry never carries a live capability. */
  check(!page.url().includes("token="), `the token stayed in the address bar: ${page.url()}`);

  /* 2. Creation with the defaults — nothing typed, nothing chosen. */
  await page.click(".catalog-card:first-of-type .catalog-actions button");
  await page.waitForSelector("#create-screen:not([hidden])", { timeout: UI_TIMEOUT });
  await titleBecomes(page, (title) => title.startsWith("Create FlashIndex —"), "the creation screen never titled itself");
  check(await page.$eval("#create-title", (node) => document.activeElement === node), "creation heading did not receive focus");
  check(await page.$eval("#catalog-nav", (node) => !node.hasAttribute("aria-current")), "creation falsely marks Catalog as the current page");
  const offered = await page.inputValue("#create-parent");
  check(offered.length > 0, "the Location field was not prefilled");
  check(!fs.existsSync(service.workspace), "the workspace exists; this journey proves nothing");

  const verdict = await until(page, "the preflight verdict", async () => {
    const label = await textOf(page, "#preflight-location-label");
    return label && label !== "Checking location" && (await textOf(page, "#preflight-path"))
      ? label
      : null;
  });
  check(
    verdict === "Will be created at",
    `preflight refused the default location: ${await textOf(page, "#preflight-path")}`,
  );
  check(
    (await textOf(page, "#preflight-note")).includes("will be created"),
    "the page did not say the workspace folder would be created",
  );
  check(
    !(await page.isDisabled("#create-submit")),
    "Create project is disabled on a machine with no workspace",
  );
  check(!fs.existsSync(service.workspace), "a preflight created a directory");
  log(`preflight accepts the default location ${offered}`);

  /* A changed field immediately has a real loading state, then an inline,
   * discoverable refusal. The submit control stays in the tab order so its
   * reason is reachable from a keyboard. */
  const missingParent = path.join(service.home, "missing-parent");
  await page.fill("#create-parent", missingParent);
  check((await textOf(page, "#preflight-location-label")) === "Checking location", "preflight left an orphaned label while loading");
  check((await textOf(page, "#preflight-path")).length > 0, "preflight loading state has no value");
  check(await page.$eval("#create-preflight", (node) => node.getAttribute("aria-busy") === "true"), "preflight loading state is not exposed");
  await until(page, "the inaccessible location refusal", async () =>
    (await page.getAttribute("#create-parent", "aria-invalid")) === "true" ? "refused" : null,
  );
  check((await page.getAttribute("#create-submit", "aria-disabled")) === "true", "refused creation is not aria-disabled");
  check(await page.$eval("#create-submit", (node) => node.disabled === false), "refused creation disappeared from the tab order");
  const describedBy = await page.getAttribute("#create-parent", "aria-describedby");
  check(describedBy.includes("create-parent-status"), "Location is not connected to its refusal");
  check((await textOf(page, "#create-parent-status")).length > 0, "Location refusal has no inline message");

  await page.fill("#create-parent", offered);
  await until(page, "the restored default location", async () =>
    (await page.getAttribute("#create-submit", "aria-disabled")) === "false" ? "ready" : null,
  );

  await page.click("#create-submit");
  await page.waitForSelector("#build-screen:not([hidden])", { timeout: UI_TIMEOUT });
  const projectPath = path.join(service.workspace, await page.inputValue("#create-name").catch(() => ""));
  log("project created from the defaults, build screen reached");

  /* Where the project actually landed, taken from the service's own registry
   * rather than reconstructed. */
  const registry = JSON.parse(fs.readFileSync(path.join(service.home, "projects.json"), "utf8"));
  const entries = Array.isArray(registry) ? registry : registry.projects || [];
  const root = entries.map((entry) => entry.root || entry.path).filter(Boolean).pop();
  check(root && fs.existsSync(root), `the created project is not on disk: ${root ?? projectPath}`);
  log(`project root ${root}`);

  await titleBecomes(page, (title) => title.includes("Scan files — FlashIndex — DeltaForge"), "the build screen never titled itself");
  check(await page.$eval("#instruction-title", (node) => document.activeElement === node), "build heading did not receive focus");
  check(await page.$eval("#build-nav", (node) => node.getAttribute("aria-current") === "page"), "Build navigation has no current-page state");
  const currentLabel = await page.getAttribute("#build-rail .rail-step.here", "aria-label");
  check(!/current, current step/.test(currentLabel), `current step is announced twice: ${currentLabel}`);

  /* At the 1120px collapse the rail must follow the primary work, not push it
   * down by fourteen rows. */
  await page.setViewportSize({ width: 1000, height: 720 });
  const responsive = await page.evaluate(() => {
    const action = document.querySelector("#primary-action").getBoundingClientRect();
    const rail = document.querySelector(".build-rail").getBoundingClientRect();
    return { actionTop: action.top, actionBottom: action.bottom, railTop: rail.top, height: innerHeight };
  });
  check(responsive.actionTop >= 0 && responsive.actionBottom < responsive.height, `Run checks fell below the fold: ${JSON.stringify(responsive)}`);
  check(responsive.railTop > responsive.actionBottom, `collapsed rail still precedes Run checks: ${JSON.stringify(responsive)}`);
  await page.setViewportSize({ width: 1280, height: 800 });

  const measuredMarker = await page.$eval("#build-rail .rail-title-tail .rail-gate", (gate) => {
    const tail = gate.parentElement;
    const marker = gate.getBoundingClientRect();
    const line = tail.getBoundingClientRect();
    return {
      whiteSpace: getComputedStyle(tail).whiteSpace,
      markerTop: marker.top,
      markerBottom: marker.bottom,
      lineTop: line.top,
      lineBottom: line.bottom,
    };
  });
  check(measuredMarker.whiteSpace === "nowrap", `measured title tail can wrap: ${JSON.stringify(measuredMarker)}`);
  check(measuredMarker.markerTop >= measuredMarker.lineTop && measuredMarker.markerBottom <= measuredMarker.lineBottom, `measurement diamond orphaned: ${JSON.stringify(measuredMarker)}`);

  /* 3. The sections a learner reads, rendered. */
  for (const section of ["#section-background", "#section-requirements", "#section-expected"]) {
    const body = await textOf(page, section);
    check(body.length > 0, `${section} rendered empty`);
  }

  /* 4. The first run fails, with a diagnosis. */
  await page.click("#primary-action");
  await until(
    page,
    "the first run to finish with a diagnosis",
    async () => !(await isHidden(page, "#diagnosis")),
    RUN_TIMEOUT,
  );
  const requirement = await textOf(page, "#diagnosis-requirement");
  check(requirement.length > 0, "the diagnosis has no requirement");
  log(`first run failed with a diagnosis: ${requirement.slice(0, 60)}…`);

  /* 5. Help is reachable from the page. */
  const helpLabel = await textOf(page, "#reveal-help");
  await page.click("#reveal-help");
  await until(page, "a revealed hint", async () => {
    const levels = await textOf(page, "#help-levels");
    return levels.length > 0 ? levels : null;
  });
  log(`hint revealed (${helpLabel})`);

  /* 6. The learner writes code. The one action that is not a browser action,
   * standing in for their editor. */
  const reference = path.join(REPO, "tools", "reference_solutions", "flashindex_rust", "src", "main.rs");
  fs.copyFileSync(reference, path.join(root, "src", "main.rs"));

  await page.click("#primary-action");
  await until(
    page,
    "the passing run",
    async () => (await textOf(page, "#result-title")).includes("pass") || !(await isHidden(page, "#snapshot-action")),
    RUN_TIMEOUT,
  );
  check(!(await isHidden(page, "#snapshot-action")), "the pass did not offer a snapshot");
  log("checks pass");

  /* 7. The snapshot. */
  await page.click("#snapshot-action");
  const snapshot = await until(page, "the snapshot to be recorded", async () => {
    const detail = await textOf(page, "#snapshot-notice-detail");
    return detail.length > 0 ? detail : null;
  });
  check(detailIsSuccess(snapshot), `the snapshot failed: ${snapshot}`);
  log(`snapshot recorded: ${snapshot.slice(0, 70)}`);

  /* 8. The performance loop: predict, measure, reflect. */
  await page.click("#performance-nav");
  await page.waitForSelector("#performance-screen:not([hidden])", { timeout: UI_TIMEOUT });
  await titleBecomes(page, (title) => title === "Performance — FlashIndex — DeltaForge", "the performance screen never titled itself");
  check(await page.$eval("#performance-title", (node) => document.activeElement === node), "performance heading did not receive focus");
  check(await page.$eval("#performance-nav", (node) => node.getAttribute("aria-current") === "page"), "Performance navigation has no current-page state");
  await until(page, "the prediction prompt", async () =>
    (await isHidden(page, "#prediction-section")) ? null : "shown",
  );
  const prompt = await textOf(page, "#prediction-prompt");
  check(prompt.length > 0, "the prediction prompt is empty");
  await page.fill("#prediction-input", "Directory reads dominate.");
  await page.click("#prediction-save");
  await until(page, "the prediction to be recorded", async () =>
    (await isHidden(page, "#prediction-recorded")) ? null : "recorded",
  );
  log("prediction recorded before the measurement");

  await page.click("#benchmark-action");
  const measurement = await until(
    page,
    "the benchmark",
    async () => {
      const body = await textOf(page, "#measurements");
      return body.includes("scan_basic_project") ? body : null;
    },
    RUN_TIMEOUT,
  );
  check(/\d/.test(measurement), "the measurement carries no numbers");
  log("benchmark measured");

  /* 9. The record. */
  await page.click("#overview-nav");
  await page.waitForSelector("#overview-screen:not([hidden])", { timeout: UI_TIMEOUT });
  await page.click("#export-report");
  const exported = await until(page, "the exported record", async () => {
    const status = await textOf(page, "#export-status");
    return status.startsWith("Written to") ? status : null;
  });
  const record = exported.replace("Written to ", "").trim();
  const written = fs.readFileSync(path.isAbsolute(record) ? record : path.join(root, record), "utf8");
  check(written.includes("1 of 14 steps complete."), "the record does not state the progress");
  check(written.includes("scan_basic_project"), "the record does not carry the measurement");
  check(written.includes("Directory reads dominate."), "the record does not carry the prediction");
  log(`record written to ${record}`);

  /* A health failure still names its project, renders prose as prose, and
   * preserves inline code. Force a harmless pin mismatch in this disposable
   * project to exercise the actual route. */
  const statePath = path.join(root, ".deltaforge", "state.json");
  const savedState = JSON.parse(fs.readFileSync(statePath, "utf8"));
  savedState.pack_digest = "browser-journey-forced-mismatch";
  fs.writeFileSync(statePath, `${JSON.stringify(savedState, null, 2)}\n`);
  await page.click("#build-nav");
  await page.waitForSelector("#health-screen:not([hidden])", { timeout: UI_TIMEOUT });
  check((await textOf(page, "#project-name")) === "FlashIndex", `health header lost the project name: ${await textOf(page, "#project-name")}`);
  await titleBecomes(page, (title) => title.includes("— FlashIndex — DeltaForge"), "the health screen never titled itself");
  check(await page.$eval("#health-title", (node) => document.activeElement === node), "health heading did not receive focus");
  check(await page.$eval("#health-detail", (node) => node.scrollWidth <= node.clientWidth + 1), "health prose overflows horizontally");
  await page.evaluate(() => renderHealthDetail(`Review \`deltaforge sync-pack\` before adopting the change. ${"Long readable prose ".repeat(80)}`));
  check((await page.locator("#health-detail code").count()) > 0, "health prose renders backticks literally instead of inline code");
  check(await page.$eval("#health-detail", (node) => node.scrollWidth <= node.clientWidth + 1), "long health prose does not wrap");
  log("project health keeps its identity and wraps readable prose");

  /* 10. And when the service goes away, the page says so.
   *
   * The workbench exits by itself after thirty minutes idle. `EventSource`
   * retries forever and reports nothing, so in 1.0 that left "Reconnecting…"
   * in a corner of a dead page, permanently, with no way back. */
  // Past this point the page is talking to a socket that is gone, so its
  // console errors are the expected outcome rather than a defect.
  expectNetworkErrors = true;
  service.child.kill("SIGKILL");
  await until(
    page,
    "the disconnected banner",
    async () => ((await isHidden(page, "#disconnected")) ? null : "shown"),
    60_000,
  );
  const notice = await textOf(page, "#disconnected");
  check(notice.includes("deltaforge"), `the disconnected notice says nothing useful: ${notice}`);
  log("a stopped service is reported on the page, with the way back");
}

function detailIsSuccess(detail) {
  return detail.includes("commit") && !/could not|failed|error/i.test(detail);
}

/* ----------------------------------------------------------------- driver */

const scratch = fs.mkdtempSync(path.join(os.tmpdir(), "deltaforge-journey-"));
let service;
let browser;
let expectNetworkErrors = false;
const pageErrors = [];

try {
  check(fs.existsSync(binary()), `no deltaforge binary at ${binary()} — build it first`);
  service = await startService(scratch);
  browser = await chromium.launch({ headless: !HEADED });
  const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });

  page.on("pageerror", (error) => pageErrors.push(`uncaught: ${error.message}`));
  page.on("console", (message) => {
    if (message.type() !== "error") return;
    if (expectNetworkErrors && /ERR_CONNECTION|Failed to load resource/.test(message.text())) return;
    pageErrors.push(`console.error: ${message.text()}`);
  });

  await journey(page, service);

  if (pageErrors.length) {
    throw new Error(`the page reported errors:\n  ${pageErrors.join("\n  ")}`);
  }
  console.log("\nJourney complete: catalog to exported record, with no page errors.");
} catch (error) {
  console.error(`\nJourney failed: ${error.message}`);
  if (pageErrors.length) console.error(`Page errors:\n  ${pageErrors.join("\n  ")}`);
  process.exitCode = 1;
} finally {
  if (browser) await browser.close().catch(() => {});
  if (service) {
    service.child.kill("SIGTERM");
    await new Promise((resolve) => setTimeout(resolve, 500));
    service.child.kill("SIGKILL");
  }
  if (KEEP) console.log(`Scratch kept at ${scratch}`);
  else fs.rmSync(scratch, { recursive: true, force: true });
}
