const FILE_TYPES: [&str; 2] = ["fasta", "fastq"];

use std::{env,io};
use std::fs;

mod integrity;
mod fasta;
mod fastq;
mod margins;
mod scan;

use margins::{Header,Footer};

// FASTA reference: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/
// FASTQ reference: https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let pipeline = args.get(1).expect("ERROR: Must provide at least one arg");
    assert!(FILE_TYPES.contains(&pipeline.as_str()), "\nERROR: First arg must be 'fasta' or 'fastq'");

    for path in args.iter().skip(2) {
            // Run basic file integrity checks
            let size: usize = integrity::integrity_checks(&path);

            let abs_path = fs::canonicalize(&path)?.to_string_lossy().into_owned();

            // print bytes
            println!("\n{abs_path} is {size} bytes");

            // Load file
            let contents: Vec<u8> = fs::read(&path)?;

            // Check headers
            let is_gzip: bool = Header::new(&contents, &path);

            let bytewise_checks_input: Vec<u8> = if is_gzip {
                scan::decode_reader(&contents).unwrap()

            } else {
                contents.clone()
            };

            // Footer
            println!("\nFooter checks:");
            let footer = Footer::new(&contents, &size);
            if footer.bgzf_eof {println!("- contains valid BGZF EOF bytes"); }
            if footer.newline {println!("- contains final newline"); }

            // Byte-wise checks:
            println!("\nByte-wise checks:");
            let bytewise_results = scan::bytewise_checks(&bytewise_checks_input, &pipeline.to_string());
            if !bytewise_results.is_ascii { println!("- contains non-ASCII bytes"); }
            if bytewise_results.contains_offensive_bytes { println!("- contains unsupported ASCII bytes"); }
            if bytewise_results.trailing_whitespace { println!("- contains trailing whitespace"); }
            if bytewise_results.long_lines { println!("- contains lines longer than 80 characters"); }
            if bytewise_results.empty_lines { println!("- contains empty lines"); }

            if *&pipeline.as_str() == "fasta" {

                println!("\nFastA file checks:");

                if bytewise_results.empty_record { println!("- contains empty record"); }

                let fasta = fasta::Fasta::new(&contents, &path);
                if fasta.valid_start {
                    println!("- starts with '>'")
                }
                if fasta.valid_extension {
                    println!("- has valid extension")
                }
            } else if *&pipeline.as_str() == "fastq" {
                println!("\nFastQ file checks:");
                let fastq = fastq::Fastq::new(&contents, &bytewise_results.line_count, &path);

                if !fastq.valid_extension {
                    println!("- does not have recognized extension")
                }

                if !fastq.valid_start {
                    println!("- fastq does not start with '@'")
                }

                if !fastq.four_line_entries {
                    println!("- does not have four-line entries")
                }
            }

    }

Ok(())

}
