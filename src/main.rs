const FILE_TYPES: [&str; 2] = ["fasta", "fastq"];

use std::{env,io};
use std::fs;

use seqlint::{bytewise_checks, check_headers, integrity_checks, check_footer};

mod fasta;
mod fastq;

// FASTA reference: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/
// FASTQ reference: https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let pipeline = args.get(1).expect("Must provide at least one arg");
    assert!(FILE_TYPES.contains(&pipeline.as_str()), "\nFirst arg must be 'fasta' or 'fastq'\n");

    for path in args.iter().skip(2) {
            // Run basic file integrity checks
            let size: usize = integrity_checks(&path);

            // print bytes
            println!("\n{path} is {size} bytes\n");

            // Load file
            let contents: Vec<u8> = fs::read(&path)?;

            // Check headers
            let is_gzip: bool = check_headers(&contents, &path);

            // Footer
            let footer = check_footer(&contents, &size);
            if footer.bgzf_eof {println!("{path} contains valid BGZF EOF bytes"); }
            if footer.newline {println!("{path} contains final newline\n"); }

            // Byte-wise checks:
            let bytewise_results = bytewise_checks(&contents);
            if !bytewise_results.is_ascii && !is_gzip { println!("{path} contains non-ASCII bytes"); }
            if bytewise_results.contains_offensive_bytes && !is_gzip { println!("{path} contains unsupported ASCII bytes"); }
            if bytewise_results.trailing_whitespace && !is_gzip { println!("{path} contains trailing whitespace"); }
            if bytewise_results.long_lines && !is_gzip { println!("{path} contains lines longer than 80 characters"); }
            if bytewise_results.empty_lines && !is_gzip { println!("{path} contains empty lines"); }

            if *&pipeline.as_str() == "fasta" {
                if fasta::Fasta::valid_start(&contents) {
                    println!("\nFastA starts with '>'")
                }
            } else if *&pipeline.as_str() == "fastq" {
                if fastq::Fastq::valid_start(&contents) {
                    println!("\nFastQ starts with '@'")
                }
            }

    }

Ok(())

}
