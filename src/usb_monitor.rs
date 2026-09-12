use rusb::UsbContext;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

// ST-Link USB identification
const STLINK_VID: u16 = 0x0483;

const STLINK_PIDS: [u16; 3] = [
    0x3748,
    0x374B,
    0x374E,
];

// STM32 USB DFU identification
const STM32_DFU_VID: u16 = 0x0483;
const STM32_DFU_PID: u16 = 0xDF11;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct UsbDeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_type: String,
}

pub fn identify_usb_device(vendor_id: u16, product_id: u16) -> String {
    if vendor_id == STLINK_VID && STLINK_PIDS.contains(&product_id) {
        "ST-Link".to_string()
    } else if vendor_id == STM32_DFU_VID && product_id == STM32_DFU_PID {
        "STM32 USB DFU".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn enumerate_usb_devices() -> Result<Vec<UsbDeviceInfo>, String> {
    let context = rusb::Context::new()
        .map_err(|error| format!("USB initialization failed: {}", error))?;

    let devices = context
        .devices()
        .map_err(|error| format!("USB device enumeration failed: {}", error))?;

    let mut result = Vec::new();

    for device in devices.iter() {
        let descriptor = device
            .device_descriptor()
            .map_err(|error| {
                format!("Failed to read USB device descriptor: {}", error)
            })?;

        let vendor_id = descriptor.vendor_id();
        let product_id = descriptor.product_id();

        result.push(UsbDeviceInfo {
            vendor_id,
            product_id,
            device_type: identify_usb_device(vendor_id, product_id),
        });
    }

    Ok(result)
}

// Background USB monitoring
pub fn start_usb_monitor(app_handle: AppHandle) {
    thread::spawn(move || {
        let mut previous_devices: Vec<UsbDeviceInfo> = Vec::new();

        loop {
            match enumerate_usb_devices() {
                Ok(current_devices) => {
                    // Detect newly connected devices
                    for device in &current_devices {
                        if !previous_devices.contains(device) {
                            println!(
                                "USB CONNECTED: VID={:04X}, PID={:04X}, Type={}",
                                device.vendor_id,
                                device.product_id,
                                device.device_type
                            );

                            if let Err(error) =
                                app_handle.emit("usb-device-connected", device)
                            {
                                eprintln!(
                                    "Failed to emit USB connected event: {}",
                                    error
                                );
                            }
                        }
                    }

                    // Detect disconnected devices
                    for device in &previous_devices {
                        if !current_devices.contains(device) {
                            println!(
                                "USB DISCONNECTED: VID={:04X}, PID={:04X}, Type={}",
                                device.vendor_id,
                                device.product_id,
                                device.device_type
                            );

                            if let Err(error) =
                                app_handle.emit("usb-device-disconnected", device)
                            {
                                eprintln!(
                                    "Failed to emit USB disconnected event: {}",
                                    error
                                );
                            }
                        }
                    }

                    // Save the current list for the next comparison
                    previous_devices = current_devices;
                }

                Err(error) => {
                    eprintln!("USB monitor error: {}", error);
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}