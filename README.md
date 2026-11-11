# LocalfirstAI — Local-First AI Agent

A lightweight desktop AI agent that keeps conversations, session state, memory, and artifacts on the user's computer; executes tools through the local OS; and uses OpenRouter as the primary LLM gateway with automatic free-model routing and quota-aware fallback.

## Architecture Overview

```
React + TypeScript Desktop UI (apps/desktop/src)        Tauri Host: Rust (apps/desktop/src-tauri/src)
- Chat compositor, streaming renderer, tool cards       - Secure frontend-to-runtime IPC
- Model routing status, cancellation/retry controls     - Local filesystem ops
- Settings: API key, workspace, safety controls         - Launch and supervise child processes
                                                         - OS notifications, lifecycle
                                                         - OS keychain for secrets

Agent Runtime (crates/agent-runtime)                    Model Router (crates/model-router)
 - Agent turn loop                                       - OpenRouter free-model routing
 - Canonical provider-independent conversation history   - Free model registry + capability metadata
 - Persist state before/after critical actions           - Quota-aware retry and failover

Context Engine (crates/context-engine)                  Tool Policy (crates/tool-policy)
 - Canonical message format                              - Tool approval policy
 - Context management, compaction                        - Destructive/out-of-workspace approvals

Local Tool Executor (crates/local-executor)             Local Store (crates/local-store)
 - shell.run, filesystem.read/write/list/search          - JSONL + JSON + Markdown persistence
 - Git status/diff/log tools                             - Session/task state, memory files

Provider: OpenRouter (crates/provider-openrouter)     UI (apps/desktop/src)
 - OpenRouter API client                                 - Chat UI for humans
```

## Repository Structure

```
LocalfirstAI/
  apps/desktop/                     # React + Tauri desktop app
  crates/                           # 7 Rust crates
    agent-runtime/                  # Turn loop + orchestration
    model-router/                   # Free model routing + failover
    context-engine/                 # Canonical messages
    tool-policy/                    # Approval policy
    local-executor/                 # Shell + filesystem tools
    local-store/                    # JSONL/JSON persistence
    provider-openrouter/            # OpenRouter HTTP client
  docs/                             # BRD, architecture
  tests/                            # unit, integration, e2e
```
