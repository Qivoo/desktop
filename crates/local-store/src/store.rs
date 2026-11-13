use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSession {
    pub session_id: String,
    pub messages: Vec<crate::context_engine::CanonicalMessage>,
    pub task_state: crate::context_engine::TaskState,
    pub workspace_path: String,
    pub attachments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub session_id: String,
    pub event_type: String,
    pub timestamp: String,
    pub payload: serde_json::Value,
}

pub struct LocalStore {
    pub base_path: PathBuf,
    pub schema_version: u32,
}

impl LocalStore {
    pub fn open(base_path: PathBuf) -> Self {
        fs::create_dir_all(&base_path).ok();
        fs::create_dir_all(base_path.join("sessions")).ok();
        fs::create_dir_all(base_path.join("memory")).ok();
        fs::create_dir_all(base_path.join("logs")).ok();
        fs::create_dir_all(base_path.join("cache")).ok();
        Self { base_path, schema_version: 1 }
    }

    pub async fn save_session(&self, session_id: &str, session: &AgentSession) -> Result<(), StoreError> {
        let session_path = self.session_path(session_id);
        fs::create_dir_all(&session_path).await?;
        let state = serde_json::json!({
            "schema_version": self.schema_version,
            "session_id": session.session_id,
            "task_state": session.task_state,
            "workspace_path": session.workspace_path,
            "attachments": session.attachments,
            "created_at": chrono::Utc::now().to_rfc3339(),
        });
        let state_path = session_path.join("state.json");
        fs::write(&state_path, serde_json::to_string_pretty(&state)?).await?;
        info!(session_id, "Session saved");
        Ok(())
    }

    pub async fn load_session(&self, session_id: &str) -> Result<AgentSession, StoreError> {
        let session_path = self.session_path(session_id);
        let state_path = session_path.join("state.json");
        if !state_path.exists().await? {
            return Err(StoreError::SessionNotFound(session_id.to_string()));
        }
        let data = fs::read_to_string(&state_path).await?;
        let state: serde_json::Value = serde_json::from_str(&data)?;
        Ok(AgentSession {
            session_id: state["session_id"].as_str().unwrap_or(session_id).to_string(),
            messages: Vec::new(),
            task_state: serde_json::from_value(state["task_state"].clone()).unwrap_or_default(),
            workspace_path: state["workspace_path"].as_str().unwrap_or("").to_string(),
            attachments: state["attachments"].as_array().cloned().unwrap_or_default(),
        })
    }

    pub async fn append_event(&self, session_id: &str, event: &Event) -> Result<(), StoreError> {
        let event_path = self.base_path.join("sessions").join(session_id).join("events.jsonl");
        let entry = serde_json::json!(event);
        let line = serde_json::to_string(&entry)?;
        fs::append_to_file(&event_path, format!("{}\n", line)).await?;
        Ok(())
    }

    pub async fn append_message(&self, session_id: &str, message: &crate::context_engine::CanonicalMessage) -> Result<(), StoreError> {
        let event = Event {
            event_id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            event_type: "message".to_string(),
            timestamp: Utc::now().to_rfc3339(),
            payload: serde_json::to_value(message)?,
        };
        self.append_event(session_id, &event).await
    }

    fn session_path(&self, session_id: &str) -> PathBuf {
        self.base_path.join("sessions").join(session_id)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
}
