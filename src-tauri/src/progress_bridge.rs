use tauri::{AppHandle, Emitter};

use crate::progress_parser::{
    parse_progress_line,
    ProgressEvent,
};

pub const FLASH_PROGRESS_EVENT: &str = "flash-progress";

pub fn emit_progress(
    app_handle: &AppHandle,
    event: &ProgressEvent,
) -> Result<(), String> {
    app_handle
        .emit(FLASH_PROGRESS_EVENT, event)
        .map_err(|error| {
            format!("Failed to emit flash progress event: {}", error)
        })
}

pub fn process_output_line(
    app_handle: &AppHandle,
    transport: &str,
    line: &str,
) -> Result<Option<ProgressEvent>, String> {
    match parse_progress_line(line, transport) {
        Some(event) => {
            emit_progress(app_handle, &event)?;
            Ok(Some(event))
        }

        None => Ok(None),
    }
}