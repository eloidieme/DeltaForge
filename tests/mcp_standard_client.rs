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
    assert_eq!(tools.len(), 12);
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
        "diagnose_pack",
        "validate_pack",
        "check_reference",
    ] {
        assert!(names.contains(&expected), "missing MCP tool {expected}");
    }

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

    #[cfg(unix)]
    {
        let outside = temp_path("symlink-outside");
        fs::create_dir_all(&outside)?;
        let link = pack_root.join("stages/01_first_stage/fixtures/linked");
        std::os::unix::fs::symlink(&outside, &link)?;
        let symlink_write = client
            .call_tool(tool_params(
                "write_fixture_file",
                serde_json::json!({
                    "project": "safeauthor",
                    "packs_dir": packs_dir,
                    "stage": "01_first_stage",
                    "fixture": "linked",
                    "path": "escape.txt",
                    "content": "must not cross symlink"
                }),
            ))
            .await?;
        assert_eq!(symlink_write.is_error, Some(true));
        assert!(!outside.join("escape.txt").exists());
        let _ = fs::remove_file(link);
        let _ = fs::remove_dir_all(outside);
    }

    let tests_path = pack_root.join("stages/01_first_stage/tests.yaml");
    let valid_tests = fs::read_to_string(&tests_path)?;
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
