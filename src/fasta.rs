const VALID_FASTA_EXTENSIONS: [&str; 8] = [
    ".fasta", ".fa", ".faa", ".fna", ".ffn", ".fas", ".frn", ".mpfa",
];

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
        println!("\nFASTA checks (quick):");
        if self.valid_start {
            println!("- starts with '>'")
        }
        if self.valid_extension {
            println!("- has valid extension")
        }
    }
}
