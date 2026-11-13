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
        // 1. Try explicit model if session specifies one
        if let Some(model_id) = &session.task_state.current_step {
            if let Some(model) = self.registry.get(model_id) {
                return Ok(model.id.clone());
            }
        }

        // 2. Resolve alias
        if let Some(model_id) = self.registry.resolve_alias("openrouter/free") {
            return Ok(model_id);
        }

        // 3. List free models
        let models = self.registry.list_free_models();
        if models.is_empty() {
            return Err(ModelRouterError::ModelNotFound("no free models available".to_string()));
        }

        // 4. Select best model using failover tracker
        self.select_with_failover(&models)
    }

    fn select_with_failover(&self, models: &[&ModelInfo]) -> Result<String, ModelRouterError> {
        // Reset attempt counter on new turn
        self.failover_tracker.current_attempt = 0;

        // Filter out models in cooldown or with recent failures
        let eligible: Vec<&ModelInfo> = models
            .iter()
            .filter(|m| {
                m.cooldown_ms == 0 // MVP: only filter by cooldown
            })
            .collect();

        if eligible.is_empty() {
            return Err(ModelRouterError::ModelNotFound(
                "no eligible free models".to_string(),
            ));
        }

        // Simple selection: first eligible model
        // In MVP, use priority: models with tools support > general models
        let best = eligible
            .iter()
            .find(|m| m.supports_tools)
            .or_else(|| eligible.first())
            .map(|m| m.id.clone());

        Ok(best.unwrap_or_else(|| eligible[0].id.clone()))
    }

    pub fn record_failure(&mut self, model_id: &str) {
        // In MVP, track failures but don't change selection logic
        // Later: mark model as unavailable for cooldown period
        self.failover_tracker.last_failover_model = Some(model_id.to_string());
        self.failover_tracker.current_attempt += 1;
    }

    pub fn record_success(&mut self, model_id: &str) {
        // Update last successful request
        if let Some(info) = self.registry.get(model_id) {
            // For MVP, we don't persist this immediately
            // Later: update model registry cache
        }
    }

    pub fn get_failover_status(&self) -> (Option<String>, u32) {
        (
            self.failover_tracker.last_failover_model.clone(),
            self.failover_tracker.current_attempt,
        )
    }

    pub fn max_failover_attempts(&self) -> u32 {
        self.max_failover_attempts
    }
}
