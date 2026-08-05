const VALID_FASTQ_EXTENSIONS: [&str; 2] = [".fastq", ".fq"];

#[derive(Debug)]
pub struct Fastq {
    pub valid_extension: bool,
    pub valid_start: bool,
    pub four_line_entries: bool,
}

impl Fastq {
    pub fn new(contents: &Vec<u8>, line_count: &usize, path: &String) -> Self {
        Fastq {
            valid_extension: VALID_FASTQ_EXTENSIONS
            .iter().any(|&ext| path.ends_with(ext) || path.ends_with(&format!("{}.gz", ext))),
            valid_start: contents.starts_with(&[0x40]),
            four_line_entries: (line_count % 4 == 0),
        }
    }
}