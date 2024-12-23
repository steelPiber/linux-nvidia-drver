use std::process::Command;


fn get_nvidia_driver_version() -> Result<String, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nvidia_driver_version() {
        match get_nvidia_driver_version() {
            Ok(version) => {
                assert!(!version.is_empty());
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }
}