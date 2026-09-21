use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub boards: Vec<Board>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub mcu: String,
    pub transport: String,
    pub versions: Vec<FirmwareVersion>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirmwareVersion {
    pub version: String,
    pub channel: String,
    pub url: String,
    pub sha256: String,
    pub signature_url: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FirmwareSelection {
    pub board_id: String,
    pub board_name: String,
    pub mcu: String,
    pub transport: String,
    pub version: String,
    pub channel: String,
    pub firmware_url: String,
    pub signature_url: String,
    pub sha256: String,
    pub size_bytes: u64,
}

pub fn load_manifest(manifest_path: &str) -> Result<Manifest, String> {
    let manifest_data = fs::read_to_string(manifest_path)
        .map_err(|error| format!("Failed to read manifest: {}", error))?;

    serde_json::from_str(&manifest_data)
        .map_err(|error| format!("Failed to parse manifest JSON: {}", error))
}

pub fn select_firmware(
    manifest_path: &str,
    board_id: &str,
    version: &str,
    channel: &str,
) -> Result<FirmwareSelection, String> {
    let manifest = load_manifest(manifest_path)?;

    let board = manifest
        .boards
        .iter()
        .find(|board| board.id == board_id)
        .ok_or_else(|| {
            format!("Board not found in manifest: {}", board_id)
        })?;

    let firmware = board
        .versions
        .iter()
        .find(|firmware| {
            firmware.version == version
                && firmware.channel == channel
        })
        .ok_or_else(|| {
            format!(
                "Firmware version/channel not found: {} / {}",
                version, channel
            )
        })?;

    Ok(FirmwareSelection {
        board_id: board.id.clone(),
        board_name: board.name.clone(),
        mcu: board.mcu.clone(),
        transport: board.transport.clone(),
        version: firmware.version.clone(),
        channel: firmware.channel.clone(),
        firmware_url: firmware.url.clone(),
        signature_url: firmware.signature_url.clone(),
        sha256: firmware.sha256.clone(),
        size_bytes: firmware.size_bytes,
    })
}

pub async fn download_firmware(
    url: &str,
    output_path: &str,
) -> Result<String, String> {
    download_file(url, output_path, "Firmware").await
}

pub async fn download_signature(
    url: &str,
    output_path: &str,
) -> Result<String, String> {
    download_file(url, output_path, "Signature").await
}

async fn download_file(
    url: &str,
    output_path: &str,
    file_type: &str,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| {
            format!(
                "{} download request failed: {}",
                file_type, error
            )
        })?;

    if !response.status().is_success() {
        return Err(format!(
            "{} download failed with HTTP status: {}",
            file_type,
            response.status()
        ));
    }

    let file_data = response
        .bytes()
        .await
        .map_err(|error| {
            format!(
                "Failed to read {} data: {}",
                file_type, error
            )
        })?;

    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|error| {
                format!(
                    "Failed to create output directory: {}",
                    error
                )
            })?;
    }

    fs::write(output_path, &file_data)
        .map_err(|error| {
            format!(
                "Failed to save {}: {}",
                file_type, error
            )
        })?;

    Ok(output_path.to_string())
}

pub fn validate_firmware_size(
    file_path: &str,
    expected_size: u64,
) -> Result<bool, String> {
    let metadata = fs::metadata(file_path)
        .map_err(|error| {
            format!(
                "Failed to read firmware metadata: {}",
                error
            )
        })?;

    let actual_size = metadata.len();

    Ok(actual_size == expected_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_manifest(filename: &str) -> String {
        let manifest = r#"
        {
            "boards": [
                {
                    "id": "ansa-f446re",
                    "name": "ANSA OS Nucleo-F446RE",
                    "mcu": "STM32F446RE",
                    "transport": "swd",
                    "versions": [
                        {
                            "version": "1.2.0",
                            "channel": "stable",
                            "url": "https://ansa.dev/firmware/f446re/ansa-1.2.0.bin",
                            "sha256": "a1b2c3",
                            "signature_url": "https://ansa.dev/firmware/f446re/ansa-1.2.0.bin.sig",
                            "size_bytes": 131072
                        }
                    ]
                }
            ]
        }
        "#;

        fs::write(filename, manifest)
            .expect("Failed to create test manifest");

        filename.to_string()
    }

    #[test]
    fn selects_valid_firmware() {
        let manifest_path =
            create_test_manifest("test_manifest_valid.json");

        let result = select_firmware(
            &manifest_path,
            "ansa-f446re",
            "1.2.0",
            "stable",
        );

        assert!(result.is_ok());

        let firmware = result.unwrap();

        assert_eq!(firmware.board_id, "ansa-f446re");
        assert_eq!(firmware.version, "1.2.0");
        assert_eq!(firmware.channel, "stable");
        assert_eq!(firmware.mcu, "STM32F446RE");
        assert_eq!(firmware.transport, "swd");
        assert_eq!(firmware.size_bytes, 131072);
        assert_eq!(
            firmware.signature_url,
            "https://ansa.dev/firmware/f446re/ansa-1.2.0.bin.sig"
        );

        fs::remove_file(manifest_path)
            .expect("Failed to remove test manifest");
    }

    #[test]
    fn rejects_unknown_board() {
        let manifest_path =
            create_test_manifest("test_manifest_unknown_board.json");

        let result = select_firmware(
            &manifest_path,
            "unknown-board",
            "1.2.0",
            "stable",
        );

        assert!(result.is_err());

        fs::remove_file(manifest_path)
            .expect("Failed to remove test manifest");
    }

    #[test]
    fn rejects_unknown_version_or_channel() {
        let manifest_path =
            create_test_manifest("test_manifest_unknown_version.json");

        let result = select_firmware(
            &manifest_path,
            "ansa-f446re",
            "9.9.9",
            "stable",
        );

        assert!(result.is_err());

        let result = select_firmware(
            &manifest_path,
            "ansa-f446re",
            "1.2.0",
            "beta",
        );

        assert!(result.is_err());

        fs::remove_file(manifest_path)
            .expect("Failed to remove test manifest");
    }

    #[test]
    fn validates_firmware_size_success() {
        let path = "test_firmware_size.bin";

        fs::write(path, vec![0u8; 100])
            .expect("Failed to create test firmware");

        let result = validate_firmware_size(path, 100)
            .expect("Size validation failed");

        assert!(result);

        fs::remove_file(path)
            .expect("Failed to remove test firmware");
    }

    #[test]
    fn validates_firmware_size_failure() {
        let path = "test_firmware_size_invalid.bin";

        fs::write(path, vec![0u8; 100])
            .expect("Failed to create test firmware");

        let result = validate_firmware_size(path, 200)
            .expect("Size validation failed");

        assert!(!result);

        fs::remove_file(path)
            .expect("Failed to remove test firmware");
    }
}