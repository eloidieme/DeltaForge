use std::collections::BTreeMap;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
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
        [command, key, value] if command == "memory" => {
            println!("{key}={value}");
            Ok(())
        }
        [command, path, key, value] if command == "write-log" => {
            append_line(Path::new(path), &format!("SET {key} {value}"))?;
            println!("wrote {key}");
            Ok(())
        }
        [command, path, key] if command == "delete-log" => {
            append_line(Path::new(path), &format!("DEL {key}"))?;
            println!("deleted {key}");
            Ok(())
        }
        [command, path, key] if command == "get" => {
            if let Some(value) = recover(Path::new(path))?.get(key).and_then(Clone::clone) {
                println!("{value}");
            }
            Ok(())
        }
        [command, input, output] if command == "compact" => {
            compact(Path::new(input), Path::new(output))
        }
        [command, path] if command == "stats" => stats(Path::new(path)),
        _ => Err("usage: minikv <memory|write-log|delete-log|get|compact|stats> ...".to_string()),
    }
}

fn append_line(path: &Path, line: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    writeln!(file, "{line}").map_err(|error| error.to_string())
}

fn compact(input: &Path, output: &Path) -> Result<(), String> {
    let recovered = recover(input)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let mut out = String::new();
    for (key, value) in recovered {
        if let Some(value) = value {
            out.push_str(&format!("SET {key} {value}\n"));
        }
    }
    fs::write(output, out).map_err(|error| error.to_string())?;
    println!("compacted {}", output.display());
    Ok(())
}

fn stats(path: &Path) -> Result<(), String> {
    let source = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut entries = 0;
    let mut tombstones = 0;
    for line in source.lines() {
        entries += 1;
        if line.starts_with("DEL ") {
            tombstones += 1;
        }
    }
    let live_keys = recover(path)?
        .values()
        .filter(|value| value.is_some())
        .count();
    println!("entries: {entries}");
    println!("live_keys: {live_keys}");
    println!("tombstones: {tombstones}");
    Ok(())
}

fn recover(path: &Path) -> Result<BTreeMap<String, Option<String>>, String> {
    let source = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut values = BTreeMap::new();
    for line in source.lines() {
        if let Some(rest) = line.strip_prefix("SET ") {
            let Some((key, value)) = rest.split_once(' ') else {
                return Err(format!("malformed SET line: {line}"));
            };
            values.insert(key.to_string(), Some(value.to_string()));
        } else if let Some(key) = line.strip_prefix("DEL ") {
            values.insert(key.to_string(), None);
        } else if !line.trim().is_empty() {
            return Err(format!("malformed log line: {line}"));
        }
    }
    Ok(values)
}
