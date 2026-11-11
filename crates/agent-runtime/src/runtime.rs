use tracing::info;
use crate::events::*;
use crate::store::*;
use crate::router::*;
use crate::policy::*;
use crate::provider_openrouter::*;

pub struct AgentRuntime {
    pub store: LocalStore,
    pub router: ModelRouter,
    pub policy: PolicyEngine,
    pub provider: OpenRouterClient,
}

impl AgentRuntime {
    pub async fn new(store: LocalStore, router: ModelRouter, policy: PolicyEngine, provider: OpenRouterClient) -> Self {
        Self { store, router, policy, provider }
    }

    pub async fn run_turn(&self, session_id: &str, user_prompt: &str) -> Result<TurnResult, TurnError> {
        info!(session_id, "Starting agent turn");
        self.store.append_event(session_id, &Event::TurnStarted { session_id: session_id.to_string() }).await?;

        let mut session = self.store.load_session(session_id).await?;
        if session.messages.is_empty() {
            session.task_state.goal = user_prompt.to_string();
            session.task_state.pending_steps = vec![user_prompt.to_string()];
        }

        let context = self.router.build_context(&session);
        let result = self.model_loop(&mut session, &context).await?;

        self.store.save_session(session_id, &session).await?;
        self.store.append_event(session_id, &Event::TurnCompleted { session_id: session_id.to_string(), result: result.reason.clone() }).await?;

        Ok(TurnResult { session_id: session_id.to_string(), messages: result.messages, completed: true })
    }

    async fn model_loop(&self, session: &mut AgentSession, context: &Context) -> Result<TurnResult, TurnError> {
        info!(session_id = %session.session_id, "Model loop started");
        Ok(TurnResult { session_id: session.session_id.clone(), messages: vec![], completed: true })
    }
}

pub struct TurnResult {
    pub session_id: String,
    pub messages: Vec<crate::context_engine::CanonicalMessage>,
    pub completed: bool,
    pub reason: String,
}
