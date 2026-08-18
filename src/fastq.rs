const VALID_FASTQ_EXTENSIONS: [&str; 2] = [".fastq", ".fq"];

use seqlint::{fail, info, warn, log};

#[derive(Debug)]
pub struct FastqQuick {
    pub valid_extension: bool,
    pub ora_extension: bool,
    pub valid_start: bool,
    pub four_line_entries: bool,
    pub paired_end_r1: bool,
    pub paired_end_r2: bool,
}

impl FastqQuick {
    pub fn new(contents: &Vec<u8>, line_count: &usize, path: &String) -> Self {
        let path_no_gz: &str = path.strip_suffix(".gz").unwrap_or(path);

        FastqQuick {
            valid_extension: VALID_FASTQ_EXTENSIONS
                .iter()
                .any(|&ext| path_no_gz.ends_with(ext)),
            ora_extension: path_no_gz.ends_with("ora"),
            valid_start: contents.starts_with(&[b'@']),
            four_line_entries: (line_count % 4 == 0),
            paired_end_r1: path_no_gz.contains("_R1_")
                || path_no_gz.ends_with("_01.fastq")
                || path_no_gz.ends_with("_01.fq"),
            paired_end_r2: path_no_gz.contains("_R2_")
                || path_no_gz.ends_with("_02.fastq")
                || path_no_gz.ends_with("_02.fq"),
        }
    }

    pub fn report(&self) {
        log!("== FASTQ checks (quick) ==");
        if !self.valid_extension && !self.ora_extension {
            warn!("does not have recognized extension")
        } else if !self.valid_extension && self.ora_extension {
            warn!("DRAGEN ORA (Original Read Archive) file extension"); // Technically dead code, but works. keeping for now
        }

        if !self.valid_start {
            fail!("fastq does not start with '@'")
        }

        if !self.four_line_entries {
            fail!("does not have four-line entries")
        }

        if self.paired_end_r1 {
            info!("paired-end R1 file")
        } else if self.paired_end_r2 {
            info!("paired-end R2 file")
        }
    }
}
