use clap::{Parser, ValueEnum};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::collections::HashSet;

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
    let mut seen_paths = HashSet::new();

    let pipeline_selection = match args.pipeline {
        Pipeline::Fasta => "fasta".to_string(),
        Pipeline::Fastq => "fastq".to_string(),
    };

    for path_buf in &args.files {
        let canonical_path = fs::canonicalize(&path_buf)?;
        let path = canonical_path.to_string_lossy().into_owned();

        // Skip duplicate user-provided paths
        if !seen_paths.insert(canonical_path.clone()) {
            println!("\nWARNING: skipping {path} as it was provided more than once\n");
            continue
        }

        // Run basic file integrity checks
        let size: usize = integrity::integrity_checks(&path);

        // print bytes
        println!("\n{path} is {size} bytes");

        // Load file
        let contents: Vec<u8> = fs::read(&path)?;

        // Check headers
        let header_results = Header::new(&contents);
        header_results.report();

        let bytewise_checks_input: Vec<u8> = if header_results.gzip_magic {
            scan::decode_reader(&contents).unwrap()
        } else {
            contents.clone()
        };

        assert!(bytewise_checks_input.len() > 0, "\n- file contents empty\n");

        // Footer
        let footer_results = Footer::new(&contents, &size);
        footer_results.report();

        // Byte-wise checks:
        let (bytewise_results, fastq_results, fasta_results) = scan::bytewise_checks(&bytewise_checks_input, &pipeline_selection);
        bytewise_results.report();

        match args.pipeline {
            Pipeline::Fasta => {
                let fasta_quick = fasta::FastaQuick::new(&contents, &path);
                fasta_quick.report();

                if let Some(fa) = fasta_results {
                    fa.report();
                }
            }
            Pipeline::Fastq => {
                let fastq_quick = fastq::FastqQuick::new(&contents, &bytewise_results.line_count, &path);
                fastq_quick.report();

                if let Some(fq) = fastq_results {
                    fq.report();
                }
            }
        }
    }

    Ok(())
}
