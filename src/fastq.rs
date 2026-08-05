pub struct Fastq {
    pub valid_start: bool
}

impl Fastq {
    pub fn valid_start (contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0x40])
    }
}