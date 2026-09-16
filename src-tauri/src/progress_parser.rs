use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ProgressEvent {
    pub transport: String,
    pub stage: String,
    pub progress: u8,
    pub message: String,
    pub done: bool,
    pub success: Option<bool>,
}

pub fn parse_progress_line(
    line: &str,
    transport: &str,
) -> Option<ProgressEvent> {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return None;
    }

    let extracted_progress = extract_percentage(trimmed);

    let success_message = contains_success_message(trimmed);
    let error_message = contains_error_message(trimmed);

    // Normal progress messages contain a percentage.
    //
    // Completion messages may not contain a percentage, so:
    // - success message -> 100%
    // - error message   -> 0%
    // - unrelated line  -> ignore
    let progress = match extracted_progress {
        Some(value) => value,
        None if success_message => 100,
        None if error_message => 0,
        None => return None,
    };

    let stage = detect_stage(trimmed);

    let done = progress >= 100 || success_message || error_message;

    let success = if success_message {
        Some(true)
    } else if error_message {
        Some(false)
    } else {
        None
    };

    Some(ProgressEvent {
        transport: transport.to_string(),
        stage,
        progress,
        message: trimmed.to_string(),
        done,
        success,
    })
}

fn extract_percentage(line: &str) -> Option<u8> {
    let bytes = line.as_bytes();

    for i in 0..bytes.len() {
        if bytes[i] != b'%' {
            continue;
        }

        let mut start = i;

        while start > 0 && bytes[start - 1].is_ascii_digit() {
            start -= 1;
        }

        if start == i {
            continue;
        }

        let number = &line[start..i];

        if let Ok(value) = number.parse::<u8>() {
            if value <= 100 {
                return Some(value);
            }
        }
    }

    None
}

fn detect_stage(line: &str) -> String {
    let lower = line.to_ascii_lowercase();

    if lower.contains("verify")
        || lower.contains("verifying")
        || lower.contains("read progress")
    {
        "verifying".to_string()
    } else if lower.contains("erase")
        || lower.contains("erasing")
    {
        "erasing".to_string()
    } else if lower.contains("download")
        || lower.contains("program")
        || lower.contains("write")
        || lower.contains("writing")
    {
        "flashing".to_string()
    } else if lower.contains("connect")
        || lower.contains("opening")
    {
        "connecting".to_string()
    } else {
        "flashing".to_string()
    }
}

fn contains_success_message(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();

    lower.contains("download done")
        || lower.contains("download verified successfully")
        || lower.contains("file download complete")
        || lower.contains("successfully")
        || lower == "done!"
}

fn contains_error_message(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();

    lower.contains("error:")
        || lower.contains("error ")
        || lower.contains("failed")
        || lower.contains("failure")
        || lower.contains("no debug probe")
        || lower.contains("no st-link")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dfu_util_progress() {
        let line =
            "Download [=========================] 100%       226117 bytes";

        let event = parse_progress_line(line, "dfu")
            .expect("Progress should be detected");

        assert_eq!(event.transport, "dfu");
        assert_eq!(event.stage, "flashing");
        assert_eq!(event.progress, 100);
        assert!(event.done);
    }

    #[test]
    fn parses_stlink_progress() {
        let line = "Download in Progress: 50%";

        let event = parse_progress_line(line, "stlink")
            .expect("Progress should be detected");

        assert_eq!(event.transport, "stlink");
        assert_eq!(event.progress, 50);
        assert_eq!(event.stage, "flashing");
    }

    #[test]
    fn detects_verification_stage() {
        let line = "Read progress: 100%";

        let event = parse_progress_line(line, "stlink")
            .expect("Progress should be detected");

        assert_eq!(event.stage, "verifying");
        assert_eq!(event.progress, 100);
    }

    #[test]
    fn detects_success() {
        let line = "Download verified successfully";

        let event = parse_progress_line(line, "stlink")
            .expect("Progress should be detected");

        assert_eq!(event.progress, 100);
        assert_eq!(event.done, true);
        assert_eq!(event.success, Some(true));
    }

    #[test]
    fn ignores_non_progress_line() {
        let line = "Opening DFU capable USB device...";

        let event = parse_progress_line(line, "dfu");

        assert!(event.is_none());
    }

    #[test]
    fn detects_error_message_with_percentage() {
        let line = "Error: flashing failed at 35%";

        let event = parse_progress_line(line, "stlink")
            .expect("Progress should be detected");

        assert_eq!(event.progress, 35);
        assert_eq!(event.done, true);
        assert_eq!(event.success, Some(false));
    }
}