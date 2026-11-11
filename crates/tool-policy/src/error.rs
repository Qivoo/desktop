use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToolPolicyError {
    #[error("Policy evaluation failed: {0}")]
    Evaluation(String),
    #[error("Destructive command blocked: {0}")]
    DestructiveBlocked(String),
}
