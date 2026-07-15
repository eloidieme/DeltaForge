use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::application;
use crate::context::{GlobalOptions, ProjectContext};
use crate::fs_util::atomic_write;

const RECORD_FILE: &str = "workbench.json";
const API_VERSION: &str = "v1";
const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");
const PROBE_TIMEOUT: Duration = Duration::from_millis(500);
const STARTUP_TIMEOUT: Duration = Duration::from_secs(4);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const EVENT_POLL_INTERVAL: Duration = Duration::from_millis(500);
const IDLE_TIMEOUT: Duration = Duration::from_secs(30 * 60);
const MAX_REQUEST_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServiceRecord {
    port: u16,
    pid: u32,
    token: String,
    version: String,
}

#[derive(Debug)]
struct Shared {
    options: GlobalOptions,
    token: String,
    port: u16,
    clients: AtomicUsize,
    last_activity: Mutex<Instant>,
    record_path: PathBuf,
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    target: String,
    headers: BTreeMap<String, String>,
}

pub fn launch(options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let record_path = context.root.join(".deltaforge").join(RECORD_FILE);
    let mut record = read_compatible_record(&record_path);

    if record.is_none() {
        let token = capability_token(&context.root);
        spawn_service(&context.root, options, &token)?;
        record = wait_for_service(&record_path);
    }

    let record = record.context("the DeltaForge workbench did not start in time")?;
    let url = format!("http://127.0.0.1:{}/?token={}", record.port, record.token);
    if std::env::var_os("DELTAFORGE_NO_BROWSER").is_some() {
        println!("DeltaForge is ready at {url}");
        println!("You can run checks with: deltaforge test");
    } else {
        match crate::learning_web::open_in_browser(url.as_ref()) {
            Ok(()) => println!("DeltaForge is ready."),
            Err(error) => {
                println!("DeltaForge is ready at {url}");
                println!("Browser opening failed: {error:#}");
                println!("You can still run checks with: deltaforge test");
            }
        }
    }
    Ok(())
}
fn spawn_service(root: &Path, options: &GlobalOptions, token: &str) -> Result<()> {
    let executable = std::env::current_exe().context("failed to locate the deltaforge binary")?;
    let mut command = Command::new(executable);
    command
        .arg("--project-dir")
        .arg(root)
        .arg("__workbench")
        .arg("--token")
        .arg(token)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(packs_dir) = &options.packs_dir {
        command.arg("--packs-dir").arg(packs_dir);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
        .spawn()
        .context("failed to start the DeltaForge workbench service")?;
    Ok(())
}

fn wait_for_service(record_path: &Path) -> Option<ServiceRecord> {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    while Instant::now() < deadline {
        if let Some(record) = read_compatible_record(record_path) {
            return Some(record);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    None
}

fn read_compatible_record(record_path: &Path) -> Option<ServiceRecord> {
    let source = fs::read_to_string(record_path).ok()?;
    let record: ServiceRecord = serde_json::from_str(&source).ok()?;
    if record.version != SERVICE_VERSION || !probe(&record) {
        let _ = fs::remove_file(record_path);
        return None;
    }
    Some(record)
}

fn probe(record: &ServiceRecord) -> bool {
    let path = format!("/api/{API_VERSION}/health?token={}", record.token);
    let Some(body) = http_get(record.port, &path) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) else {
        return false;
    };
    value.get("service") == Some(&serde_json::Value::String("deltaforge".to_string()))
        && value.get("version") == Some(&serde_json::Value::String(SERVICE_VERSION.to_string()))
}

pub fn serve(options: &GlobalOptions, token: String) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let record_path = context.root.join(".deltaforge").join(RECORD_FILE);
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
        .context("failed to bind the DeltaForge workbench to loopback")?;
    let port = listener.local_addr()?.port();
    let record = ServiceRecord {
        port,
        pid: std::process::id(),
        token: token.clone(),
        version: SERVICE_VERSION.to_string(),
    };
    atomic_write(&record_path, serde_json::to_string(&record)?)?;

    let shared = Arc::new(Shared {
        options: GlobalOptions {
            project_dir: Some(context.root),
            packs_dir: options.packs_dir.clone(),
        },
        token,
        port,
        clients: AtomicUsize::new(0),
        last_activity: Mutex::new(Instant::now()),
        record_path,
    });
    spawn_idle_watchdog(Arc::clone(&shared));

    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            continue;
        };
        let shared = Arc::clone(&shared);
        std::thread::spawn(move || {
            let _ = handle_connection(stream, &shared);
        });
    }
    Ok(())
}

fn spawn_idle_watchdog(shared: Arc<Shared>) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(30));
            let idle = shared
                .last_activity
                .lock()
                .map(|last| last.elapsed())
                .unwrap_or_default();
            if shared.clients.load(Ordering::SeqCst) == 0 && idle > IDLE_TIMEOUT {
                let _ = fs::remove_file(&shared.record_path);
                std::process::exit(0);
            }
        }
    });
}
fn handle_connection(mut stream: TcpStream, shared: &Shared) -> Result<()> {
    stream.set_read_timeout(Some(REQUEST_TIMEOUT))?;
    let Some(request) = read_request(&mut stream)? else {
        return Ok(());
    };
    *shared
        .last_activity
        .lock()
        .expect("workbench lock poisoned") = Instant::now();

    if !authorized(&request, shared) {
        return respond(
            &mut stream,
            "403 Forbidden",
            "application/json",
            r#"{"error":"forbidden"}"#,
        );
    }

    let path = request
        .target
        .split_once('?')
        .map_or(request.target.as_str(), |(path, _)| path);
    if request.method != "GET" {
        return respond(
            &mut stream,
            "405 Method Not Allowed",
            "application/json",
            r#"{"error":"method_not_allowed"}"#,
        );
    }

    match path {
        "/" => respond(
            &mut stream,
            "200 OK",
            "text/html; charset=utf-8",
            &workbench_html(&shared.token),
        ),
        "/api/v1/health" => {
            let body = serde_json::json!({
                "service": "deltaforge",
                "api": API_VERSION,
                "version": SERVICE_VERSION,
                "pid": std::process::id(),
            })
            .to_string();
            respond(&mut stream, "200 OK", "application/json", &body)
        }
        "/api/v1/state" => {
            let state = application::load_workbench_state(&shared.options)?;
            let body = serde_json::to_string(&state)?;
            respond(&mut stream, "200 OK", "application/json", &body)
        }
        "/api/v1/events" => serve_events(stream, shared),
        _ => respond(
            &mut stream,
            "404 Not Found",
            "application/json",
            r#"{"error":"not_found"}"#,
        ),
    }
}

fn read_request(stream: &mut TcpStream) -> Result<Option<HttpRequest>> {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    while !bytes.windows(4).any(|window| window == b"\r\n\r\n") {
        if bytes.len() >= MAX_REQUEST_BYTES {
            bail!("request headers exceed the workbench limit");
        }
        match stream.read(&mut chunk) {
            Ok(0) if bytes.is_empty() => return Ok(None),
            Ok(0) => break,
            Ok(read) => bytes.extend_from_slice(&chunk[..read]),
            Err(error) if bytes.is_empty() => return Err(error.into()),
            Err(_) => return Ok(None),
        }
    }
    let text = std::str::from_utf8(&bytes).context("request headers are not UTF-8")?;
    let mut lines = text.split("\r\n");
    let request_line = lines.next().context("request line is missing")?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .context("request method is missing")?
        .to_string();
    let target = parts
        .next()
        .context("request target is missing")?
        .to_string();
    if parts.next() != Some("HTTP/1.1") || parts.next().is_some() {
        bail!("unsupported HTTP request line");
    }

    let mut headers = BTreeMap::new();
    for line in lines.take_while(|line| !line.is_empty()) {
        let (name, value) = line.split_once(':').context("malformed HTTP header")?;
        let name = name.trim().to_ascii_lowercase();
        if headers.insert(name, value.trim().to_string()).is_some() {
            bail!("duplicate HTTP headers are not accepted");
        }
    }
    Ok(Some(HttpRequest {
        method,
        target,
        headers,
    }))
}

fn authorized(request: &HttpRequest, shared: &Shared) -> bool {
    let expected_host = format!("127.0.0.1:{}", shared.port);
    if request.headers.get("host") != Some(&expected_host) {
        return false;
    }
    let token = request
        .target
        .split_once('?')
        .and_then(|(_, query)| {
            query
                .split('&')
                .find_map(|pair| pair.strip_prefix("token="))
        })
        .unwrap_or_default();
    if token != shared.token {
        return false;
    }
    let expected_origin = format!("http://{expected_host}");
    request
        .headers
        .get("origin")
        .is_none_or(|origin| origin == &expected_origin)
}

fn serve_events(mut stream: TcpStream, shared: &Shared) -> Result<()> {
    stream.set_read_timeout(None)?;
    let headers = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Type: text/event-stream\r\n",
        "Cache-Control: no-cache\r\n",
        "X-Content-Type-Options: nosniff\r\n",
        "Referrer-Policy: no-referrer\r\n",
        "Connection: keep-alive\r\n\r\n"
    );
    stream.write_all(headers.as_bytes())?;

    struct ClientGuard<'a>(&'a AtomicUsize);
    impl Drop for ClientGuard<'_> {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }
    shared.clients.fetch_add(1, Ordering::SeqCst);
    let _guard = ClientGuard(&shared.clients);
    let mut previous = String::new();

    loop {
        let state = application::load_workbench_state(&shared.options)?;
        let serialized = serde_json::to_string(&state)?;
        let payload = if serialized != previous {
            previous = serialized.clone();
            format!("event: state\ndata: {serialized}\n\n")
        } else {
            ": keep-alive\n\n".to_string()
        };
        if stream.write_all(payload.as_bytes()).is_err() {
            return Ok(());
        }
        *shared
            .last_activity
            .lock()
            .expect("workbench lock poisoned") = Instant::now();
        std::thread::sleep(EVENT_POLL_INTERVAL);
    }
}

fn respond(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) -> Result<()> {
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nContent-Security-Policy: default-src 'self'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; connect-src 'self'; img-src 'self' data:\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nX-Frame-Options: DENY\r\nConnection: close\r\n\r\n{body}",
        body.len(),
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn http_get(port: u16, path: &str) -> Option<String> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
    let mut stream = TcpStream::connect_timeout(&address, PROBE_TIMEOUT).ok()?;
    stream.set_read_timeout(Some(PROBE_TIMEOUT)).ok()?;
    stream.set_write_timeout(Some(PROBE_TIMEOUT)).ok()?;
    let request =
        format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).ok()?;
    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;
    if !response.starts_with("HTTP/1.1 200") {
        return None;
    }
    response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body.to_string())
}

fn capability_token(root: &Path) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in root
        .to_string_lossy()
        .as_bytes()
        .iter()
        .chain(nanos.to_le_bytes().iter())
        .chain(std::process::id().to_le_bytes().iter())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}{nanos:x}")
}
fn workbench_html(token: &str) -> String {
    let token_json = serde_json::to_string(token).unwrap_or_else(|_| "\"\"".to_string());
    let html = r###"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="color-scheme" content="dark">
<title>DeltaForge</title>
<style>
:root {
  --bg: #090b10; --panel: #10141d; --panel-2: #151b27; --line: #252d3c;
  --ink: #f2f5fb; --muted: #8e99aa; --blue: #75a7ff; --cyan: #6de3d1;
  --amber: #f4c26b; --radius: 18px;
  font-family: Inter, ui-sans-serif, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}
* { box-sizing: border-box; }
body {
  margin: 0; min-height: 100vh; color: var(--ink);
  background: radial-gradient(circle at 70% -10%, rgba(70,113,196,.18), transparent 38rem), var(--bg);
}
button { font: inherit; }
.shell { min-height: 100vh; display: grid; grid-template-columns: 250px minmax(420px,1fr) 330px; }
.rail {
  border-right: 1px solid var(--line); padding: 30px 24px;
  display: flex; flex-direction: column; gap: 34px;
}
.brand { font-weight: 720; letter-spacing: -.03em; font-size: 1.08rem; }
.brand span { color: var(--blue); }
.eyebrow {
  color: var(--muted); font: 650 .68rem/1 ui-monospace, SFMono-Regular, Consolas, monospace;
  letter-spacing: .14em; text-transform: uppercase;
}
.project-name { margin-top: 9px; font-weight: 650; }
.cap-list { display: grid; gap: 8px; }
.cap {
  padding: 11px 12px; border: 1px solid transparent; border-radius: 11px;
  color: var(--muted); font-size: .86rem;
}
.cap.current { color: var(--ink); background: var(--panel); border-color: var(--line); }
.rail-foot { margin-top: auto; color: var(--muted); font-size: .76rem; line-height: 1.55; }
main { padding: 46px clamp(34px,5vw,76px); }
.mission { max-width: 820px; margin: 0 auto; }
.kicker { color: var(--cyan); font: 650 .72rem/1 ui-monospace, monospace; letter-spacing: .12em; }
h1 { margin: 18px 0 14px; font-size: clamp(2.35rem,5vw,4.5rem); line-height: .98; letter-spacing: -.055em; }
.lede { max-width: 640px; color: #b7c0ce; font-size: 1.06rem; line-height: 1.65; }
.action-row { display: flex; align-items: center; gap: 14px; margin: 30px 0 36px; }
.primary {
  border: 0; border-radius: 999px; padding: 13px 22px; color: #07101a;
  background: var(--blue); font-weight: 720; cursor: pointer;
}
.primary:disabled { cursor: default; opacity: .55; }
.status { color: var(--muted); font-size: .84rem; }
.card-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 14px; }
.card {
  min-height: 150px; padding: 20px; border: 1px solid var(--line);
  border-radius: var(--radius); background: rgba(16,20,29,.78);
}
.card h2 { margin: 9px 0 8px; font-size: 1rem; }
.card p { margin: 0; color: var(--muted); line-height: 1.55; font-size: .9rem; }
.evidence { border-left: 1px solid var(--line); padding: 30px 24px; background: rgba(9,11,16,.5); }
.evidence h2 { font-size: .94rem; margin: 9px 0 26px; }
.signal { padding: 16px; border-radius: 14px; background: var(--panel); border: 1px solid var(--line); }
.signal strong { display: block; font-size: .9rem; margin-bottom: 6px; }
.signal p { color: var(--muted); margin: 0; font-size: .82rem; line-height: 1.5; }
.signal.fresh strong { color: var(--cyan); }
.signal.stale strong { color: var(--amber); }
.contract { margin-top: 28px; display: grid; gap: 13px; }
.contract-row { display: flex; gap: 10px; color: #b7c0ce; font-size: .82rem; line-height: 1.45; }
.dot { width: 6px; height: 6px; margin-top: 6px; border-radius: 50%; background: var(--blue); flex: 0 0 auto; }
@media (max-width: 1040px) {
  .shell { grid-template-columns: 210px 1fr; }
  .evidence { grid-column: 2; border-top: 1px solid var(--line); border-left: 0; }
}
@media (max-width: 720px) {
  .shell { display: block; }
  .rail { border-right: 0; border-bottom: 1px solid var(--line); }
  .cap-list, .rail-foot { display: none; }
  main { padding: 34px 22px; }
  .card-grid { grid-template-columns: 1fr; }
}
</style>
</head>
<body>
<div class="shell">
  <aside class="rail">
    <div class="brand">Delta<span>Forge</span></div>
    <div><div class="eyebrow">Project</div><div class="project-name" id="project">Loading?</div></div>
    <div>
      <div class="eyebrow">Capabilities</div>
      <div class="cap-list">
        <div class="cap current" id="current-cap">Current capability</div>
        <div class="cap" id="next-cap">Next capability locked</div>
      </div>
    </div>
    <div class="rail-foot">Local workbench<br>No account ? Works offline</div>
  </aside>
  <main>
    <section class="mission">
      <div class="kicker">CURRENT MISSION</div>
      <h1 id="mission-title">Loading your mission</h1>
      <p class="lede">Make file discovery deterministic: walk a project, ignore generated directories, and emit stable root-relative paths.</p>
      <div class="action-row">
        <button class="primary" disabled>Run checks</button>
        <span class="status" id="activity">Connecting to the local workbench?</span>
      </div>
      <div class="card-grid">
        <article class="card">
          <div class="eyebrow">Why it matters</div>
          <h2>Reliable inputs create reliable systems.</h2>
          <p>Indexing, builds, search, and analysis all depend on discovering the same files in the same order.</p>
        </article>
        <article class="card">
          <div class="eyebrow">Definition of done</div>
          <h2>One canonical path stream.</h2>
          <p>Nested files are included, ignored directories disappear, paths are portable, and ordering never changes.</p>
        </article>
      </div>
    </section>
  </main>
  <aside class="evidence">
    <div class="eyebrow">Evidence</div>
    <h2>Your latest proof</h2>
    <div class="signal" id="signal">
      <strong id="signal-title">No run yet</strong>
      <p id="signal-copy">Run checks when you are ready. DeltaForge will preserve the result for your next session.</p>
    </div>
    <div class="contract">
      <div class="eyebrow">Contract</div>
      <div class="contract-row"><span class="dot"></span><span>Walk nested directories recursively.</span></div>
      <div class="contract-row"><span class="dot"></span><span>Skip build and dependency directories.</span></div>
      <div class="contract-row"><span class="dot"></span><span>Print stable, forward-slash relative paths.</span></div>
      <div class="contract-row"><span class="dot"></span><span>Fail clearly when the root is missing.</span></div>
    </div>
  </aside>
</div>
<script>
const token = __TOKEN_JSON__;
const stateUrl = "/api/v1/state?token=" + encodeURIComponent(token);
const eventsUrl = "/api/v1/events?token=" + encodeURIComponent(token);

function render(state) {
  document.querySelector("#project").textContent = state.project + " ? " + state.language;
  document.querySelector("#mission-title").textContent = state.capability.title;
  document.querySelector("#current-cap").textContent = state.capability.title;
  document.querySelector("#next-cap").textContent =
    state.capability.next_id ? "Next ? " + state.capability.next_id : "Final capability";
  document.querySelector("#activity").textContent =
    state.recovered_interrupted_job ? "Previous run was safely recovered" : "Last activity ? " + state.last_activity_at;
  const signal = document.querySelector("#signal");
  const title = document.querySelector("#signal-title");
  const copy = document.querySelector("#signal-copy");
  signal.className = "signal";
  if (state.freshness === "fresh") {
    signal.classList.add("fresh");
    title.textContent = state.capability.completed ? "Capability acquired" : "Result is current";
    copy.textContent = "This evidence matches the source currently on disk.";
  } else if (state.freshness === "stale") {
    signal.classList.add("stale");
    title.textContent = "Source changed";
    copy.textContent = "Your previous result is preserved, but it no longer proves the current code.";
  } else {
    title.textContent = "No run yet";
    copy.textContent = "Run checks when you are ready. DeltaForge will preserve the result for your next session.";
  }
}

fetch(stateUrl).then(response => response.json()).then(render);
const events = new EventSource(eventsUrl);
events.addEventListener("state", event => render(JSON.parse(event.data)));
events.onerror = () => {
  document.querySelector("#activity").textContent = "Reconnecting to the local workbench?";
};
</script>
</body>
</html>"###;
    html.replace("__TOKEN_JSON__", &token_json)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_shared(port: u16) -> Shared {
        Shared {
            options: GlobalOptions::default(),
            token: "secret-token".to_string(),
            port,
            clients: AtomicUsize::new(0),
            last_activity: Mutex::new(Instant::now()),
            record_path: PathBuf::from("unused-workbench-record.json"),
        }
    }

    fn raw_request(target: &str, host: &str, origin: Option<&str>) -> String {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let shared = Arc::new(test_shared(port));
        let shared_for_server = Arc::clone(&shared);
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_connection(stream, &shared_for_server).unwrap();
        });

        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        let mut request = format!("GET {target} HTTP/1.1\r\nHost: {host}\r\n");
        if let Some(origin) = origin {
            request.push_str(&format!("Origin: {origin}\r\n"));
        }
        request.push_str("Connection: close\r\n\r\n");
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        server.join().unwrap();
        response
    }

    #[test]
    fn rejects_missing_token_wrong_host_and_hostile_origin() {
        let port = 43123;
        let shared = test_shared(port);
        let request = |target: &str, host: &str, origin: Option<&str>| HttpRequest {
            method: "GET".to_string(),
            target: target.to_string(),
            headers: [
                ("host".to_string(), host.to_string()),
                ("origin".to_string(), origin.unwrap_or_default().to_string()),
            ]
            .into_iter()
            .filter(|(name, _)| name != "origin" || origin.is_some())
            .collect(),
        };

        assert!(!authorized(&request("/", "127.0.0.1:43123", None), &shared));
        assert!(!authorized(
            &request("/?token=secret-token", "localhost:43123", None),
            &shared
        ));
        assert!(!authorized(
            &request(
                "/?token=secret-token",
                "127.0.0.1:43123",
                Some("https://attacker.example")
            ),
            &shared
        ));
        assert!(authorized(
            &request(
                "/?token=secret-token",
                "127.0.0.1:43123",
                Some("http://127.0.0.1:43123")
            ),
            &shared
        ));
    }

    #[test]
    fn service_never_serves_guessed_project_paths() {
        let probe = raw_request("/../Cargo.toml?token=secret-token", "127.0.0.1:0", None);
        assert!(probe.starts_with("HTTP/1.1 403"));

        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let shared = Arc::new(test_shared(port));
        let shared_for_server = Arc::clone(&shared);
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_connection(stream, &shared_for_server).unwrap();
        });
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        let request = format!(
            "GET /../Cargo.toml?token=secret-token HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
        );
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        server.join().unwrap();
        assert!(response.starts_with("HTTP/1.1 404"));
        assert!(!response.contains("[package]"));
    }

    #[test]
    fn shell_uses_the_new_workbench_surface() {
        let html = workbench_html("secret-token");
        assert!(html.contains("CURRENT MISSION"));
        assert!(html.contains("/api/v1/state?token="));
        assert!(html.contains("Local workbench"));
        assert!(!html.contains("warm"));
    }
}
