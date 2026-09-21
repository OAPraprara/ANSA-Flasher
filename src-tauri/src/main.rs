mod usb_monitor;
mod downloader;
mod verifier;
mod dfu_wrapper;
mod stlink_wrapper;
mod progress_parser;
mod progress_bridge;
mod crypto;
mod flash_gate;

use std::sync::Mutex;

struct AppState {
    last_device_count: Mutex<usize>,
}

#[tauri::command]
fn get_usb_devices() -> Result<Vec<usb_monitor::UsbDeviceInfo>, String> {
    usb_monitor::enumerate_usb_devices()
}

#[tauri::command]
async fn download_firmware(
    url: String,
    output_path: String,
) -> Result<String, String> {
    downloader::download_firmware(&url, &output_path).await
}

#[tauri::command]
async fn download_signature(
    url: String,
    output_path: String,
) -> Result<String, String> {
    downloader::download_signature(&url, &output_path).await
}

#[tauri::command]
fn select_firmware(
    manifest_path: String,
    board_id: String,
    version: String,
    channel: String,
) -> Result<downloader::FirmwareSelection, String> {
    downloader::select_firmware(
        &manifest_path,
        &board_id,
        &version,
        &channel,
    )
}

#[tauri::command]
fn verify_firmware_sha256(
    file_path: String,
    expected_hash: String,
) -> Result<bool, String> {
    verifier::verify_sha256(&file_path, &expected_hash)
}

#[tauri::command]
fn validate_firmware_size(
    file_path: String,
    expected_size: u64,
) -> Result<bool, String> {
    downloader::validate_firmware_size(
        &file_path,
        expected_size,
    )
}

#[tauri::command]
fn verify_firmware_signature(
    firmware_path: String,
    signature_path: String,
) -> Result<bool, String> {
    crypto::verify_signature(
        &firmware_path,
        &signature_path,
    )
}

// DFU flashing command with live progress support
#[tauri::command]
fn flash_firmware_dfu(
    app_handle: tauri::AppHandle,
    firmware_path: String,
) -> Result<dfu_wrapper::DfuFlashResult, String> {
    dfu_wrapper::flash_firmware(
        &app_handle,
        &firmware_path,
    )
}

// ST-Link flashing command with live progress support
#[tauri::command]
fn flash_firmware_stlink(
    app_handle: tauri::AppHandle,
    firmware_path: String,
) -> Result<stlink_wrapper::StLinkFlashResult, String> {
    stlink_wrapper::flash_firmware(
        &app_handle,
        &firmware_path,
    )
}

// Verified DFU flashing command
#[tauri::command]
fn flash_firmware_dfu_verified(
    app_handle: tauri::AppHandle,
    firmware_path: String,
    signature_path: String,
    expected_sha256: String,
    expected_size: u64,
) -> Result<dfu_wrapper::DfuFlashResult, String> {

    // Verification MUST pass before flashing
    flash_gate::verify_before_flash(
        &firmware_path,
        &signature_path,
        &expected_sha256,
        expected_size,
    )?;

    // Only reached if all verification checks pass
    dfu_wrapper::flash_firmware(
        &app_handle,
        &firmware_path,
    )
}

// Verified ST-Link flashing command
#[tauri::command]
fn flash_firmware_stlink_verified(
    app_handle: tauri::AppHandle,
    firmware_path: String,
    signature_path: String,
    expected_sha256: String,
    expected_size: u64,
) -> Result<stlink_wrapper::StLinkFlashResult, String> {

    // Verification MUST pass before flashing
    flash_gate::verify_before_flash(
        &firmware_path,
        &signature_path,
        &expected_sha256,
        expected_size,
    )?;

    // Only reached if all verification checks pass
    stlink_wrapper::flash_firmware(
        &app_handle,
        &firmware_path,
    )
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            last_device_count: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            get_usb_devices,
            download_firmware,
            download_signature,
            select_firmware,
            verify_firmware_sha256,
            validate_firmware_size,
            verify_firmware_signature,
            flash_firmware_dfu,
            flash_firmware_stlink,
            flash_firmware_dfu_verified,
            flash_firmware_stlink_verified
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            usb_monitor::start_usb_monitor(app_handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}