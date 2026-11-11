//! Local-First AI Agent - Tauri Desktop Host
//!
//! Responsibilities:
//! - Secure frontend-to-runtime IPC
//! - Local filesystem operations
//! - Launch and supervise local child processes
//! - Emit progress, streaming, and tool events to the UI
//! - OS notifications and application lifecycle
//! - Keychain integration for secrets

use std::path::PathBuf;
use tauri::{Manager, State};
use tauri_plugin_log::{Target, TargetKind};
use tracing::info;

#[tauri::command]
fn get_workspace_path(app: tauri::AppHandle) -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".localfirst-ai-agent").join("workspace")
}

#[tauri::command]
fn read_file(app: tauri::AppHandle, path: String) -> Result<String, String> {
    let ws = get_workspace_path(app);
    let full = ws.join(path);
    if !full.starts_with(&ws) {
        return Err("Path is outside workspace".to_string());
    }
    tracing::info!(path, "Reading file");
    std::fs::read_to_string(&full)
        .map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
fn write_file(app: tauri::AppHandle, path: String, content: String) -> Result<usize, String> {
    let ws = get_workspace_path(app);
    let full = ws.join(path);
    if !full.starts_with(&ws) {
        return Err("Path is outside workspace".to_string());
    }
    tracing::info!(path, "Writing file");
    std::fs::write(&full, content)
        .map(|n| n.len())
        .map_err(|e| format!("Failed to write file: {}", e))
}

#[tauri::command]
fn list_dir(app: tauri::AppHandle, path: String) -> Result<Vec<String>, String> {
    let ws = get_workspace_path(app);
    let full = ws.join(path);
    if !full.starts_with(&ws) {
        return Err("Path is outside workspace".to_string());
    }
    std::fs::read_dir(&full)
        .map(|mut entries| {
            entries.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).collect()
        })
        .map_err(|e| format!("Failed to list directory: {}", e))
}

#[tauri::command]
fn execute_shell(app: tauri::AppHandle, command: String, args: Vec<String>) -> Result<String, String> {
    use std::process::Command;
    info!(command, args, "Executing shell command");
    let mut cmd = Command::new(&command);
    cmd.args(&args);
    let output = cmd.output();
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            Ok(format!("exit_code={}\nstdout={}\nstderr={}", out.status, stdout, stderr))
        }
        Err(e) => Err(format!("Command failed: {}", e)),
    }
}

#[tauri::command]
fn cancel_turn() -> Result<(), String> {
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stdout)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .targets([Target::new(TargetKind::Stdout), Target::new(TargetKind::LogDir)])
            .build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_workspace_path, read_file, write_file, list_dir, execute_shell, cancel_turn,
        ])
        .setup(|app| {
            info!(app_name = app.package_info().name, "Local-First AI Agent starting");
            Ok(())
        });

    app.run(tauri::generate_context!());
}
