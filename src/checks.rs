use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
pub struct IntegrityCheck {
    pub readonly: bool,
    pub is_file: bool,
    pub is_empty: bool,
}

pub fn integrity_check(file: &String) -> io::Result<IntegrityCheck> {
    let file = File::open(file)?;
    let metadata = file.metadata()?;

    Ok(IntegrityCheck {
        readonly: metadata.permissions().readonly(),
        is_file: metadata.is_file(),
        is_empty: metadata.len() == 0,
    })
}

#[derive(Debug, Default)]
pub struct LineEndingCheck {
    pub contains_lf: bool,
    pub contains_cr: bool,
    pub contains_crlf: bool,
}

pub fn special_char_check(file: &String) -> io::Result<LineEndingCheck> {
    let mut file = File::open(file)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut check = LineEndingCheck::default();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'\r' if bytes.get(i + 1) == Some(&b'\n') => {
                check.contains_crlf = true;
                i += 1;
            }
            b'\r' => {
                check.contains_cr = true;
            }
            b'\n' => {
                check.contains_lf = true;
            }
            _ => {}
        }

        i += 1;
    }

    Ok(check)
}
