use std::path::PathBuf;

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
    assert_eq!(tools.len(), 6);
    for expected in [
        "inspect_packs",
        "create_pack",
        "add_stage",
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

fn tool_text_as_json(result: &rmcp::model::CallToolResult) -> anyhow::Result<serde_json::Value> {
    let text = result
        .content
        .first()
        .and_then(|content| content.as_text())
        .map(|content| content.text.as_str())
        .ok_or_else(|| anyhow::anyhow!("tool result did not contain text content"))?;
    Ok(serde_json::from_str(text)?)
}
