use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToolError {
    #[error("Unknown tool: {0}")]
    UnknownTool(String),
    #[error("Command execution failed: {0}")]
    Command(#[from] tokio::process::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Output too large: {0} bytes (max {max_bytes})")]
    OutputTooLarge { actual: usize, max_bytes: usize },
    #[error("File not found: {0}")]
    FileNotFound(String),
}
