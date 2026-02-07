use std::io::Write;
use std::process::{Command, Stdio};

/// Open a session by focusing its terminal or IDE window
///
/// This finds the parent application of the Claude process and activates it.
/// Works with Terminal, iTerm2, Zed, VS Code, Cursor, and other applications.
pub fn open_session(pid: u32, project_path: String) -> Result<(), String> {
    // Find the parent application by walking up the process tree
    let app_name = find_parent_app(pid)?;

    // Extract project name from path for window matching
    let project_name = std::path::Path::new(&project_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    eprintln!("[open_session] App: {}, Project: {}, Path: {}", app_name, project_name, project_path);

    // For Zed, use the CLI to open/focus the project window
    if app_name == "Zed" {
        let zed_cli = "/Applications/Zed.app/Contents/MacOS/cli";

        eprintln!("[open_session] Using Zed CLI to open: {}", project_path);

        let output = Command::new(zed_cli)
            .arg(&project_path)
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    eprintln!("[open_session] Zed CLI succeeded");
                    return Ok(());
                } else {
                    let error = String::from_utf8_lossy(&out.stderr);
                    eprintln!("[open_session] Zed CLI error: {}", error);
                }
            }
            Err(e) => {
                eprintln!("[open_session] Failed to run Zed CLI: {}", e);
            }
        }
    }

    // Fallback for other apps or if Zed CLI fails: just activate the app
    let script = format!(r#"tell application "{}" to activate"#, app_name);

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("[open_session] AppleScript error: {}", error);
    }

    Ok(())
}

/// Find the parent GUI application for a given process ID
fn find_parent_app(pid: u32) -> Result<String, String> {
    let mut current_pid = pid;

    eprintln!("[open_session] Starting with PID: {}", pid);

    // Walk up the process tree to find a GUI application
    for i in 0..20 {
        // Get the command/path for current process
        let comm_output = Command::new("ps")
            .arg("-o")
            .arg("comm=")
            .arg("-p")
            .arg(current_pid.to_string())
            .output()
            .map_err(|e| format!("Failed to execute ps: {}", e))?;

        let comm = String::from_utf8_lossy(&comm_output.stdout).trim().to_string();
        eprintln!("[open_session] Step {}: PID {} -> comm: {}", i, current_pid, comm);

        // Check if this is a known GUI application
        if let Some(app_name) = get_app_name(&comm) {
            eprintln!("[open_session] Found app: {}", app_name);
            return Ok(app_name.to_string());
        }

        // Get parent PID
        let ppid_output = Command::new("ps")
            .arg("-o")
            .arg("ppid=")
            .arg("-p")
            .arg(current_pid.to_string())
            .output()
            .map_err(|e| format!("Failed to execute ps: {}", e))?;

        let ppid_str = String::from_utf8_lossy(&ppid_output.stdout).trim().to_string();
        let ppid: u32 = ppid_str.parse().unwrap_or(1);
        eprintln!("[open_session] Parent PID: {}", ppid);

        // Move to parent
        if ppid <= 1 {
            eprintln!("[open_session] Reached root, checking current comm one more time");
            // Check current process one more time before giving up
            if let Some(app_name) = get_app_name(&comm) {
                eprintln!("[open_session] Found app at root: {}", app_name);
                return Ok(app_name.to_string());
            }
            break;
        }
        current_pid = ppid;
    }

    eprintln!("[open_session] Falling back to Terminal");
    // Fallback to Terminal
    Ok("Terminal".to_string())
}

/// Map process command names to application names
fn get_app_name(comm: &str) -> Option<&'static str> {
    let comm_lower = comm.to_lowercase();

    // Check for .app bundle paths first (e.g., /Applications/Zed.app/Contents/MacOS/zed)
    if comm_lower.contains(".app/") || comm_lower.contains(".app") {
        // Extract the app name from the bundle path
        if comm_lower.contains("zed.app") {
            return Some("Zed");
        }
        if comm_lower.contains("visual studio code.app") || comm_lower.contains("code.app") {
            return Some("Visual Studio Code");
        }
        if comm_lower.contains("cursor.app") {
            return Some("Cursor");
        }
        if comm_lower.contains("windsurf.app") {
            return Some("Windsurf");
        }
        if comm_lower.contains("iterm.app") || comm_lower.contains("iterm2.app") {
            return Some("iTerm");
        }
        if comm_lower.contains("terminal.app") {
            return Some("Terminal");
        }
        if comm_lower.contains("alacritty.app") {
            return Some("Alacritty");
        }
        if comm_lower.contains("kitty.app") {
            return Some("kitty");
        }
        if comm_lower.contains("warp.app") {
            return Some("Warp");
        }
        if comm_lower.contains("hyper.app") {
            return Some("Hyper");
        }
        if comm_lower.contains("sublime text.app") {
            return Some("Sublime Text");
        }
    }

    // Extract the base name from the path
    let base_name = comm.rsplit('/').next().unwrap_or(comm);

    match base_name.to_lowercase().as_str() {
        // Terminals
        "terminal" => Some("Terminal"),
        "iterm2" | "iterm" => Some("iTerm"),
        "alacritty" => Some("Alacritty"),
        "kitty" => Some("kitty"),
        "warp" => Some("Warp"),
        "hyper" => Some("Hyper"),

        // IDEs
        "zed" | "zed-editor" => Some("Zed"),
        "code" | "code helper" | "electron" => Some("Visual Studio Code"),
        "cursor" => Some("Cursor"),
        "windsurf" => Some("Windsurf"),

        // Other editors
        "sublime_text" | "subl" => Some("Sublime Text"),
        "atom" => Some("Atom"),

        _ => None,
    }
}

/// Stop a session by sending SIGINT to the process
///
/// This gracefully terminates the Claude process by sending a SIGINT signal,
/// which is equivalent to pressing Ctrl+C.
pub fn stop_session(pid: u32) -> Result<(), String> {
    // Use the kill command to send SIGINT (signal 2)
    let output = Command::new("kill")
        .arg("-2") // SIGINT
        .arg(pid.to_string())
        .output()
        .map_err(|e| format!("Failed to execute kill command: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to stop process {}: {}", pid, error));
    }

    Ok(())
}

/// Send a prompt to a session by spawning the Claude CLI
///
/// This function:
/// 1. Spawns `claude` with --continue and --session-id flags
/// 2. Pipes the prompt to stdin
/// 3. Returns immediately (doesn't wait for response)
pub fn send_prompt(session_id: String, prompt: String) -> Result<(), String> {
    // Spawn the claude command with --continue and --session-id
    let mut child = Command::new("claude")
        .arg("--continue")
        .arg("--session-id")
        .arg(&session_id)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to spawn claude command: {}", e))?;

    // Get stdin handle and write the prompt
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(prompt.as_bytes())
            .map_err(|e| format!("Failed to write prompt to stdin: {}", e))?;

        stdin
            .write_all(b"\n")
            .map_err(|e| format!("Failed to write newline to stdin: {}", e))?;
    } else {
        return Err("Failed to open stdin for claude process".to_string());
    }

    // Don't wait for the process to complete - it will run in the background
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_session_invalid_pid() {
        // Try to stop a non-existent process
        let result = stop_session(999999);
        assert!(result.is_err());
    }

    #[test]
    #[ignore] // This test requires manual verification
    fn test_open_session() {
        // Use current process PID for testing
        let result = open_session(std::process::id());
        println!("Result: {:?}", result);
    }

    #[test]
    #[ignore] // This test requires a real session ID and claude CLI
    fn test_send_prompt() {
        let result = send_prompt(
            "test-session-id".to_string(),
            "Hello from the test!".to_string(),
        );
        println!("Result: {:?}", result);
    }
}
