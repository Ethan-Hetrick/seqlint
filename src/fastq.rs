const VALID_FASTQ_EXTENSIONS: [&str; 2] = [".fastq", ".fq"];

#[derive(Debug)]
pub struct Fastq {
    pub valid_extension: bool,
    pub valid_start: bool,
    pub four_line_entries: bool,
    pub paired_end_r1: bool,
    pub paired_end_r2: bool,
}

impl Fastq {
    pub fn new(contents: &Vec<u8>, line_count: &usize, path: &String) -> Self {
        let path_no_gz: &str = path.strip_suffix(".gz").unwrap_or(path);

        Fastq {
            valid_extension: VALID_FASTQ_EXTENSIONS
                .iter().any(|&ext| path_no_gz.ends_with(ext)),
            valid_start: contents.starts_with(&[b'@']),
            four_line_entries: (line_count % 4 == 0),
            paired_end_r1: path_no_gz.contains("_R1_") || path_no_gz.ends_with("_01.fastq") || path_no_gz.ends_with("_01.fq"),
            paired_end_r2: path_no_gz.contains("_R2_") || path_no_gz.ends_with("_02.fastq") || path_no_gz.ends_with("_02.fq"),
        }
    }
}