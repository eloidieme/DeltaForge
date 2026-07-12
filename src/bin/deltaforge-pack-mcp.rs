use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use deltaforge::authoring::{
    AddStageRequest, CheckReferenceRequest, DeleteFixtureFileRequest, ListFixtureFilesRequest,
    NewPackRequest, ReadFixtureFileRequest, ReadPackRequest, ReadStageDataRequest,
    ReadStageDocumentRequest, ReplaceStageBenchmarksRequest, ReplaceStageTestsRequest,
    UpdatePackMetadataRequest, UpdateStageMetadataRequest, WriteFixtureFileRequest,
    WriteStageDocumentRequest, add_stage, check_reference, create_pack, delete_fixture_file,
    diagnose_pack, list_fixture_files, read_fixture_file, read_pack_manifest,
    read_stage_benchmarks, read_stage_document, read_stage_tests, replace_stage_benchmarks,
    replace_stage_tests, update_pack_metadata, update_stage_metadata, write_fixture_file,
    write_stage_document,
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

    while let Some(message) = read_message(&mut reader)? {
        let request = match message {
            Ok(request) => request,
            Err(error) => {
                write_message(
                    &mut stdout,
                    &json!({
                        "jsonrpc": "2.0",
                        "id": null,
                        "error": {"code": -32700, "message": error}
                    }),
                )?;
                continue;
            }
        };
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
            let requested = request
                .pointer("/params/protocolVersion")
                .and_then(Value::as_str);
            match negotiate_protocol(requested) {
                Ok(protocol_version) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": protocol_version,
                    "capabilities": {
                        "tools": {}
                    },
                    "instructions": "Follow inspect_packs -> read/ground -> mutate -> read/validate. Read the manifest, relevant stage documents, structured tests and benchmarks, and fixture listings/content before changing them. Use only the constrained pack tools instead of unconstrained filesystem editing. Mutations require an explicit packs_dir. Treat status=blocked as a hard stop. Preserve every returned structured field during read-modify-replace workflows, including performance_gates. A pack is not ready until validate_pack and check_reference both return status=ok. Never copy internal reference solutions into learner templates.",
                    "serverInfo": {
                        "name": "deltaforge-pack-mcp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }
                }),
                Err(message) => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {"code": -32602, "message": message}
                }),
            }
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
                                "pack": null,
                                "path": null,
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
        "update_pack_metadata" => {
            ensure_only_arguments(
                &arguments,
                &[
                    "project",
                    "packs_dir",
                    "name",
                    "description",
                    "version",
                    "topics",
                ],
            )?;
            let report = update_pack_metadata(&UpdatePackMetadataRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: required_path(&arguments, "packs_dir")?,
                name: optional_string_strict(&arguments, "name")?,
                description: optional_string_strict(&arguments, "description")?,
                version: optional_string_strict(&arguments, "version")?,
                topics: optional_string_array(&arguments, "topics")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "update_stage_metadata" => {
            ensure_only_arguments(&arguments, &["project", "packs_dir", "stage", "title"])?;
            let report = update_stage_metadata(&UpdateStageMetadataRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: required_path(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                title: required_string(&arguments, "title")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "write_stage_document" => {
            ensure_only_arguments(
                &arguments,
                &["project", "packs_dir", "stage", "document", "content"],
            )?;
            let report = write_stage_document(&WriteStageDocumentRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: required_path(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                document: required_string(&arguments, "document")?,
                content: required_string(&arguments, "content")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "replace_stage_tests" => {
            ensure_only_arguments(&arguments, &["project", "packs_dir", "stage", "tests"])?;
            let report = replace_stage_tests(&ReplaceStageTestsRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: required_path(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                tests: required_array_value(&arguments, "tests")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "write_fixture_file" => {
            ensure_only_arguments(
                &arguments,
                &[
                    "project",
                    "packs_dir",
                    "stage",
                    "fixture",
                    "path",
                    "content",
                    "overwrite",
                ],
            )?;
            let report = write_fixture_file(&WriteFixtureFileRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: required_path(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                fixture: required_string(&arguments, "fixture")?,
                path: required_path(&arguments, "path")?,
                content: required_string(&arguments, "content")?,
                overwrite: optional_bool_strict(&arguments, "overwrite")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "replace_stage_benchmarks" => {
            ensure_only_arguments(
                &arguments,
                &[
                    "project",
                    "packs_dir",
                    "stage",
                    "benchmarks",
                    "performance_gates",
                ],
            )?;
            let report = replace_stage_benchmarks(&ReplaceStageBenchmarksRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: required_path(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                benchmarks: required_array_value(&arguments, "benchmarks")?,
                performance_gates: optional_array_value(&arguments, "performance_gates")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "read_pack_manifest" => {
            ensure_only_arguments(&arguments, &["project", "packs_dir"])?;
            let report = read_pack_manifest(&ReadPackRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: optional_path_strict(&arguments, "packs_dir")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "read_stage_document" => {
            ensure_only_arguments(&arguments, &["project", "packs_dir", "stage", "document"])?;
            let report = read_stage_document(&ReadStageDocumentRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: optional_path_strict(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                document: required_string(&arguments, "document")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "read_stage_tests" => {
            ensure_only_arguments(&arguments, &["project", "packs_dir", "stage"])?;
            let report = read_stage_tests(&ReadStageDataRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: optional_path_strict(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "read_stage_benchmarks" => {
            ensure_only_arguments(&arguments, &["project", "packs_dir", "stage"])?;
            let report = read_stage_benchmarks(&ReadStageDataRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: optional_path_strict(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "list_fixture_files" => {
            ensure_only_arguments(&arguments, &["project", "packs_dir", "stage", "fixture"])?;
            let report = list_fixture_files(&ListFixtureFilesRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: optional_path_strict(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                fixture: optional_string_strict(&arguments, "fixture")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "read_fixture_file" => {
            ensure_only_arguments(
                &arguments,
                &["project", "packs_dir", "stage", "fixture", "path"],
            )?;
            let report = read_fixture_file(&ReadFixtureFileRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: optional_path_strict(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                fixture: required_string(&arguments, "fixture")?,
                path: required_path(&arguments, "path")?,
            })?;
            Ok(serde_json::to_value(report)?)
        }
        "delete_fixture_file" => {
            ensure_only_arguments(
                &arguments,
                &[
                    "project",
                    "packs_dir",
                    "stage",
                    "fixture",
                    "path",
                    "confirm",
                ],
            )?;
            let report = delete_fixture_file(&DeleteFixtureFileRequest {
                project: required_string(&arguments, "project")?,
                packs_dir: required_path(&arguments, "packs_dir")?,
                stage: required_string(&arguments, "stage")?,
                fixture: required_string(&arguments, "fixture")?,
                path: required_path(&arguments, "path")?,
                confirm: optional_bool_strict(&arguments, "confirm")?,
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
            })?
            .packs;
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
            true,
            false
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
            true,
            false
        ),
        tool(
            "update_pack_metadata",
            "Update selected manifest metadata for a pack in an explicit packs directory.",
            json!({
                "type": "object",
                "additionalProperties": false,
                "required": ["project", "packs_dir"],
                "properties": {
                    "project": {"type": "string", "minLength": 1},
                    "packs_dir": {"type": "string", "minLength": 1},
                    "name": {"type": "string", "minLength": 1, "maxLength": 4096},
                    "description": {"type": "string", "minLength": 1, "maxLength": 4096},
                    "version": {"type": "string", "minLength": 1, "maxLength": 4096},
                    "topics": {
                        "type": "array",
                        "minItems": 1,
                        "maxItems": 256,
                        "items": {"type": "string", "minLength": 1, "maxLength": 4096}
                    }
                }
            }),
            false,
            true,
            true
        ),
        tool(
            "update_stage_metadata",
            "Rename one existing stage without changing its id or path.",
            authoring_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "title": {"type": "string", "minLength": 1, "maxLength": 4096}
                }),
                &["stage", "title"]
            ),
            false,
            true,
            true
        ),
        tool(
            "write_stage_document",
            "Replace one known stage document: instructions, hints, or design_prompt.",
            authoring_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "document": {"type": "string", "enum": ["instructions", "hints", "design_prompt"]},
                    "content": {"type": "string", "minLength": 1, "maxLength": 1048576}
                }),
                &["stage", "document", "content"]
            ),
            false,
            true,
            true
        ),
        tool(
            "replace_stage_tests",
            "Validate and atomically replace a stage test suite from structured test definitions.",
            authoring_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "tests": {
                        "type": "array",
                        "minItems": 1,
                        "items": test_definition_schema()
                    }
                }),
                &["stage", "tests"]
            ),
            false,
            true,
            true
        ),
        tool(
            "write_fixture_file",
            "Create or explicitly overwrite one UTF-8 fixture file beneath a named stage fixture.",
            authoring_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "fixture": {"type": "string", "minLength": 1},
                    "path": {"type": "string", "minLength": 1},
                    "content": {"type": "string", "maxLength": 1048576},
                    "overwrite": {"type": "boolean", "default": false}
                }),
                &["stage", "fixture", "path", "content"]
            ),
            false,
            true,
            false
        ),
        tool(
            "replace_stage_benchmarks",
            "Validate and atomically replace stage benchmarks from structured definitions.",
            authoring_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "benchmarks": {
                        "type": "array",
                        "minItems": 1,
                        "items": benchmark_definition_schema()
                    },
                    "performance_gates": {
                        "type": "array",
                        "items": performance_gate_schema()
                    }
                }),
                &["stage", "benchmarks"]
            ),
            false,
            true,
            true
        ),
        tool(
            "read_pack_manifest",
            "Read the complete parsed manifest for one discovered pack.",
            read_target_schema(json!({}), &[]),
            true,
            false,
            true
        ),
        tool(
            "read_stage_document",
            "Read one constrained stage document; an absent optional design prompt returns null content.",
            read_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "document": {"type": "string", "enum": ["instructions", "hints", "design_prompt"]}
                }),
                &["stage", "document"]
            ),
            true,
            false,
            true
        ),
        tool(
            "read_stage_tests",
            "Read tests.yaml as the structured tests array accepted by replace_stage_tests.",
            read_target_schema(
                json!({"stage": {"type": "string", "minLength": 1}}),
                &["stage"]
            ),
            true,
            false,
            true
        ),
        tool(
            "read_stage_benchmarks",
            "Read the complete structured benchmarks and performance_gates accepted by replace_stage_benchmarks; an absent optional file returns null.",
            read_target_schema(
                json!({"stage": {"type": "string", "minLength": 1}}),
                &["stage"]
            ),
            true,
            false,
            true
        ),
        tool(
            "list_fixture_files",
            "List sorted fixture names or recursively list sorted regular files and byte sizes in one fixture.",
            read_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "fixture": {"type": "string", "minLength": 1}
                }),
                &["stage"]
            ),
            true,
            false,
            true
        ),
        tool(
            "read_fixture_file",
            "Read one UTF-8 regular fixture file up to 1 MiB without following symlinks.",
            read_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "fixture": {"type": "string", "minLength": 1},
                    "path": {"type": "string", "minLength": 1}
                }),
                &["stage", "fixture", "path"]
            ),
            true,
            false,
            true
        ),
        tool(
            "delete_fixture_file",
            "Permanently delete one confirmed regular file beneath an explicitly selected authored fixture.",
            authoring_target_schema(
                json!({
                    "stage": {"type": "string", "minLength": 1},
                    "fixture": {"type": "string", "minLength": 1},
                    "path": {"type": "string", "minLength": 1},
                    "confirm": {"type": "boolean"}
                }),
                &["stage", "fixture", "path", "confirm"]
            ),
            false,
            true,
            false
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
            false,
            true
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
            false,
            true
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
            false,
            true
        )
    ])
}

fn authoring_target_schema(extra_properties: Value, required: &[&str]) -> Value {
    let mut properties = serde_json::Map::from_iter([
        (
            "project".to_string(),
            json!({"type": "string", "minLength": 1}),
        ),
        (
            "packs_dir".to_string(),
            json!({"type": "string", "minLength": 1}),
        ),
    ]);
    properties.extend(
        extra_properties
            .as_object()
            .expect("authoring schema properties are an object")
            .clone(),
    );
    let mut required_fields = vec![json!("project"), json!("packs_dir")];
    required_fields.extend(required.iter().map(|field| json!(field)));
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": required_fields,
        "properties": properties
    })
}

fn read_target_schema(extra_properties: Value, required: &[&str]) -> Value {
    let mut properties = serde_json::Map::from_iter([
        (
            "project".to_string(),
            json!({"type": "string", "minLength": 1}),
        ),
        (
            "packs_dir".to_string(),
            json!({"type": "string", "minLength": 1}),
        ),
    ]);
    properties.extend(
        extra_properties
            .as_object()
            .expect("read schema properties are an object")
            .clone(),
    );
    let mut required_fields = vec![json!("project")];
    required_fields.extend(required.iter().map(|field| json!(field)));
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": required_fields,
        "properties": properties
    })
}

fn test_definition_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["name", "command", "expect"],
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "fixture": {"type": "string", "minLength": 1},
            "stdin": {"type": "string"},
            "env": {
                "type": "object",
                "additionalProperties": {"type": "string"}
            },
            "command": {
                "type": "array",
                "minItems": 1,
                "items": {"type": "string"}
            },
            "expect": {
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "exit_code": {"type": "integer"},
                    "stdout_exact": {"type": "string"},
                    "stdout_contains": string_array_schema(),
                    "stdout_not_contains": string_array_schema(),
                    "stderr_contains": string_array_schema(),
                    "file_exists": string_array_schema(),
                    "file_not_exists": string_array_schema(),
                    "file_contains": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "additionalProperties": false,
                            "required": ["path", "contains"],
                            "properties": {
                                "path": {"type": "string"},
                                "contains": {"type": "string"}
                            }
                        }
                    },
                    "regex_match": string_array_schema(),
                    "json_equals": {},
                    "timeout_ms": {"type": "integer", "minimum": 1}
                }
            }
        }
    })
}

fn benchmark_definition_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["name", "fixture", "command"],
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "fixture": {"type": "string", "minLength": 1},
            "command": {
                "type": "array",
                "minItems": 1,
                "items": {"type": "string"}
            },
            "matrix": {
                "type": "object",
                "description": "Optional parameter matrix: name -> non-empty list of scalar values. The cartesian product of all parameters is measured; {name} placeholders in command args are expanded per point.",
                "propertyNames": {"pattern": "^[A-Za-z_][A-Za-z0-9_]*$"},
                "additionalProperties": {
                    "type": "array",
                    "minItems": 1,
                    "items": {"type": ["string", "number", "boolean"]}
                }
            },
            "iterations": {"type": "integer", "minimum": 1},
            "warmup": {"type": "integer", "minimum": 0},
            "timeout_ms": {"type": "integer", "minimum": 1}
        }
    })
}

fn performance_gate_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["name", "benchmark", "metric"],
        "oneOf": [
            {"required": ["min"], "not": {"required": ["max"]}},
            {"required": ["max"], "not": {"required": ["min"]}}
        ],
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "benchmark": {"type": "string", "minLength": 1},
            "metric": {"enum": ["runtime_median_ms", "runtime_p95_ms", "throughput_mb_s", "peak_memory_mb", "speedup"]},
            "min": {"type": "number"},
            "max": {"type": "number"},
            "params": {"type": "object", "additionalProperties": {"type": "string"}},
            "advice": {"type": "array", "items": {"type": "string"}}
        }
    })
}

fn string_array_schema() -> Value {
    json!({"type": "array", "items": {"type": "string"}})
}

fn tool(
    name: &str,
    description: &str,
    input_schema: Value,
    read_only: bool,
    destructive: bool,
    idempotent: bool,
) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
        "annotations": {
            "readOnlyHint": read_only,
            "destructiveHint": destructive,
            "idempotentHint": idempotent,
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

fn ensure_only_arguments(arguments: &Value, allowed: &[&str]) -> Result<()> {
    let object = arguments
        .as_object()
        .context("tool arguments must be an object")?;
    if let Some(key) = object.keys().find(|key| !allowed.contains(&key.as_str())) {
        bail!("unknown tool argument {key}");
    }
    Ok(())
}

fn optional_string(arguments: &Value, key: &str) -> Option<String> {
    arguments
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn optional_string_strict(arguments: &Value, key: &str) -> Result<Option<String>> {
    match arguments.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => bail!("optional argument {key} must be a string"),
    }
}

fn optional_string_array(arguments: &Value, key: &str) -> Result<Option<Vec<String>>> {
    let Some(value) = arguments.get(key) else {
        return Ok(None);
    };
    let array = value
        .as_array()
        .with_context(|| format!("optional argument {key} must be an array"))?;
    array
        .iter()
        .map(|item| {
            item.as_str()
                .map(ToString::to_string)
                .with_context(|| format!("every {key} item must be a string"))
        })
        .collect::<Result<Vec<_>>>()
        .map(Some)
}

fn optional_array_value(arguments: &Value, key: &str) -> Result<Option<Value>> {
    match arguments.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(value) if value.is_array() => Ok(Some(value.clone())),
        Some(_) => bail!("optional argument {key} must be an array"),
    }
}

fn required_array_value(arguments: &Value, key: &str) -> Result<Value> {
    let value = arguments
        .get(key)
        .with_context(|| format!("missing required array argument {key}"))?;
    if !value.is_array() {
        bail!("required argument {key} must be an array");
    }
    Ok(value.clone())
}

fn required_path(arguments: &Value, key: &str) -> Result<PathBuf> {
    Ok(PathBuf::from(required_string(arguments, key)?))
}

fn optional_path(arguments: &Value, key: &str) -> Option<PathBuf> {
    optional_string(arguments, key).map(PathBuf::from)
}

fn optional_path_strict(arguments: &Value, key: &str) -> Result<Option<PathBuf>> {
    optional_string_strict(arguments, key).map(|value| value.map(PathBuf::from))
}

fn optional_bool(arguments: &Value, key: &str) -> bool {
    arguments.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn optional_bool_strict(arguments: &Value, key: &str) -> Result<bool> {
    match arguments.get(key) {
        None | Some(Value::Null) => Ok(false),
        Some(Value::Bool(value)) => Ok(*value),
        Some(_) => bail!("optional argument {key} must be a boolean"),
    }
}

fn negotiate_protocol(requested: Option<&str>) -> std::result::Result<&'static str, String> {
    const SUPPORTED: [&str; 4] = ["2024-11-05", "2025-03-26", "2025-06-18", "2025-11-25"];
    let requested =
        requested.ok_or_else(|| "initialize params.protocolVersion is required".to_string())?;
    Ok(SUPPORTED
        .iter()
        .copied()
        .find(|version| *version == requested)
        .unwrap_or("2025-11-25"))
}

fn read_message(reader: &mut impl BufRead) -> Result<Option<std::result::Result<Value, String>>> {
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
        return Ok(Some(serde_json::from_str(trimmed).map_err(|error| {
            format!("failed to parse newline-delimited MCP message: {error}")
        })));
    }
}

fn write_message(writer: &mut impl Write, value: &Value) -> Result<()> {
    let body = serde_json::to_vec(value)?;
    writer.write_all(&body)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}
