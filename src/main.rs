const FILE_TYPES: [&str; 2] = ["fasta", "fastq"];

use std::{env,io};
use std::fs;

use seqlint::{bytewise_checks, integrity_checks, decode_reader};

mod fasta;
mod fastq;

use seqlint::{Header,Footer};

// FASTA reference: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/
// FASTQ reference: https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let pipeline = args.get(1).expect("Must provide at least one arg");
    assert!(FILE_TYPES.contains(&pipeline.as_str()), "\nFirst arg must be 'fasta' or 'fastq'\n");

    for path in args.iter().skip(2) {
            // Run basic file integrity checks
            let size: usize = integrity_checks(&path);

            let abs_path = fs::canonicalize(&path)?.to_string_lossy().into_owned();

            println!("\nAbsolute path:{}", &abs_path);

            // print bytes
            println!("\n{path} is {size} bytes\n");

            // Load file
            let contents: Vec<u8> = fs::read(&path)?;

            // Check headers
            let is_gzip: bool = Header::new(&contents, &path);

            let bytewise_checks_input: Vec<u8> = if is_gzip {
                decode_reader(&contents).unwrap()

            } else {
                contents.clone()
            };

            // Footer
            let footer = Footer::new(&contents, &size);
            if footer.bgzf_eof {println!("{path} contains valid BGZF EOF bytes"); }
            if footer.newline {println!("{path} contains final newline\n"); }

            // Byte-wise checks:

            let bytewise_results = bytewise_checks(&bytewise_checks_input, &pipeline.to_string());
            if !bytewise_results.is_ascii { println!("{path} contains non-ASCII bytes"); }
            if bytewise_results.contains_offensive_bytes { println!("{path} contains unsupported ASCII bytes"); }
            if bytewise_results.trailing_whitespace { println!("{path} contains trailing whitespace"); }
            if bytewise_results.long_lines { println!("{path} contains lines longer than 80 characters\n"); }
            if bytewise_results.empty_lines { println!("{path} contains empty lines"); }

            if *&pipeline.as_str() == "fasta" {
                let fasta = fasta::Fasta::new(&contents, &path);
                if fasta.valid_start {
                    println!("\nFastA starts with '>'")
                }
                if fasta.valid_extension {
                    println!("\nFastA has valid extension")
                }
            } else if *&pipeline.as_str() == "fastq" {
                let fastq = fastq::Fastq::new(&contents, &bytewise_results.line_count, &path);

                if fastq.valid_extension {
                    println!("\nFastQ has valid extension")
                }

                if fastq.valid_start {
                    println!("\nFastQ starts with '@'")
                }

                if fastq.four_line_entries {
                    println!("\nFastQ has four-line entries")
                }
            }

    }

Ok(())

}
