use std::{env,io};
use std::fs;

use seqlint::{bytewise_checks,check_final_newline};

// FASTA reference: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/
// FASTQ reference: https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files

#[derive(Debug)]
struct Header {
    utf_bom: bool,
    gzip_magic: bool,
}

impl Header {
    fn utf_bom(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0xEF, 0xBB, 0xBF])
    }

    fn gzip_magic(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0x1F, 0x8B])
    }
}

fn main() -> io::Result<()> {

    for path in env::args().skip(1) {

            // check: does exist
            assert!(fs::exists(&path).unwrap(), "\n\tERROR: Unable to access file {path}\n");

            // check: is not dir
            // TODO: better error message for no read access
            let metadata = fs::metadata(&path)?;
            assert!(!metadata.is_dir(), "\nERROR: {path} is a directory\n");

            // check: is not empty
            let size = metadata.len() as usize;
            assert!(*&size > 0, "\nERROR: {path} is empty\n");

            // check: has more than 3 bytes
            assert!(*&size >= 3, "\nERROR: {path} < 3 bytes, unable to process\n");

            // print bytes
            println!("\n{path} is {size} bytes\n");

            // Load file
            let contents: Vec<u8> = fs::read(&path)?;

            // check: headers
            let header = Header {
                utf_bom: Header::utf_bom(&contents),
                gzip_magic: Header::gzip_magic(&contents),
            };

            // Error if BOM exists
            assert!(!header.utf_bom, "\n\nERROR: {path} contains UTF BOM. Remove it using:\n\n\t\tdos2unix --remove-bom {path}\n");

            // Print if file is gzipped
            if header.gzip_magic { println!("\n{path} is gzip-compressed\n"); }

            // Byte-wise checks:
            let bytewise_results = bytewise_checks(&contents);

            if !bytewise_results.is_ascii && !header.gzip_magic { println!("{path} contains non-ASCII bytes"); }
            if bytewise_results.contains_offensive_bytes && !header.gzip_magic { println!("{path} contains unsupported ASCII bytes"); }
            if bytewise_results.trailing_whitespace && !header.gzip_magic { println!("{path} contains trailing whitespace"); }
            if bytewise_results.long_lines && !header.gzip_magic { println!("{path} contains lines longer than 80 characters"); }
            if bytewise_results.empty_lines && !header.gzip_magic { println!("{path} contains empty lines"); }

            // check: FASTA header
            if contents.starts_with(&[0x3E]) {
                println!("\n{path} contains FASTA header");
            // check: FASTQ header
            } else if contents.starts_with(&[0x40]) {
                println!("\n{path} contains FASTQ header");
            }

            let has_final_newline = check_final_newline(&contents, size);

            if !has_final_newline {println!("\nWARNING: {path} does not contain a final newline character")}

    }

    Ok(())

}