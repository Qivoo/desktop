use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("OpenRouter error: {message}")]
    OpenRouter { message: String, code: Option<String> },
    #[error("Timeout after {0:?} seconds")]
    Timeout(std::time::Duration),
    #[error("Invalid API key")]
    InvalidApiKey,
}
