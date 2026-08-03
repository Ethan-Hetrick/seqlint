use std::{env,io};
use std::fs;

// TODO: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/

fn main() -> io::Result<()> {

    for path in env::args().skip(1) {

            // Check if file exists
            assert!(fs::exists(&path).unwrap(), "ERROR: Unable to access file {path}");

            // Make sure file is not a dir
            let metadata = fs::metadata(&path)?;
            assert!(!metadata.is_dir(), "{path} is a directory");

            // Make sure file is not empty
            assert!(metadata.len() > 0, "{path} is empty");

            // Load file
            let contents: Vec<u8> = fs::read(&path)?;

            // Check for UTF BOM
            assert_ne!(contents[0..3], [0xEF, 0xBB, 0xBF], "{path} contains UTF BOM. To fix:\n\tdos2unix --remove-bom {path}\n");
    }

    Ok(())

}