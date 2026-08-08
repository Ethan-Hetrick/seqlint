use std::fs;

pub fn integrity_checks(path: &String) -> usize {
    // check: does exist
    assert!(fs::exists(&path).unwrap(), "\n\tERROR: Unable to access file {path}\n");
    let metadata = fs::metadata(&path).expect("REASON");

    // check: is not dir
    // TODO: better error message for no read access
    assert!(!metadata.is_dir(), "\nERROR: {path} is a directory\n");

    // check: is not empty
    let size = metadata.len() as usize;
    assert!(*&size > 0, "\nERROR: {path} is empty\n");

    // check: has more than 3 bytes
    assert!(*&size >= 4, "\nERROR: {path} < 3 bytes, unable to process\n");

    size
}