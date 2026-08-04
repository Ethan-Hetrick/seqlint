#[derive(Debug)]
pub struct ByteWiseCheck {
    pub is_ascii: bool,
    pub contains_offensive_bytes: bool,
    pub trailing_whitespace: bool,
    pub long_lines: bool,
    pub empty_lines: bool,
}

pub fn bytewise_checks(contents: &[u8]) -> ByteWiseCheck {

    // Offending ASCII bytes
    // The below are mostly control characters with exceptions like NUL and CR
    let offenders: Vec<u8> = vec![
        0x00, // NUL
        0x01, // SOH
        0x02, // STX
        0x03, // ETX
        0x04, // EOT
        0x05, // ENQ
        0x06, // ACK
        0x07, // BEL
        0x08, // BS
        0x0B, // VT
        0x0C, // FF
        0x0D, // CR
        0x0E, // SO
        0x0F, // SI
        0x10, // DLE
        0x11, // DC1
        0x12, // DC2
        0x13, // DC3
        0x14, // DC4
        0x15, // NAK
        0x16, // SYN
        0x17, // ETB
        0x18, // CAN
        0x19, // EM
        0x1A, // SUB
        0x1B, // ESC
        0x1C, // FS
        0x1D, // GS
        0x1E, // RS
        0x1F, // US
        0x7F, // DEL
    ];

    let whitespaces: Vec<u8> = vec![
        0x09, // HT (TAB),
        0x20, // space,
    ];

    let mut counter: u32 = 0;
    let mut i: usize = 0;
    let mut long: bool = false;
    let mut trailing: bool = false;
    let mut emptyline: bool = true;
    let mut emptyline_check: bool = false;
    let mut is_ascii: bool = true;
    let mut contains_offensive_bytes: bool = false;
    for byte in contents.iter() {
        // increase index
        i += 1;

        // check: all bytes are ASCII
        if !byte.is_ascii() {
            is_ascii = false;
        }
        // check: offending ASCII bytes
        if offenders.contains(byte) {
            contains_offensive_bytes = true;
        }

        // count newlines
        if *byte == 0x0A {
            if emptyline && !emptyline_check {
                emptyline_check = true;
            } else {
                emptyline = true;
            }
            counter = 0;

            // check: trailing whitespace
            if i > 2 && whitespaces.contains(&contents[i - 2]) && !trailing {
                trailing = true;
            }
        } else {
            counter += 1;
            if counter > 80 && !long {
                long = true;
            }

            if !whitespaces.contains(byte) {
                emptyline = false;
            }
        }
    }

    ByteWiseCheck {
        is_ascii: is_ascii,
        contains_offensive_bytes: contains_offensive_bytes,
        trailing_whitespace: trailing,
        long_lines: long,
        empty_lines: emptyline_check,
    }
}

pub fn check_final_newline (contents: &[u8], size: usize) -> bool {
    contents[*&size - 1] == 0x0A
}