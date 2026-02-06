# Claude Code Session Monitor - Design Document

**Date**: 2026-02-06
**Status**: Draft
**Author**: Vincent Lee + Claude

## Overview

A macOS menu bar application that detects and displays all active Claude Code sessions on the machine, showing real-time status and providing full control over each session.

## Goals

- **Monitor own work**: See all Claude Code sessions at a glance to switch between them
- **Background task oversight**: Monitor long-running agent tasks without switching terminal windows
- **Full control**: View conversations, send prompts, and stop sessions from one place

## Non-Goals

- Team coordination / multi-user monitoring
- Resource usage tracking (CPU, memory, API costs)
- Session recording or analytics

---

## Architecture

### High-Level Components

```
┌─────────────────────────────────────────────────────────┐
│                    Menu Bar Icon                         │
│              (shows active session count)                │
└─────────────────┬───────────────────────────────────────┘
                  │ click
                  ▼
┌─────────────────────────────────────────────────────────┐
│                   Popover View                           │
│  ┌───────────────────────────────────────────────────┐  │
│  │ 🟢 my-project (main) - 2m ago                     │  │
│  │    "Add authentication to API"                    │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ 🟠 cli-tool (feature/args) - 15m ago              │  │
│  │    "Running tests in background"                  │  │
│  └───────────────────────────────────────────────────┘  │
│                    [Open Full View]                      │
└─────────────────────────────────────────────────────────┘
```

### Technology Stack

| Layer | Technology |
|-------|------------|
| Backend | Rust (Tauri) |
| Frontend | Svelte + TypeScript |
| Build | Vite |
| Platform | macOS (menu bar + windowed) |

### Component Responsibilities

- **Rust backend**: Process detection, session file parsing, polling loop, system actions (SIGINT, open apps)
- **Svelte frontend**: UI rendering, user interactions, state management
- **Tauri IPC**: Bridge between Rust and Svelte via commands and events

---

## User Interface

### Menu Bar Popover

Compact view showing active sessions:

- Session project name and git branch
- Status indicator icon
- Time since last activity
- First prompt summary (truncated)
- Button to open full window view

### Full Window View

Three-panel layout for detailed session management:

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  Claude Code Monitor                                            [—] [□] [✕]  │
├─────────────────────┬────────────────────────────────────────────────────────┤
│  SESSIONS           │  my-project (main)                                     │
│                     │  /Users/you/code/my-project                            │
│  🟢 my-project      │  ─────────────────────────────────────────────────────│
│     main · 2m       │                                                        │
│                     │  USER                                      10:32 AM    │
│  🟠 cli-tool        │  Add authentication to the API endpoints              │
│     feature · 15m   │                                                        │
│                     │  CLAUDE                                    10:33 AM    │
│                     │  I'll add JWT authentication. Let me first read the   │
│                     │  existing route handlers...                            │
│                     │                                                        │
│                     │  [Read] src/routes/api.ts                              │
│                     │  [Edit] src/middleware/auth.ts                         │
│                     │                                                        │
├─────────────────────┼────────────────────────────────────────────────────────┤
│ [⏹ Stop] [↗ Open]  │  [Send prompt to session...]                     [Send]│
└─────────────────────┴────────────────────────────────────────────────────────┘
```

**Left panel**: Session list with status indicators
**Right panel**: Conversation history with messages and tool calls
**Bottom bar**: Action buttons and prompt input

### Menu Bar Icon States

| State | Icon |
|-------|------|
| No sessions running | Gray dot |
| All sessions waiting for input | Green dot |
| Any session needs permission | Orange dot |
| Any session working | Blue dot with pulse |

---

## Session Detection

### Data Sources

Sessions are detected by combining process information with Claude Code's file structure:

| Source | Location | Purpose |
|--------|----------|---------|
| Process list | `pgrep -f claude` | Find running Claude processes |
| Session index | `~/.claude/projects/*/sessions-index.json` | Session metadata |
| Conversation | `~/.claude/projects/*/<sessionId>.jsonl` | Message history |
| Debug logs | `~/.claude/debug/<sessionId>.txt` | Lifecycle events |

### Session Data Model

```rust
struct Session {
    id: String,                    // UUID
    pid: u32,                      // Process ID
    project_name: String,          // Directory name
    project_path: PathBuf,         // Full working directory
    git_branch: Option<String>,
    first_prompt: String,          // Summary shown in list
    message_count: u32,
    modified: DateTime<Utc>,       // Last activity time
    status: SessionStatus,
}

enum SessionStatus {
    Working,           // 🔵 Executing tools/thinking
    NeedsPermission,   // 🟠 Waiting for user approval
    WaitingForInput,   // 🟢 Idle, ready for prompt
    Connecting,        // ⚪ Session starting up
}
```

### Status Detection Logic

Only sessions with active `claude` processes are shown.

```rust
fn detect_status(session: &Session) -> SessionStatus {
    // 1. Find claude process with matching working directory
    let process = find_process_for_session(session);
    if process.is_none() {
        return None; // Don't show - session closed
    }

    // 2. Parse last few entries of session JSONL
    let recent_entries = tail_jsonl(&session.jsonl_path, 10);

    // 3. Determine status from last entry
    match recent_entries.last() {
        ToolUse { completed: false } => SessionStatus::NeedsPermission,
        ToolUse { completed: true } => SessionStatus::Working,
        AssistantMessage => SessionStatus::WaitingForInput,
        _ => SessionStatus::Connecting,
    }
}
```

### Process Matching

Match Claude processes to sessions by comparing working directories:

```rust
fn find_process_for_session(session: &Session) -> Option<Process> {
    let claude_processes = get_processes_by_name("claude");

    for process in claude_processes {
        let cwd = get_process_cwd(process.pid)?;
        if cwd == session.project_path {
            return Some(process);
        }
    }
    None
}
```

---

## Actions

### Open Session

Focus the terminal or IDE window running the session:

```rust
fn open_session_window(pid: u32) -> Result<()> {
    let parent = get_parent_process(pid);

    match detect_app(parent) {
        "Terminal" => run("open -a Terminal"),
        "iTerm2" => run("osascript -e 'tell app \"iTerm\" to activate'"),
        "Code" => run("code --goto {project_path}"),
        "Zed" => run("zed {project_path}"),
        _ => run("open -a Terminal"),
    }
}
```

### Stop Session

Send interrupt signal to stop Claude:

```rust
fn stop_session(pid: u32) -> Result<()> {
    // Send SIGINT (same as Ctrl+C)
    kill(Pid::from_raw(pid as i32), Signal::SIGINT)?;
    Ok(())
}
```

### Send Prompt

Pipe prompt to Claude CLI:

```rust
fn send_prompt(session_id: &str, prompt: &str) -> Result<()> {
    let mut child = Command::new("claude")
        .args(["--continue", "--session-id", session_id])
        .stdin(Stdio::piped())
        .spawn()?;

    child.stdin.take().unwrap().write_all(prompt.as_bytes())?;
    Ok(())
}
```

This spawns a new Claude process that continues the existing session. The response appears in the original terminal and in the updated JSONL file.

---

## Polling & State Management

### Backend Polling Loop

```rust
pub fn start_polling(app_handle: AppHandle) {
    std::thread::spawn(move || {
        loop {
            // 1. Detect running Claude processes
            let processes = find_claude_processes();

            // 2. Match to session files
            let sessions: Vec<Session> = processes
                .iter()
                .filter_map(|p| match_session_to_process(p))
                .collect();

            // 3. Emit update to frontend
            app_handle.emit_all("sessions-updated", &sessions).ok();

            // 4. Sleep 2-3 seconds
            std::thread::sleep(Duration::from_secs(2));
        }
    });
}
```

### Frontend State Store

```typescript
// stores/sessions.ts
import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

export const sessions = writable<Session[]>([]);
export const selectedSession = writable<string | null>(null);

// Listen for backend updates
listen('sessions-updated', (event) => {
    sessions.set(event.payload as Session[]);
});
```

---

## Project Structure

```
claude-session-monitor/
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs               # Tauri entry point
│   │   ├── lib.rs                # Expose commands to frontend
│   │   ├── session/
│   │   │   ├── mod.rs
│   │   │   ├── detector.rs       # Find running Claude processes
│   │   │   ├── parser.rs         # Parse session JSONL files
│   │   │   └── status.rs         # Determine session status
│   │   ├── actions/
│   │   │   ├── mod.rs
│   │   │   ├── open.rs           # Focus terminal/IDE window
│   │   │   ├── stop.rs           # Send SIGINT
│   │   │   └── prompt.rs         # Send prompt via CLI
│   │   └── polling.rs            # Background polling loop
│   ├── Cargo.toml
│   └── tauri.conf.json           # Menu bar + window config
│
├── src/                          # Svelte frontend
│   ├── lib/
│   │   ├── components/
│   │   │   ├── SessionList.svelte
│   │   │   ├── SessionItem.svelte
│   │   │   ├── ConversationView.svelte
│   │   │   ├── MessageBubble.svelte
│   │   │   ├── ToolCallBlock.svelte
│   │   │   └── PromptInput.svelte
│   │   ├── stores/
│   │   │   └── sessions.ts       # Svelte store for session state
│   │   └── api.ts                # Tauri invoke wrappers
│   ├── MenuBar.svelte            # Popover view
│   ├── FullWindow.svelte         # Expanded view
│   └── main.ts
│
├── package.json
└── vite.config.ts
```

### Tauri Commands

```rust
#[tauri::command]
fn get_sessions() -> Vec<Session>;

#[tauri::command]
fn get_conversation(session_id: String) -> Conversation;

#[tauri::command]
fn send_prompt(session_id: String, prompt: String) -> Result<()>;

#[tauri::command]
fn stop_session(session_id: String) -> Result<()>;

#[tauri::command]
fn open_session(session_id: String) -> Result<()>;
```

---

## Error Handling

| Scenario | Behavior |
|----------|----------|
| Claude process exits while viewing | Remove from list on next poll, show "Session ended" toast |
| Session JSONL file is very large | Stream last 1000 messages, paginate on scroll up |
| Multiple Claude processes for same project | Show as separate sessions with PID suffix |
| Claude Code not installed | Show "Claude Code not detected" with install link |
| `~/.claude` directory doesn't exist | Show empty state: "No Claude Code sessions found" |
| Permission denied reading session files | Show session with "Unable to read" status |
| Sending prompt fails | Show error toast, keep prompt text for retry |

### Graceful Degradation

- Process detection fails → fall back to file modification times
- App detection fails → default to opening Terminal
- JSONL parsing fails for one session → skip it, show others

---

## Future Considerations

Not in scope for v1, but could be added later:

- **Keyboard shortcuts**: Global hotkey to open/focus monitor
- **Notifications**: Desktop notifications when session needs permission
- **History view**: Browse and resume closed sessions
- **Multi-machine**: Monitor sessions on remote machines via SSH
- **Themes**: Dark/light mode matching system preference

---

## Open Questions

1. **CLI continuation support**: Verify `claude --continue --session-id` works as expected
2. **Permission approval**: Can we approve permissions programmatically, or only via the terminal?
3. **IDE integration**: Should we support VS Code/Zed extensions for tighter integration?
