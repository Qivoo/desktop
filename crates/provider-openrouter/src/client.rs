use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
    pub tools: Option<Vec<ToolDefinition>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: Option<String>,
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Choice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

pub struct OpenRouterClient {
    pub client: reqwest::Client,
    pub base_url: String,
    pub timeout: Duration,
}

impl OpenRouterClient {
    pub async fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            timeout: Duration::from_secs(120),
        }
    }

    pub async fn chat_completion(&self, request: &ChatCompletionRequest) -> Result<ChatCompletionResponse, ApiError> {
        let url = format!("{}/chat/completions", self.base_url);
        info!(url, model = %request.model, "Sending chat completion request");

        let response = timeout(self.timeout, self.client.post(&url))
            .await
            .map_err(|e| ApiError::Timeout(Duration::from_secs(30)))??
            .json::<ChatCompletionResponse>()
            .await?
            .map_err(|e| ApiError::Http(reqwest::Error::new(e, "Invalid response")))?;

        info!(model = %request.model, choices = response.choices.len(), "Received chat completion response");
        Ok(response)
    }

    pub async fn chat_completion_stream(&self, request: &ChatCompletionRequest) -> Result<ChatCompletionResponse, ApiError> {
        let url = format!("{}/chat/completions", self.base_url);
        info!(url, model = %request.model, stream = request.stream, "Streaming chat completion request");

        let response = timeout(self.timeout, self.client.post(&url))
            .await
            .map_err(|e| ApiError::Timeout(Duration::from_secs(30)))?;

        if response.status().is_success() {
            let response = response.json::<ChatCompletionResponse>().await?;
            info!(model = %request.model, choices = response.choices.len(), "Stream received");
            Ok(response)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status, body, "Stream failed");
            Err(ApiError::OpenRouter { message: body, code: Some(status.to_string()) })
        }
    }
}
