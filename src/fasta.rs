const VALID_FASTA_EXTENSIONS: [&str; 8] = [
    ".fasta", ".fa", ".faa", ".fna", ".ffn", ".fas", ".frn", ".mpfa",
];

use seqlint::{pass, warn, info};

pub struct FastaQuick {
    pub valid_extension: bool,
    pub valid_start: bool,
}

impl FastaQuick {
    pub fn new(contents: &Vec<u8>, path: &String) -> Self {
        FastaQuick {
            valid_extension: VALID_FASTA_EXTENSIONS
                .iter()
                .any(|&ext| path.ends_with(ext) || path.ends_with(&format!("{}.gz", ext))),
            valid_start: contents.starts_with(&[b'>']),
        }
    }

    pub fn report(&self) {
        info!("== FASTA checks (quick) ==");

        if self.valid_start {
            pass!("starts with '>'")
        } else {
            warn!("does not start with '>'")
        }

        if self.valid_extension {
            pass!("has valid file extension")
        } else {
            warn!("does not have recognized file extension")
        }
    }
}
