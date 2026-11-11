use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub provider: String,
    pub name: String,
    pub modalities: Vec<String>,
    pub supports_tools: bool,
    pub free_tier: bool,
    pub context_length: usize,
    pub max_tokens: usize,
    pub cooldown_ms: u64,
    pub last_successful_request: Option<String>,
    pub capability_tags: Vec<String>,
}

impl ModelInfo {
    pub fn is_free(&self) -> bool { self.free_tier }
    pub fn supports_tools(&self) -> bool { self.supports_tools }
    pub fn supports_modality(&self, modality: &str) -> bool {
        self.modalities.iter().any(|m| m == modality)
    }
}
