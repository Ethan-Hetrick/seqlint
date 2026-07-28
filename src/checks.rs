use std::io;
use std::fs::File;

pub fn is_empty(compressed_file: &String) -> io::Result<bool> {
     let f = File::open(&compressed_file)?;

    Ok(f.metadata()?.len() == 0)
 }