#[derive(Debug)]
pub struct Fastq {
    pub valid_start: bool,
    pub four_line_entries: bool,
}

impl Fastq {
    pub fn new(contents: &[u8], size: &usize) -> Self {
        Fastq {
            valid_start: contents.starts_with(&[0x40]),
            four_line_entries: (size % 4 == 0),
        }
    }
}