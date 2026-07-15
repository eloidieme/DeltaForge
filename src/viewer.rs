//! Local live-viewer server for the generated learning and test-report pages.
//!
//! The CLI stays authoritative: commands keep writing static HTML into
//! `.deltaforge/ui/`. The viewer serves those files over `127.0.0.1` and
//! pushes a server-sent event whenever a command bumps the version marker,
//! so one persistent browser tab follows the learner's terminal instead of
//! a new tab opening on every failed run. Nothing is ever bound to a
//! non-loopback address and only known page names inside the UI directory
//! are served.

use std::fs;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::fs_util::atomic_write;

const VERSION_FILE: &str = ".version";
const VIEWER_FILE: &str = "viewer.json";
const WATCH_INTERVAL: Duration = Duration::from_millis(250);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
const IDLE_SHUTDOWN: Duration = Duration::from_secs(30 * 60);
const PROBE_TIMEOUT: Duration = Duration::from_millis(500);
const SPAWN_WAIT: Duration = Duration::from_secs(3);
const MAX_REQUEST_LINE_BYTES: usize = 16 * 1024;
const MAX_REQUEST_BYTES: usize = 256 * 1024;

#[derive(Debug, Serialize, Deserialize)]
struct ViewerRecord {
    port: u16,
    pid: u32,
    /// Shutdown token: `serve --stop` reads it from this file, which a
    /// drive-by web page cannot do, so only local commands can stop the
    /// viewer. Empty for viewers started by older binaries.
    #[serde(default)]
    token: String,
}

/// How a live open request was satisfied.
pub enum LiveOpen {
    /// No tab was connected, so a browser tab was opened at this URL.
    OpenedTab(String),
    /// At least one tab is connected and will follow the version bump.
    Updated(String),
}

/// A probed, confirmed-alive viewer.
pub struct LiveStatus {
    pub url: String,
    pub clients: usize,
}

struct Shared {
    version: Mutex<String>,
    changed: Condvar,
    clients: AtomicUsize,
    shutdown_token: String,
}

/// Record a new UI version so connected tabs reload, and name the page a
/// connected tab should show next (`None` keeps every tab on its current
/// page and merely refreshes it).
pub fn bump_version(ui_dir: &Path, page: Option<&str>) -> Result<()> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    let marker = serde_json::json!({
        "nonce": format!("{nonce}-{}", std::process::id()),
        "page": page,
    });
    atomic_write(&ui_dir.join(VERSION_FILE), marker.to_string())
}

/// Probe the recorded viewer for this UI directory, if one is alive.
pub fn live_status(ui_dir: &Path) -> Option<LiveStatus> {
    let record = fs::read_to_string(ui_dir.join(VIEWER_FILE)).ok()?;
    let record: ViewerRecord = serde_json::from_str(&record).ok()?;
    let body = http_get(record.port, "/status")?;
    let status: serde_json::Value = serde_json::from_str(&body).ok()?;
    if status.get("deltaforge_viewer") != Some(&serde_json::Value::Bool(true)) {
        return None;
    }
    Some(LiveStatus {
        url: format!("http://127.0.0.1:{}/", record.port),
        clients: status.get("clients")?.as_u64()? as usize,
    })
}

/// Ensure a viewer is running for this UI directory and show `page` in the
/// browser: reuse the connected tab when one exists, otherwise open a new
/// tab. Falls back with an error so callers can open the file directly.
pub fn open_live(ui_dir: &Path, page: &str) -> Result<LiveOpen> {
    bump_version(ui_dir, Some(page))?;
    let status = match live_status(ui_dir) {
        Some(status) => status,
        None => spawn_viewer(ui_dir)?,
    };
    let url = format!("{}{page}", status.url);
    if status.clients > 0 {
        return Ok(LiveOpen::Updated(url));
    }
    crate::learning_web::open_in_browser(url.as_ref())?;
    Ok(LiveOpen::OpenedTab(url))
}

fn spawn_viewer(ui_dir: &Path) -> Result<LiveStatus> {
    let executable = std::env::current_exe().context("failed to locate the deltaforge binary")?;
    let mut command = Command::new(executable);
    command
        .arg("serve")
        .arg("--quiet")
        .arg("--auto")
        .arg("--ui-dir")
        .arg(ui_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command.spawn().context("failed to start the live viewer")?;

    let deadline = Instant::now() + SPAWN_WAIT;
    while Instant::now() < deadline {
        if let Some(status) = live_status(ui_dir) {
            return Ok(status);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    bail!("the live viewer did not start in time")
}

/// Run the viewer server for `ui_dir` until the process is stopped. When
/// `auto` is set the server exits by itself after thirty minutes without a
/// connected tab, so auto-spawned viewers never outlive a working session.
pub fn serve(ui_dir: &Path, auto: bool, open: bool, quiet: bool) -> Result<()> {
    fs::create_dir_all(ui_dir)
        .with_context(|| format!("failed to create UI directory {}", ui_dir.display()))?;
    if let Some(existing) = live_status(ui_dir) {
        if !quiet {
            println!("A live viewer is already running at {}", existing.url);
        }
        if open {
            crate::learning_web::open_in_browser(existing.url.as_ref())?;
        }
        return Ok(());
    }
    if !ui_dir.join(VERSION_FILE).exists() {
        bump_version(ui_dir, None)?;
    }

    let listener = bind_listener(ui_dir)?;
    let port = listener.local_addr()?.port();
    let token = shutdown_token(ui_dir);
    let record = ViewerRecord {
        port,
        pid: std::process::id(),
        token: token.clone(),
    };
    atomic_write(
        &ui_dir.join(VIEWER_FILE),
        serde_json::to_string(&record)?,
    )?;

    let url = format!("http://127.0.0.1:{port}/");
    if !quiet {
        println!("Serving the live report at {url} (Ctrl-C to stop)");
    }
    if open {
        let _ = crate::learning_web::open_in_browser(url.as_ref());
    }
    serve_on(listener, ui_dir.to_path_buf(), auto, token)
}

/// Stop the viewer recorded for this UI directory. Returns whether a
/// running viewer was actually stopped.
pub fn stop(ui_dir: &Path) -> Result<bool> {
    let record_path = ui_dir.join(VIEWER_FILE);
    let Ok(record) = fs::read_to_string(&record_path) else {
        return Ok(false);
    };
    let Ok(record) = serde_json::from_str::<ViewerRecord>(&record) else {
        let _ = fs::remove_file(&record_path);
        return Ok(false);
    };
    if live_status(ui_dir).is_none() {
        let _ = fs::remove_file(&record_path);
        return Ok(false);
    }
    if record.token.is_empty() {
        bail!(
            "the running viewer was started by an older deltaforge and cannot be stopped remotely; end process {} manually",
            record.pid
        );
    }
    http_get(record.port, &format!("/shutdown?token={}", record.token))
        .context("the running viewer rejected the shutdown request")?;
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        if live_status(ui_dir).is_none() {
            let _ = fs::remove_file(&record_path);
            return Ok(true);
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    bail!(
        "the viewer acknowledged the shutdown but is still answering; end process {} manually",
        record.pid
    )
}

fn shutdown_token(ui_dir: &Path) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in ui_dir
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

/// Prefer a stable per-project port so the learner's bookmarked tab keeps
/// working across sessions; fall back to an ephemeral port when it is taken
/// by something that is not a deltaforge viewer.
fn bind_listener(ui_dir: &Path) -> Result<TcpListener> {
    let loopback = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let preferred = preferred_port(ui_dir);
    if let Ok(listener) = TcpListener::bind(SocketAddr::new(loopback, preferred)) {
        return Ok(listener);
    }
    TcpListener::bind(SocketAddr::new(loopback, 0)).context("failed to bind the live viewer")
}

fn preferred_port(ui_dir: &Path) -> u16 {
    let canonical = ui_dir
        .canonicalize()
        .unwrap_or_else(|_| ui_dir.to_path_buf());
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in canonical.to_string_lossy().as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    49152 + (hash % 16000) as u16
}

fn serve_on(listener: TcpListener, ui_dir: PathBuf, auto: bool, shutdown_token: String) -> Result<()> {
    let shared = Arc::new(Shared {
        version: Mutex::new(read_version(&ui_dir)),
        changed: Condvar::new(),
        clients: AtomicUsize::new(0),
        shutdown_token,
    });

    let watcher_shared = Arc::clone(&shared);
    let watcher_dir = ui_dir.clone();
    std::thread::spawn(move || watch_version(&watcher_dir, &watcher_shared, auto));

    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let shared = Arc::clone(&shared);
        let ui_dir = ui_dir.clone();
        std::thread::spawn(move || {
            let _ = handle_connection(stream, &shared, &ui_dir);
        });
    }
    Ok(())
}

fn watch_version(ui_dir: &Path, shared: &Shared, auto: bool) {
    let mut last_active = Instant::now();
    loop {
        std::thread::sleep(WATCH_INTERVAL);
        let current = read_version(ui_dir);
        {
            let mut version = shared.version.lock().expect("viewer lock poisoned");
            if *version != current {
                *version = current;
                shared.changed.notify_all();
            }
        }
        if auto {
            if shared.clients.load(Ordering::SeqCst) > 0 {
                last_active = Instant::now();
            } else if last_active.elapsed() > IDLE_SHUTDOWN {
                let _ = fs::remove_file(ui_dir.join(VIEWER_FILE));
                std::process::exit(0);
            }
        }
    }
}

fn read_version(ui_dir: &Path) -> String {
    fs::read_to_string(ui_dir.join(VERSION_FILE)).unwrap_or_default()
}

fn handle_connection(mut stream: TcpStream, shared: &Shared, ui_dir: &Path) -> Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let target = match read_request(&mut stream) {
        Request::Get(target) => target,
        Request::Unsupported => {
            return respond_simple(
                &mut stream,
                "405 Method Not Allowed",
                "text/plain",
                "This viewer serves GET requests only.",
            );
        }
        // Browsers open speculative connections that never send a request;
        // answering them would put junk on a socket the browser may reuse.
        Request::Silent => return Ok(()),
    };
    let (path, query) = target.split_once('?').unwrap_or((target.as_str(), ""));

    match path {
        "/shutdown" => {
            let provided = query
                .split('&')
                .find_map(|pair| pair.strip_prefix("token="))
                .unwrap_or_default();
            if shared.shutdown_token.is_empty() || provided != shared.shutdown_token {
                return respond_simple(&mut stream, "403 Forbidden", "text/plain", "bad token");
            }
            respond_simple(&mut stream, "200 OK", "application/json", "{\"stopping\":true}")?;
            let _ = fs::remove_file(ui_dir.join(VIEWER_FILE));
            // Give the response bytes a moment to leave the kernel buffer.
            std::thread::sleep(Duration::from_millis(50));
            std::process::exit(0);
        }
        "/" => {
            let default_page = if ui_dir.join("test-report.html").exists() {
                "/test-report.html"
            } else {
                "/learning.html"
            };
            let response = format!(
                "HTTP/1.1 302 Found\r\nLocation: {default_page}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            stream.write_all(response.as_bytes())?;
            Ok(())
        }
        "/status" => {
            let body = serde_json::json!({
                "deltaforge_viewer": true,
                "clients": shared.clients.load(Ordering::SeqCst),
            })
            .to_string();
            respond_simple(&mut stream, "200 OK", "application/json", &body)
        }
        "/events" => serve_events(stream, shared),
        _ => serve_page(&mut stream, ui_dir, path),
    }
}

enum Request {
    Get(String),
    Unsupported,
    Silent,
}

/// Read one request. Only the request line matters for routing, so it is
/// parsed as soon as it is complete; the remaining headers are drained
/// best-effort because cookies scoped to `127.0.0.1` are shared across every
/// local port and routinely exceed a small buffer.
fn read_request(stream: &mut TcpStream) -> Request {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 4096];
    while !buffer.windows(2).any(|window| window == b"\r\n") {
        if buffer.len() > MAX_REQUEST_LINE_BYTES {
            return Request::Unsupported;
        }
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => buffer.extend_from_slice(&chunk[..read]),
            Err(_) if buffer.is_empty() => return Request::Silent,
            Err(_) => return Request::Unsupported,
        }
    }
    if buffer.is_empty() {
        return Request::Silent;
    }
    let request = String::from_utf8_lossy(&buffer);
    let Some(line) = request.lines().next() else {
        return Request::Silent;
    };
    let mut parts = line.split(' ');
    let method = parts.next().unwrap_or_default().to_string();
    let Some(target) = parts.next().map(str::to_string) else {
        return Request::Unsupported;
    };
    // Drain the rest of the headers so the response is not cut off by a
    // connection reset while the client is still sending; give up quietly
    // at a generous cap.
    while !buffer.windows(4).any(|window| window == b"\r\n\r\n")
        && buffer.len() <= MAX_REQUEST_BYTES
    {
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => buffer.extend_from_slice(&chunk[..read]),
        }
    }
    if method == "GET" {
        Request::Get(target)
    } else {
        Request::Unsupported
    }
}

fn serve_page(stream: &mut TcpStream, ui_dir: &Path, path: &str) -> Result<()> {
    let name = path.trim_start_matches('/');
    let is_safe_page = !name.is_empty()
        && name.ends_with(".html")
        && !name.contains("..")
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'));
    if !is_safe_page {
        return respond_simple(stream, "404 Not Found", "text/plain", "not found");
    }
    let file = ui_dir.join(name);
    let Ok(mut body) = fs::read_to_string(&file) else {
        // The known pages get a placeholder that reloads itself once the
        // page is generated, so navigation links work before the first run.
        let hint = match name {
            "learning.html" => "Run <code>deltaforge instructions</code> or <code>deltaforge overview</code> to generate the learning guide.",
            "test-report.html" => "Run <code>deltaforge test</code> to generate the test report.",
            _ => return respond_simple(stream, "404 Not Found", "text/plain", "not found"),
        };
        let mut body = format!(
            r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="color-scheme" content="light dark"><title>DeltaForge</title>
<style>body{{margin:0;min-height:100vh;display:grid;place-items:center;font:16px/1.6 ui-sans-serif,-apple-system,sans-serif}}main{{text-align:center;padding:2rem}}h1{{font:500 2.2rem Georgia,serif;letter-spacing:-.02em}}p{{color:#777}}code{{font-family:ui-monospace,monospace}}</style>
</head><body><main><h1>Nothing here yet</h1><p>{hint}</p><p>This page updates by itself as soon as it exists.</p></main></body></html>"#
        );
        body.push_str(&live_reload_script(ui_dir, name));
        return respond_simple(stream, "200 OK", "text/html; charset=utf-8", &body);
    };
    body.push_str(&live_reload_script(ui_dir, name));
    respond_simple(stream, "200 OK", "text/html; charset=utf-8", &body)
}

/// The generated files remain valid standalone documents; the reload hook is
/// appended only when a page travels through the viewer. Embedding the
/// version that was current at serve time closes the gap where a bump lands
/// between rendering the page and the event stream connecting.
fn live_reload_script(ui_dir: &Path, page: &str) -> String {
    let current = serde_json::json!({
        "raw": read_version(ui_dir),
        "page": page,
    });
    format!(
        r#"
<script>
(() => {{
  const current = {current};
  const source = new EventSource("/events");
  source.onmessage = (event) => {{
    if (event.data === current.raw) return;
    let marker;
    try {{ marker = JSON.parse(event.data); }} catch {{ marker = null; }}
    if (marker && marker.page && marker.page !== current.page) {{
      location.href = "/" + marker.page;
    }} else {{
      location.reload();
    }}
  }};
}})();
</script>
"#
    )
}

fn serve_events(mut stream: TcpStream, shared: &Shared) -> Result<()> {
    stream.set_read_timeout(None)?;
    stream.write_all(
        b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\n\r\n",
    )?;

    struct ClientGuard<'a>(&'a AtomicUsize);
    impl Drop for ClientGuard<'_> {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }
    shared.clients.fetch_add(1, Ordering::SeqCst);
    let _guard = ClientGuard(&shared.clients);

    let mut known = shared.version.lock().expect("viewer lock poisoned").clone();
    stream.write_all(sse_event(&known).as_bytes())?;
    loop {
        let (version, timeout) = {
            let guard = shared.version.lock().expect("viewer lock poisoned");
            let (guard, timeout) = shared
                .changed
                .wait_timeout(guard, HEARTBEAT_INTERVAL)
                .expect("viewer lock poisoned");
            (guard.clone(), timeout.timed_out())
        };
        let payload = if !timeout && version != known {
            known = version;
            sse_event(&known)
        } else {
            ": keep-alive\n\n".to_string()
        };
        if stream.write_all(payload.as_bytes()).is_err() {
            return Ok(());
        }
    }
}

fn sse_event(version: &str) -> String {
    // SSE data lines cannot contain raw newlines; the marker is one JSON line.
    format!("data: {}\n\n", version.replace('\n', " "))
}

fn respond_simple(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &str,
) -> Result<()> {
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
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
    let request = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).ok()?;
    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;
    if !response.starts_with("HTTP/1.1 200") {
        return None;
    }
    let body = response.split("\r\n\r\n").nth(1)?;
    Some(body.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn start_test_server(ui_dir: &Path) -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let dir = ui_dir.to_path_buf();
        std::thread::spawn(move || serve_on(listener, dir, false, "test-token".to_string()));
        port
    }

    fn temp_ui_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("deltaforge-viewer-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn raw_get(port: u16, path: &str) -> String {
        let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let request = format!("GET {path} HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n");
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        let _ = stream.read_to_string(&mut response);
        response
    }

    #[test]
    fn serves_known_pages_with_live_reload_and_rejects_traversal() {
        let ui_dir = temp_ui_dir("pages");
        fs::write(ui_dir.join("test-report.html"), "<h1>report</h1>").unwrap();
        fs::write(ui_dir.join("secret.txt"), "secret").unwrap();
        bump_version(&ui_dir, Some("test-report.html")).unwrap();
        let port = start_test_server(&ui_dir);

        let page = raw_get(port, "/test-report.html");
        assert!(page.contains("<h1>report</h1>"));
        assert!(page.contains("EventSource"));

        assert!(raw_get(port, "/secret.txt").starts_with("HTTP/1.1 404"));
        assert!(raw_get(port, "/../Cargo.toml").starts_with("HTTP/1.1 404"));
        assert!(raw_get(port, "/missing.html").starts_with("HTTP/1.1 404"));

        // A known page that has not been generated yet gets a live-updating
        // placeholder instead of a dead link.
        let placeholder = raw_get(port, "/learning.html");
        assert!(placeholder.starts_with("HTTP/1.1 200"));
        assert!(placeholder.contains("Nothing here yet"));
        assert!(placeholder.contains("EventSource"));

        let root = raw_get(port, "/");
        assert!(root.starts_with("HTTP/1.1 302"));
        assert!(root.contains("Location: /test-report.html"));

        let status = raw_get(port, "/status");
        assert!(status.contains("\"deltaforge_viewer\":true"));

        // Shutdown requires the token from viewer.json; wrong or missing
        // tokens are refused so a drive-by web page cannot stop the viewer.
        assert!(raw_get(port, "/shutdown").starts_with("HTTP/1.1 403"));
        assert!(raw_get(port, "/shutdown?token=wrong").starts_with("HTTP/1.1 403"));
        assert!(raw_get(port, "/status").contains("\"deltaforge_viewer\":true"));
        let _ = fs::remove_dir_all(&ui_dir);
    }

    #[test]
    fn large_cookie_headers_do_not_break_get_requests() {
        let ui_dir = temp_ui_dir("cookies");
        fs::write(ui_dir.join("test-report.html"), "<h1>report</h1>").unwrap();
        bump_version(&ui_dir, None).unwrap();
        let port = start_test_server(&ui_dir);

        // Cookies on 127.0.0.1 are shared across every local port, so real
        // browser requests routinely carry tens of kilobytes of headers.
        let cookie = "x".repeat(64 * 1024);
        let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let request = format!(
            "GET /test-report.html HTTP/1.1\r\nHost: x\r\nCookie: c={cookie}\r\nConnection: close\r\n\r\n"
        );
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        let _ = stream.read_to_string(&mut response);
        assert!(
            response.starts_with("HTTP/1.1 200"),
            "large-header GET failed: {}",
            response.lines().next().unwrap_or("<empty>")
        );
        assert!(response.contains("<h1>report</h1>"));

        let post = raw_get(port, "/test-report.html").starts_with("HTTP/1.1 200");
        assert!(post, "plain GET must still succeed");
        let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        stream
            .write_all(b"POST /status HTTP/1.1\r\nHost: x\r\nContent-Length: 0\r\n\r\n")
            .unwrap();
        let mut response = String::new();
        let _ = stream.read_to_string(&mut response);
        assert!(response.starts_with("HTTP/1.1 405"), "POST should be 405: {response}");
        let _ = fs::remove_dir_all(&ui_dir);
    }

    #[test]
    fn version_bump_reaches_event_stream() {
        let ui_dir = temp_ui_dir("events");
        bump_version(&ui_dir, None).unwrap();
        let port = start_test_server(&ui_dir);

        let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        stream
            .write_all(b"GET /events HTTP/1.1\r\nHost: x\r\n\r\n")
            .unwrap();

        let mut received = String::new();
        let mut chunk = [0_u8; 1024];
        let deadline = Instant::now() + Duration::from_secs(3);
        while !received.contains("data: ") && Instant::now() < deadline {
            if let Ok(read) = stream.read(&mut chunk) {
                received.push_str(&String::from_utf8_lossy(&chunk[..read]));
            }
        }
        assert!(received.contains("data: "), "missing initial event: {received}");

        bump_version(&ui_dir, Some("learning.html")).unwrap();
        let deadline = Instant::now() + Duration::from_secs(3);
        while !received.contains("learning.html") && Instant::now() < deadline {
            if let Ok(read) = stream.read(&mut chunk) {
                received.push_str(&String::from_utf8_lossy(&chunk[..read]));
            }
        }
        assert!(
            received.contains("learning.html"),
            "bump never arrived: {received}"
        );
        let _ = fs::remove_dir_all(&ui_dir);
    }
}
