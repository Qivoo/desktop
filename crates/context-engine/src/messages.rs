use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

impl std::string::ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::System => "system".to_string(),
            Role::User => "user".to_string(),
            Role::Assistant => "assistant".to_string(),
            Role::Tool => "tool".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentPart {
    Text(String),
    ImageUrl { url: String, detail: String },
    File { media_type: String, data: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalToolResult {
    pub tool_call_id: String,
    pub content: serde_json::Value,
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalMessage {
    pub id: String,
    pub role: Role,
    pub content: Option<Vec<ContentPart>>,
    pub tool_calls: Option<Vec<CanonicalToolCall>>,
    pub tool_result: Option<CanonicalToolResult>,
    pub created_at: String,
    pub model_id: Option<String>,
}

impl CanonicalMessage {
    pub fn new_text(role: Role, text: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role,
            content: Some(vec![ContentPart::Text(text)]),
            tool_calls: None,
            tool_result: None,
            created_at: Utc::now().to_rfc3339(),
            model_id: None,
        }
    }

    pub fn new_tool_call(role: Role, id: &str, name: &str, arguments: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role,
            content: None,
            tool_calls: Some(vec![CanonicalToolCall {
                id: id.to_string(),
                name: name.to_string(),
                arguments,
            }]),
            tool_result: None,
            created_at: Utc::now().to_rfc3339(),
            model_id: None,
        }
    }

    pub fn new_tool_result(role: Role, tool_call_id: &str, content: serde_json::Value, is_error: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role,
            content: None,
            tool_calls: None,
            tool_result: Some(CanonicalToolResult {
                tool_call_id: tool_call_id.to_string(),
                content,
                is_error,
            }),
            created_at: Utc::now().to_rfc3339(),
            model_id: None,
        }
    }

    pub fn with_model_id(mut self, model_id: String) -> Self {
        self.model_id = Some(model_id);
        self
    }

    pub fn is_system(&self) -> bool { self.role == Role::System }
    pub fn is_user(&self) -> bool { self.role == Role::User }
    pub fn is_assistant(&self) -> bool { self.role == Role::Assistant }
    pub fn is_tool(&self) -> bool { self.role == Role::Tool }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    pub goal: String,
    pub completed_steps: Vec<String>,
    pub pending_steps: Vec<String>,
    pub current_step: Option<String>,
}

impl Default for TaskState {
    fn default() -> Self {
        Self {
            goal: String::new(),
            completed_steps: Vec::new(),
            pending_steps: Vec::new(),
            current_step: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSession {
    pub session_id: String,
    pub messages: Vec<CanonicalMessage>,
    pub task_state: TaskState,
    pub workspace_path: String,
    pub attachments: Vec<String>,
}

impl AgentSession {
    pub fn new(workspace_path: String, goal: String) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            messages: Vec::new(),
            task_state: TaskState { goal, completed_steps: Vec::new(), pending_steps: vec![goal.clone()], current_step: None },
            workspace_path,
            attachments: Vec::new(),
        }
    }

    pub fn add_user_message(&mut self, content: String) {
        self.messages.push(CanonicalMessage::new_text(Role::User, content));
    }

    pub fn add_assistant_message(&mut self, content: String) {
        self.messages.push(CanonicalMessage::new_text(Role::Assistant, content));
    }

    pub fn add_tool_call(&mut self, tool_call_id: &str, name: &str, arguments: serde_json::Value) {
        self.messages.push(CanonicalMessage::new_tool_call(Role::Assistant, tool_call_id, name, arguments));
    }

    pub fn add_tool_result(&mut self, tool_call_id: &str, content: serde_json::Value, is_error: bool) {
        self.messages.push(CanonicalMessage::new_tool_result(Role::Tool, tool_call_id, content, is_error));
    }

    pub fn get_last_user_message(&self) -> Option<&CanonicalMessage> {
        self.messages.iter().rev().find(|m| m.is_user())
    }
}
