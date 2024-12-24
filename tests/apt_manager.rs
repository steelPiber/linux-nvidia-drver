use std::process::Command;
use std::error::Error;

pub fn install_nvidia_packages() -> Result<(), Box<dyn Error>> {
    let kernel_version = String::from_utf8(Command::new("uname").arg("-r").output()?.stdout)?;
    println!("Detected kernel: {}", kernel_version.trim());

    Command::new("sudo")
        .args(&["apt-get", "update"])
        .status()?;


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_nvidia_packages() {
        match install_nvidia_packages() {
            Ok(_) => {
                assert!(true);
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }
}