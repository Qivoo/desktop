use crate::error::ModelRouterError;
use crate::model_info::ModelInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistry {
    pub models: HashMap<String, ModelInfo>,
    pub cache_file: String,
    pub last_refresh: Option<String>,
}

impl ModelRegistry {
    pub fn new(cache_file: String) -> Self {
        Self {
            models: HashMap::new(),
            cache_file,
            last_refresh: None,
        }
    }

    pub fn insert(&mut self, model: ModelInfo) {
        self.models.insert(model.id.clone(), model);
    }

    pub fn list_all(&self) -> Vec<&ModelInfo> {
        self.models.values().collect()
    }

    pub fn get(&self, model_id: &str) -> Option<&ModelInfo> {
        self.models.get(model_id)
    }

    pub fn list_free_models(&self) -> Vec<&ModelInfo> {
        self.models.values()
            .filter(|m| m.free_tier)
            .collect()
    }

    pub fn list_models_with_tools(&self) -> Vec<&ModelInfo> {
        self.models.values()
            .filter(|m| m.supports_tools)
            .collect()
    }

    pub fn resolve_alias(&self, alias: &str) -> Option<String> {
        if alias == "openrouter/free" || alias == "free" {
            // Return first free model that supports tools
            for model in self.list_models_with_tools() {
                return Some(model.id.clone());
            }
            // Fallback to any free model
            for model in self.list_free_models() {
                return Some(model.id.clone());
            }
        }
        self.models.get(alias).map(|m| m.id.clone())
    }

    pub async fn refresh_from_openrouter(
        &mut self,
        base_url: &str,
        api_key: &str,
    ) -> Result<(), ModelRouterError> {
        info!(base_url, "Refreshing model registry from OpenRouter");
        // Build the OpenRouter free models endpoint URL
        let url = format!(
            "{}/models?extra_body[key]=free&exclude_default=false&page_size=100",
            base_url.trim_end_matches('/')
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<OpenRouterModelsResponse>()
            .await?;

        // Clear existing models
        self.models.clear();
        self.last_refresh = Some(chrono::Utc::now().to_rfc3339());

        // Parse models
        for model_data in response.data {
            let model_info = ModelInfo::from_openrouter(&model_data)?;
            self.models.insert(model_info.id.clone(), model_info);
        }

        // Cache registry
        let cache_dir = std::path::Path::new(&self.cache_file).parent();
        if let Some(dir) = cache_dir {
            std::fs::create_dir_all(dir)?;
        }
        let cache = serde_json::to_string_pretty(&self.models)?;
        fs::write(self.cache_file, cache).await?;

        info!(count = self.models.len(), "Model registry refreshed");
        Ok(())
    }
}

// OpenRouter models response structure
#[derive(Debug, Deserialize)]
pub struct OpenRouterModelsResponse {
    pub data: Vec<OpenRouterModelData>,
    pub status: u16,
}

#[derive(Debug, Deserialize)]
pub struct OpenRouterModelData {
    #[serde(rename = "id")]
    pub model_id: String,
    pub name: String,
    pub context_length: Option<u32>,
    pub context_window: Option<u32>,
    #[serde(rename = "modalities")]
    pub modalities: Vec<String>,
    pub max_tokens: Option<u32>,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub pricing: Option<serde_json::Value>,
    pub AAD: Option<bool>,
    pub archived: Option<bool>,
    pub color: Option<String>,
    pub config: Option<serde_json::Value>,
    pub created: Option<u64>,
    pub credential: Option<String>,
    pub disabled: Option<bool>,
    pub context_window_auto_adjust: Option<bool>,
    #[serde(rename = "do_text_embeddings")]
    pub do_text_embeddings: Option<bool>,
    #[serde(rename = "do_audio_transcription")]
    pub do_audio_transcription: Option<bool>,
    #[serde(rename = "do_text_to_speech")]
    pub do_text_to_speech: Option<bool>,
    #[serde(rename = "do_image_generation")]
    pub do_image_generation: Option<bool>,
    #[serde(rename = "do_object_detection")]
    pub do_object_detection: Option<bool>,
    pub hardware: Option<Vec<String>>,
    pub home: Option<bool>,
    pub input_price: Option<String>,
    pub max_batch_size: Option<u32>,
    pub max_image_input_size: Option<u32>,
    pub max_completion_tokens: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub num_retries: Option<u32>,
    pub owner: Option<String>,
    #[serde(rename = "per_input_txn_price")]
    pub per_input_txn_price: Option<String>,
    #[serde(rename = "per_minute")]
    pub per_minute: Option<String>,
    #[serde(rename = "per_output_txn_price")]
    pub per_output_txn_price: Option<String>,
    pub prompt: Option<String>,
    pub quality: Option<u32>,
    pub recency: Option<f64>,
    pub revenue_share: Option<bool>,
    pub sub: Option<String>,
    pub speed: Option<u32>,
    pub status: Option<String>,
    pub supported_parameters: Option<Vec<String>>,
    pub tunable: Option<bool>,
    pub vendor: Option<String>,
    pub vram: Option<u32>,
}

impl OpenRouterModelData {
    /// Extract free tier status from pricing data
    fn is_free_tier(&self) -> bool {
        // Check if the model is marked as free or has free pricing
        match &self.pricing {
            Some(p) => {
                if let Some(input) = p.get("prompt") {
                    if let Some(s) = input.as_str() {
                        return s.contains("0") || s.contains("free");
                    }
                }
                if let Some(output) = p.get("completion") {
                    if let Some(s) = output.as_str() {
                        return s.contains("0") || s.contains("free");
                    }
                }
                false
            }
            None => false,
        }
    }

    /// Extract modality support from pricing or config
    fn get_modalities(&self) -> Vec<String> {
        let mut modalities = Vec::new();
        if self.do_text_embeddings.unwrap_or(false) {
            modalities.push("text_embeddings".to_string());
        }
        if self.do_audio_transcription.unwrap_or(false) {
            modalities.push("audio".to_string());
        }
        if self.do_text_to_speech.unwrap_or(false) {
            modalities.push("text_to_speech".to_string());
        }
        if self.do_image_generation.unwrap_or(false) {
            modalities.push("image_generation".to_string());
        }
        // Default: text
        if modalities.is_empty() {
            modalities.push("text".to_string());
        }
        modalities
    }
}

impl ModelInfo {
    pub fn from_openrouter(data: &OpenRouterModelData) -> Result<Self, ModelRouterError> {
        let is_free = data.is_free_tier();
        let modalities = data.get_modalities();

        // Determine free tier based on model status and pricing
        let free_tier = data.status.as_deref() == Some("free") || data.is_free_tier();

        Ok(Self {
            id: data.model_id.clone(),
            provider: data.vendor.clone().unwrap_or_else(|| "unknown".to_string()),
            name: data.name.clone(),
            modalities,
            supports_tools: false, // Will be filled by provider metadata
            free_tier,
            context_length: data.context_window.unwrap_or(data.context_length.unwrap_or(32768)) as usize,
            max_tokens: data.max_completion_tokens.unwrap_or(8192) as usize,
            cooldown_ms: 0,
            last_successful_request: None,
            capability_tags: Self::extract_tags(data),
        })
    }

    fn extract_tags(data: &OpenRouterModelData) -> Vec<String> {
        let mut tags = Vec::new();
        if data.do_text_embeddings.unwrap_or(false) {
            tags.push("text_embeddings".to_string());
        }
        if data.do_audio_transcription.unwrap_or(false) {
            tags.push("audio".to_string());
        }
        if data.do_text_to_speech.unwrap_or(false) {
            tags.push("text_to_speech".to_string());
        }
        if data.do_image_generation.unwrap_or(false) {
            tags.push("image_generation".to_string());
        }
        if data.max_completion_tokens.unwrap_or(0) > 8192 {
            tags.push("large_context".to_string());
        }
        // Default: general
        if tags.is_empty() {
            tags.push("general".to_string());
        }
        tags
    }
}
