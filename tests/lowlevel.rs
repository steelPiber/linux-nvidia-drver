use std::process::Command;

#[cfg(test)]
mod tests {

    #[test]
    fn test_switch_to_cli_and_check_daemon() {
        // Check if we're in a GUI session
        let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
        if session_type == "x11" || session_type == "wayland" {
            // Switch to CLI (example: multi-user.target)
            let _ = Command::new("systemctl")
                .args(&["isolate", "multi-user.target"])
                .status();
        }

        // Check daemon process status (replace "example-daemon" appropriately)
        let daemon_status = Command::new("systemctl")
            .args(&["is-active", "test-daemon"])
            .output()
            .expect("Failed to check daemon status");

        // Validate daemon is active
        assert!(
            String::from_utf8_lossy(&daemon_status.stdout).contains("active"),
            "Daemon is not active"
        );
    }
}