use std::{env,io};
use std::fs;

use seqlint::{bytewise_checks,check_final_newline, check_headers, integrity_checks};

// FASTA reference: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/
// FASTQ reference: https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files

#[derive(Debug)]
pub struct Header {
    pub utf_bom: bool,
    pub gzip_magic: bool,
}

impl Header {
    pub fn utf_bom(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0xEF, 0xBB, 0xBF])
    }

    pub fn gzip_magic(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0x1F, 0x8B])
    }
}

fn main() -> io::Result<()> {

    for path in env::args().skip(1) {
            // Run basic file integrity checks
            let size: usize = integrity_checks(&path);

            // print bytes
            println!("\n{path} is {size} bytes\n");

            // Load file
            let contents: Vec<u8> = fs::read(&path)?;

            // Check headers
            let is_gzip: bool = check_headers(&contents, &path);

            // Byte-wise checks:
            let bytewise_results = bytewise_checks(&contents);

            if !bytewise_results.is_ascii && !is_gzip { println!("{path} contains non-ASCII bytes"); }
            if bytewise_results.contains_offensive_bytes && !is_gzip { println!("{path} contains unsupported ASCII bytes"); }
            if bytewise_results.trailing_whitespace && !is_gzip { println!("{path} contains trailing whitespace"); }
            if bytewise_results.long_lines && !is_gzip { println!("{path} contains lines longer than 80 characters"); }
            if bytewise_results.empty_lines && !is_gzip { println!("{path} contains empty lines"); }

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