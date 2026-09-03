use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use base64::Engine as _;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::application;
use crate::context::GlobalOptions;
use crate::fs_util::atomic_write_private;

const API_VERSION: &str = "v1";
const SERVICE_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-app3");
const PROBE_TIMEOUT: Duration = Duration::from_millis(500);
const STARTUP_TIMEOUT: Duration = Duration::from_secs(4);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const EVENT_POLL_INTERVAL: Duration = Duration::from_millis(500);
const MAX_SOURCE_POLL_INTERVAL: Duration = Duration::from_secs(5);
const IDLE_TIMEOUT: Duration = Duration::from_secs(30 * 60);
/// How many accepted-but-unauthenticated connections may be in flight at once.
/// Far above what a browser opens for the workbench (a handful of fetches plus
/// one SSE stream per tab, and the stream stops counting as soon as it
/// authenticates), and far below unbounded.
const MAX_PRE_AUTH_CONNECTIONS: usize = 128;
const MAX_HEADER_BYTES: usize = 32 * 1024;
const MAX_BODY_BYTES: usize = 8 * 1024;
/// Header the page sends the capability token in once it has loaded, instead
/// of the query string. Lower-cased to match `read_request`'s header keys.
const CAPABILITY_HEADER: &str = "x-deltaforge-token";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServiceRecord {
    port: u16,
    pid: u32,
    token: String,
    version: String,
    /// A random value, not itself a capability, that only the process which
    /// wrote this (owner-only) record file could have read. Used to confirm
    /// the listener on `port` is this same service before the capability
    /// token is ever sent to it. See `verify_identity`.
    probe_id: String,
}

#[derive(Debug, Clone)]
struct ServiceStatus {
    version: String,
    pid: u32,
    clients: usize,
}

struct StartupLease {
    file: File,
}

impl StartupLease {
    fn acquire(path: PathBuf) -> Result<Self> {
        crate::project_registry::ensure_private_application_home()?;
        let deadline = Instant::now() + STARTUP_TIMEOUT;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .with_context(|| format!("failed to open startup lease {}", path.display()))?;
        loop {
            match file.try_lock_exclusive() {
                Ok(()) => return Ok(Self { file }),
                Err(error) if crate::fs_util::lock_unavailable(&error) => {
                    if Instant::now() >= deadline {
                        bail!("another DeltaForge workbench launch is still starting");
                    }
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(error) => return Err(error).context("failed to lock workbench startup lease"),
            }
        }
    }
}

impl Drop for StartupLease {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}

#[derive(Debug)]
struct Shared {
    default_project_id: Option<String>,
    token: String,
    probe_id: String,
    session_id: String,
    port: u16,
    clients: AtomicUsize,
    last_activity: Mutex<Instant>,
    record_path: PathBuf,
    run_starting: Mutex<BTreeSet<String>>,
    shutting_down: AtomicBool,
    idle_timeout: Duration,
    focus_revision: AtomicUsize,
    focus_target: Mutex<String>,
    /// Projects with a live project-event stream, counted per open tab. The
    /// source watcher has no reason to walk every project a learner has ever
    /// registered; only these can currently receive the event.
    watched_projects: Mutex<BTreeMap<String, WatchedProject>>,
    /// Connections accepted but not yet past the token check. Bounded by
    /// `MAX_PRE_AUTH_CONNECTIONS`; see `PreAuthSlot`.
    pre_auth: AtomicUsize,
    /// Whether `POST /api/v1/__panic` exists. Read once from
    /// `DELTAFORGE_PANIC_PROBE` at start, so the route is present only in a
    /// service that was deliberately started to test panic recovery.
    panic_probe: bool,
}

#[derive(Debug, Clone)]
struct WatchedProject {
    options: GlobalOptions,
    clients: usize,
}

/// A permit to occupy one of the bounded pre-authentication slots, released
/// the moment a connection proves it holds the capability token.
///
/// The service accepts a connection and spawns a thread for it before knowing
/// whether the peer is the learner's browser or anything else that can reach
/// loopback, so without a bound an unauthenticated peer can make the service
/// hold arbitrarily many threads and sockets just by connecting. A plain
/// worker pool is the wrong shape here: `/api/v1/events` is a long-lived SSE
/// stream that holds its thread for as long as the tab is open, so a fixed
/// pool would be filled by legitimate readers and stop serving anyone. Bounding
/// only the *unauthenticated* window keeps that from happening — an authorized
/// client gives its slot back immediately and then streams for as long as it
/// likes, and the cap it left behind still applies to everyone who has not
/// proved anything.
struct PreAuthSlot {
    shared: Arc<Shared>,
    held: bool,
}

impl PreAuthSlot {
    /// Reserve a slot, or return `None` when the cap is already reached.
    fn reserve(shared: &Arc<Shared>) -> Option<Self> {
        let previous = shared
            .pre_auth
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |count| {
                (count < MAX_PRE_AUTH_CONNECTIONS).then_some(count + 1)
            });
        previous.ok().map(|_| Self {
            shared: Arc::clone(shared),
            held: true,
        })
    }

    /// Give the slot back now: this connection has authenticated, so whatever
    /// it does next is a legitimate client's business and must not be counted
    /// against the unauthenticated bound.
    fn release(&mut self) {
        if std::mem::take(&mut self.held) {
            self.shared.pre_auth.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

impl Drop for PreAuthSlot {
    fn drop(&mut self) {
        self.release();
    }
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    target: String,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct StartRunBody {
    #[serde(default)]
    filter: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RerunBody {
    test: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct StartBenchmarkBody {
    /// Save the measurement to the project's benchmark history. The browser
    /// always saves: a measurement the learner cannot compare later is of no
    /// use to them.
    #[serde(default = "default_true")]
    save: bool,
}

fn default_true() -> bool {
    true
}

/// Creation request from the browser. `parent_directory` is the one path a
/// browser request may supply; `crate::creation::resolve_target` decides
/// whether it is acceptable.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateProjectBody {
    #[serde(default)]
    pack: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    parent_directory: Option<String>,
    #[serde(default)]
    name: String,
    #[serde(default = "default_true")]
    git: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct LearnerNoteBody {
    #[serde(default)]
    text: String,
    #[serde(default)]
    skipped: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyBody {}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExportReportBody {
    #[serde(default)]
    format: ExportFormat,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ExportFormat {
    #[default]
    Markdown,
    Html,
    Json,
}

impl From<ExportFormat> for crate::reporting::ReportFormat {
    fn from(format: ExportFormat) -> Self {
        match format {
            ExportFormat::Markdown => Self::Markdown,
            ExportFormat::Html => Self::Html,
            ExportFormat::Json => Self::Json,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ProjectOpenKind {
    Editor,
    Folder,
}

struct ProjectOpenCommand {
    command: Command,
    application: String,
}

#[derive(Debug, Serialize)]
struct ProjectSummary {
    id: String,
    name: String,
    description: String,
    language: String,
    path: String,
    current_step: String,
    current_step_number: usize,
    total_steps: usize,
    completed_steps: usize,
    last_opened_at: u64,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue: Option<String>,
}

pub fn launch(options: &GlobalOptions) -> Result<()> {
    let root = if options.project_dir.is_some() {
        Some(crate::context::locate_project_root(options)?)
    } else {
        crate::context::locate_project_root(options).ok()
    };
    let startup_lease = StartupLease::acquire(crate::project_registry::startup_lock_path()?)?;
    let registered = root
        .as_deref()
        .map(|root| crate::project_registry::register(root, options.packs_dir.as_deref()))
        .transpose()?;
    let record_path = crate::project_registry::service_record_path()?;
    let mut service = read_compatible_record(&record_path)?;

    if service.is_none() {
        let token = capability_token()?;
        spawn_service(root.as_deref(), options, &token)?;
        service = wait_for_service(&record_path)?;
    }

    let (record, status) = service.context("the DeltaForge workbench did not start in time")?;
    drop(startup_lease);
    let route = registered.as_ref().map_or_else(
        || "/projects".to_string(),
        |project| format!("/projects/{}/overview", project.id),
    );
    let url = format!(
        "http://127.0.0.1:{}{}?token={}",
        record.port, route, record.token
    );
    if status.clients > 0 && request_focus(&record, &route) {
        println!("DeltaForge is ready at {url}");
        return Ok(());
    }
    if std::env::var_os("DELTAFORGE_NO_BROWSER").is_some() {
        println!("DeltaForge is ready at {url}");
    } else {
        match open_in_browser(url.as_ref()) {
            Ok(()) => println!("DeltaForge is ready at {url}"),
            Err(error) => {
                println!("DeltaForge is ready at {url}");
                println!("Browser opening failed: {error:#}");
            }
        }
    }
    Ok(())
}

fn open_in_browser(target: &std::ffi::OsStr) -> Result<()> {
    let mut command = browser_command(target)?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
        .spawn()
        .with_context(|| format!("failed to open {} in a browser", target.to_string_lossy()))?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("open");
    command.arg(target);
    Ok(command)
}

#[cfg(target_os = "linux")]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("xdg-open");
    command.arg(target);
    Ok(command)
}

#[cfg(windows)]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("rundll32");
    command.arg("url.dll,FileProtocolHandler").arg(target);
    Ok(command)
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
fn browser_command(_target: &std::ffi::OsStr) -> Result<Command> {
    bail!("opening a browser is not supported on this operating system")
}

fn spawn_service(root: Option<&Path>, options: &GlobalOptions, token: &str) -> Result<()> {
    let executable = std::env::current_exe().context("failed to locate the deltaforge binary")?;
    let mut command = Command::new(executable);
    if let Some(root) = root {
        command.arg("--project-dir").arg(root);
    }
    command
        .arg("__workbench")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    // Integration tests use the real detached launcher. Give a service whose
    // test process aborts before cleanup a bounded lifetime without exposing a
    // learner-facing timeout setting or weakening the 30-minute default.
    if let Some(timeout) = test_idle_timeout_from_environment()? {
        command
            .arg("--idle-timeout-ms")
            .arg(timeout.as_millis().to_string());
    }
    if let Some(packs_dir) = &options.packs_dir {
        command.arg("--packs-dir").arg(packs_dir);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let mut child = command
        .spawn()
        .context("failed to start the DeltaForge workbench service")?;
    let mut stdin = child
        .stdin
        .take()
        .context("workbench service stdin pipe is missing")?;
    stdin
        .write_all(token.as_bytes())
        .context("failed to send the workbench capability to the service")?;
    drop(stdin);
    Ok(())
}

fn test_idle_timeout_from_environment() -> Result<Option<Duration>> {
    parse_test_idle_timeout(std::env::var_os("DELTAFORGE_TEST_IDLE_TIMEOUT_MS"))
}

fn parse_test_idle_timeout(value: Option<std::ffi::OsString>) -> Result<Option<Duration>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let value = value
        .to_str()
        .context("DELTAFORGE_TEST_IDLE_TIMEOUT_MS must be valid UTF-8")?;
    let milliseconds = value
        .parse::<u64>()
        .context("DELTAFORGE_TEST_IDLE_TIMEOUT_MS must be a positive integer")?;
    if milliseconds == 0 {
        bail!("DELTAFORGE_TEST_IDLE_TIMEOUT_MS must be greater than zero");
    }
    Ok(Some(Duration::from_millis(milliseconds)))
}

pub fn service_idle_timeout(argument_ms: Option<u64>) -> Result<Option<Duration>> {
    match argument_ms {
        Some(milliseconds) => Ok(Some(Duration::from_millis(milliseconds))),
        None => test_idle_timeout_from_environment(),
    }
}

pub fn service_token(argument: Option<String>) -> Result<String> {
    let token = match argument {
        Some(token) => token,
        None => {
            let mut token = String::new();
            std::io::stdin()
                .read_to_string(&mut token)
                .context("failed to read the workbench capability from stdin")?;
            token
        }
    };
    if token.is_empty() {
        bail!("workbench capability token must not be empty");
    }
    Ok(token)
}

fn wait_for_service(record_path: &Path) -> Result<Option<(ServiceRecord, ServiceStatus)>> {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    while Instant::now() < deadline {
        if let Some(record) = read_compatible_record(record_path)? {
            return Ok(Some(record));
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(None)
}

fn read_compatible_record(record_path: &Path) -> Result<Option<(ServiceRecord, ServiceStatus)>> {
    let source = match fs::read_to_string(record_path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let record: ServiceRecord = match serde_json::from_str(&source) {
        Ok(record) => record,
        Err(_) => {
            let _ = fs::remove_file(record_path);
            return Ok(None);
        }
    };
    let Some(status) = probe(&record) else {
        remove_record_if_matches(record_path, &record);
        return Ok(None);
    };
    if record.version != SERVICE_VERSION || status.version != SERVICE_VERSION {
        replace_incompatible_service(record_path, &record)?;
        return Ok(None);
    }
    if record.pid != status.pid {
        // The listener passed identity verification (it knew `probe_id`, a
        // value only readable from this owner-only record file) yet reports a
        // pid different from the one recorded at startup. That should never
        // happen for a legitimate, unmodified service; treat it as absent
        // rather than trusting either the stale local value or the remote
        // claim.
        remove_record_if_matches(record_path, &record);
        return Ok(None);
    }
    Ok(Some((record, status)))
}

/// Confirm the process listening on `record.port` is the DeltaForge service
/// that wrote this record, before ever sending it the capability token or
/// treating its responses (version, pid, clients) as authoritative.
///
/// `probe_id` is not a capability — it is served to anyone who asks, with no
/// token required — but only the process that created this exact
/// owner-only-readable record file could have read it back off disk. An
/// unrelated (or hostile) process that happens to be listening on the
/// recorded port cannot know it, so a mismatch here means "this is not our
/// service" and the token must not be sent to it.
fn verify_identity(record: &ServiceRecord) -> Option<()> {
    let body = http_get(record.port, &format!("/api/{API_VERSION}/identity"))?;
    let value = serde_json::from_str::<serde_json::Value>(&body).ok()?;
    (value.get("service")?.as_str()? == "deltaforge"
        && value.get("probe_id")?.as_str()? == record.probe_id)
        .then_some(())
}

fn probe(record: &ServiceRecord) -> Option<ServiceStatus> {
    verify_identity(record)?;
    let path = format!("/api/{API_VERSION}/health?token={}", record.token);
    let body = http_get(record.port, &path)?;
    let value = serde_json::from_str::<serde_json::Value>(&body).ok()?;
    (value.get("service")?.as_str()? == "deltaforge").then_some(ServiceStatus {
        version: value.get("version")?.as_str()?.to_string(),
        pid: u32::try_from(value.get("pid")?.as_u64()?).ok()?,
        clients: usize::try_from(
            value
                .get("clients")
                .and_then(|value| value.as_u64())
                .unwrap_or(0),
        )
        .ok()?,
    })
}

fn replace_incompatible_service(record_path: &Path, record: &ServiceRecord) -> Result<()> {
    if !request_shutdown(record) {
        bail!(
            "an incompatible DeltaForge workbench is still active; finish or cancel its check run, then launch DeltaForge again"
        );
    }
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    while Instant::now() < deadline {
        if probe(record).is_none() {
            remove_record_if_matches(record_path, record);
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    bail!("the incompatible DeltaForge workbench did not stop in time")
}

fn remove_record_if_matches(record_path: &Path, expected: &ServiceRecord) {
    let current = fs::read_to_string(record_path)
        .ok()
        .and_then(|source| serde_json::from_str::<ServiceRecord>(&source).ok());
    if current.as_ref().is_some_and(|record| {
        record.port == expected.port && record.pid == expected.pid && record.token == expected.token
    }) {
        let _ = fs::remove_file(record_path);
    }
}

fn request_focus(record: &ServiceRecord, route: &str) -> bool {
    let path = format!(
        "/api/{API_VERSION}/focus?token={}&route={route}",
        record.token
    );
    let origin = format!("http://127.0.0.1:{}", record.port);
    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nOrigin: {origin}\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}",
        record.port
    );
    http_exchange(record.port, &request)
        .is_some_and(|response| response.starts_with("HTTP/1.1 202"))
}

pub fn exit() -> Result<()> {
    let record_path = crate::project_registry::service_record_path()?;
    let source = match fs::read_to_string(&record_path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!("DeltaForge is not running.");
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let Ok(record) = serde_json::from_str::<ServiceRecord>(&source) else {
        let _ = fs::remove_file(record_path);
        println!("DeltaForge is not running.");
        return Ok(());
    };
    if probe(&record).is_none() {
        remove_record_if_matches(&record_path, &record);
        println!("DeltaForge is not running.");
        return Ok(());
    }
    if !request_shutdown(&record) {
        bail!("DeltaForge could not stop while a check run is active; cancel it and try again");
    }
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    while Instant::now() < deadline {
        if probe(&record).is_none() {
            remove_record_if_matches(&record_path, &record);
            println!("DeltaForge stopped.");
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    bail!("DeltaForge accepted the stop request but did not exit in time")
}

fn request_shutdown(record: &ServiceRecord) -> bool {
    let path = format!("/api/{API_VERSION}/service/shutdown?token={}", record.token);
    let origin = format!("http://127.0.0.1:{}", record.port);
    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nOrigin: {origin}\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}",
        record.port
    );
    http_exchange(record.port, &request)
        .is_some_and(|response| response.starts_with("HTTP/1.1 202"))
}

pub fn serve(options: &GlobalOptions, token: String, idle_timeout: Option<Duration>) -> Result<()> {
    if token.is_empty() {
        bail!("workbench capability token must not be empty");
    }
    let idle_timeout = idle_timeout.unwrap_or(IDLE_TIMEOUT);
    if idle_timeout.is_zero() {
        bail!("workbench idle timeout must be greater than zero");
    }
    let root = if options.project_dir.is_some() {
        Some(crate::context::locate_project_root(options)?)
    } else {
        None
    };
    let registered = root
        .as_deref()
        .map(|root| crate::project_registry::register(root, options.packs_dir.as_deref()))
        .transpose()?;
    let session_id = format!(
        "{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    if root.is_some() {
        let initial_session_id = registered.as_ref().map_or_else(
            || session_id.clone(),
            |project| format!("{}-{}", session_id, project.id),
        );
        let _ = application::load_workbench_state_for_session(options, &initial_session_id);
        let _ = application::observe_source_changes(options);
    }
    let record_path = crate::project_registry::service_record_path()?;
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
        .context("failed to bind the DeltaForge workbench to loopback")?;
    let port = listener.local_addr()?.port();
    let probe_id = random_hex_id(16)?;
    let record = ServiceRecord {
        port,
        pid: std::process::id(),
        token: token.clone(),
        version: SERVICE_VERSION.to_string(),
        probe_id: probe_id.clone(),
    };
    let home = crate::project_registry::ensure_private_application_home()?;
    install_panic_log(&home);
    atomic_write_private(&record_path, serde_json::to_string(&record)?)?;

    let shared = Arc::new(Shared {
        default_project_id: registered.map(|project| project.id),
        token,
        probe_id,
        session_id,
        port,
        clients: AtomicUsize::new(0),
        last_activity: Mutex::new(Instant::now()),
        record_path,
        run_starting: Mutex::new(BTreeSet::new()),
        shutting_down: AtomicBool::new(false),
        idle_timeout,
        focus_revision: AtomicUsize::new(0),
        focus_target: Mutex::new("/projects".to_string()),
        watched_projects: Mutex::new(BTreeMap::new()),
        pre_auth: AtomicUsize::new(0),
        panic_probe: std::env::var_os("DELTAFORGE_PANIC_PROBE").is_some(),
    });
    spawn_idle_watchdog(Arc::clone(&shared));
    spawn_source_watcher(Arc::clone(&shared));

    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            continue;
        };
        // Shed the connection before spending a thread on it. Closing without
        // a reply is deliberate: the accept loop is single-threaded, so writing
        // even a short 503 to a peer that never reads would stall every
        // subsequent accept — exactly the outcome the cap exists to prevent.
        let Some(slot) = PreAuthSlot::reserve(&shared) else {
            drop(stream);
            continue;
        };
        let shared = Arc::clone(&shared);
        std::thread::spawn(move || serve_one_connection(stream, shared, slot));
    }
    Ok(())
}

/// One connection, on its own thread, with a panic contained to it.
///
/// `handle_connection` runs behind `catch_unwind` because a panic in a handler
/// used to be terminal for the whole service, not just the request: the thread
/// died while holding one of the shared mutexes, every later `lock()` returned
/// a poisoned error, and each of those was an `expect` — so every subsequent
/// request panicked too. The browser hung with no error surface at all, and
/// the only trace went to a terminal the learner has probably closed, because
/// this runs in the background.
///
/// `Shared` holds a timestamp and a set of run identifiers behind mutexes;
/// there is no invariant a half-finished handler can break. So the answer to
/// a panicking request is a 500 for that request, a line in the panic log, and
/// a service that still works.
fn serve_one_connection(stream: TcpStream, shared: Arc<Shared>, slot: PreAuthSlot) {
    let reply = stream.try_clone();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = handle_connection(stream, &shared, slot);
    }));
    if outcome.is_err()
        && let Ok(mut reply) = reply
    {
        // Best effort: the handler may already have written a response, in
        // which case this is discarded by the client as trailing bytes on a
        // connection that is closing anyway. Saying nothing is worse — that is
        // the hang.
        let _ = respond(
            &mut reply,
            "500 Internal Server Error",
            "application/json",
            r#"{"error":"DeltaForge hit an internal error handling that request. The workbench is still running; try again."}"#,
        );
    }
}

/// Record panics to a file under the DeltaForge home, and keep the default
/// behaviour on stderr.
///
/// The workbench is started by a launcher and then detached. Without this, the
/// one artefact of a crash goes to a terminal that is closed by the time
/// anybody looks — which is the same reason `catch_unwind` above is not enough
/// on its own: a contained panic that leaves no trace is a bug nobody can
/// report.
pub fn install_panic_log(home: &Path) {
    let log_path = home.join("panic.log");
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let entry = format!(
            "{} thread {:?}\n{info}\n\n",
            humantime_now(),
            std::thread::current().name().unwrap_or("unnamed"),
        );
        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
        {
            use std::io::Write as _;
            let _ = file.write_all(entry.as_bytes());
        }
        previous(info);
    }));
}

/// An RFC 3339 timestamp, without pulling in a date library for one line.
fn humantime_now() -> String {
    let seconds = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default();
    format!("unix:{seconds}")
}

fn spawn_source_watcher(shared: Arc<Shared>) {
    std::thread::spawn(move || {
        let mut probes = HashMap::<String, SourceProbe>::new();
        let mut interval = EVENT_POLL_INTERVAL;
        loop {
            let watched = crate::sync::lock(&shared.watched_projects).clone();
            probes.retain(|id, _| watched.contains_key(id));
            let mut changed = false;
            for (id, project) in &watched {
                let probe = probes.entry(id.clone()).or_insert_with(SourceProbe::new);
                let stamp = project_directory_stamp(
                    project
                        .options
                        .project_dir
                        .as_deref()
                        .unwrap_or_else(|| Path::new(".")),
                );
                let directory_changed = probe.directory_stamp != stamp;
                if directory_changed || probe.last_walk.elapsed() >= MAX_SOURCE_POLL_INTERVAL {
                    changed |= application::observe_source_changes(&project.options)
                        .ok()
                        .flatten()
                        .is_some();
                    probe.directory_stamp = stamp;
                    probe.last_walk = Instant::now();
                }
            }
            if changed {
                *crate::sync::lock(&shared.last_activity) = Instant::now();
                interval = EVENT_POLL_INTERVAL;
            } else {
                interval = (interval * 2).min(MAX_SOURCE_POLL_INTERVAL);
            }
            std::thread::sleep(interval);
        }
    });
}

struct SourceProbe {
    directory_stamp: Option<Vec<(PathBuf, SystemTime)>>,
    last_walk: Instant,
}

impl SourceProbe {
    fn new() -> Self {
        Self {
            directory_stamp: None,
            last_walk: Instant::now() - MAX_SOURCE_POLL_INTERVAL,
        }
    }
}

/// A cheap signal for the common editor behaviour of atomically replacing a
/// file. Only the project root and its immediate directories are inspected;
/// a periodic full walk remains the correctness backstop for in-place writes
/// and changes deeper in the tree.
fn project_directory_stamp(root: &Path) -> Option<Vec<(PathBuf, SystemTime)>> {
    let mut stamp = vec![(PathBuf::new(), fs::metadata(root).ok()?.modified().ok()?)];
    for entry in fs::read_dir(root).ok()? {
        let entry = entry.ok()?;
        let metadata = entry.metadata().ok()?;
        if metadata.is_dir() {
            stamp.push((
                entry.file_name().into(),
                metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            ));
        }
    }
    stamp.sort_by(|left, right| left.0.cmp(&right.0));
    Some(stamp)
}

fn spawn_idle_watchdog(shared: Arc<Shared>) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(
                shared
                    .idle_timeout
                    .min(Duration::from_secs(30))
                    .max(Duration::from_millis(10)),
            );
            // Not `.lock().map(…).unwrap_or_default()`: that reported zero
            // idle time on a poisoned lock, so a service that had panicked
            // once would never shut down again.
            let idle = crate::sync::lock(&shared.last_activity).elapsed();
            let run_starting = !crate::sync::lock(&shared.run_starting).is_empty();
            let run_active = run_starting
                || crate::project_registry::list()
                    .unwrap_or_default()
                    .into_iter()
                    .any(|project| {
                        application::run_is_active(&options_for_entry(&project)).unwrap_or(false)
                    });
            if shared.clients.load(Ordering::SeqCst) == 0
                && !run_active
                && idle >= shared.idle_timeout
            {
                let _ = fs::remove_file(&shared.record_path);
                std::process::exit(0);
            }
        }
    });
}
fn handle_connection(
    mut stream: TcpStream,
    shared: &Arc<Shared>,
    mut pre_auth: PreAuthSlot,
) -> Result<()> {
    configure_request_stream(&stream)?;
    let Some(request) = read_request(&mut stream)? else {
        return Ok(());
    };
    *crate::sync::lock(&shared.last_activity) = Instant::now();

    let path = request
        .target
        .split_once('?')
        .map_or(request.target.as_str(), |(path, _)| path);

    // Deliberately unauthenticated: a launcher must be able to confirm which
    // service is listening on this port *before* it sends the capability
    // token. See `verify_identity`.
    //
    // The Host check is a consistency guard, not an anti-fingerprinting one —
    // a browser fetching `http://127.0.0.1:<port>/` always sends the matching
    // Host, so it costs a cross-origin page nothing. What actually keeps this
    // endpoint from telling another page anything is that the response carries
    // no CORS header (so its body is unreadable cross-origin) and is served
    // `application/json` with `nosniff` (so it cannot be run as a script).
    // `probe_id` is the only interesting value here, and neither route exposes it.
    if request.method == "GET" && path == "/api/v1/identity" {
        let expected_host = format!("127.0.0.1:{}", shared.port);
        if request.headers.get("host") != Some(&expected_host) {
            return respond(
                &mut stream,
                "403 Forbidden",
                "application/json",
                r#"{"error":"forbidden"}"#,
            );
        }
        let body = serde_json::json!({
            "service": "deltaforge",
            "api": API_VERSION,
            "probe_id": shared.probe_id,
        })
        .to_string();
        return respond(&mut stream, "200 OK", "application/json", &body);
    }

    if !authorized(&request, shared) {
        // A page route reached without a capability is a person, not a script:
        // a reload after the token was stripped from the address bar, a
        // bookmark, or a link opened in a new tab. Handing them raw JSON is a
        // dead end, so say what happened and how to get back.
        if request.method == "GET" && (path == "/" || is_page_route(path)) {
            return respond(
                &mut stream,
                "403 Forbidden",
                "text/html; charset=utf-8",
                UNAUTHORIZED_PAGE,
            );
        }
        return respond(
            &mut stream,
            "403 Forbidden",
            "application/json",
            r#"{"error":"forbidden"}"#,
        );
    }
    // Past the token check. Everything below may block for as long as the
    // client wants (`/api/v1/events` streams until the tab closes), so the
    // pre-authentication slot has to come back now rather than at return.
    pre_auth.release();

    if request.method != "GET" && request.method != "POST" {
        return respond(
            &mut stream,
            "405 Method Not Allowed",
            "application/json",
            r#"{"error":"method_not_allowed"}"#,
        );
    }
    let project_api = matches!(
        path,
        "/api/v1/project-health"
            | "/api/v1/state"
            | "/api/v1/capability"
            | "/api/v1/events"
            | "/api/v1/runs"
            | "/api/v1/runs/rerun"
            | "/api/v1/runs/cancel"
            | "/api/v1/benchmarks"
            | "/api/v1/predictions"
            | "/api/v1/reflections"
            | "/api/v1/snapshots"
            | "/api/v1/snapshots/preview"
            | "/api/v1/reports"
            | "/api/v1/hints"
            | "/api/v1/capabilities/next"
            | "/api/v1/project/repin-pack"
            | "/api/v1/project/open-editor"
            | "/api/v1/project/open-folder"
    );
    if project_api && project_request(shared, &request).is_err() {
        return respond(
            &mut stream,
            "404 Not Found",
            "application/json",
            r#"{"error":"project_not_found"}"#,
        );
    }

    // `dispatch` is fallible, and its failures used to leave the connection
    // closed with nothing written at all: the spawn site discards the error, so
    // a browser saw a bare network failure with no status and no message. Any
    // error that reaches here is answered instead.
    if let Err(error) = dispatch(&mut stream, shared, &request, path) {
        return respond_internal_error(&mut stream, &error);
    }
    Ok(())
}

/// Bound both halves of a normal HTTP exchange. The read bound prevents a
/// client from dribbling headers forever; the write bound is equally
/// important because an authenticated client that stops reading must not
/// retain a worker thread indefinitely.
fn configure_request_stream(stream: &TcpStream) -> Result<()> {
    stream.set_read_timeout(Some(REQUEST_TIMEOUT))?;
    stream.set_write_timeout(Some(REQUEST_TIMEOUT))?;
    Ok(())
}

/// Route one authorized request. Every arm either writes a response or returns
/// the error that stopped it; `handle_connection` turns the latter into a 500.
///
/// An arm that has already begun writing a response must not return `Err`, or
/// that 500 would be appended to a reply in progress. The two streaming arms
/// hold to this by reporting their own failures inside the event stream; see
/// `serve_events`.
fn dispatch(
    stream: &mut TcpStream,
    shared: &Arc<Shared>,
    request: &HttpRequest,
    path: &str,
) -> Result<()> {
    match (request.method.as_str(), path) {
        ("GET", "/") | ("GET", "/projects") | ("GET", "/catalog") | ("GET", "/create") => respond(
            stream,
            "200 OK",
            "text/html; charset=utf-8",
            &workbench_html(&shared.token),
        ),
        ("GET", route) if project_page_route(route) => {
            let project_id = route.split('/').nth(2).unwrap_or_default();
            if crate::project_registry::resolve(project_id).is_err() {
                return respond(
                    stream,
                    "404 Not Found",
                    "text/plain; charset=utf-8",
                    "Project not found",
                );
            }
            respond(
                stream,
                "200 OK",
                "text/html; charset=utf-8",
                &workbench_html(&shared.token),
            )
        }
        ("GET", "/api/v1/health") => {
            let body = serde_json::json!({
                "service": "deltaforge",
                "api": API_VERSION,
                "version": SERVICE_VERSION,
                "pid": std::process::id(),
                "clients": shared.clients.load(Ordering::SeqCst),
            })
            .to_string();
            respond(stream, "200 OK", "application/json", &body)
        }
        ("GET", "/api/v1/project-health") => {
            let (_, options) = project_request(shared, request)?;
            let health = application::load_project_health(&options)?;
            respond(
                stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&health)?,
            )
        }
        ("GET", "/api/v1/catalog") => {
            let catalog = application::load_catalog(&GlobalOptions::default())?;
            respond(
                stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&catalog)?,
            )
        }
        ("GET", "/api/v1/workspace") => {
            let body = match crate::creation::default_workspace() {
                Ok(path) => serde_json::json!({"default_directory": path.display().to_string()}),
                Err(error) => serde_json::json!({"error": format!("{error:#}")}),
            };
            respond(stream, "200 OK", "application/json", &body.to_string())
        }
        ("POST", "/api/v1/projects/preflight") | ("POST", "/api/v1/projects") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: CreateProjectBody = match parse_json_body(request) {
                Ok(body) => body,
                Err(_) => {
                    return respond(
                        stream,
                        "400 Bad Request",
                        "application/json",
                        r#"{"error":"invalid_json"}"#,
                    );
                }
            };
            create_project(stream, shared, path, body)
        }
        ("GET", "/api/v1/projects") => {
            let projects = load_project_summaries()?;
            respond(
                stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&projects)?,
            )
        }
        ("POST", "/api/v1/focus") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            if parse_json_body::<EmptyBody>(request).is_err() {
                return respond(
                    stream,
                    "400 Bad Request",
                    "application/json",
                    r#"{"error":"invalid_json"}"#,
                );
            }
            let route = query_value(&request.target, "route")
                .filter(|route| route.starts_with("/projects"))
                .unwrap_or("/projects")
                .to_string();
            *crate::sync::lock(&shared.focus_target) = route;
            shared.focus_revision.fetch_add(1, Ordering::SeqCst);
            respond(
                stream,
                "202 Accepted",
                "application/json",
                r#"{"status":"focus_requested"}"#,
            )
        }
        ("GET", "/api/v1/state") => {
            let (project_id, options) = project_request(shared, request)?;
            // A direct state read is also a declaration that this is the
            // project the caller is viewing. Browser tabs keep an event stream
            // open and are watched continuously; API clients that poll this
            // endpoint still get fresh source state without making the
            // background watcher scan every registered project.
            let _ = application::observe_source_changes(&options)?;
            let state = application::load_workbench_state_for_session(
                &options,
                &project_session_id(shared, &project_id),
            )?;
            let body = serde_json::to_string(&state)?;
            respond(stream, "200 OK", "application/json", &body)
        }
        ("GET", "/api/v1/capability") => {
            let (_, options) = project_request(shared, request)?;
            let content = application::load_capability_content(&options)?;
            let body = serde_json::to_string(&content)?;
            respond(stream, "200 OK", "application/json", &body)
        }
        ("GET", "/api/v1/app-events") => serve_app_events(stream, shared),
        ("GET", "/api/v1/events") => {
            let (project_id, options) = project_request(shared, request)?;
            serve_events(stream, shared, request, project_id, options)
        }
        ("POST", "/api/v1/runs") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: StartRunBody = match parse_json_body(request) {
                Ok(body) => body,
                Err(_) => {
                    return respond(
                        stream,
                        "400 Bad Request",
                        "application/json",
                        r#"{"error":"invalid_json"}"#,
                    );
                }
            };
            let (project_id, options) = project_request(shared, request)?;
            start_run(stream, Arc::clone(shared), project_id, options, body.filter)
        }
        ("POST", "/api/v1/runs/rerun") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: RerunBody = match parse_json_body(request) {
                Ok(body) => body,
                Err(_) => {
                    return respond(
                        stream,
                        "400 Bad Request",
                        "application/json",
                        r#"{"error":"invalid_json"}"#,
                    );
                }
            };
            let (project_id, options) = project_request(shared, request)?;
            start_run(
                stream,
                Arc::clone(shared),
                project_id,
                options,
                Some(body.test),
            )
        }
        ("POST", "/api/v1/benchmarks") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: StartBenchmarkBody = match parse_json_body(request) {
                Ok(body) => body,
                Err(_) => {
                    return respond(
                        stream,
                        "400 Bad Request",
                        "application/json",
                        r#"{"error":"invalid_json"}"#,
                    );
                }
            };
            let (project_id, options) = project_request(shared, request)?;
            start_benchmark_run(stream, Arc::clone(shared), project_id, options, body.save)
        }
        ("POST", "/api/v1/predictions") | ("POST", "/api/v1/reflections") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: LearnerNoteBody = match parse_json_body(request) {
                Ok(body) => body,
                Err(_) => {
                    return respond(
                        stream,
                        "400 Bad Request",
                        "application/json",
                        r#"{"error":"invalid_json"}"#,
                    );
                }
            };
            let (_, options) = project_request(shared, request)?;
            let recorded = if path == "/api/v1/predictions" {
                application::record_prediction(&options, body.text, body.skipped)
            } else {
                application::record_reflection(&options, body.text, body.skipped)
            };
            match recorded {
                Ok(state) => respond(
                    stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&state)?,
                ),
                Err(error) => respond(
                    stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("GET", "/api/v1/snapshots/preview") => {
            let (_, options) = project_request(shared, request)?;
            let preview = application::preview_stage_snapshot(&options)?;
            respond(
                stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&preview)?,
            )
        }
        ("POST", "/api/v1/snapshots") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            if parse_json_body::<EmptyBody>(request).is_err() {
                return respond(
                    stream,
                    "400 Bad Request",
                    "application/json",
                    r#"{"error":"invalid_json"}"#,
                );
            }
            let (_, options) = project_request(shared, request)?;
            match application::create_stage_snapshot(&options, false) {
                Ok(outcome) => respond(
                    stream,
                    "201 Created",
                    "application/json",
                    &serde_json::to_string(&outcome)?,
                ),
                Err(error) => respond(
                    stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/reports") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: ExportReportBody = match parse_json_body(request) {
                Ok(body) => body,
                Err(_) => {
                    return respond(
                        stream,
                        "400 Bad Request",
                        "application/json",
                        r#"{"error":"invalid_json"}"#,
                    );
                }
            };
            let (_, options) = project_request(shared, request)?;
            match application::export_report(&options, body.format.into()) {
                Ok(exported) => respond(
                    stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&exported)?,
                ),
                Err(error) => respond(
                    stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/runs/cancel") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let (_, options) = project_request(shared, request)?;
            cancel_run(stream, &options)
        }
        ("POST", "/api/v1/hints") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let (_, options) = project_request(shared, request)?;
            match application::reveal_next_hint(&options) {
                Ok(content) => respond(
                    stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&content)?,
                ),
                Err(error) => respond(
                    stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/capabilities/next") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let (_, options) = project_request(shared, request)?;
            match application::begin_next_capability(&options) {
                Ok(state) => respond(
                    stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&state)?,
                ),
                Err(error) => respond(
                    stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/project/repin-pack") => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            if parse_json_body::<EmptyBody>(request).is_err() {
                return respond(
                    stream,
                    "400 Bad Request",
                    "application/json",
                    r#"{"error":"invalid_json"}"#,
                );
            }
            let (_, options) = project_request(shared, request)?;
            match application::repin_current_pack(&options) {
                Ok(health) => respond(
                    stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&health)?,
                ),
                Err(error) => respond(
                    stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/project/open-editor") => {
            let (_, options) = project_request(shared, request)?;
            open_project(stream, shared, request, &options, ProjectOpenKind::Editor)
        }
        ("POST", "/api/v1/project/open-folder") => {
            let (_, options) = project_request(shared, request)?;
            open_project(stream, shared, request, &options, ProjectOpenKind::Folder)
        }
        ("POST", "/api/v1/service/shutdown") => shutdown_service(stream, shared, request),
        // A handler that panics on purpose, so panic recovery is tested
        // rather than asserted about. It exists only when
        // `DELTAFORGE_PANIC_PROBE` is set in the service's environment, and
        // is authenticated like every other mutation — a released binary
        // started normally has no such route.
        ("POST", "/api/v1/__panic") if shared.panic_probe => {
            if !authorized_mutation(request, shared) {
                return respond(
                    stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            // Panicking while holding a shared lock is the shape that used to
            // be terminal: the mutex stayed poisoned and every later request
            // panicked on it.
            let _held = crate::sync::lock(&shared.run_starting);
            panic!("deliberate panic from the DeltaForge panic probe");
        }
        ("POST", _) | ("GET", _) => respond(
            stream,
            "404 Not Found",
            "application/json",
            r#"{"error":"not_found"}"#,
        ),
        _ => respond(
            stream,
            "404 Not Found",
            "application/json",
            r#"{"error":"not_found"}"#,
        ),
    }
}

/// Answer a request the router could not complete. The message is the same
/// actionable text the CLI prints for the same failure, so a learner who hits
/// one in the browser is told what to fix rather than left with a dead page.
fn respond_internal_error(stream: &mut TcpStream, error: &anyhow::Error) -> Result<()> {
    let body = serde_json::json!({"error": format!("{error:#}")}).to_string();
    respond(
        stream,
        "500 Internal Server Error",
        "application/json",
        &body,
    )
}

/// The project pages the browser can be pointed at directly. Every route the
/// page's own router understands must be here too, or a reload or a bookmark
/// on that page returns 404 while in-app navigation works.
const PROJECT_PAGES: [&str; 4] = ["overview", "build", "performance", "runs"];

/// Whether a path is one the workbench serves as a page rather than as data.
fn is_page_route(path: &str) -> bool {
    matches!(path, "/projects" | "/catalog" | "/create") || project_page_route(path)
}

/// Shown when someone opens a workbench page without a capability token — a
/// reload, a bookmark, or a link opened in a new tab. The token is deliberately
/// absent from the address bar and from every link the page renders (see
/// `docs/safety.md`), so this is a normal thing to land on rather than an
/// attack, and it should read like a signpost.
const UNAUTHORIZED_PAGE: &str = concat!(
    "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">",
    "<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">",
    "<title>DeltaForge</title><style>",
    ":root{color-scheme:light dark}",
    "body{font-family:ui-sans-serif,system-ui,-apple-system,\"Segoe UI\",sans-serif;",
    "line-height:1.6;max-width:34rem;margin:0 auto;padding:4rem 1.5rem}",
    "h1{font-size:1.4rem;margin:0 0 .6rem}p{margin:0 0 1rem}",
    "code{background:rgba(127,127,127,.16);padding:.15rem .4rem;border-radius:4px}",
    "</style></head><body>",
    "<h1>This tab is not connected to DeltaForge</h1>",
    "<p>The workbench link carries a one-time key that is removed from the address ",
    "bar once the page has loaded, so it cannot be reloaded, bookmarked, or opened ",
    "in a new tab.</p>",
    "<p>Run <code>deltaforge</code> in your terminal to open a connected tab.</p>",
    "</body></html>",
);

fn project_page_route(route: &str) -> bool {
    let mut parts = route.strip_prefix("/projects/").unwrap_or("").split('/');
    let (Some(project), Some(page), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    !project.is_empty() && PROJECT_PAGES.contains(&page)
}

fn read_request(stream: &mut TcpStream) -> Result<Option<HttpRequest>> {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    let header_end = loop {
        if let Some(index) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
            let end = index + 4;
            if end > MAX_HEADER_BYTES {
                bail!("request headers exceed the workbench limit");
            }
            break end;
        }
        if bytes.len() >= MAX_HEADER_BYTES {
            bail!("request headers exceed the workbench limit");
        }
        match stream.read(&mut chunk) {
            Ok(0) if bytes.is_empty() => return Ok(None),
            Ok(0) => bail!("request headers ended before the header terminator"),
            Ok(read) => bytes.extend_from_slice(&chunk[..read]),
            Err(error) if bytes.is_empty() => return Err(error.into()),
            Err(_) => return Ok(None),
        }
    };
    let text =
        std::str::from_utf8(&bytes[..header_end]).context("request headers are not UTF-8")?;
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
    if headers.contains_key("transfer-encoding") {
        bail!("transfer-encoded request bodies are not accepted");
    }
    let content_length = headers
        .get("content-length")
        .map(|value| value.parse::<usize>().context("invalid Content-Length"))
        .transpose()?
        .unwrap_or_default();
    if content_length > MAX_BODY_BYTES {
        bail!("request body exceeds the workbench limit");
    }
    while bytes.len() < header_end + content_length {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            bail!("request body ended before Content-Length");
        }
        bytes.extend_from_slice(&chunk[..read]);
    }
    Ok(Some(HttpRequest {
        method,
        target,
        headers,
        body: bytes[header_end..header_end + content_length].to_vec(),
    }))
}

fn authorized(request: &HttpRequest, shared: &Shared) -> bool {
    let expected_host = format!("127.0.0.1:{}", shared.port);
    if request.headers.get("host") != Some(&expected_host) {
        return false;
    }
    let query_token = request
        .target
        .split_once('?')
        .and_then(|(_, query)| {
            query
                .split('&')
                .find_map(|pair| pair.strip_prefix("token="))
        })
        .unwrap_or_default();
    // The page removes the token from its own URL (and so from the address
    // bar and browser history) after its first load, keeping it only in
    // memory and sending it in this header for every request it makes
    // itself from then on. The query string stays accepted because it is
    // the only way to authorize that first page load, and because
    // `EventSource` (used for the live event stream) cannot set a custom
    // header at all. Both checks always run, so which one matched is not
    // observable from timing.
    let header_token = request
        .headers
        .get(CAPABILITY_HEADER)
        .map(String::as_str)
        .unwrap_or_default();
    let query_ok = constant_time_eq(query_token.as_bytes(), shared.token.as_bytes());
    let header_ok = constant_time_eq(header_token.as_bytes(), shared.token.as_bytes());
    if !(query_ok || header_ok) {
        return false;
    }
    let expected_origin = format!("http://{expected_host}");
    request
        .headers
        .get("origin")
        .is_none_or(|origin| origin == &expected_origin)
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let mut difference = left.len() ^ right.len();
    for index in 0..right.len() {
        let left = left.get(index).copied().unwrap_or_default();
        let right = right.get(index).copied().unwrap_or_default();
        difference |= usize::from(left ^ right);
    }
    difference == 0
}

fn authorized_mutation(request: &HttpRequest, shared: &Shared) -> bool {
    let expected_origin = format!("http://127.0.0.1:{}", shared.port);
    authorized(request, shared)
        && request.headers.get("origin") == Some(&expected_origin)
        && request
            .headers
            .get("content-type")
            .is_some_and(|value| value.eq_ignore_ascii_case("application/json"))
}

fn parse_json_body<T>(request: &HttpRequest) -> Result<T>
where
    T: for<'de> Deserialize<'de> + Default,
{
    if request.body.is_empty() {
        return Ok(T::default());
    }
    serde_json::from_slice(&request.body).context("invalid JSON request body")
}

fn start_run(
    stream: &mut TcpStream,
    shared: Arc<Shared>,
    project_id: String,
    options: GlobalOptions,
    filter: Option<String>,
) -> Result<()> {
    if filter
        .as_ref()
        .is_some_and(|value| value.trim().is_empty() || value.len() > 200)
    {
        return respond(
            stream,
            "400 Bad Request",
            "application/json",
            r#"{"error":"invalid_test_filter"}"#,
        );
    }
    let mut starting = crate::sync::lock(&shared.run_starting);
    if shared.shutting_down.load(Ordering::SeqCst)
        || starting.contains(&project_id)
        || application::run_is_active(&options)?
    {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"run_already_active"}"#,
        );
    }
    starting.insert(project_id.clone());
    drop(starting);

    let worker = Arc::clone(&shared);
    std::thread::spawn(move || {
        let request = application::TestRunRequest {
            stage: None,
            all: false,
            filter,
            list_tests: false,
            fail_fast: false,
            no_build: false,
            keep_temp: false,
            trigger: application::RunTrigger::Workbench,
        };
        let mut sink = application::NullEventSink;
        if let Err(error) = application::run_tests(&options, request, &mut sink)
            && !format!("{error:#}").contains("already active")
        {
            let _ = application::publish_event(
                &options,
                &application::RunEvent::JobInterrupted {
                    job_id: "pending".to_string(),
                    reason: format!("{error:#}"),
                },
            );
        }
        crate::sync::lock(&worker.run_starting).remove(&project_id);
        *crate::sync::lock(&worker.last_activity) = Instant::now();
    });
    respond(
        stream,
        "202 Accepted",
        "application/json",
        r#"{"status":"accepted"}"#,
    )
}

/// Serve both halves of the creation flow. `preflight` reports what is wrong
/// without changing anything; `create` writes the project and registers it.
/// Both resolve the target through the same guarded function, so the browser's
/// earlier preflight is a convenience and never the authority.
fn create_project(
    stream: &mut TcpStream,
    shared: &Shared,
    path: &str,
    body: CreateProjectBody,
) -> Result<()> {
    // Creation is the only browser operation that names a filesystem location.
    // Refuse an oversized field before it reaches path resolution.
    if body.pack.len() > 128
        || body.language.len() > 64
        || body.name.len() > 128
        || body
            .parent_directory
            .as_ref()
            .is_some_and(|parent| parent.len() > 4096)
    {
        return respond(
            stream,
            "400 Bad Request",
            "application/json",
            r#"{"error":"invalid_request"}"#,
        );
    }
    let parent = body.parent_directory.as_deref().map(Path::new);
    let options = GlobalOptions::default();

    if path.ends_with("/preflight") {
        return match application::preflight_project(
            &options,
            &body.pack,
            &body.language,
            parent,
            &body.name,
        ) {
            Ok(preflight) => respond(
                stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&preflight)?,
            ),
            Err(error) => respond(
                stream,
                "400 Bad Request",
                "application/json",
                &serde_json::json!({"error": format!("{error:#}")}).to_string(),
            ),
        };
    }

    if shared.shutting_down.load(Ordering::SeqCst) {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"service_stopping"}"#,
        );
    }
    let request = application::CreateProjectRequest {
        pack: body.pack,
        language: body.language,
        parent_directory: parent.map(Path::to_path_buf),
        name: body.name,
        git: body.git,
        stage: None,
    };
    match application::create_project(&options, request) {
        Ok(created) => respond(
            stream,
            "201 Created",
            "application/json",
            &serde_json::to_string(&created)?,
        ),
        Err(error) => respond(
            stream,
            "409 Conflict",
            "application/json",
            &serde_json::json!({"error": format!("{error:#}")}).to_string(),
        ),
    }
}

/// Start a benchmark job on the same guard rails as a test run: one job per
/// project at a time, run on a worker thread, progress delivered through the
/// project's event stream rather than this response.
fn start_benchmark_run(
    stream: &mut TcpStream,
    shared: Arc<Shared>,
    project_id: String,
    options: GlobalOptions,
    save: bool,
) -> Result<()> {
    let mut starting = crate::sync::lock(&shared.run_starting);
    if shared.shutting_down.load(Ordering::SeqCst)
        || starting.contains(&project_id)
        || application::run_is_active(&options)?
    {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"run_already_active"}"#,
        );
    }
    starting.insert(project_id.clone());
    drop(starting);

    let worker = Arc::clone(&shared);
    std::thread::spawn(move || {
        let request = application::BenchmarkRunRequest {
            stage: None,
            all: false,
            iterations: None,
            warmup: None,
            save,
            compare: true,
            trigger: application::RunTrigger::Workbench,
        };
        let mut sink = application::NullEventSink;
        if let Err(error) = application::run_benchmarks(&options, request, &mut sink)
            && !format!("{error:#}").contains("already active")
        {
            let _ = application::publish_event(
                &options,
                &application::RunEvent::JobInterrupted {
                    job_id: "pending".to_string(),
                    reason: format!("{error:#}"),
                },
            );
        }
        crate::sync::lock(&worker.run_starting).remove(&project_id);
        *crate::sync::lock(&worker.last_activity) = Instant::now();
    });
    respond(
        stream,
        "202 Accepted",
        "application/json",
        r#"{"status":"accepted"}"#,
    )
}

fn cancel_run(stream: &mut TcpStream, options: &GlobalOptions) -> Result<()> {
    match application::cancel_active_run(options) {
        Ok(job_id) => respond(
            stream,
            "202 Accepted",
            "application/json",
            &serde_json::json!({"status": "cancelling", "job_id": job_id}).to_string(),
        ),
        Err(_) => respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"no_active_run"}"#,
        ),
    }
}

fn shutdown_service(stream: &mut TcpStream, shared: &Shared, request: &HttpRequest) -> Result<()> {
    if !authorized_mutation(request, shared) {
        return respond(
            stream,
            "403 Forbidden",
            "application/json",
            r#"{"error":"forbidden"}"#,
        );
    }
    if parse_json_body::<EmptyBody>(request).is_err() {
        return respond(
            stream,
            "400 Bad Request",
            "application/json",
            r#"{"error":"invalid_json"}"#,
        );
    }
    let run_starting = crate::sync::lock(&shared.run_starting);
    let run_active = !run_starting.is_empty()
        || crate::project_registry::list()
            .unwrap_or_default()
            .into_iter()
            .any(|project| {
                application::run_is_active(&options_for_entry(&project)).unwrap_or(false)
            });
    if run_active {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"run_active"}"#,
        );
    }
    shared.shutting_down.store(true, Ordering::SeqCst);
    drop(run_starting);

    let _ = fs::remove_file(&shared.record_path);
    let response = respond(
        stream,
        "202 Accepted",
        "application/json",
        r#"{"status":"stopping"}"#,
    );
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(50));
        std::process::exit(0);
    });
    response
}

fn open_project(
    stream: &mut TcpStream,
    shared: &Shared,
    request: &HttpRequest,
    options: &GlobalOptions,
    kind: ProjectOpenKind,
) -> Result<()> {
    if !authorized_mutation(request, shared) {
        return respond(
            stream,
            "403 Forbidden",
            "application/json",
            r#"{"error":"forbidden"}"#,
        );
    }
    if parse_json_body::<EmptyBody>(request).is_err() {
        return respond(
            stream,
            "400 Bad Request",
            "application/json",
            r#"{"error":"invalid_json"}"#,
        );
    }
    let target = match application::project_open_target(options) {
        Ok(target) => target,
        Err(error) => {
            return respond(
                stream,
                "409 Conflict",
                "application/json",
                &serde_json::json!({"error": format!("could not locate project: {error}")})
                    .to_string(),
            );
        }
    };
    let editor = std::env::var("VISUAL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| std::env::var("EDITOR").ok());
    let resolved = match project_open_command(kind, &target, editor.as_deref()) {
        Ok(resolved) => resolved,
        Err(error) => {
            return respond(
                stream,
                "409 Conflict",
                "application/json",
                &serde_json::json!({"error": format!("could not open project: {error}")})
                    .to_string(),
            );
        }
    };
    match launch_project_command(resolved.command, &resolved.application) {
        Ok(()) => respond(
            stream,
            "202 Accepted",
            "application/json",
            &serde_json::json!({
                "status": "opened",
                "application": resolved.application,
            })
            .to_string(),
        ),
        Err(error) => respond(
            stream,
            "409 Conflict",
            "application/json",
            &serde_json::json!({"error": format!("could not open project: {error}")}).to_string(),
        ),
    }
}

fn project_open_command(
    kind: ProjectOpenKind,
    target: &Path,
    editor: Option<&str>,
) -> Result<ProjectOpenCommand> {
    if matches!(kind, ProjectOpenKind::Editor)
        && let Some(editor) = editor
    {
        let mut parts = editor.split_whitespace();
        let program = parts
            .next()
            .filter(|part| !part.is_empty())
            .context("the configured VISUAL or EDITOR command is empty")?;
        if !terminal_only_editor(program) {
            let mut command = Command::new(program);
            command.args(parts).arg(target);
            return Ok(ProjectOpenCommand {
                command,
                application: program.to_string(),
            });
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mut command = Command::new("open");
        if matches!(kind, ProjectOpenKind::Editor) {
            let application = [
                "Cursor",
                "Visual Studio Code",
                "Zed",
                "Sublime Text",
                "Nova",
                "RustRover",
            ]
            .into_iter()
            .find(|application| macos_application_exists(application))
            .context(
                "no supported graphical editor was found; set VISUAL or EDITOR to a GUI editor command such as 'cursor', 'code', or 'zed'",
            )?;
            command.arg("-a").arg(application).arg(target);
            return Ok(ProjectOpenCommand {
                command,
                application: application.to_string(),
            });
        }
        command.arg(target);
        Ok(ProjectOpenCommand {
            command,
            application: "Finder".to_string(),
        })
    }
    #[cfg(target_os = "linux")]
    {
        let (program, application) = if matches!(kind, ProjectOpenKind::Editor) {
            [
                ("cursor", "Cursor"),
                ("code", "Visual Studio Code"),
                ("zed", "Zed"),
                ("codium", "VSCodium"),
                ("subl", "Sublime Text"),
            ]
            .into_iter()
            .find(|(program, _)| command_exists(program))
            .context(
                "no supported graphical editor was found; set VISUAL or EDITOR to a GUI editor command",
            )?
        } else {
            ("xdg-open", "file manager")
        };
        let mut command = Command::new(program);
        command.arg(target);
        Ok(ProjectOpenCommand {
            command,
            application: application.to_string(),
        })
    }
    #[cfg(windows)]
    {
        let (program, application) = if matches!(kind, ProjectOpenKind::Editor) {
            [
                ("cursor.cmd", "Cursor"),
                ("code.cmd", "Visual Studio Code"),
                ("zed.exe", "Zed"),
                ("codium.cmd", "VSCodium"),
                ("subl.exe", "Sublime Text"),
            ]
            .into_iter()
            .find(|(program, _)| command_exists(program))
            .context(
                "no supported graphical editor was found; set VISUAL or EDITOR to a GUI editor command",
            )?
        } else {
            ("explorer", "File Explorer")
        };
        let mut command = Command::new(program);
        command.arg(target);
        Ok(ProjectOpenCommand {
            command,
            application: application.to_string(),
        })
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        let _ = kind;
        let _ = target;
        bail!("opening a project is unsupported on this platform")
    }
}

fn terminal_only_editor(program: &str) -> bool {
    Path::new(program)
        .file_stem()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "vi" | "vim" | "nvim" | "nano" | "emacs"))
}

fn launch_project_command(mut command: Command, application: &str) -> Result<()> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut child = command
        .spawn()
        .with_context(|| format!("failed to start {application}"))?;
    let deadline = Instant::now() + Duration::from_millis(750);
    loop {
        match child.try_wait()? {
            Some(status) if status.success() => return Ok(()),
            Some(status) => bail!("{application} exited with status {status}"),
            None if Instant::now() >= deadline => {
                std::thread::spawn(move || {
                    let _ = child.wait();
                });
                return Ok(());
            }
            None => std::thread::sleep(Duration::from_millis(20)),
        }
    }
}

#[cfg(target_os = "macos")]
fn macos_application_exists(application: &str) -> bool {
    Command::new("open")
        .arg("-Ra")
        .arg(application)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

#[cfg(any(target_os = "linux", windows))]
fn command_exists(program: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|paths| {
        std::env::split_paths(&paths).any(|directory| directory.join(program).is_file())
    })
}

/// Stream one project's run events until the tab closes.
///
/// Errors are reported inside the stream rather than returned, because the
/// response status is already on the wire by the time any of this work runs —
/// a 500 is no longer available, and returning would drop the connection with
/// nothing said. The page renders `stream_error` next to the run activity.
fn serve_events(
    stream: &mut TcpStream,
    shared: &Shared,
    request: &HttpRequest,
    project_id: String,
    options: GlobalOptions,
) -> Result<()> {
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
    {
        let mut watched = crate::sync::lock(&shared.watched_projects);
        let project = watched
            .entry(project_id.clone())
            .or_insert_with(|| WatchedProject {
                options: options.clone(),
                clients: 0,
            });
        project.clients += 1;
    }
    struct ProjectWatchGuard<'a> {
        shared: &'a Shared,
        project_id: String,
    }
    impl Drop for ProjectWatchGuard<'_> {
        fn drop(&mut self) {
            let mut watched = crate::sync::lock(&self.shared.watched_projects);
            if let Some(project) = watched.get_mut(&self.project_id) {
                project.clients = project.clients.saturating_sub(1);
                if project.clients == 0 {
                    watched.remove(&self.project_id);
                }
            }
        }
    }
    let _watch_guard = ProjectWatchGuard {
        shared,
        project_id: project_id.clone(),
    };
    if let Err(error) = stream_project_events(stream, shared, request, &project_id, &options) {
        let payload = format!(
            "event: stream_error\ndata: {}\n\n",
            serde_json::json!({"error": format!("{error:#}")})
        );
        let _ = stream.write_all(payload.as_bytes());
    }
    Ok(())
}

fn stream_project_events(
    stream: &mut TcpStream,
    shared: &Shared,
    request: &HttpRequest,
    project_id: &str,
    options: &GlobalOptions,
) -> Result<()> {
    let mut cursor = query_value(&request.target, "after")
        .and_then(|value| value.parse::<u64>().ok())
        .or_else(|| {
            request
                .headers
                .get("last-event-id")
                .and_then(|value| value.parse::<u64>().ok())
        })
        .unwrap_or(crate::run_journal::cursor(project_root(options)?)?);
    let mut previous = String::new();
    let mut previous_state_stamp = None;
    let mut focus_revision = shared.focus_revision.load(Ordering::SeqCst);

    loop {
        let mut run_event_arrived = false;
        let current_focus_revision = shared.focus_revision.load(Ordering::SeqCst);
        if current_focus_revision != focus_revision {
            focus_revision = current_focus_revision;
            let target = crate::sync::lock(&shared.focus_target).clone();
            let payload = format!(
                "event: focus\ndata: {}\n\n",
                serde_json::json!({"route": target})
            );
            if stream.write_all(payload.as_bytes()).is_err() {
                return Ok(());
            }
        }
        for entry in crate::run_journal::entries_after(project_root(options)?, cursor)? {
            if entry.id != cursor + 1 {
                // The journal trimmed events between what this client last
                // saw and what survives now (it was disconnected too long, or
                // a very large run compacted past it). Say so explicitly
                // rather than silently jumping the sequence.
                let gap_payload = format!(
                    "event: gap\ndata: {}\n\n",
                    serde_json::json!({"after": cursor, "next": entry.id})
                );
                if stream.write_all(gap_payload.as_bytes()).is_err() {
                    return Ok(());
                }
            }
            let serialized = serde_json::to_string(&entry.event)?;
            let payload = format!("id: {}\nevent: run\ndata: {serialized}\n\n", entry.id);
            if stream.write_all(payload.as_bytes()).is_err() {
                return Ok(());
            }
            cursor = entry.id;
            run_event_arrived = true;
        }
        let state_stamp = state_file_stamp(project_root(options)?);
        let payload = if previous_state_stamp != state_stamp || run_event_arrived {
            previous_state_stamp = state_stamp;
            let state = application::load_workbench_state_for_session(
                options,
                &project_session_id(shared, project_id),
            )?;
            let serialized = serde_json::to_string(&state)?;
            if serialized != previous {
                previous = serialized.clone();
                format!("event: state\ndata: {serialized}\n\n")
            } else {
                ": keep-alive\n\n".to_string()
            }
        } else {
            ": keep-alive\n\n".to_string()
        };
        if stream.write_all(payload.as_bytes()).is_err() {
            return Ok(());
        }
        *crate::sync::lock(&shared.last_activity) = Instant::now();
        std::thread::sleep(EVENT_POLL_INTERVAL);
    }
}

fn state_file_stamp(root: &Path) -> Option<(u64, SystemTime)> {
    let metadata = fs::metadata(root.join(".deltaforge").join("state.json")).ok()?;
    Some((metadata.len(), metadata.modified().ok()?))
}

fn serve_app_events(stream: &mut TcpStream, shared: &Shared) -> Result<()> {
    stream.set_read_timeout(None)?;
    stream.write_all(
        concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Type: text/event-stream\r\n",
            "Cache-Control: no-cache\r\n",
            "X-Content-Type-Options: nosniff\r\n",
            "Referrer-Policy: no-referrer\r\n",
            "Connection: keep-alive\r\n\r\n"
        )
        .as_bytes(),
    )?;
    struct ClientGuard<'a>(&'a AtomicUsize);
    impl Drop for ClientGuard<'_> {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }
    shared.clients.fetch_add(1, Ordering::SeqCst);
    let _guard = ClientGuard(&shared.clients);
    let mut focus_revision = shared.focus_revision.load(Ordering::SeqCst);
    loop {
        let current = shared.focus_revision.load(Ordering::SeqCst);
        let payload = if current != focus_revision {
            focus_revision = current;
            let target = crate::sync::lock(&shared.focus_target).clone();
            format!(
                "event: focus\ndata: {}\n\n",
                serde_json::json!({"route": target})
            )
        } else {
            ": keep-alive\n\n".to_string()
        };
        if stream.write_all(payload.as_bytes()).is_err() {
            return Ok(());
        }
        *crate::sync::lock(&shared.last_activity) = Instant::now();
        std::thread::sleep(EVENT_POLL_INTERVAL);
    }
}

fn options_for_entry(project: &crate::project_registry::RegisteredProject) -> GlobalOptions {
    GlobalOptions {
        project_dir: Some(project.path.clone()),
        packs_dir: project.packs_dir.clone(),
    }
}

fn project_request(shared: &Shared, request: &HttpRequest) -> Result<(String, GlobalOptions)> {
    let id = query_value(&request.target, "project")
        .map(str::to_string)
        .or_else(|| shared.default_project_id.clone())
        .context("select a registered project first")?;
    let project = crate::project_registry::resolve(&id)?;
    Ok((id, options_for_entry(&project)))
}

fn project_session_id(shared: &Shared, project_id: &str) -> String {
    format!("{}-{project_id}", shared.session_id)
}

fn load_project_summaries() -> Result<Vec<ProjectSummary>> {
    crate::project_registry::list()?
        .into_iter()
        .map(|project| {
            let options = options_for_entry(&project);
            let health = application::load_project_health(&options)?;
            let state_path = project.path.join(".deltaforge").join("state.json");
            let state = crate::state::ProjectState::read_from(&state_path).ok();
            let content = application::load_capability_content(&options).ok();
            let current_position = content
                .as_ref()
                .and_then(|content| content.roadmap.iter().find(|step| step.current))
                .map_or(0, |step| step.position);
            let healthy = matches!(health.status, application::ProjectHealthStatus::Healthy);
            Ok(ProjectSummary {
                id: project.id,
                name: content
                    .as_ref()
                    .map(|content| content.project_overview.name.clone())
                    .or_else(|| state.as_ref().map(|state| state.project.clone()))
                    .unwrap_or_else(|| "Unavailable project".to_string()),
                description: content
                    .as_ref()
                    .map(|content| content.project_overview.description.clone())
                    .unwrap_or_else(|| {
                        "This project needs attention before it can continue.".to_string()
                    }),
                language: state
                    .as_ref()
                    .map(|state| state.language.clone())
                    .unwrap_or_default(),
                path: crate::fs_util::display_path(&project.path),
                current_step: content
                    .as_ref()
                    .map(|content| content.title.clone())
                    .unwrap_or_else(|| "Unavailable".to_string()),
                current_step_number: current_position,
                total_steps: content.as_ref().map_or(0, |content| content.roadmap.len()),
                completed_steps: state
                    .as_ref()
                    .map_or(0, |state| state.completed_stages.len()),
                last_opened_at: project.last_opened_at,
                status: if healthy {
                    "healthy"
                } else {
                    "needs_attention"
                },
                issue: health.issue.map(|issue| issue.title),
            })
        })
        .collect()
}

fn project_root(options: &GlobalOptions) -> Result<&Path> {
    options
        .project_dir
        .as_deref()
        .context("workbench project root is missing")
}

fn query_value<'a>(target: &'a str, name: &str) -> Option<&'a str> {
    target.split_once('?').and_then(|(_, query)| {
        query.split('&').find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            (key == name).then_some(value)
        })
    })
}

fn respond(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) -> Result<()> {
    let content_security_policy = content_security_policy(content_type, body);
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nContent-Security-Policy: {content_security_policy}\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nX-Frame-Options: DENY\r\nConnection: close\r\n\r\n{body}",
        body.len(),
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

/// A hash-based CSP for the compile-time assets embedded in the page.
///
/// The capability token makes the assembled HTML different for each service,
/// so the script hashes are derived from the response body rather than copied
/// into a constant that would silently go stale. Inline style attributes are
/// separately restricted with CSP 3's `unsafe-hashes`: this allows only the
/// exact declarations already present in the shell or emitted by the client,
/// without granting arbitrary inline CSS.
fn content_security_policy(content_type: &str, body: &str) -> String {
    if !content_type.starts_with("text/html") {
        return "default-src 'none'; frame-ancestors 'none'; base-uri 'none'".to_string();
    }

    let script_hashes = inline_tag_contents(body, "script")
        .into_iter()
        .map(csp_hash)
        .collect::<Vec<_>>()
        .join(" ");
    let style_hashes = inline_tag_contents(body, "style")
        .into_iter()
        .map(csp_hash)
        .collect::<Vec<_>>()
        .join(" ");

    let mut attribute_styles = html_attribute_values(body, "style");
    // `app.js` creates these declarations at runtime. Width is an integer
    // percentage (completed steps / total steps, rounded), so enumerate the
    // finite set rather than allow arbitrary inline styles.
    attribute_styles.extend([
        "color:var(--text-2);font-size:.88rem;margin-top:4px".to_string(),
        "color:var(--text-3);font-size:.78rem".to_string(),
        "margin-top:4px".to_string(),
        "margin-top:10px".to_string(),
        "text-align:left;color:var(--contradiction)".to_string(),
        "color:var(--contradiction);margin-top:4px".to_string(),
        "text-align: center;".to_string(),
        "text-align: right;".to_string(),
    ]);
    for percent in 0..=100 {
        attribute_styles.push(format!("width: {percent}%;"));
    }
    attribute_styles.sort();
    attribute_styles.dedup();
    let attribute_hashes = attribute_styles
        .iter()
        .map(|style| csp_hash(style))
        .collect::<Vec<_>>()
        .join(" ");

    format!(
        "default-src 'self'; script-src 'self' {script_hashes}; script-src-attr 'none'; style-src 'self' {style_hashes}; style-src-attr 'unsafe-hashes' {attribute_hashes}; connect-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'none'; frame-ancestors 'none'"
    )
}

fn csp_hash(source: &str) -> String {
    let digest = Sha256::digest(source.as_bytes());
    format!(
        "'sha256-{}'",
        base64::engine::general_purpose::STANDARD.encode(digest)
    )
}

fn inline_tag_contents<'a>(html: &'a str, tag: &str) -> Vec<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut rest = html;
    let mut contents = Vec::new();
    while let Some((_, after_open)) = rest.split_once(&open) {
        let Some((content, after_close)) = after_open.split_once(&close) else {
            break;
        };
        contents.push(content);
        rest = after_close;
    }
    contents
}

fn html_attribute_values(html: &str, attribute: &str) -> Vec<String> {
    let marker = format!(r#"{attribute}=""#);
    let mut rest = html;
    let mut values = Vec::new();
    while let Some((_, after_marker)) = rest.split_once(&marker) {
        let Some((value, after_quote)) = after_marker.split_once('"') else {
            break;
        };
        values.push(value.to_string());
        rest = after_quote;
    }
    values
}

fn http_get(port: u16, path: &str) -> Option<String> {
    let response = http_get_response(port, path)?;
    if !response.starts_with("HTTP/1.1 200") {
        return None;
    }
    response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body.to_string())
}

fn http_get_response(port: u16, path: &str) -> Option<String> {
    let request =
        format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n");
    http_exchange(port, &request)
}

fn http_exchange(port: u16, request: &str) -> Option<String> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
    let mut stream = TcpStream::connect_timeout(&address, PROBE_TIMEOUT).ok()?;
    stream.set_read_timeout(Some(PROBE_TIMEOUT)).ok()?;
    stream.set_write_timeout(Some(PROBE_TIMEOUT)).ok()?;
    stream.write_all(request.as_bytes()).ok()?;
    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;
    Some(response)
}

fn capability_token() -> Result<String> {
    random_hex_id(32)
}

fn random_hex_id(bytes_len: usize) -> Result<String> {
    let mut bytes = vec![0_u8; bytes_len];
    getrandom::fill(&mut bytes)
        .map_err(|error| anyhow::anyhow!("operating system randomness is unavailable: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}
/// The single-page workbench, assembled at compile time from four source
/// files. It is served inline rather than as separate assets so the page
/// carries no subresource requests and the capability token stays in exactly
/// one place.
///
/// `ui/core.js` is loaded first and holds the page's pure decisions, so
/// `node --test tests/ui` can execute them without a browser.
fn workbench_html(token: &str) -> String {
    include_str!("ui/index.html")
        .replace("__STYLE__", include_str!("ui/app.css"))
        .replace("__CORE__", include_str!("ui/core.js"))
        .replace("__SCRIPT__", include_str!("ui/app.js"))
        .replace("__TOKEN_JSON__", &serde_json::json!(token).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_timeout_is_short_but_rejects_invalid_values() {
        assert_eq!(parse_test_idle_timeout(None).unwrap(), None);
        assert_eq!(
            parse_test_idle_timeout(Some("10000".into())).unwrap(),
            Some(Duration::from_secs(10))
        );
        assert!(parse_test_idle_timeout(Some("0".into())).is_err());
        assert!(parse_test_idle_timeout(Some("not-a-number".into())).is_err());
    }

    fn test_shared(port: u16) -> Shared {
        Shared {
            default_project_id: None,
            token: "secret-token".to_string(),
            probe_id: "test-probe-id".to_string(),
            session_id: "test-session".to_string(),
            port,
            clients: AtomicUsize::new(0),
            last_activity: Mutex::new(Instant::now()),
            record_path: PathBuf::from("unused-workbench-record.json"),
            run_starting: Mutex::new(BTreeSet::new()),
            shutting_down: AtomicBool::new(false),
            idle_timeout: IDLE_TIMEOUT,
            focus_revision: AtomicUsize::new(0),
            focus_target: Mutex::new("/projects".to_string()),
            watched_projects: Mutex::new(BTreeMap::new()),
            pre_auth: AtomicUsize::new(0),
            panic_probe: false,
        }
    }

    /// Accepting a connection costs a thread and a socket before the peer has
    /// proved anything, so the unauthenticated window has to be bounded — and
    /// authenticating has to hand the slot straight back, or a few open tabs
    /// holding SSE streams would exhaust the bound and lock everyone out.
    #[test]
    fn pre_authentication_connections_are_bounded_and_freed_by_authenticating() {
        let shared = Arc::new(test_shared(0));

        let mut slots = (0..MAX_PRE_AUTH_CONNECTIONS)
            .map(|index| {
                PreAuthSlot::reserve(&shared)
                    .unwrap_or_else(|| panic!("slot {index} within the cap must be available"))
            })
            .collect::<Vec<_>>();
        assert!(
            PreAuthSlot::reserve(&shared).is_none(),
            "a connection over the cap must be shed, not accepted"
        );

        // Authenticating returns the slot, so a long-lived authorized stream
        // never occupies the unauthenticated bound.
        slots[0].release();
        let recovered = PreAuthSlot::reserve(&shared).expect("released slot must be reusable");
        assert!(PreAuthSlot::reserve(&shared).is_none());

        // Releasing twice must not double-count the slot back.
        slots[1].release();
        slots[1].release();
        let _one = PreAuthSlot::reserve(&shared).expect("slot freed once must be reusable");
        assert!(
            PreAuthSlot::reserve(&shared).is_none(),
            "a second release of the same slot must not manufacture capacity"
        );

        // Dropping a connection's guard frees its slot too, so a peer that
        // stalls and times out cannot leak the bound away.
        drop(recovered);
        drop(slots);
        assert!(PreAuthSlot::reserve(&shared).is_some());
    }

    #[test]
    fn capability_tokens_are_random_and_fixed_length() {
        let first = capability_token().unwrap();
        let second = capability_token().unwrap();
        assert_eq!(first.len(), 64);
        assert_eq!(second.len(), 64);
        assert_ne!(first, second);
        assert!(service_token(Some(String::new())).is_err());
    }

    fn raw_request(target: &str, host: &str, origin: Option<&str>) -> String {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let shared = Arc::new(test_shared(port));
        let shared_for_server = Arc::clone(&shared);
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_connection(
                stream,
                &shared_for_server,
                PreAuthSlot::reserve(&shared_for_server).unwrap(),
            )
            .unwrap();
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

    /// A person landing on a page route without a token — a reload, a
    /// bookmark, a link opened in a new tab — gets a readable signpost. The
    /// token is deliberately absent from the address bar and from every link
    /// the page renders, so this is a normal landing, not an attack, and raw
    /// JSON was a dead end. Data routes still answer with JSON.
    #[test]
    fn an_unauthenticated_page_route_explains_itself() {
        let page = raw_request("/projects", "127.0.0.1:0", None);
        assert!(page.starts_with("HTTP/1.1 403"), "{page}");
        assert!(page.contains("text/html"), "{page}");
        assert!(page.contains("Run <code>deltaforge</code>"), "{page}");

        let data = raw_request("/api/v1/state", "127.0.0.1:0", None);
        assert!(data.starts_with("HTTP/1.1 403"), "{data}");
        assert!(data.contains(r#"{"error":"forbidden"}"#), "{data}");

        assert!(is_page_route("/projects"));
        assert!(is_page_route("/catalog"));
        assert!(is_page_route("/projects/some-id/build"));
        assert!(!is_page_route("/api/v1/state"));
        assert!(!is_page_route("/projects/some-id/unknown"));
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
            body: Vec::new(),
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
    fn authorized_accepts_the_capability_header_in_place_of_the_query_string() {
        // Every request the page makes after its first load sends the token
        // as a header instead of a query parameter, so the address bar and
        // browser history never carry it (see docs/safety.md).
        let port = 43124;
        let shared = test_shared(port);
        let request_with = |target: &str, header: Option<&str>| HttpRequest {
            method: "GET".to_string(),
            target: target.to_string(),
            headers: [
                ("host".to_string(), format!("127.0.0.1:{port}")),
                ("origin".to_string(), format!("http://127.0.0.1:{port}")),
            ]
            .into_iter()
            .chain(header.map(|value| (CAPABILITY_HEADER.to_string(), value.to_string())))
            .collect(),
            body: Vec::new(),
        };

        assert!(authorized(
            &request_with("/", Some("secret-token")),
            &shared
        ));
        assert!(!authorized(
            &request_with("/", Some("wrong-token")),
            &shared
        ));
        assert!(!authorized(&request_with("/", None), &shared));
        // Either credential alone is enough: the query string still works
        // (for the initial page load and `EventSource`) even once a header
        // is also accepted.
        assert!(authorized(
            &request_with("/?token=secret-token", None),
            &shared
        ));
    }

    #[test]
    fn mutations_require_exact_origin_and_json_content_type() {
        let port = 43123;
        let shared = test_shared(port);
        let request = |origin: Option<&str>, content_type: Option<&str>| HttpRequest {
            method: "POST".to_string(),
            target: "/api/v1/runs?token=secret-token".to_string(),
            headers: [
                ("host", Some("127.0.0.1:43123")),
                ("origin", origin),
                ("content-type", content_type),
            ]
            .into_iter()
            .filter_map(|(name, value)| value.map(|value| (name.to_string(), value.to_string())))
            .collect(),
            body: b"{}".to_vec(),
        };
        assert!(!authorized_mutation(
            &request(None, Some("application/json")),
            &shared
        ));
        assert!(!authorized_mutation(
            &request(Some("https://attacker.example"), Some("application/json")),
            &shared
        ));
        assert!(!authorized_mutation(
            &request(Some("http://127.0.0.1:43123"), Some("text/plain")),
            &shared
        ));
        assert!(authorized_mutation(
            &request(Some("http://127.0.0.1:43123"), Some("application/json")),
            &shared
        ));
    }

    #[test]
    fn request_reader_collects_a_bounded_json_body() {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            read_request(&mut stream).unwrap().unwrap()
        });
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        stream
            .write_all(
                b"POST /api/v1/runs?token=x HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: 16\r\n\r\n{\"filter\":\"one\"}",
            )
            .unwrap();
        let request = server.join().unwrap();
        assert_eq!(request.method, "POST");
        assert_eq!(request.body, br#"{"filter":"one"}"#);
    }

    #[test]
    fn normal_requests_bound_reads_and_writes() {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            configure_request_stream(&stream).unwrap();
            (
                stream.read_timeout().unwrap(),
                stream.write_timeout().unwrap(),
            )
        });
        let _client = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        let (read, write) = server.join().unwrap();
        assert_eq!(read, Some(REQUEST_TIMEOUT));
        assert_eq!(write, Some(REQUEST_TIMEOUT));
    }

    #[test]
    fn page_csp_hashes_embedded_assets_and_inline_declarations() {
        let html = workbench_html("secret-token");
        let policy = content_security_policy("text/html; charset=utf-8", &html);
        assert!(!policy.contains("'unsafe-inline'"), "{policy}");
        assert!(policy.contains("script-src-attr 'none'"), "{policy}");
        assert!(
            policy.contains("style-src-attr 'unsafe-hashes'"),
            "{policy}"
        );

        let scripts = inline_tag_contents(&html, "script");
        assert_eq!(scripts.len(), 2, "the shell embeds core.js and app.js");
        for source in scripts {
            assert!(policy.contains(&csp_hash(source)), "missing script hash");
        }
        let styles = inline_tag_contents(&html, "style");
        assert_eq!(styles.len(), 1, "the shell embeds app.css");
        assert!(policy.contains(&csp_hash(styles[0])), "missing style hash");

        for declaration in ["margin-top:14px", "width: 50%;", "text-align: center;"] {
            assert!(
                policy.contains(&csp_hash(declaration)),
                "missing hash for {declaration}"
            );
        }
        assert_eq!(
            content_security_policy("application/json", "{}"),
            "default-src 'none'; frame-ancestors 'none'; base-uri 'none'"
        );
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
            handle_connection(
                stream,
                &shared_for_server,
                PreAuthSlot::reserve(&shared_for_server).unwrap(),
            )
            .unwrap();
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
    fn the_shell_carries_every_surface_and_its_api() {
        let html = workbench_html("secret-token");
        // Composition: all three source files reached the page.
        assert!(html.contains("--accent"), "stylesheet is missing");
        assert!(html.contains("renderRoute()"), "script is missing");
        assert!(html.contains(r#"const token = "secret-token";"#));
        assert!(!html.contains("__TOKEN_JSON__"));
        assert!(!html.contains("__STYLE__"));
        assert!(!html.contains("__SCRIPT__"));

        // Every screen the 1.0 journey needs.
        for id in [
            "projects-screen",
            "catalog-screen",
            "create-screen",
            "overview-screen",
            "build-screen",
            "performance-screen",
            "runs-screen",
            "health-screen",
        ] {
            assert!(
                html.contains(&format!("id=\"{id}\"")),
                "missing screen {id}"
            );
        }

        // Every route the page calls must exist in the router above.
        for route in [
            "/api/v1/projects",
            "/api/v1/catalog",
            "/api/v1/workspace",
            "/api/v1/projects/preflight",
            "/api/v1/state",
            "/api/v1/capability",
            "/api/v1/project-health",
            "/api/v1/runs",
            "/api/v1/runs/cancel",
            "/api/v1/runs/rerun",
            "/api/v1/benchmarks",
            "/api/v1/predictions",
            "/api/v1/reflections",
            "/api/v1/snapshots",
            "/api/v1/snapshots/preview",
            "/api/v1/reports",
            "/api/v1/hints",
            "/api/v1/capabilities/next",
            "/api/v1/project/open-editor",
            "/api/v1/project/open-folder",
        ] {
            assert!(html.contains(route), "page never calls {route}");
        }

        // Every project page the page's own router understands must also be a
        // route the service serves, or a reload on that page returns 404.
        for page in PROJECT_PAGES {
            assert!(
                html.contains(page),
                "the page router does not know about {page}"
            );
            assert!(
                project_page_route(&format!("/projects/some-id/{page}")),
                "the service does not serve /projects/<id>/{page}"
            );
        }
        assert!(!project_page_route("/projects//build"));
        assert!(!project_page_route("/projects/some-id/build/extra"));
        assert!(!project_page_route("/projects/some-id/unknown"));
        assert!(!project_page_route("/elsewhere/some-id/build"));

        // Light and dark are both first-class, and motion respects the
        // learner's preference.
        assert!(html.contains("light-dark(#f5f5f3, #0f1114)"));
        assert!(html.contains("[data-theme=\"dark\"]"));
        assert!(html.contains("prefers-reduced-motion: reduce"));
        assert!(html.contains("skip-link"));

        // The reveal ceiling comes from the service, not from a number the
        // page picked. Re-deriving it offered a level the service refused on
        // every pack whose ladder is not five rungs long.
        assert!(
            html.contains("content.available_help_levels"),
            "the page must read the reveal ceiling the service sends"
        );
        assert!(
            !html.contains("Math.min(content.help_levels"),
            "the page must not re-derive the reveal ceiling"
        );

        // Links never carry the capability token: clicks are routed in-page,
        // so a token in an href only leaks through "Copy link address" and
        // into a new tab's history.
        assert!(
            !html.contains("?token=${encodeURIComponent(token)}"),
            "navigation links must not carry the capability token"
        );

        // No surface tells the learner to open a terminal.
        assert!(!html.contains("deltaforge init"));
        assert!(!html.contains("deltaforge bench"));
        assert!(!html.contains("deltaforge test"));
    }

    #[test]
    fn configured_editor_is_passed_the_project_without_a_shell() {
        let target = Path::new("/tmp/learner-project");
        let resolved =
            project_open_command(ProjectOpenKind::Editor, target, Some("code --reuse-window"))
                .unwrap();

        assert_eq!(resolved.application, "code");
        assert_eq!(resolved.command.get_program(), "code");
        assert_eq!(
            resolved
                .command
                .get_args()
                .map(|value| value.to_string_lossy().to_string())
                .collect::<Vec<_>>(),
            ["--reuse-window", "/tmp/learner-project"]
        );
        assert!(terminal_only_editor("/usr/bin/nvim"));
        assert!(!terminal_only_editor("cursor"));
    }
}
