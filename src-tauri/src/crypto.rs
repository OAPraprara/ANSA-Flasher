use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{
    Signature,
    Signer,
    SigningKey,
    Verifier,
    VerifyingKey,
    pkcs8::DecodePublicKey,
};
use std::fs;

const TRUSTED_PUBLIC_KEY_BASE64: &str =
    "MCowBQYDK2VwAyEA+srI7LYNn8yS39z8xuHlewqNFzaYggrR1ExJWWgGxXQ=";

fn load_trusted_public_key() -> Result<VerifyingKey, String> {
    let der = STANDARD
        .decode(TRUSTED_PUBLIC_KEY_BASE64)
        .map_err(|error| {
            format!("Failed to decode trusted public key: {}", error)
        })?;

    VerifyingKey::from_public_key_der(&der)
        .map_err(|error| {
            format!("Invalid trusted Ed25519 public key: {}", error)
        })
}

pub fn verify_signature(
    firmware_path: &str,
    signature_path: &str,
) -> Result<bool, String> {
    let firmware = fs::read(firmware_path)
        .map_err(|error| {
            format!("Failed to read firmware: {}", error)
        })?;

    let signature_base64 = fs::read_to_string(signature_path)
        .map_err(|error| {
            format!("Failed to read signature: {}", error)
        })?;

    let signature_bytes = STANDARD
        .decode(signature_base64.trim())
        .map_err(|error| {
            format!("Failed to decode firmware signature: {}", error)
        })?;

    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|error| {
            format!("Invalid Ed25519 signature: {}", error)
        })?;

    let public_key = load_trusted_public_key()?;

    Ok(public_key.verify(&firmware, &signature).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trusted_public_key_is_valid() {
        let result = load_trusted_public_key();

        assert!(
            result.is_ok(),
            "Trusted public key should be valid"
        );
    }

    #[test]
    fn rejects_invalid_signature() {
        let firmware_path = "test_crypto_firmware.bin";
        let signature_path = "test_crypto_signature.sig";

        fs::write(firmware_path, b"ANSA test firmware")
            .expect("Failed to create test firmware");

        let invalid_signature =
            STANDARD.encode([0u8; 64]);

        fs::write(signature_path, invalid_signature)
            .expect("Failed to create test signature");

        let result =
            verify_signature(firmware_path, signature_path)
                .expect("Signature verification should execute");

        assert!(!result);

        fs::remove_file(firmware_path)
            .expect("Failed to remove test firmware");

        fs::remove_file(signature_path)
            .expect("Failed to remove test signature");
    }

    #[test]
    fn rejects_modified_firmware() {
        let firmware_path = "test_crypto_modified.bin";
        let signature_path = "test_crypto_modified.sig";

        let signing_key =
            SigningKey::from_bytes(&[7u8; 32]);

        let original_firmware =
            b"ANSA test firmware";

        let signature =
            signing_key.sign(original_firmware);

        fs::write(
            firmware_path,
            b"MODIFIED firmware",
        )
        .expect("Failed to create firmware");

        fs::write(
            signature_path,
            STANDARD.encode(signature.to_bytes()),
        )
        .expect("Failed to create signature");

        /*
         * This signature was created by a test key, not the
         * trusted release key. Therefore verification must fail.
         */
        let result =
            verify_signature(firmware_path, signature_path)
                .expect("Signature verification should execute");

        assert!(!result);

        fs::remove_file(firmware_path)
            .expect("Failed to remove firmware");

        fs::remove_file(signature_path)
            .expect("Failed to remove signature");
    }
}