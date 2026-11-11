//! Tauri library for the Local-First AI Agent desktop host.

pub mod state {
    use serde::{Deserialize, Serialize};
    use tokio::sync::mpsc;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ToolCall {
        pub tool_call_id: String,
        pub tool: String,
        pub arguments: serde_json::Value,
    }

    pub struct ApprovalChannel {
        tx: mpsc::Sender<ToolCall>,
    }

    impl ApprovalChannel {
        pub fn new() -> (Self, mpsc::Receiver<ToolCall>) {
            let (tx, rx) = mpsc::channel(10);
            (Self { tx }, rx)
        }
    }
}
