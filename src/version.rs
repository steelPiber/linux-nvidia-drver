use std::process::Command;

pub fn get_nvidia_driver_version() -> Result<String, String> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=driver_version")
        .arg("--format=csv,noheader")
        .output()
        .map_err(|e| format!("Failed to execute nvidia-smi: {}", e))?;

    if output.status.success() {
        let version = String::from_utf8(output.stdout)
            .map_err(|e| format!("Failed to parse output: {}", e))?;
        Ok(version.trim().to_string())
    } else {
        Err(format!(
            "nvidia-smi failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}