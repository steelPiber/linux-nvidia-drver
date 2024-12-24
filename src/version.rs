use std::process::Command;

// NVIDIA 드라이버 버전을 가져오는 함수
// map_err :
pub fn get_nvidia_driver_version() -> Result<String, String> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=driver_version") // 드라이버 버전만 출력
        .arg("--format=csv,noheader")// CSV 형식으로 출력
        .output()
        .map_err(|e| format!("Failed to execute nvidia-smi: {}", e))?; // 에러 처리

    // 성공적으로 실행되었을 때
    if output.status.success() {
        let version = String::from_utf8(output.stdout)
            .map_err(|e| format!("Failed to parse output: {}", e))?;
        Ok(version.trim().to_string())
    } else { // 실패했을 때
        Err(format!(
            "nvidia-smi failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}