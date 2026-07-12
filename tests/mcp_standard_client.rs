use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rmcp::{
    ServiceExt,
    model::CallToolRequestParams,
    transport::{ConfigureCommandExt, TokioChildProcess},
};

fn deltaforge_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_deltaforge"))
}

fn deltaforge_pack_mcp_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_deltaforge-pack-mcp"))
}

#[tokio::test]
async fn official_rust_sdk_negotiates_lists_calls_and_closes() -> anyhow::Result<()> {
    let transport = TokioChildProcess::new(
        tokio::process::Command::new(deltaforge_pack_mcp_bin()).configure(|command| {
            command.env("DELTAFORGE_BIN", deltaforge_bin());
        }),
    )?;

    let client = ().serve(transport).await?;
    let server = client
        .peer_info()
        .expect("initialize returns server information");
    assert_eq!(server.protocol_version.to_string(), "2025-11-25");
    assert_eq!(server.server_info.name, "deltaforge-pack-mcp");

    let tools = client.list_all_tools().await?;
    let names = tools
        .iter()
        .map(|tool| tool.name.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(tools.len(), 19);
    for expected in [
        "inspect_packs",
        "create_pack",
        "add_stage",
        "update_pack_metadata",
        "update_stage_metadata",
        "write_stage_document",
        "replace_stage_tests",
        "write_fixture_file",
        "replace_stage_benchmarks",
        "read_pack_manifest",
        "read_stage_document",
        "read_stage_tests",
        "read_stage_benchmarks",
        "list_fixture_files",
        "read_fixture_file",
        "delete_fixture_file",
        "diagnose_pack",
        "validate_pack",
        "check_reference",
    ] {
        assert!(names.contains(&expected), "missing MCP tool {expected}");
    }
    for name in [
        "read_pack_manifest",
        "read_stage_document",
        "read_stage_tests",
        "read_stage_benchmarks",
        "list_fixture_files",
        "read_fixture_file",
    ] {
        let tool = tools.iter().find(|tool| tool.name == name).unwrap();
        let tool = serde_json::to_value(tool)?;
        assert_eq!(tool["annotations"]["readOnlyHint"], true, "{name}");
        assert_eq!(tool["annotations"]["destructiveHint"], false, "{name}");
        assert_eq!(tool["annotations"]["idempotentHint"], true, "{name}");
        assert_eq!(tool["inputSchema"]["additionalProperties"], false, "{name}");
    }
    let delete = serde_json::to_value(
        tools
            .iter()
            .find(|tool| tool.name == "delete_fixture_file")
            .unwrap(),
    )?;
    assert_eq!(delete["annotations"]["readOnlyHint"], false);
    assert_eq!(delete["annotations"]["destructiveHint"], true);
    assert_eq!(delete["annotations"]["idempotentHint"], false);
    assert_eq!(delete["inputSchema"]["additionalProperties"], false);
    assert!(
        delete["inputSchema"]["required"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("confirm"))
    );

    let inspect = client
        .call_tool(CallToolRequestParams::new("inspect_packs"))
        .await?;
    assert_ne!(inspect.is_error, Some(true));
    let inspect_report = tool_text_as_json(&inspect)?;
    assert_eq!(inspect_report["status"], "ok");
    assert!(
        inspect_report["packs"]
            .as_array()
            .is_some_and(|packs| packs.len() >= 4)
    );

    let blocked = client
        .call_tool(CallToolRequestParams::new("create_pack"))
        .await?;
    assert_eq!(blocked.is_error, Some(true));
    let blocked_report = tool_text_as_json(&blocked)?;
    assert_eq!(blocked_report["status"], "blocked");
    assert!(
        blocked_report["problems"][0]
            .as_str()
            .is_some_and(|problem| problem.contains("missing required string argument id"))
    );

    client.cancel().await?;
    Ok(())
}

#[tokio::test]
async fn official_client_exercises_every_safe_authoring_tool() -> anyhow::Result<()> {
    let packs_dir = temp_path("safe-authoring-tools");
    let transport = TokioChildProcess::new(
        tokio::process::Command::new(deltaforge_pack_mcp_bin()).configure(|command| {
            command.env("DELTAFORGE_BIN", deltaforge_bin());
        }),
    )?;
    let client = ().serve(transport).await?;

    let create = client
        .call_tool(tool_params(
            "create_pack",
            serde_json::json!({
                "id": "safeauthor",
                "name": "Safe Author",
                "description": "Initial description",
                "dest": packs_dir
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&create)?["status"], "ok");

    let added_stage = client
        .call_tool(tool_params(
            "add_stage",
            serde_json::json!({
                "pack_dir": packs_dir.join("safeauthor"),
                "id": "02_grounded_stage",
                "title": "Grounded stage"
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&added_stage)?["status"], "ok");

    let manifest = client
        .call_tool(tool_params(
            "read_pack_manifest",
            serde_json::json!({"project": "safeauthor", "packs_dir": packs_dir}),
        ))
        .await?;
    let manifest = tool_text_as_json(&manifest)?;
    assert_eq!(manifest["status"], "ok");
    assert_eq!(manifest["manifest"]["id"], "safeauthor");
    assert_eq!(manifest["manifest"]["stages"][1]["id"], "02_grounded_stage");
    assert_eq!(
        manifest["manifest"]["languages"]["rust"]["run"]["command"][0],
        "cargo"
    );

    let missing_document = client
        .call_tool(tool_params(
            "read_stage_document",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "document": "design_prompt"
            }),
        ))
        .await?;
    let missing_document = tool_text_as_json(&missing_document)?;
    assert_eq!(missing_document["status"], "ok");
    assert_eq!(missing_document["content"], serde_json::Value::Null);

    let added_stage_document = client
        .call_tool(tool_params(
            "read_stage_document",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "02_grounded_stage",
                "document": "instructions"
            }),
        ))
        .await?;
    assert!(
        tool_text_as_json(&added_stage_document)?["content"]
            .as_str()
            .is_some_and(|content| content.contains("Grounded stage"))
    );

    let missing_benchmarks = client
        .call_tool(tool_params(
            "read_stage_benchmarks",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage"
            }),
        ))
        .await?;
    let missing_benchmarks = tool_text_as_json(&missing_benchmarks)?;
    assert_eq!(missing_benchmarks["status"], "ok");
    assert_eq!(missing_benchmarks["benchmarks"], serde_json::Value::Null);
    assert_eq!(
        missing_benchmarks["performance_gates"],
        serde_json::Value::Null
    );

    let scaffold_fixture = client
        .call_tool(tool_params(
            "read_fixture_file",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "fixture": "example",
                "path": "input.txt"
            }),
        ))
        .await?;
    assert_eq!(
        tool_text_as_json(&scaffold_fixture)?["content"],
        "hello deltaforge\n"
    );

    let metadata = client
        .call_tool(tool_params(
            "update_pack_metadata",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "name": "Safe Authoring Pack",
                "description": "A constrained authoring integration pack",
                "version": "0.2.0",
                "topics": ["systems", "testing"]
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&metadata)?["status"], "ok");

    let stage_metadata = client
        .call_tool(tool_params(
            "update_stage_metadata",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "title": "Echo safely"
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&stage_metadata)?["status"], "ok");

    let document = client
        .call_tool(tool_params(
            "write_stage_document",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "document": "instructions",
                "content": "# Echo safely\n\nPrint the provided fixture value.\n"
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&document)?["status"], "ok");

    let fixture = client
        .call_tool(tool_params(
            "write_fixture_file",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "fixture": "sample",
                "path": "input.txt",
                "content": "hello safe authoring\n"
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&fixture)?["status"], "ok");

    let duplicate_fixture = client
        .call_tool(tool_params(
            "write_fixture_file",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "fixture": "sample",
                "path": "input.txt",
                "content": "must require explicit overwrite\n"
            }),
        ))
        .await?;
    assert_eq!(duplicate_fixture.is_error, Some(true));
    let overwrite_fixture = client
        .call_tool(tool_params(
            "write_fixture_file",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "fixture": "sample",
                "path": "input.txt",
                "content": "hello safe authoring\n",
                "overwrite": true
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&overwrite_fixture)?["status"], "ok");

    for (path, content) in [
        ("z-last.txt", "last\n"),
        ("nested/a-first.txt", "first\n"),
        ("nested/remove-me.txt", "delete me\n"),
    ] {
        let written = client
            .call_tool(tool_params(
                "write_fixture_file",
                serde_json::json!({
                    "project": "safeauthor",
                    "packs_dir": packs_dir,
                    "stage": "01_first_stage",
                    "fixture": "sample",
                    "path": path,
                    "content": content
                }),
            ))
            .await?;
        assert_eq!(tool_text_as_json(&written)?["status"], "ok");
    }

    let fixture_names = client
        .call_tool(tool_params(
            "list_fixture_files",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage"
            }),
        ))
        .await?;
    assert_eq!(
        tool_text_as_json(&fixture_names)?["fixtures"],
        serde_json::json!(["example", "sample"])
    );
    let fixture_files = client
        .call_tool(tool_params(
            "list_fixture_files",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "fixture": "sample"
            }),
        ))
        .await?;
    let fixture_files = tool_text_as_json(&fixture_files)?;
    let listed_paths = fixture_files["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["path"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        listed_paths,
        [
            "input.txt",
            "nested/a-first.txt",
            "nested/remove-me.txt",
            "z-last.txt"
        ]
    );
    assert_eq!(fixture_files["files"][0]["size"], 21);

    let tests = client
        .call_tool(tool_params(
            "replace_stage_tests",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "tests": [{
                    "name": "echoes a fixture value",
                    "fixture": "sample",
                    "command": ["echo", "hello safe authoring"],
                    "expect": {"exit_code": 0, "stdout_exact": "hello safe authoring\n"}
                }]
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&tests)?["status"], "ok");

    let tests_read = client
        .call_tool(tool_params(
            "read_stage_tests",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage"
            }),
        ))
        .await?;
    let mut tests_array = tool_text_as_json(&tests_read)?["tests"].clone();
    tests_array[0]["name"] = serde_json::json!("echoes grounded fixture value");
    let tests_replaced = client
        .call_tool(tool_params(
            "replace_stage_tests",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "tests": tests_array
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&tests_replaced)?["status"], "ok");
    let tests_reread = client
        .call_tool(tool_params(
            "read_stage_tests",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage"
            }),
        ))
        .await?;
    assert_eq!(
        tool_text_as_json(&tests_reread)?["tests"][0]["name"],
        "echoes grounded fixture value"
    );

    let benchmarks = client
        .call_tool(tool_params(
            "replace_stage_benchmarks",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "benchmarks": [{
                    "name": "echo_fixture",
                    "fixture": "sample",
                    "command": ["echo", "{fixture_path}"],
                    "iterations": 2,
                    "warmup": 1,
                    "timeout_ms": 1000
                }],
                "performance_gates": [{
                    "name": "echo throughput",
                    "benchmark": "echo_fixture",
                    "metric": "throughput_mb_s",
                    "min": 0
                }]
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&benchmarks)?["status"], "ok");

    let benchmarks_read = client
        .call_tool(tool_params(
            "read_stage_benchmarks",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage"
            }),
        ))
        .await?;
    let benchmarks_read = tool_text_as_json(&benchmarks_read)?;
    let preserved_gates = benchmarks_read["performance_gates"].clone();
    let mut benchmark_array = benchmarks_read["benchmarks"].clone();
    benchmark_array[0]["iterations"] = serde_json::json!(3);
    let benchmark_replaced = client
        .call_tool(tool_params(
            "replace_stage_benchmarks",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "benchmarks": benchmark_array,
                "performance_gates": preserved_gates.clone()
            }),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&benchmark_replaced)?["status"], "ok");
    let benchmarks_reread = client
        .call_tool(tool_params(
            "read_stage_benchmarks",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage"
            }),
        ))
        .await?;
    let benchmarks_reread = tool_text_as_json(&benchmarks_reread)?;
    assert_eq!(benchmarks_reread["benchmarks"][0]["iterations"], 3);
    assert_eq!(benchmarks_reread["performance_gates"], preserved_gates);

    let invalid_gate = client
        .call_tool(tool_params(
            "replace_stage_benchmarks",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "benchmarks": [{
                    "name": "echo_fixture",
                    "fixture": "sample",
                    "command": ["echo", "{fixture_path}"]
                }],
                "performance_gates": [{
                    "name": "invalid bounds",
                    "benchmark": "echo_fixture",
                    "metric": "throughput_mb_s",
                    "min": 0,
                    "max": 1
                }]
            }),
        ))
        .await?;
    assert_eq!(invalid_gate.is_error, Some(true));
    let invalid_gate_report = tool_text_as_json(&invalid_gate)?;
    assert_eq!(invalid_gate_report["status"], "blocked");
    assert!(
        invalid_gate_report["problems"][0]
            .as_str()
            .is_some_and(|problem| problem.contains("exactly one finite min or max"))
    );

    let delete_arguments = serde_json::json!({
        "project": "safeauthor",
        "packs_dir": packs_dir,
        "stage": "01_first_stage",
        "fixture": "sample",
        "path": "nested/remove-me.txt"
    });
    let unconfirmed_delete = client
        .call_tool(tool_params("delete_fixture_file", delete_arguments.clone()))
        .await?;
    assert_eq!(unconfirmed_delete.is_error, Some(true));
    assert_eq!(tool_text_as_json(&unconfirmed_delete)?["status"], "blocked");
    let mut confirmed_arguments = delete_arguments;
    confirmed_arguments["confirm"] = serde_json::json!(true);
    let confirmed_delete = client
        .call_tool(tool_params(
            "delete_fixture_file",
            confirmed_arguments.clone(),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&confirmed_delete)?["status"], "ok");
    assert!(
        !packs_dir
            .join("safeauthor/stages/01_first_stage/fixtures/sample/nested/remove-me.txt")
            .exists()
    );
    assert!(
        packs_dir
            .join("safeauthor/stages/01_first_stage/fixtures/sample/nested/a-first.txt")
            .is_file()
    );
    let repeated_delete = client
        .call_tool(tool_params("delete_fixture_file", confirmed_arguments))
        .await?;
    assert_eq!(repeated_delete.is_error, Some(true));
    assert_eq!(tool_text_as_json(&repeated_delete)?["status"], "blocked");

    let validate = client
        .call_tool(tool_params(
            "validate_pack",
            serde_json::json!({"project": "safeauthor", "packs_dir": packs_dir}),
        ))
        .await?;
    assert_eq!(tool_text_as_json(&validate)?["status"], "ok");

    let pack_root = packs_dir.join("safeauthor");
    let manifest: serde_yaml::Value =
        serde_yaml::from_str(&fs::read_to_string(pack_root.join("project.yaml"))?)?;
    assert_eq!(manifest["name"], "Safe Authoring Pack");
    assert_eq!(manifest["version"], "0.2.0");
    assert_eq!(manifest["stages"][0]["title"], "Echo safely");
    assert_eq!(
        fs::read_to_string(pack_root.join("stages/01_first_stage/fixtures/sample/input.txt"))?,
        "hello safe authoring\n"
    );

    let unsafe_calls = [
        (
            "read_fixture_file",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir,
                "stage": "01_first_stage", "fixture": "sample", "path": "../input.txt"
            }),
        ),
        (
            "read_fixture_file",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir,
                "stage": "01_first_stage", "fixture": "sample", "path": "/tmp/escape"
            }),
        ),
        (
            "read_fixture_file",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir,
                "stage": "01_first_stage", "fixture": "sample", "path": "nested/../../escape"
            }),
        ),
        (
            "list_fixture_files",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir,
                "stage": "01_first_stage", "fixture": "../sample"
            }),
        ),
        (
            "delete_fixture_file",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir,
                "stage": "01_first_stage", "fixture": "sample", "path": "../input.txt",
                "confirm": true
            }),
        ),
    ];
    for (name, arguments) in unsafe_calls {
        let result = client.call_tool(tool_params(name, arguments)).await?;
        assert_eq!(result.is_error, Some(true), "{name} accepted unsafe path");
        let report = tool_text_as_json(&result)?;
        assert_eq!(report["status"], "blocked");
        assert!(report.get("pack").is_some());
        assert!(report.get("path").is_some());
    }

    let unexpected = client
        .call_tool(tool_params(
            "read_stage_tests",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir,
                "stage": "01_first_stage", "unexpected": true
            }),
        ))
        .await?;
    assert_eq!(unexpected.is_error, Some(true));

    let tests_path = pack_root.join("stages/01_first_stage/tests.yaml");
    let valid_tests = fs::read_to_string(&tests_path)?;
    fs::write(&tests_path, "tests: [\n")?;
    let malformed_tests = client
        .call_tool(tool_params(
            "read_stage_tests",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir, "stage": "01_first_stage"
            }),
        ))
        .await?;
    assert_eq!(malformed_tests.is_error, Some(true));
    assert!(
        tool_text_as_json(&malformed_tests)?["problems"][0]
            .as_str()
            .is_some_and(|problem| problem.contains("malformed"))
    );
    fs::write(&tests_path, &valid_tests)?;
    fs::remove_file(&tests_path)?;
    let missing_tests = client
        .call_tool(tool_params(
            "read_stage_tests",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir, "stage": "01_first_stage"
            }),
        ))
        .await?;
    assert_eq!(missing_tests.is_error, Some(true));
    assert!(
        tool_text_as_json(&missing_tests)?["problems"][0]
            .as_str()
            .is_some_and(|problem| problem.contains("missing"))
    );
    fs::write(&tests_path, &valid_tests)?;

    let benchmarks_path = pack_root.join("stages/01_first_stage/benchmarks.yaml");
    let valid_benchmarks = fs::read_to_string(&benchmarks_path)?;
    fs::write(&benchmarks_path, "benchmarks: [\n")?;
    let malformed_benchmarks = client
        .call_tool(tool_params(
            "read_stage_benchmarks",
            serde_json::json!({
                "project": "safeauthor", "packs_dir": packs_dir, "stage": "01_first_stage"
            }),
        ))
        .await?;
    assert_eq!(malformed_benchmarks.is_error, Some(true));
    fs::write(&benchmarks_path, valid_benchmarks)?;

    let binary_path = pack_root.join("stages/01_first_stage/fixtures/sample/binary.dat");
    fs::write(&binary_path, [0xff, 0xfe, 0xfd])?;
    let large_path = pack_root.join("stages/01_first_stage/fixtures/sample/large.txt");
    fs::write(&large_path, vec![b'x'; 1024 * 1024 + 1])?;
    for path in ["binary.dat", "large.txt"] {
        let result = client
            .call_tool(tool_params(
                "read_fixture_file",
                serde_json::json!({
                    "project": "safeauthor", "packs_dir": packs_dir,
                    "stage": "01_first_stage", "fixture": "sample", "path": path
                }),
            ))
            .await?;
        assert_eq!(result.is_error, Some(true), "{path}");
        assert_eq!(
            tool_text_as_json(&result)?["content"],
            serde_json::Value::Null
        );
    }
    fs::remove_file(binary_path)?;
    fs::remove_file(large_path)?;

    {
        let outside = temp_path("symlink-outside");
        fs::create_dir_all(&outside)?;
        fs::write(outside.join("outside.txt"), "outside\n")?;
        let link = pack_root.join("stages/01_first_stage/fixtures/linked");
        if try_create_dir_symlink(&outside, &link) {
            for (name, arguments) in [
                (
                    "read_fixture_file",
                    serde_json::json!({
                        "project": "safeauthor", "packs_dir": packs_dir,
                        "stage": "01_first_stage", "fixture": "linked", "path": "outside.txt"
                    }),
                ),
                (
                    "list_fixture_files",
                    serde_json::json!({
                        "project": "safeauthor", "packs_dir": packs_dir,
                        "stage": "01_first_stage", "fixture": "linked"
                    }),
                ),
                (
                    "delete_fixture_file",
                    serde_json::json!({
                        "project": "safeauthor", "packs_dir": packs_dir,
                        "stage": "01_first_stage", "fixture": "linked", "path": "outside.txt",
                        "confirm": true
                    }),
                ),
            ] {
                let result = client.call_tool(tool_params(name, arguments)).await?;
                assert_eq!(result.is_error, Some(true), "{name} followed a symlink");
            }
            assert!(outside.join("outside.txt").is_file());
            let _ = fs::remove_file(link);
        }
        let _ = fs::remove_dir_all(outside);
    }

    let invalid_tests = client
        .call_tool(tool_params(
            "replace_stage_tests",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "tests": [{"name": "no assertions", "command": ["echo"], "expect": {}}]
            }),
        ))
        .await?;
    assert_eq!(invalid_tests.is_error, Some(true));
    assert_eq!(fs::read_to_string(&tests_path)?, valid_tests);

    let traversal = client
        .call_tool(tool_params(
            "write_fixture_file",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "fixture": "sample",
                "path": "../escape.txt",
                "content": "must not be written"
            }),
        ))
        .await?;
    assert_eq!(traversal.is_error, Some(true));
    assert!(
        !pack_root
            .join("stages/01_first_stage/fixtures/escape.txt")
            .exists()
    );

    let windows_traversal = client
        .call_tool(tool_params(
            "write_fixture_file",
            serde_json::json!({
                "project": "safeauthor",
                "packs_dir": packs_dir,
                "stage": "01_first_stage",
                "fixture": "sample",
                "path": "..\\escape.txt",
                "content": "must not be written"
            }),
        ))
        .await?;
    assert_eq!(windows_traversal.is_error, Some(true));

    client.cancel().await?;
    let _ = fs::remove_dir_all(packs_dir);
    Ok(())
}

fn tool_text_as_json(result: &rmcp::model::CallToolResult) -> anyhow::Result<serde_json::Value> {
    let text = result
        .content
        .first()
        .and_then(|content| content.as_text())
        .map(|content| content.text.as_str())
        .ok_or_else(|| anyhow::anyhow!("tool result did not contain text content"))?;
    Ok(serde_json::from_str(text)?)
}

fn tool_params(name: &str, arguments: serde_json::Value) -> CallToolRequestParams {
    CallToolRequestParams::new(name.to_string()).with_arguments(
        arguments
            .as_object()
            .expect("tool arguments are an object")
            .clone(),
    )
}

fn temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "deltaforge-mcp-{name}-{}-{nanos}",
        std::process::id()
    ))
}

#[cfg(unix)]
fn try_create_dir_symlink(target: &std::path::Path, link: &std::path::Path) -> bool {
    std::os::unix::fs::symlink(target, link).is_ok()
}

#[cfg(windows)]
fn try_create_dir_symlink(target: &std::path::Path, link: &std::path::Path) -> bool {
    std::os::windows::fs::symlink_dir(target, link).is_ok()
}
