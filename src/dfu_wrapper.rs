use std::path::PathBuf;
use std::process::Command;

const STM32_DFU_VID: &str = "0483";
const STM32_DFU_PID: &str = "DF11";

fn get_dfu_util_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // src-tauri -> project root
    path.pop();

    path.push("binaries");
    path.push("dfu-util-windows.exe");

    path
}

pub fn flash_firmware(firmware_path: &str) -> Result<String, String> {
    let dfu_util = get_dfu_util_path();

    if !dfu_util.exists() {
        return Err(format!(
            "dfu-util executable not found: {}",
            dfu_util.display()
        ));
    }

    if !PathBuf::from(firmware_path).exists() {
        return Err(format!(
            "Firmware file not found: {}",
            firmware_path
        ));
    }

    println!("Starting STM32 DFU flashing...");
    println!("dfu-util: {}", dfu_util.display());
    println!("Firmware: {}", firmware_path);

    let output = Command::new(&dfu_util)
        .args([
            "-d",
            &format!("{}:{}", STM32_DFU_VID, STM32_DFU_PID),
            "-a",
            "0",
            "-D",
            firmware_path,
        ])
        .output()
        .map_err(|error| {
            format!("Failed to start dfu-util: {}", error)
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        Ok(format!(
            "DFU flashing completed successfully.\n{}{}",
            stdout, stderr
        ))
    } else {
        Err(format!(
            "DFU flashing failed.\n{}{}",
            stdout, stderr
        ))
    }
}