use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

const OUTPUT_LIMIT: usize = 1024 * 1024;
const STREAM_CHUNK_BYTES: usize = 4 * 1024;
const STREAM_QUEUE_CHUNKS: usize = 64;
const TRUNCATED_MARKER: &[u8] = b"\n[deltaforge: output truncated after 1 MiB]\n";
type StreamChunk = (&'static str, Vec<u8>);
type OutputSink<'a> = &'a mut dyn FnMut(&'static str, &[u8]);

struct CommandRunOptions<'a> {
    timeout_ms: u64,
    stdin: Option<&'a str>,
    envs: &'a BTreeMap<String, String>,
    measure: bool,
    cancel_path: Option<&'a Path>,
    output_sink: Option<OutputSink<'a>>,
}

/// A finished command plus best-effort peak-memory data.
pub struct MeasuredOutput {
    pub output: Output,
    /// Approximate peak resident set size of the child process, sampled from
    /// the poll loop (Linux `VmHWM`, macOS `proc_pid_rusage` resident size,
    /// Windows `PeakWorkingSetSize`). `None` when sampling is unsupported on
    /// this OS or every sample failed; a very short-lived process may exit
    /// before the first sample lands.
    pub peak_rss_bytes: Option<u64>,
}

pub fn run_command(
    command: &[String],
    cwd: &Path,
    timeout_ms: u64,
    stdin: Option<&str>,
    envs: &BTreeMap<String, String>,
) -> Result<Output> {
    Ok(run_command_impl(
        command,
        cwd,
        CommandRunOptions {
            timeout_ms,
            stdin,
            envs,
            measure: false,
            cancel_path: None,
            output_sink: None,
        },
    )?
    .output)
}

pub fn run_command_cancellable(
    command: &[String],
    cwd: &Path,
    timeout_ms: u64,
    stdin: Option<&str>,
    envs: &BTreeMap<String, String>,
    cancel_path: Option<&Path>,
) -> Result<Output> {
    Ok(run_command_impl(
        command,
        cwd,
        CommandRunOptions {
            timeout_ms,
            stdin,
            envs,
            measure: false,
            cancel_path,
            output_sink: None,
        },
    )?
    .output)
}

pub fn run_command_cancellable_streaming(
    command: &[String],
    cwd: &Path,
    timeout_ms: u64,
    stdin: Option<&str>,
    envs: &BTreeMap<String, String>,
    cancel_path: Option<&Path>,
    output_sink: &mut dyn FnMut(&'static str, &[u8]),
) -> Result<Output> {
    Ok(run_command_impl(
        command,
        cwd,
        CommandRunOptions {
            timeout_ms,
            stdin,
            envs,
            measure: false,
            cancel_path,
            output_sink: Some(output_sink),
        },
    )?
    .output)
}

/// Like [`run_command`], additionally sampling the child's peak memory.
/// Sampling failures never fail the command. Only benchmarking should use
/// this; the test-runner path stays on [`run_command`] and pays nothing.
pub fn run_command_measured(
    command: &[String],
    cwd: &Path,
    timeout_ms: u64,
    stdin: Option<&str>,
    envs: &BTreeMap<String, String>,
) -> Result<MeasuredOutput> {
    run_command_impl(
        command,
        cwd,
        CommandRunOptions {
            timeout_ms,
            stdin,
            envs,
            measure: true,
            cancel_path: None,
            output_sink: None,
        },
    )
}

fn run_command_impl(
    command: &[String],
    cwd: &Path,
    options: CommandRunOptions<'_>,
) -> Result<MeasuredOutput> {
    let CommandRunOptions {
        timeout_ms,
        stdin,
        envs,
        measure,
        cancel_path,
        mut output_sink,
    } = options;
    if command.is_empty() {
        bail!("cannot run empty command");
    }
    if timeout_ms == 0 {
        bail!("command timeout must be greater than 0");
    }

    let mut process = Command::new(&command[0]);
    process
        .args(&command[1..])
        .envs(envs)
        .current_dir(cwd)
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure_process_group(&mut process);

    let mut child = process
        .spawn()
        .with_context(|| format!("failed to spawn command {}", command.join(" ")))?;
    let stdout = child
        .stdout
        .take()
        .context("child stdout pipe is missing")?;
    let stderr = child
        .stderr
        .take()
        .context("child stderr pipe is missing")?;
    let (stream_sender, stream_receiver) = if output_sink.is_some() {
        let (sender, receiver) = sync_channel(STREAM_QUEUE_CHUNKS);
        (Some(sender), Some(receiver))
    } else {
        (None, None)
    };
    let stdout_sender = stream_sender.clone();
    let stderr_sender = stream_sender.clone();
    drop(stream_sender);
    let stdout_reader =
        thread::spawn(move || read_bounded_streaming(stdout, "stdout", stdout_sender));
    let stderr_reader =
        thread::spawn(move || read_bounded_streaming(stderr, "stderr", stderr_sender));
    let stdin_writer = stdin.map(|input| {
        let bytes = input.as_bytes().to_vec();
        let mut pipe = child.stdin.take().expect("piped stdin is present");
        thread::spawn(move || pipe.write_all(&bytes))
    });

    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    let mut timed_out = false;
    let mut cancelled = false;
    let mut peak_rss_bytes: Option<u64> = None;
    let status = loop {
        if let (Some(receiver), Some(sink)) = (stream_receiver.as_ref(), output_sink.as_mut()) {
            drain_available_stream_output(receiver, *sink);
        }
        // Sample before try_wait: once the child is reaped its memory
        // accounting is gone (except on Windows, where the open handle
        // keeps it readable).
        if measure && let Some(sample) = sample_peak_rss(&child) {
            peak_rss_bytes = Some(peak_rss_bytes.map_or(sample, |peak| peak.max(sample)));
        }
        if let Some(status) = child
            .try_wait()
            .with_context(|| format!("failed to poll command {}", command.join(" ")))?
        {
            break status;
        }
        if Instant::now() >= deadline {
            timed_out = true;
            terminate_process_tree(&mut child);
            break child.wait().with_context(|| {
                format!("failed to wait for timed-out command {}", command.join(" "))
            })?;
        }
        if cancel_path.is_some_and(Path::exists) {
            cancelled = true;
            terminate_process_tree(&mut child);
            break child.wait().with_context(|| {
                format!("failed to wait for cancelled command {}", command.join(" "))
            })?;
        }
        thread::sleep(Duration::from_millis(10));
    };

    if let (Some(receiver), Some(sink)) = (stream_receiver.as_ref(), output_sink.as_mut()) {
        while let Ok((stream, bytes)) = receiver.recv() {
            sink(stream, &bytes);
        }
    }

    if let Some(writer) = stdin_writer {
        let _ = writer.join();
    }
    let stdout = join_reader(stdout_reader, "stdout")?;
    let stderr = join_reader(stderr_reader, "stderr")?;
    if timed_out {
        bail!(
            "command timed out after {timeout_ms} ms: {}\nstdout:\n{}\nstderr:\n{}",
            command.join(" "),
            String::from_utf8_lossy(&stdout),
            String::from_utf8_lossy(&stderr)
        );
    }
    if cancelled {
        bail!("run cancelled");
    }
    Ok(MeasuredOutput {
        output: Output {
            status,
            stdout,
            stderr,
        },
        peak_rss_bytes,
    })
}

/// Best-effort snapshot of the child's memory usage. Linux reports the kernel
/// high-water mark directly; macOS reports current resident size (the caller's
/// poll loop keeps the max); Windows reports the peak working set.
#[cfg(target_os = "linux")]
fn sample_peak_rss(child: &std::process::Child) -> Option<u64> {
    let status = std::fs::read_to_string(format!("/proc/{}/status", child.id())).ok()?;
    let line = status
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))?;
    let kilobytes: u64 = line.trim().trim_end_matches("kB").trim().parse().ok()?;
    Some(kilobytes * 1024)
}

#[cfg(target_os = "macos")]
fn sample_peak_rss(child: &std::process::Child) -> Option<u64> {
    // Mirrors struct rusage_info_v0 from <libproc.h> (flavor RUSAGE_INFO_V0).
    #[repr(C)]
    #[derive(Default)]
    struct RusageInfoV0 {
        ri_uuid: [u8; 16],
        ri_user_time: u64,
        ri_system_time: u64,
        ri_pkg_idle_wkups: u64,
        ri_interrupt_wkups: u64,
        ri_pageins: u64,
        ri_wired_size: u64,
        ri_resident_size: u64,
        ri_phys_footprint: u64,
        ri_proc_start_abstime: u64,
        ri_proc_exit_abstime: u64,
    }
    const RUSAGE_INFO_V0: i32 = 0;
    unsafe extern "C" {
        fn proc_pid_rusage(pid: i32, flavor: i32, buffer: *mut RusageInfoV0) -> i32;
    }

    let pid = i32::try_from(child.id()).ok()?;
    let mut info = RusageInfoV0::default();
    let result = unsafe { proc_pid_rusage(pid, RUSAGE_INFO_V0, &mut info) };
    (result == 0 && info.ri_resident_size > 0).then_some(info.ri_resident_size)
}

#[cfg(windows)]
fn sample_peak_rss(child: &std::process::Child) -> Option<u64> {
    use std::os::windows::io::AsRawHandle;

    // Mirrors PROCESS_MEMORY_COUNTERS from <psapi.h>.
    #[repr(C)]
    #[derive(Default)]
    struct ProcessMemoryCounters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set_size: usize,
        working_set_size: usize,
        quota_peak_paged_pool_usage: usize,
        quota_paged_pool_usage: usize,
        quota_peak_non_paged_pool_usage: usize,
        quota_non_paged_pool_usage: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
    }
    unsafe extern "system" {
        fn K32GetProcessMemoryInfo(
            process: *mut std::ffi::c_void,
            counters: *mut ProcessMemoryCounters,
            cb: u32,
        ) -> i32;
    }

    let mut counters = ProcessMemoryCounters {
        cb: u32::try_from(std::mem::size_of::<ProcessMemoryCounters>()).ok()?,
        ..Default::default()
    };
    let result = unsafe {
        K32GetProcessMemoryInfo(child.as_raw_handle().cast(), &mut counters, counters.cb)
    };
    (result != 0 && counters.peak_working_set_size > 0)
        .then_some(counters.peak_working_set_size as u64)
}

#[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
fn sample_peak_rss(_child: &std::process::Child) -> Option<u64> {
    None
}

fn read_bounded_streaming(
    mut reader: impl Read,
    stream: &'static str,
    mut sender: Option<SyncSender<StreamChunk>>,
) -> std::io::Result<Vec<u8>> {
    let mut captured = Vec::new();
    let mut buffer = [0_u8; STREAM_CHUNK_BYTES];
    let mut truncated = false;
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        if sender
            .as_ref()
            .is_some_and(|sender| sender.send((stream, buffer[..read].to_vec())).is_err())
        {
            sender = None;
        }
        let remaining = OUTPUT_LIMIT.saturating_sub(captured.len());
        captured.extend_from_slice(&buffer[..read.min(remaining)]);
        truncated |= read > remaining;
    }
    if truncated {
        captured.extend_from_slice(TRUNCATED_MARKER);
    }
    Ok(captured)
}

fn drain_available_stream_output(
    receiver: &Receiver<StreamChunk>,
    sink: &mut dyn FnMut(&'static str, &[u8]),
) {
    while let Ok((stream, bytes)) = receiver.try_recv() {
        sink(stream, &bytes);
    }
}

fn join_reader(
    handle: thread::JoinHandle<std::io::Result<Vec<u8>>>,
    stream: &str,
) -> Result<Vec<u8>> {
    handle
        .join()
        .map_err(|_| anyhow::anyhow!("{stream} reader thread panicked"))?
        .with_context(|| format!("failed to read child {stream}"))
}

#[cfg(unix)]
fn configure_process_group(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    command.process_group(0);
}

#[cfg(not(unix))]
fn configure_process_group(_command: &mut Command) {}

#[cfg(unix)]
fn terminate_process_tree(child: &mut std::process::Child) {
    unsafe extern "C" {
        fn kill(pid: i32, signal: i32) -> i32;
    }
    const SIGKILL: i32 = 9;
    let _ = unsafe { kill(-(child.id() as i32), SIGKILL) };
    let _ = child.kill();
}

#[cfg(windows)]
fn terminate_process_tree(child: &mut std::process::Child) {
    let _ = Command::new("taskkill")
        .args(["/PID", &child.id().to_string(), "/T", "/F"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let _ = child.kill();
}

#[cfg(not(any(unix, windows)))]
fn terminate_process_tree(child: &mut std::process::Child) {
    let _ = child.kill();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn measured_command_reports_peak_memory() {
        let measured = run_command_measured(
            &["sleep".to_string(), "0.1".to_string()],
            Path::new("/"),
            5_000,
            None,
            &BTreeMap::new(),
        )
        .unwrap();
        assert!(measured.output.status.success());
        assert!(
            measured.peak_rss_bytes.unwrap_or(0) > 0,
            "expected a peak memory sample on this OS"
        );
    }

    #[cfg(windows)]
    #[test]
    fn measured_command_reports_peak_memory() {
        let measured = run_command_measured(
            &[
                "ping".to_string(),
                "-n".to_string(),
                "2".to_string(),
                "127.0.0.1".to_string(),
            ],
            Path::new("C:\\"),
            10_000,
            None,
            &BTreeMap::new(),
        )
        .unwrap();
        assert!(measured.output.status.success());
        assert!(
            measured.peak_rss_bytes.unwrap_or(0) > 0,
            "expected a peak memory sample on this OS"
        );
    }

    #[cfg(unix)]
    #[test]
    fn chatty_process_is_drained_and_bounded() {
        let error = run_command(
            &["yes".to_string()],
            Path::new("/"),
            50,
            None,
            &BTreeMap::new(),
        )
        .unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains("timed out"));
        assert!(message.contains("output truncated"));
        assert!(message.len() < OUTPUT_LIMIT + 4096);
    }

    #[cfg(unix)]
    #[test]
    fn streaming_output_is_delivered_before_the_process_finishes() {
        let cancel_path = std::env::temp_dir().join(format!(
            "deltaforge-stream-cancel-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let started = Instant::now();
        let mut received = String::new();
        let mut sink = |stream, bytes: &[u8]| {
            if stream == "stdout" {
                received.push_str(&String::from_utf8_lossy(bytes));
                let _ = std::fs::write(&cancel_path, b"cancel");
            }
        };
        let error = run_command_cancellable_streaming(
            &[
                "sh".to_string(),
                "-c".to_string(),
                "printf 'ready\\n'; sleep 5; printf 'late\\n'".to_string(),
            ],
            Path::new("/"),
            10_000,
            None,
            &BTreeMap::new(),
            Some(&cancel_path),
            &mut sink,
        )
        .unwrap_err();

        assert!(format!("{error:#}").contains("run cancelled"));
        assert!(received.contains("ready"));
        assert!(!received.contains("late"));
        assert!(started.elapsed() < Duration::from_secs(2));
        let _ = std::fs::remove_file(cancel_path);
    }
}
