mod usb_monitor;
mod downloader;
mod verifier;

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
fn verify_firmware_sha256(
    file_path: String,
    expected_hash: String,
) -> Result<bool, String> {
    verifier::verify_sha256(&file_path, &expected_hash)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            last_device_count: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            get_usb_devices,
            download_firmware,
            verify_firmware_sha256
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            usb_monitor::start_usb_monitor(app_handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}