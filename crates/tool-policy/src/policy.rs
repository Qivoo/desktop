use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Decision {
    Allow,
    Deny,
    RequestApproval,
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub tool: String,
    pub tool_call_id: String,
    pub arguments: Value,
}

pub struct PolicyEngine {
    pub workspace: PathBuf,
    pub allowlist: Vec<String>,
    pub blocklist: Vec<String>,
    pub require_approval: bool,
}

impl PolicyEngine {
    pub fn new(workspace: PathBuf) -> Self {
        Self {
            workspace,
            allowlist: vec![
                "shell.run".to_string(),
                "filesystem.read".to_string(),
                "filesystem.list".to_string(),
            ],
            blocklist: vec![],
            require_approval: true,
        }
    }

    pub fn evaluate(&self, tool_call: &ToolCall) -> Decision {
        if tool_call.tool == "shell.run" {
            let args = tool_call.arguments.clone();
            let cmd = args.get("command").and_then(Value::as_str).unwrap_or("");
            let is_destructive = cmd.contains("rm ") || cmd.contains("rm -rf") || cmd.contains("mkfs") || cmd.contains("format") || cmd.contains("shutdown") || cmd.contains("reboot");
            if is_destructive {
                return Decision::Deny;
            }
            if cmd.contains("curl ") || cmd.contains("wget ") {
                return Decision::RequestApproval;
            }
        }
        if tool_call.tool == "filesystem.write" || tool_call.tool == "filesystem.list" || tool_call.tool == "filesystem.search" {
            let path = tool_call.arguments.get("path").and_then(Value::as_str).unwrap_or("");
            if !path.is_empty() && !path.starts_with('/') && !path.starts_with('.') {
                if tool_call.tool == "filesystem.write" {
                    return Decision::RequestApproval;
                }
            }
        }
        Decision::Allow
    }
}
