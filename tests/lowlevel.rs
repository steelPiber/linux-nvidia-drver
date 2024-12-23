use std::process::Command;

fn main() {
    // Switch to CLI environment
    Command::new("sudo")
        .arg("systemctl")
        .arg("isolate")
        .arg("multi-user.target")
        .spawn()
        .expect("Failed to switch to CLI environment");
        // Check the loaded modules
        let output = Command::new("lsmod")
            .output()
            .expect("Failed to run module check command");

        pinrltn!("Module inspection:\n{}", String::from_utf8_lossy(&output.stdout));
    // Switch back to GUI environment
    Command::new("sudo")
        .arg("systemctl")
        .arg("isolate")
        .arg("graphical.target")
        .spawn()
        .expect("Failed to switch back to GUI environment");
}