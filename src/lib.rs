// IUPAC compatible byte check
pub fn is_iupac_byte(b: u8) -> bool {
    b.is_ascii_alphabetic() || matches!(b, b'-' | b'.')
}

// Offending ASCII bytes
// The below are mostly control characters with exceptions like NUL and CR
pub fn is_offender(b: u8) -> bool {
    matches!(b, 0x00..=0x08 | 0x0B..=0x1F | 0x7F)
}

pub fn is_whitespace(b: u8) -> bool {
    matches!(b, 0x9 | 0x20)
}
