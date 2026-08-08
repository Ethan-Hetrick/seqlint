// IUPAC compatible byte check
pub trait IupacByte {
    fn is_iupac_byte(&self) -> bool;
}

impl IupacByte for &u8 {
    fn is_iupac_byte(&self) -> bool {
        self.is_ascii_alphabetic() || matches!(self, b'-' | b'.')
    }
}