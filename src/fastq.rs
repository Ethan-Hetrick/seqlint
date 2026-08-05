pub struct Fastq {
    pub valid_start: bool
}

impl Fastq {
    pub fn new(contents: &[u8]) -> Self {
        Fastq {
            valid_start: contents.starts_with(&[0x40])
        }
    }
}