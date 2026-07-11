use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

const OUTPUT_LIMIT: usize = 1024 * 1024;
const TRUNCATED_MARKER: &[u8] = b"\n[deltaforge: output truncated after 1 MiB]\n";

pub fn run_command(
    command: &[String],
    cwd: &Path,
    timeout_ms: u64,
    stdin: Option<&str>,
    envs: &BTreeMap<String, String>,
) -> Result<Output> {
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
    let stdout_reader = thread::spawn(move || read_bounded(stdout));
    let stderr_reader = thread::spawn(move || read_bounded(stderr));
    let stdin_writer = stdin.map(|input| {
        let bytes = input.as_bytes().to_vec();
        let mut pipe = child.stdin.take().expect("piped stdin is present");
        thread::spawn(move || pipe.write_all(&bytes))
    });

    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    let mut timed_out = false;
    let status = loop {
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
        thread::sleep(Duration::from_millis(10));
    };

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
    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

fn read_bounded(mut reader: impl Read) -> std::io::Result<Vec<u8>> {
    let mut captured = Vec::new();
    let mut buffer = [0_u8; 16 * 1024];
    let mut truncated = false;
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
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
}
