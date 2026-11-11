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
    pub content: Option<Vec<crate::context_engine::ContentPart>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    pub goal: String,
    pub completed_steps: Vec<String>,
    pub pending_steps: Vec<String>,
    pub current_step: Option<String>,
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
}
