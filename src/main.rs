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
#[command(
    version,
    about = "Linter for biological sequence data files",
    arg_required_else_help = true)]
struct Args {
    #[arg(short, long, value_enum)]
    pipeline: Option<Pipeline>,
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

    let pipeline_selection: Option<&str> = args.pipeline
        .as_ref()
        .map(|p| match p {
            Pipeline::Fasta => "fasta",
            Pipeline::Fastq => "fastq"
        });

    for path_buf in &args.files {
        let canonical_path = fs::canonicalize(&path_buf)?;
        let path = canonical_path.to_string_lossy().into_owned();

        let equal_str = "=".repeat(path.len());

        eprintln!(
            "{equal_str}\nseqlint results for:\n\n{path}"
        );

        // Skip duplicate user-provided paths
        if !seen_paths.insert(canonical_path.clone()) {
            eprintln!("\nWARNING: skipping {path} as it was provided more than once\n");
            continue
        }

        // Run basic file integrity checks
        match integrity::integrity_checks(&path) {
            Ok(()) => {}
            Err(message) => {
                eprintln!("\nERROR: {path} {message}, skipping file checks..");
                continue
            }
        }

        // Load file
        // TODO: catch errors and print in nicer format
        let contents: Vec<u8> = fs::read(&path)?;
        let size: usize = contents.len();

        // Check headers
        let header_results = Header::new(&contents);
        header_results.report();

        let bytewise_checks_input: Vec<u8> = if header_results.gzip_magic {
            scan::decode_reader(&contents).unwrap()
        } else {
            contents.clone()
        };

        if bytewise_checks_input.is_empty() {
            println!("\nWARNING: file contents empty, skipping subsequent checks..\n");
            continue
        }

        // Footer
        let footer_results = Footer::new(&contents, &size);
        footer_results.report();

        // Byte-wise checks:
        let (bytewise_results, fastq_results, fasta_results) = scan::bytewise_checks(&bytewise_checks_input, &pipeline_selection.unwrap_or(""));
        bytewise_results.report();

        match args.pipeline {
            Some(Pipeline::Fasta) => {
                let fasta_quick = fasta::FastaQuick::new(&contents, &path);
                fasta_quick.report();

                if let Some(fa) = fasta_results {
                    fa.report();
                }
            }
            Some(Pipeline::Fastq) => {
                let fastq_quick = fastq::FastqQuick::new(&contents, &bytewise_results.line_count, &path);
                fastq_quick.report();

                if let Some(fq) = fastq_results {
                    fq.report();
                }
            }
            None => {}
        }
    }

    Ok(())
}
