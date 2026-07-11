use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command] if command == "parse" => parse_request(),
        [command] if command == "headers" => print_headers(),
        [command] if command == "keep-alive" => keep_alive(),
        [command, root, request_path] if command == "serve-file" => {
            serve_file(Path::new(root), request_path)
        }
        [command, root, request_path, start, end] if command == "range" => {
            let start = start
                .parse::<usize>()
                .map_err(|_| "invalid range".to_string())?;
            let end = end
                .parse::<usize>()
                .map_err(|_| "invalid range".to_string())?;
            range(Path::new(root), request_path, start, end)
        }
        _ => Err("usage: tinyhttp <parse|headers|keep-alive|serve-file|range> ...".to_string()),
    }
}

fn parse_request() -> Result<(), String> {
    let request = read_stdin()?;
    let (method, path, version) = request_line(&request)?;
    println!("method: {method}");
    println!("path: {path}");
    println!("version: {version}");
    Ok(())
}

fn print_headers() -> Result<(), String> {
    let request = read_stdin()?;
    for line in request.lines().skip(1) {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            break;
        }
        let Some((key, value)) = line.split_once(':') else {
            return Err(format!("malformed header: {line}"));
        };
        println!("{}: {}", key.trim().to_ascii_lowercase(), value.trim());
    }
    Ok(())
}

fn keep_alive() -> Result<(), String> {
    let request = read_stdin()?;
    let (_, _, version) = request_line(&request)?;
    let mut connection = None;
    for line in request.lines().skip(1) {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':')
            && key.eq_ignore_ascii_case("connection")
        {
            connection = Some(value.trim().to_ascii_lowercase());
        }
    }
    let keep = if version == "HTTP/1.1" {
        connection.as_deref() != Some("close")
    } else {
        connection.as_deref() == Some("keep-alive")
    };
    println!("keep-alive: {keep}");
    Ok(())
}

fn serve_file(root: &Path, request_path: &str) -> Result<(), String> {
    let path = safe_join(root, request_path)?;
    if !path.is_file() {
        println!("HTTP/1.1 404 Not Found\r");
        println!("Content-Length: 0\r");
        println!("\r");
        return Ok(());
    }
    let body = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    println!("HTTP/1.1 200 OK\r");
    println!("Content-Type: {}\r", mime_type(&path));
    println!("Content-Length: {}\r", body.len());
    println!("\r");
    print!("{body}");
    Ok(())
}

fn range(root: &Path, request_path: &str, start: usize, end: usize) -> Result<(), String> {
    let path = safe_join(root, request_path)?;
    let bytes = fs::read(&path).map_err(|error| error.to_string())?;
    if start > end || end >= bytes.len() {
        return Err("invalid range".to_string());
    }
    let body = &bytes[start..=end];
    println!("HTTP/1.1 206 Partial Content\r");
    println!("Content-Range: bytes {start}-{end}/{}\r", bytes.len());
    println!("Content-Length: {}\r", body.len());
    println!("\r");
    print!("{}", String::from_utf8_lossy(body));
    Ok(())
}

fn read_stdin() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| error.to_string())?;
    Ok(input)
}

fn request_line(request: &str) -> Result<(&str, &str, &str), String> {
    let line = request
        .lines()
        .next()
        .ok_or_else(|| "missing request line".to_string())?
        .trim_end_matches('\r');
    let parts = line.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
        [method, path, version] => Ok((method, path, version)),
        _ => Err("malformed request line".to_string()),
    }
}

fn safe_join(root: &Path, request_path: &str) -> Result<PathBuf, String> {
    let relative = request_path.trim_start_matches('/');
    let path = Path::new(relative);
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err("unsafe path".to_string());
    }
    Ok(root.join(path))
}

fn mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html",
        Some("txt") => "text/plain",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}
