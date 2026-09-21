use sha2::{Digest, Sha256};
use std::fs;

pub fn calculate_sha256(file_path: &str) -> Result<String, String> {
    let file_data = fs::read(file_path)
        .map_err(|error| format!("Failed to read firmware file: {}", error))?;

    let mut hasher = Sha256::new();
    hasher.update(&file_data);

    let hash = hasher.finalize();

    Ok(format!("{:x}", hash))
}

pub fn verify_sha256(
    file_path: &str,
    expected_hash: &str,
) -> Result<bool, String> {
    let calculated_hash = calculate_sha256(file_path)?;

    Ok(calculated_hash.eq_ignore_ascii_case(expected_hash))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_FIRMWARE: &[u8] = b"ANSA test firmware";

    fn create_test_firmware(path: &str) {
        fs::write(path, TEST_FIRMWARE)
            .expect("Failed to create test firmware");
    }

    fn expected_hash() -> String {
        let mut hasher = Sha256::new();
        hasher.update(TEST_FIRMWARE);
        format!("{:x}", hasher.finalize())
    }

    #[test]
    fn test_sha256_calculation() {
        let path = "test_firmware_calculation.bin";
        create_test_firmware(path);

        let hash = calculate_sha256(path)
            .expect("Failed to calculate SHA-256");

        assert_eq!(hash, expected_hash());

        fs::remove_file(path)
            .expect("Failed to remove test firmware");
    }

    #[test]
    fn test_sha256_verification_success() {
        let path = "test_firmware_success.bin";
        create_test_firmware(path);

        let hash = expected_hash();

        let result = verify_sha256(path, &hash)
            .expect("SHA-256 verification failed");

        assert!(result);

        fs::remove_file(path)
            .expect("Failed to remove test firmware");
    }

    #[test]
    fn test_sha256_verification_failure() {
        let path = "test_firmware_failure.bin";
        create_test_firmware(path);

        let result = verify_sha256(
            path,
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .expect("SHA-256 verification failed");

        assert!(!result);

        fs::remove_file(path)
            .expect("Failed to remove test firmware");
    }
}