use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use deltaforge::authoring::{
    AddStageRequest, CheckReferenceRequest, NewPackRequest, add_stage, check_reference,
    create_pack, diagnose_pack,
};
use deltaforge::pack::{PackSearchOptions, discover_packs_with_options, load_pack, validate_pack};
use serde_json::{Value, json};

fn main() {
    if let Err(error) = run() {
        eprintln!("deltaforge-pack-mcp error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut stdout = std::io::stdout().lock();

    while let Some(request) = read_message(&mut reader)? {
        let response = handle_request(request);
        if let Some(response) = response {
            write_message(&mut stdout, &response)?;
        }
    }

    Ok(())
}

fn handle_request(request: Value) -> Option<Value> {
    let id = request.get("id").cloned();
    let method = request.get("method").and_then(Value::as_str).unwrap_or("");

    match method {
        "initialize" => id.map(|id| {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "instructions": "Ground pack work with inspect_packs first. Treat status=blocked as a hard stop. A pack is not ready until validate_pack and check_reference both return status=ok. Never copy internal reference solutions into learner templates.",
                    "serverInfo": {
                        "name": "deltaforge-pack-mcp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }
            })
        }),
        "initialized" | "notifications/initialized" => None,
        "tools/list" => id.map(|id| {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": tools()
                }
            })
        }),
        "tools/call" => id.map(|id| match call_tool(&request) {
            Ok(result) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
                        }
                    ],
                    "isError": result.get("status").and_then(Value::as_str) == Some("blocked")
                }
            }),
            Err(error) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&json!({
                                "status": "blocked",
                                "problems": [format!("{error:#}")],
                                "next_actions": ["Fix the tool arguments or inspect the pack with diagnose_pack."]
                            })).unwrap()
                        }
                    ],
                    "isError": true
                }
            }),
        }),
        _ => id.map(|id| {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("unknown method {method}")
                }
            })
        }),
    }
}

fn call_tool(request: &Value) -> Result<Value> {
    let params = request.get("params").context("missing params")?;
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .context("missing tool name")?;
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));

    match name {
        "create_pack" => {
            let report = create_pack(&NewPackRequest {
                id: required_string(&arguments, "id")?,
                name: required_string(&arguments, "name")?,
                description: required_string(&arguments, "description")?,
                dest: required_path(&arguments, "dest")?,
                language: optional_string(&arguments, "language")
                    .unwrap_or_else(|| "rust".to_string()),
                force: optional_bool(&arguments, "force"),
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "add_stage" => {
            let report = add_stage(&AddStageRequest {
                pack_dir: required_path(&arguments, "pack_dir")?,
                id: required_string(&arguments, "id")?,
                title: required_string(&arguments, "title")?,
                force: optional_bool(&arguments, "force"),
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "diagnose_pack" => {
            let pack = load_pack(
                &required_string(&arguments, "project")?,
                &PackSearchOptions {
                    packs_dir: optional_path(&arguments, "packs_dir"),
                },
            )?;
            Ok(serde_json::to_value(diagnose_pack(&pack))?)
        }
        "validate_pack" => {
            let pack = load_pack(
                &required_string(&arguments, "project")?,
                &PackSearchOptions {
                    packs_dir: optional_path(&arguments, "packs_dir"),
                },
            )?;
            let problems = validate_pack(&pack);
            Ok(json!({
                "status": if problems.is_empty() { "ok" } else { "blocked" },
                "pack": pack.manifest.id,
                "path": pack.root,
                "problems": problems,
                "next_actions": if problems.is_empty() {
                    vec!["Run check_reference with a known-good implementation.".to_string()]
                } else {
                    vec!["Fix validation problems before adding more stages.".to_string()]
                }
            }))
        }
        "check_reference" => {
            let report = check_reference(&CheckReferenceRequest {
                project: required_string(&arguments, "project")?,
                language: optional_string(&arguments, "language")
                    .unwrap_or_else(|| "rust".to_string()),
                reference: required_path(&arguments, "reference")?,
                packs_dir: optional_path(&arguments, "packs_dir"),
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "inspect_packs" => {
            let packs = discover_packs_with_options(&PackSearchOptions {
                packs_dir: optional_path(&arguments, "packs_dir"),
            })?;
            Ok(json!({
                "status": "ok",
                "packs": packs.into_iter().map(|pack| json!({
                    "id": pack.manifest.id,
                    "name": pack.manifest.name,
                    "description": pack.manifest.description,
                    "stages": pack.manifest.stages.len(),
                    "path": pack.root
                })).collect::<Vec<_>>()
            }))
        }
        _ => bail!("unknown tool {name}"),
    }
}

fn tools() -> Value {
    json!([
        tool(
            "create_pack",
            "Scaffold a new DeltaForge pack with anti-hallucination placeholders.",
            json!({
                "type": "object",
                "required": ["id", "name", "description", "dest"],
                "properties": {
                    "id": {"type": "string"},
                    "name": {"type": "string"},
                    "description": {"type": "string"},
                    "dest": {"type": "string"},
                    "language": {"type": "string", "default": "rust"},
                    "force": {"type": "boolean", "default": false}
                }
            }),
            false,
            true
        ),
        tool(
            "add_stage",
            "Add a scaffold stage to an existing pack.",
            json!({
                "type": "object",
                "required": ["pack_dir", "id", "title"],
                "properties": {
                    "pack_dir": {"type": "string"},
                    "id": {"type": "string"},
                    "title": {"type": "string"},
                    "force": {"type": "boolean", "default": false}
                }
            }),
            false,
            true
        ),
        tool(
            "diagnose_pack",
            "Return pack authoring gaps and next actions.",
            json!({
                "type": "object",
                "required": ["project"],
                "properties": {
                    "project": {"type": "string"},
                    "packs_dir": {"type": "string"}
                }
            }),
            true,
            false
        ),
        tool(
            "validate_pack",
            "Validate a pack structurally.",
            json!({
                "type": "object",
                "required": ["project"],
                "properties": {
                    "project": {"type": "string"},
                    "packs_dir": {"type": "string"}
                }
            }),
            true,
            false
        ),
        tool(
            "check_reference",
            "Run a reference solution against all pack stages.",
            json!({
                "type": "object",
                "required": ["project", "reference"],
                "properties": {
                    "project": {"type": "string"},
                    "language": {"type": "string", "default": "rust"},
                    "reference": {"type": "string"},
                    "packs_dir": {"type": "string"}
                }
            }),
            false,
            false
        ),
        tool(
            "inspect_packs",
            "List discovered packs for agent grounding.",
            json!({
                "type": "object",
                "properties": {
                    "packs_dir": {"type": "string"}
                }
            }),
            true,
            false
        )
    ])
}

fn tool(
    name: &str,
    description: &str,
    input_schema: Value,
    read_only: bool,
    destructive: bool,
) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
        "annotations": {
            "readOnlyHint": read_only,
            "destructiveHint": destructive,
            "idempotentHint": read_only,
            "openWorldHint": false
        }
    })
}

fn required_string(arguments: &Value, key: &str) -> Result<String> {
    arguments
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .with_context(|| format!("missing required string argument {key}"))
}

fn optional_string(arguments: &Value, key: &str) -> Option<String> {
    arguments
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn required_path(arguments: &Value, key: &str) -> Result<PathBuf> {
    Ok(PathBuf::from(required_string(arguments, key)?))
}

fn optional_path(arguments: &Value, key: &str) -> Option<PathBuf> {
    optional_string(arguments, key).map(PathBuf::from)
}

fn optional_bool(arguments: &Value, key: &str) -> bool {
    arguments.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn read_message(reader: &mut impl BufRead) -> Result<Option<Value>> {
    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Ok(None);
        }
        let trimmed = line.trim_end_matches(['\r', '\n']).trim();
        if trimmed.is_empty() {
            continue;
        }
        return serde_json::from_str(trimmed)
            .context("failed to parse newline-delimited MCP message")
            .map(Some);
    }
}

fn write_message(writer: &mut impl Write, value: &Value) -> Result<()> {
    let body = serde_json::to_vec(value)?;
    writer.write_all(&body)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}
