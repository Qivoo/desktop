#[derive(Debug, Clone)]
pub enum TurnEvent {
    TurnStarted { session_id: String },
    TurnCompleted { session_id: String, result: String },
    TurnCancelled { session_id: String },
    ModelAttemptStarted { session_id: String, model_id: String, attempt: u32 },
    ModelAttemptFailed { session_id: String, model_id: String, attempt: u32, error: String },
    ModelFailover { session_id: String, from_model: String, to_model: String },
    ToolStarted { session_id: String, tool_call_id: String, tool: String },
    ToolApprovalRequired { session_id: String, tool_call_id: String, tool: String, args: String },
    ToolCompleted { session_id: String, tool_call_id: String, exit_code: i32, output_len: usize },
    ToolFailed { session_id: String, tool_call_id: String, error: String },
    ContextCompacted { session_id: String, compacted_bytes: usize },
    PersistenceError { session_id: String, error: String },
}
