use std::fs;
use std::collections::HashSet;
use walkdir::WalkDir;
use std::path::PathBuf;

enum FailureStates {
    NotReadable,
    IsDirectory,
    TooSmall,
}

impl FailureStates {
    fn failure_message(&self) -> &'static str {
        match self {
            FailureStates::NotReadable => "does not exist or is not readable",
            FailureStates::IsDirectory => "is a directory",
            FailureStates::TooSmall => "< 3 bytes",
        }
    }
}

pub fn generate_canonical_path_set(path_buffers: Vec<PathBuf>, recursive: bool, max_depth: Option<usize>, follow: bool) -> HashSet<String> {
    let mut file_set: HashSet<String> = HashSet::new();

    for path_buf in &path_buffers {
        if path_buf.to_str().is_none() {
            eprintln!(
                "\nERROR: path '{}' contains non-UTF-8 characters, skipping..",
                path_buf.display()
            );
            continue;
        }

        if recursive {
            let walker = if max_depth.is_some() {
                WalkDir::new(path_buf)
                    .max_depth(max_depth.unwrap())
                    .follow_links(follow)
                    .sort_by_file_name()
            } else {
                WalkDir::new(path_buf)
                    .follow_links(follow)
                    .sort_by_file_name()
            };

            for entry in walker {
                let entry = match entry {
                    Ok(e) => e,
                    Err(err) => {
                        eprintln!("\nWARNING: skipping unreadable entry: {err}");
                        continue;
                    }
                };

                let canonical_path = match fs::canonicalize(entry.path()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("\nERROR: Path: '{}' {e}, skipping..", entry.path().display());
                        continue;
                    }
                };

            file_set.insert(canonical_path.to_string_lossy().into_owned());
            }
            
        } else {
            let canonical_path = match fs::canonicalize(&path_buf) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("\nERROR: Path: '{}' {e}, skipping..", path_buf.display());
                        continue;
                    }
            };

            if !canonical_path.is_file() {
                        eprintln!("Skipping directory (use -R to recurse): {}", canonical_path.display());
                        continue;
            }

            file_set.insert(canonical_path.to_string_lossy().into_owned());
        }
}

    file_set
}

pub fn integrity_checks(path: &String) -> Result<(), &'static str> {
    let metadata = fs::metadata(path).map_err(|_| FailureStates::NotReadable.failure_message())?;

    if !metadata.is_file() {
        return Err(FailureStates::IsDirectory.failure_message())?;
    }

    if metadata.len() < 4 {
        return Err(FailureStates::TooSmall.failure_message());
    }

    Ok(())
}
