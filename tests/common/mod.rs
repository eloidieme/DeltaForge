//! Helpers shared by the integration suites that speak HTTP to a workbench.
//!
//! This module exists because the same defect was fixed twice. Both
//! `browser_journey.rs` and `workbench_flow.rs` carried their own copy of "send
//! a request, read the reply", and both unwrapped the read — so the Windows
//! behaviour below had to be discovered, and fixed, in each of them separately.
//! A rule written twice is the defect shape this project keeps finding.

use std::io::Read;
use std::net::TcpStream;

/// Read a complete HTTP response from a connection the server closes when it is
/// done.
///
/// Unix reports that close as end-of-stream. Windows reports it as
/// `ECONNRESET` — *after* delivering the body — so the bytes are all there and
/// the call still returns an error. Unwrapping it failed three tests on Windows
/// CI while passing on macOS and Linux.
///
/// Keep what arrived; fail only when nothing did.
pub fn read_http_response(stream: &mut TcpStream, what: &str) -> String {
    let mut response = String::new();
    if let Err(error) = stream.read_to_string(&mut response)
        && (response.is_empty() || error.kind() != std::io::ErrorKind::ConnectionReset)
    {
        panic!("failed to read the response to {what}: {error}");
    }
    response
}
