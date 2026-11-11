use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContextError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Context budget exceeded: current {current} bytes, limit {limit} bytes")]
    BudgetExceeded { current: usize, limit: usize },
    #[error("Unknown role: {0}")]
    UnknownRole(String),
}
