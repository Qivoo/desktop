use crate::messages::*;

pub struct Context {
    pub messages: Vec<CanonicalMessage>,
    pub system_prompt: String,
    pub task_state: TaskState,
    pub context_budget_bytes: usize,
    pub current_bytes: usize,
}

impl Context {
    pub fn new(workspace_path: String, goal: String) -> Self {
        let mut session = AgentSession::new(workspace_path, goal);
        let system_prompt = "You are a local-first AI assistant with access to shell and filesystem tools. You operate within the configured workspace and ask for approval before destructive or out-of-workspace operations.".to_string();
        session.messages.push(CanonicalMessage::new_text(Role::System, system_prompt));
        Self {
            messages: session.messages,
            system_prompt,
            task_state: session.task_state,
            context_budget_bytes: 8 * 1024 * 1024,
            current_bytes: 0,
        }
    }

    pub fn budget_bytes(&self) -> usize {
        self.context_budget_bytes
    }
}
