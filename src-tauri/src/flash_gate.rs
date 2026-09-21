use crate::{crypto, downloader, verifier};

pub fn verify_before_flash(
    firmware_path: &str,
    signature_path: &str,
    expected_sha256: &str,
    expected_size: u64,
) -> Result<(), String> {
    // 1. Verify firmware size
    let size_valid =
        downloader::validate_firmware_size(firmware_path, expected_size)?;

    if !size_valid {
        return Err("Firmware size verification failed".to_string());
    }

    // 2. Verify SHA-256
    let hash_valid =
        verifier::verify_sha256(firmware_path, expected_sha256)?;

    if !hash_valid {
        return Err("Firmware SHA-256 verification failed".to_string());
    }

    // 3. Verify digital signature
    let signature_valid =
        crypto::verify_signature(firmware_path, signature_path)?;

    if !signature_valid {
        return Err("Firmware signature verification failed".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::fs;

    #[test]
    fn rejects_wrong_firmware_hash() {
        let firmware_path = "test_gate_firmware.bin";
        let signature_path = "test_gate_signature.sig";

        let firmware = b"ANSA test firmware";

        fs::write(firmware_path, firmware)
            .expect("Failed to create test firmware");

        // Signature file only needs to exist because
        // SHA-256 verification should fail first.
        fs::write(signature_path, "")
            .expect("Failed to create test signature");

        let result = verify_before_flash(
            firmware_path,
            signature_path,
            "0000000000000000000000000000000000000000000000000000000000000000",
            firmware.len() as u64,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Firmware SHA-256 verification failed"
        );

        fs::remove_file(firmware_path)
            .expect("Failed to remove test firmware");

        fs::remove_file(signature_path)
            .expect("Failed to remove test signature");
    }

    #[test]
    fn rejects_wrong_firmware_size() {
        let firmware_path = "test_gate_size.bin";
        let signature_path = "test_gate_size.sig";

        let firmware = b"ANSA test firmware";

        fs::write(firmware_path, firmware)
            .expect("Failed to create test firmware");

        fs::write(signature_path, "")
            .expect("Failed to create test signature");

        let mut hasher = Sha256::new();
        hasher.update(firmware);
        let hash = format!("{:x}", hasher.finalize());

        let result = verify_before_flash(
            firmware_path,
            signature_path,
            &hash,
            9999,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Firmware size verification failed"
        );

        fs::remove_file(firmware_path)
            .expect("Failed to remove test firmware");

        fs::remove_file(signature_path)
            .expect("Failed to remove test signature");
    }
}