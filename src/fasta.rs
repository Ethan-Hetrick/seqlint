pub struct Fasta {
    pub valid_start: bool
}

impl Fasta {
    pub fn new(contents: &[u8]) -> Self {
        Fasta {
            valid_start: contents.starts_with(&[0x3E])
        }
    }
}