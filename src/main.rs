use clap::{Parser, ValueEnum};
use std::fs;
use std::io;
use std::path::PathBuf;

mod fasta;
mod fastq;
mod integrity;
mod margins;
mod scan;

use margins::{Footer, Header};

#[derive(Parser, Debug)]
#[command(version, about = "Linter for biological sequence data files")]
struct Args {
    pipeline: Pipeline,
    files: Vec<PathBuf>,
}

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "lowercase")]
enum Pipeline {
    Fasta,
    Fastq,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let pipeline_selection = match args.pipeline {
        Pipeline::Fasta => "fasta".to_string(),
        Pipeline::Fastq => "fastq".to_string(),
    };

    for path_buf in &args.files {
        let path = path_buf.to_string_lossy().into_owned();

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

        assert!(bytewise_checks_input.len() > 0, "\n- file contents empty\n");

        // Footer
        println!("\nFooter checks:");
        let footer = Footer::new(&contents, &size);
        if footer.bgzf_eof {
            println!("- contains valid BGZF EOF bytes");
        }
        if footer.newline {
            println!("- contains final newline");
        }

        // Byte-wise checks:
        println!("\nByte-wise checks:");
        let bytewise_results = scan::bytewise_checks(&bytewise_checks_input, &pipeline_selection);
        if !bytewise_results.is_ascii {
            println!("- contains non-ASCII bytes");
        }
        if bytewise_results.contains_offensive_bytes {
            println!("- contains unsupported ASCII bytes");
        }
        if bytewise_results.trailing_whitespace {
            println!("- contains trailing whitespace");
        }
        if bytewise_results.long_lines {
            println!("- contains lines longer than 80 characters");
        }
        if bytewise_results.empty_lines {
            println!("- contains empty lines");
        }

        match args.pipeline {
            Pipeline::Fasta => {
                println!("\nFastA file checks:");

                if bytewise_results.empty_record {
                    println!("- contains empty record");
                }

                let fasta = fasta::Fasta::new(&contents, &path);
                if fasta.valid_start {
                    println!("- starts with '>'")
                }
                if fasta.valid_extension {
                    println!("- has valid extension")
                }
            }
            Pipeline::Fastq => {
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

                if fastq.paired_end_r1 {
                    println!("- paired-end R1 file")
                } else if fastq.paired_end_r2 {
                    println!("- paired-end R2 file")
                }
            }
        }
    }

    Ok(())
}
