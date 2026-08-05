pub struct Fasta {
    pub valid_start: bool
}

impl Fasta {
    pub fn valid_start (contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0x3E])
    }
}