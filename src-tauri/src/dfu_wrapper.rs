use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;

use tauri::AppHandle;

use crate::progress_bridge::{emit_progress, process_output_line};
use crate::progress_parser::ProgressEvent;

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

#[derive(Debug, Clone, serde::Serialize)]
pub struct DfuFlashResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub fn flash_firmware(app_handle: &AppHandle, firmware_path: &str) -> Result<DfuFlashResult, String> {
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

    let mut child = Command::new(&dfu_util)
        .args([
            "-d",
            &format!("{}:{}", STM32_DFU_VID, STM32_DFU_PID),
            "-a",
            "0",
            "-s",
            "0x08000000:leave",
            "-D",
            firmware_path,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            format!("Failed to start dfu-util: {}", error)
        })?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Failed to capture dfu-util stdout".to_string())?;

    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "Failed to capture dfu-util stderr".to_string())?;

    let stdout_thread = {
        let app_handle = app_handle.clone();

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut captured = String::new();

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("DFU: {}", line);

                        captured.push_str(&line);
                        captured.push('\n');

                        if let Err(error) =
                            process_output_line(&app_handle, "dfu", &line)
                        {
                            eprintln!(
                                "Progress event error: {}",
                                error
                            );
                        }
                    }

                    Err(error) => {
                        eprintln!(
                            "Failed to read dfu-util stdout: {}",
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
                        eprintln!("DFU ERROR: {}", line);

                        captured.push_str(&line);
                        captured.push('\n');

                        if let Err(error) =
                            process_output_line(&app_handle, "dfu", &line)
                        {
                            eprintln!(
                                "Progress event error: {}",
                                error
                            );
                        }
                    }

                    Err(error) => {
                        eprintln!(
                            "Failed to read dfu-util stderr: {}",
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
                "Failed while waiting for dfu-util: {}",
                error
            )
        })?;

    let stdout = stdout_thread
        .join()
        .map_err(|_| "dfu-util stdout thread failed".to_string())?;

    let stderr = stderr_thread
        .join()
        .map_err(|_| "dfu-util stderr thread failed".to_string())?;

    let success = status.success();

    let result = DfuFlashResult {
        success,
        exit_code: status.code(),
        stdout,
        stderr,
    };

    if success {
        println!("DFU flashing completed successfully.");
        let _ = emit_progress(
            app_handle,
            &ProgressEvent {
                transport: "dfu".to_string(),
                stage: "done".to_string(),
                progress: 100,
                message: "Flashing complete".to_string(),
                done: true,
                success: true,
            },
        );
    } else {
        println!(
            "DFU flashing failed. Exit code: {:?}",
            result.exit_code
        );
        let _ = emit_progress(
            app_handle,
            &ProgressEvent {
                transport: "dfu".to_string(),
                stage: "error".to_string(),
                progress: 100,
                message: "Flashing failed".to_string(),
                done: true,
                success: false,
            },
        );
    }

    Ok(result)
}