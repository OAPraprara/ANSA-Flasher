use std::path::Path;
use std::fs;

pub async fn download_firmware(
    url: &str,
    output_path: &str,
) -> Result<String, String> {
    // Create HTTP client
    let client = reqwest::Client::new();

    // Send download request
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| format!("Firmware download request failed: {}", error))?;

    // Check HTTP status
    if !response.status().is_success() {
        return Err(format!(
            "Firmware download failed with HTTP status: {}",
            response.status()
        ));
    }

    // Read firmware data
    let firmware_data = response
        .bytes()
        .await
        .map_err(|error| format!("Failed to read firmware data: {}", error))?;

    // Make sure the output directory exists
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create output directory: {}", error))?;
    }

    // Save firmware to disk
    fs::write(output_path, &firmware_data)
        .map_err(|error| format!("Failed to save firmware: {}", error))?;

    Ok(output_path.to_string())
}