use serde_json::Value;
use std::path::PathBuf;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

pub struct LocalExecutor {
    pub workspace: PathBuf,
    pub timeout: Duration,
    pub max_output_bytes: usize,
}

impl LocalExecutor {
    pub fn new(workspace: PathBuf) -> Self {
        Self {
            workspace,
            timeout: Duration::from_secs(60),
            max_output_bytes: 1048576,
        }
    }

    pub async fn execute(&self, tool: &str, args: &Value) -> Result<Value, ToolError> {
        match tool {
            "shell.run" => self.run_shell(args).await,
            "filesystem.read" => self.read_file(args).await,
            "filesystem.write" => self.write_file(args).await,
            "filesystem.list" => self.list_dir(args).await,
            "filesystem.search" => self.search_files(args).await,
            "git.status" => self.git_status(args).await,
            "git.diff" => self.git_diff(args).await,
            _ => Err(ToolError::UnknownTool(tool.to_string())),
        }
    }

    async fn run_shell(&self, args: &Value) -> Result<Value, ToolError> {
        let command = args.get("command").and_then(Value::as_str).ok_or_else(|| ToolError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "missing command")))?;
        let args_list: Vec<String> = args.get("args")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let mut cmd = Command::new(command);
        cmd.args(&args_list);
        cmd.current_dir(&self.workspace);

        let output = timeout(self.timeout, cmd.output()).await??;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if stdout.len() > self.max_output_bytes {
            return Err(ToolError::OutputTooLarge { actual: stdout.len(), max_bytes: self.max_output_bytes });
        }

        Ok(Value::Object(serde_json::Map::from([
            ("tool_call_id".into(), args.get("tool_call_id").cloned().unwrap_or_default().into()),
            ("stdout".into(), Value::String(stdout)),
            ("stderr".into(), Value::String(stderr)),
            ("exit_code".into(), Value::Number(output.status.code().map(|c| c.into()).unwrap_or(1).into())),
            ("duration_ms".into(), Value::Number(0.into())),
            ("command".into(), Value::String(command.into())),
        ])))
    }

    async fn read_file(&self, args: &Value) -> Result<Value, ToolError> {
        let path = args.get("path").and_then(Value::as_str).ok_or_else(|| ToolError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "path required")))?;
        let full_path = self.workspace.join(path);
        if !full_path.starts_with(&self.workspace) {
            return Err(ToolError::Io(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "path outside workspace")));
        }
        let contents = tokio::fs::read_to_string(&full_path).await?;
        Ok(Value::Object(serde_json::Map::from([
            ("path".into(), Value::String(path.into())),
            ("content".into(), Value::String(contents)),
        ])))
    }
}
