use crate::messages::*;
use tracing::info;

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

    pub fn current_bytes(&self) -> usize {
        self.current_bytes
    }

    pub fn is_within_budget(&self) -> bool {
        self.current_bytes <= self.context_budget_bytes
    }

    pub fn add_system_prompt(&mut self, prompt: String) {
        self.system_prompt = prompt;
        if self.messages.is_empty() || self.messages[0].role != Role::System {
            self.messages.insert(0, CanonicalMessage::new_text(Role::System, prompt));
        }
    }

    pub fn compact(&mut self, budget: usize) {
        if self.messages.len() <= 3 {
            return;
        }
        self.context_budget_bytes = budget;
        self.current_bytes = 0;
        // Keep system message
        if let Some(sys) = self.messages.first() {
            self.messages = vec![sys.clone()];
            self.current_bytes += 100;
        }
        // Keep last N messages
        let keep = budget / 512;
        for msg in self.messages.iter().rev().take(keep) {
            self.messages.insert(0, msg.clone());
            self.current_bytes += 256;
        }
        info!(messages = self.messages.len(), "Context compacted");
    }
}
