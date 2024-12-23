use std::process::Command;
// apt 패키지 설치 함수
fn apt_install(package: &str) {
    let output = Command::new("apt")
        .arg("install")
        .arg(package)
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}