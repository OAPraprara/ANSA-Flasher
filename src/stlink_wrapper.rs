use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;

use tauri::AppHandle;

use crate::progress_bridge::process_output_line;

const FLASH_ADDRESS: &str = "0x08000000";

fn get_stm32_programmer_cli_path() -> PathBuf {
    // Development path: use the installed STM32CubeProgrammer CLI
    // because the currently bundled Windows executable is missing
    // required DLL dependencies.
    let installed_path = PathBuf::from(
        r"C:\Program Files\STMicroelectronics\STM32Cube\STM32CubeProgrammer\bin\STM32_Programmer_CLI.exe"
    );

    if installed_path.exists() {
        return installed_path;
    }

    // Production fallback: bundled executable.
    let mut bundled_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // src-tauri -> project root
    bundled_path.pop();

    bundled_path.push("binaries");
    bundled_path.push("STM32_Programmer_CLI.exe");

    bundled_path
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StLinkFlashResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub fn flash_firmware(
    app_handle: &AppHandle,
    firmware_path: &str,
) -> Result<StLinkFlashResult, String> {
    let cli_path = get_stm32_programmer_cli_path();

    if !cli_path.exists() {
        return Err(format!(
            "STM32_Programmer_CLI.exe not found: {}",
            cli_path.display()
        ));
    }

    let firmware = PathBuf::from(firmware_path);

    if !firmware.exists() {
        return Err(format!(
            "Firmware file not found: {}",
            firmware.display()
        ));
    }

    println!("Starting STM32 ST-Link flashing...");
    println!("CLI: {}", cli_path.display());
    println!("Firmware: {}", firmware.display());
    println!("Flash address: {}", FLASH_ADDRESS);

    let mut child = Command::new(&cli_path)
        .args([
            "-c",
            "port=SWD",
            "-w",
            firmware_path,
            FLASH_ADDRESS,
            "-v",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            format!(
                "Failed to start STM32_Programmer_CLI: {}",
                error
            )
        })?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Failed to capture STM32 CLI stdout".to_string())?;

    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "Failed to capture STM32 CLI stderr".to_string())?;

    let stdout_thread = {
        let app_handle = app_handle.clone();

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut captured = String::new();

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("ST-Link: {}", line);

                        captured.push_str(&line);
                        captured.push('\n');

                        if let Err(error) =
                            process_output_line(&app_handle, "stlink", &line)
                        {
                            eprintln!(
                                "Progress event error: {}",
                                error
                            );
                        }
                    }

                    Err(error) => {
                        eprintln!(
                            "Failed to read STM32 CLI stdout: {}",
                            error
                        );
                    }
                }
            }

            captured
        })
    };

    let stderr_thread = {
        let app_handle = app_handle.clone();

        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut captured = String::new();

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        eprintln!("ST-Link ERROR: {}", line);

                        captured.push_str(&line);
                        captured.push('\n');

                        if let Err(error) =
                            process_output_line(&app_handle, "stlink", &line)
                        {
                            eprintln!(
                                "Progress event error: {}",
                                error
                            );
                        }
                    }

                    Err(error) => {
                        eprintln!(
                            "Failed to read STM32 CLI stderr: {}",
                            error
                        );
                    }
                }
            }

            captured
        })
    };

    let status = child
        .wait()
        .map_err(|error| {
            format!(
                "Failed while waiting for STM32_Programmer_CLI: {}",
                error
            )
        })?;

    let stdout = stdout_thread
        .join()
        .map_err(|_| "STM32 CLI stdout thread failed".to_string())?;

    let stderr = stderr_thread
        .join()
        .map_err(|_| "STM32 CLI stderr thread failed".to_string())?;

    let result = StLinkFlashResult {
        success: status.success(),
        exit_code: status.code(),
        stdout,
        stderr,
    };

    if result.success {
        println!("ST-Link flashing completed successfully.");
    } else {
        println!(
            "ST-Link flashing failed. Exit code: {:?}",
            result.exit_code
        );
    }

    Ok(result)
}