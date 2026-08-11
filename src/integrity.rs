use std::fs;

enum FailureStates {
    NotReadable,
    IsDirectory,
    TooSmall,
}

impl FailureStates {
    fn failure_message (&self) -> &'static str {
        match self {
        FailureStates::NotReadable => "does not exist or is not readable",
        FailureStates::IsDirectory => "is a directory",
        FailureStates::TooSmall => "< 3 bytes"
        }
    }
}

pub fn integrity_checks(path: &String) -> Result<(), &'static str> {
    let metadata = fs::metadata(path)
        .map_err(|_| FailureStates::NotReadable.failure_message())?;

    if !metadata.is_file() {
        return Err(FailureStates::IsDirectory.failure_message())?;
    }

    if metadata.len() < 4 {
        return Err(FailureStates::TooSmall.failure_message());
    }

    Ok(())
}
