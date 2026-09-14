/**
 * Serializes custom configuration parameters into a 100-byte binary blob
 * for the Serialization Handshake with the STM32 device.
 *
 * Binary Structure (100 bytes total):
 * - Bytes 0-3  (4 bytes) : droneId (32-bit unsigned integer, Little-Endian)
 * - Bytes 4-35 (32 bytes): ssid (UTF-8 string, zero-padded)
 * - Bytes 36-99 (64 bytes): password (UTF-8 string, zero-padded)
 *
 * @param droneId A numeric identifier for the drone
 * @param ssid The WiFi network name
 * @param password The WiFi network password
 * @returns A 100-byte Uint8Array containing the serialized configuration
 */
export function serializeConfig(droneId: number, ssid: string, password: string): Uint8Array {
  // Allocate exactly 100 bytes, automatically zero-initialized
  const buffer = new ArrayBuffer(100);
  const view = new DataView(buffer);
  const uint8Array = new Uint8Array(buffer);
  const encoder = new TextEncoder();

  // 1. Write droneId as a 32-bit unsigned integer (Little-Endian)
  view.setUint32(0, droneId, true);

  // 2. Encode and write the SSID (max 32 bytes)
  const ssidBytes = encoder.encode(ssid);
  const ssidLength = Math.min(ssidBytes.length, 32);
  uint8Array.set(ssidBytes.subarray(0, ssidLength), 4);

  // 3. Encode and write the password (max 64 bytes)
  const passwordBytes = encoder.encode(password);
  const passwordLength = Math.min(passwordBytes.length, 64);
  uint8Array.set(passwordBytes.subarray(0, passwordLength), 36);

  return uint8Array;
}
