use thiserror::Error;

#[derive(Error, Debug)]
pub enum TurnError {
    #[error("Turn failed: {0}")]
    Turn(String),
    #[error("Store error: {0}")]
    Store(#[from] local_store::StoreError),
    #[error("Router error: {0}")]
    Router(#[from] model_router::ModelRouterError),
    #[error("Provider error: {0}")]
    Provider(#[from] provider_openrouter::ApiError),
    #[error("Policy error: {0}")]
    Policy(#[from] tool_policy::ToolPolicyError),
}
