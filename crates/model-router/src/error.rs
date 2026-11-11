#[derive(thiserror::Error, Debug)]
pub enum ModelRouterError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Registry load error: {0}")]
    Registry(String),
    #[error("Rate limited: {model_id} cooldown {cooldown_ms}ms")]
    RateLimited { model_id: String, cooldown_ms: u64 },
    #[error("Timeout after {0:?} seconds")]
    Timeout(std::time::Duration),
}
