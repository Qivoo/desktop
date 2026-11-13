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
        let model_id = self.router.select_best_model(session)?;
        info!(model_id, "Selected model for turn");

        // Build request
        let request = ChatCompletionRequest {
            model: model_id.clone(),
            messages: session.messages.clone(),
            temperature: Some(0.7),
            max_tokens: Some(2048),
            stream: false,
            tools: None,
        };

        // Call provider
        let response = self.provider.chat_completion(&request).await?;
        info!(model_id, usage = ?response.usage, "Provider response received");

        // Process response
        let mut assistant_message = CanonicalMessage::new_text(Role::Assistant, "".to_string());
        for choice in response.choices {
            if let Some(content) = choice.message.content {
                assistant_message.content = Some(vec![ContentPart::Text(content)]);
            }
        }
        assistant_message = assistant_message.with_model_id(model_id.clone());
        session.messages.push(assistant_message);

        // Check for tool calls
        if let Some(tool_calls) = self.extract_tool_calls(&response) {
            self.handle_tool_calls(session, tool_calls).await?
        }

        self.store.save_session(session_id, session).await?;
        self.store.append_event(session_id, &Event::TurnCompleted { session_id: session_id.to_string(), result: "completed".to_string() }).await?;

        Ok(TurnResult { session_id: session_id.to_string(), messages: session.messages.clone(), completed: true })
    }

    fn extract_tool_calls(&self, response: &ChatCompletionResponse) -> Option<Vec<ToolCall>> {
        // Handle streaming responses and tool calls
        // This is a simplified placeholder
        None
    }

    async fn handle_tool_calls(&self, session: &mut AgentSession, tool_calls: Vec<ToolCall>) -> Result<(), TurnError> {
        for tool_call in tool_calls {
            session.messages.push(CanonicalMessage::new_tool_call(Role::Assistant, &tool_call.id, &tool_call.name, tool_call.arguments.clone()));

            let decision = self.policy.evaluate(&ToolCall { tool: tool_call.name.clone(), tool_call_id: tool_call.id.clone(), arguments: tool_call.arguments.clone() });

            match decision {
                Decision::Allow => {
                    let result = self.executor.execute(&tool_call.name, &tool_call.arguments).await?;
                    session.messages.push(CanonicalMessage::new_tool_result(Role::Tool, &tool_call.id, result.clone().into(), false));
                }
                Decision::Deny => {
                    session.messages.push(CanonicalMessage::new_tool_result(Role::Tool, &tool_call.id, serde_json::json!({"error": "command blocked by policy"}), true));
                }
                Decision::RequestApproval => {
                    session.messages.push(CanonicalMessage::new_tool_result(Role::Tool, &tool_call.id, serde_json::json!({"error": "approval required"}), true));
                }
            }
        }
        Ok(())
    }
}

pub struct TurnResult {
    pub session_id: String,
    pub messages: Vec<crate::context_engine::CanonicalMessage>,
    pub completed: bool,
    pub reason: String,
}
