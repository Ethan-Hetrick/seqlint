const VERSION: &str = env!("CARGO_PKG_VERSION");

use clap::{Parser, ValueEnum};
use std::fs;
use std::io;
use std::path::PathBuf;

mod fasta;
mod fastq;
mod integrity;
mod margins;
mod scan;
use seqlint::{warn, fail, log};
use std::time::Instant;

use margins::{Footer, Header};

#[derive(Parser, Debug)]
#[command(version, about = "Linter for biological sequence data files", arg_required_else_help = true )]
struct Args {
    /// Perform file type specific checks
    #[arg(short, long, value_enum)]
    format: Option<Pipeline>,
    /// Descend into directories
    #[arg(short='R', long, default_value_t = false)]
    recursive: bool,
    /// Follow symbolic links
    #[arg(short='L', long, default_value_t = false)]
    follow: bool,
    /// Maximum depth to descend into directories
    #[arg(long)]
    max_depth: Option<usize>,
    // Positional args, can be files and/or directories
    files: Vec<PathBuf>,
}

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "lowercase")]
enum Pipeline {
    Fasta,
    Fastq,
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let args = Args::parse();

    let format_selection: Option<&str> = args.format.as_ref().map(|p| match p {
        Pipeline::Fasta => "fasta",
        Pipeline::Fastq => "fastq",
    });

    let file_set = integrity::generate_canonical_path_set(args.files, args.recursive, args.max_depth, args.follow);

    for path in file_set.iter() {

        // Print report header
        log!("== seqlint v{VERSION} ==");
        log!("== {path} ==");

        // Run basic file integrity checks
        match integrity::integrity_checks(&path) {
            Ok(()) => {}
            Err(message) => {
                fail!("{path} {message}, skipping file checks..");
                continue;
            }
        }

        // Load file
        // TODO: catch errors and print in nicer format
        let mut contents: Vec<u8> = fs::read(&path)?;
        let size: usize = contents.len();

        // Check headers
        let header_results = Header::new(&contents);
        header_results.report();

        // Footer
        let footer_results = Footer::new(&contents, &size);
        footer_results.report();

        // Remove EOF
        if footer_results.bgzf_eof {
            let new_len = contents.len() - 28;
            contents.truncate(new_len);

            contents = scan::decode_bgzf(&contents).unwrap();
        }

        let bytewise_checks_input: Vec<u8> = if header_results.gzip_magic && !footer_results.bgzf_eof {
            scan::decode_reader(&contents).unwrap()
        } else {
            contents.clone()
        };

        if bytewise_checks_input.is_empty() {
            warn!("file contents empty, skipping subsequent checks..\n");
            continue;
        }

        // Byte-wise checks:
        let (bytewise_results, fastq_results, fasta_results) =
            scan::bytewise_checks(&bytewise_checks_input, &format_selection.unwrap_or(""));
        bytewise_results.report();

        match args.format {
            Some(Pipeline::Fasta) => {
                let fasta_quick = fasta::FastaQuick::new(&contents, &path);
                fasta_quick.report();

                if let Some(fa) = fasta_results {
                    fa.report();
                }
            }
            Some(Pipeline::Fastq) => {
                let fastq_quick =
                    fastq::FastqQuick::new(&contents, &bytewise_results.line_count, &path);
                fastq_quick.report();

                if let Some(fq) = fastq_results {
                    fq.report();
                }
            }
            None => {}
        }
    }

    let execution_time = start_time.elapsed();

    log!("seqlint completed in {execution_time:?}");
    Ok(())
}
