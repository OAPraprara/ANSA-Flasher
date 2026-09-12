use sha2::{Digest, Sha256};
use std::fs;

pub fn calculate_sha256(file_path: &str) -> Result<String, String> {
    // Read the firmware file
    let file_data = fs::read(file_path)
        .map_err(|error| format!("Failed to read firmware file: {}", error))?;

    // Calculate SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&file_data);

    let hash = hasher.finalize();

    // Convert hash to hexadecimal string
    Ok(format!("{:x}", hash))
}

pub fn verify_sha256(
    file_path: &str,
    expected_hash: &str,
) -> Result<bool, String> {
    let calculated_hash = calculate_sha256(file_path)?;

    if calculated_hash.eq_ignore_ascii_case(expected_hash) {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_calculation() {
        let hash = calculate_sha256("test_firmware.bin")
            .expect("Failed to calculate SHA-256");

        assert_eq!(
            hash,
            "3d417b05b9a28c7f077a017bc2b5f48327e19abc24cbecf5a5264ade7dc87b6f"
        );
    }

    #[test]
    fn test_sha256_verification_success() {
        let result = verify_sha256(
            "test_firmware.bin",
            "3D417B05B9A28C7F077A017BC2B5F48327E19ABC24CBECF5A5264ADE7DC87B6F",
        )
        .expect("SHA-256 verification failed");

        assert!(result);
    }

    #[test]
    fn test_sha256_verification_failure() {
        let result = verify_sha256(
            "test_firmware.bin",
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .expect("SHA-256 verification failed");

        assert!(!result);
    }
}
