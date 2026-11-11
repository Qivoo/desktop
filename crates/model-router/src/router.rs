use crate::error::ModelRouterError;
use crate::model_info::ModelInfo;
use crate::registry::ModelRegistry;
use crate::context_engine::{Context, AgentSession};
use tracing::info;

pub struct ModelRouter {
    pub registry: ModelRegistry,
    pub failover_tracker: FailoverTracker,
    pub max_failover_attempts: u32,
}

pub struct FailoverTracker {
    pub last_failover_model: Option<String>,
    pub current_attempt: u32,
}

impl Default for FailoverTracker {
    fn default() -> Self {
        Self { last_failover_model: None, current_attempt: 0 }
    }
}

impl ModelRouter {
    pub fn new(registry: ModelRegistry) -> Self {
        Self {
            registry,
            failover_tracker: FailoverTracker::default(),
            max_failover_attempts: 3,
        }
    }

    pub fn build_context(&self, session: &AgentSession) -> Context {
        Context::new(session.workspace_path.clone(), session.task_state.goal.clone())
    }

    pub fn select_best_model(&self, session: &AgentSession) -> Result<String, ModelRouterError> {
        if let Some(model_id) = self.registry.resolve_alias("openrouter/free") {
            return Ok(model_id);
        }
        let models = self.registry.list_all();
        if let Some(best) = self.select_fallback(&models) {
            Ok(best)
        } else {
            Err(ModelRouterError::ModelNotFound("no free models available".to_string()))
        }
    }

    fn select_fallback(&self, models: &[ModelInfo]) -> Option<String> {
        models.iter()
            .filter(|m| m.supports_tools() && m.cooldown_ms == 0)
            .map(|m| m.id.clone())
            .next()
    }
}
