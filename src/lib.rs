// IUPAC compatible byte check
pub fn is_iupac_byte(b: u8) -> bool {
    b.is_ascii_alphabetic() && !matches!(b, b'E' | b'J' | b'O' | b'P' | b'Q' | b'X' |  b'Z')
}

// Offending ASCII bytes
// The below are mostly control characters with exceptions like NUL and CR
pub fn is_offender(b: u8) -> bool {
    matches!(b, 0x00..=0x08 | 0x0B..=0x1F | 0x7F)
}

pub fn is_whitespace(b: u8) -> bool {
    matches!(b, 0x9 | 0x20)
}

// ANSI colors
pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const GREEN: &str = "\x1b[32m";
pub const RESET: &str = "\x1b[0m";
pub const BLUE: &str = "\x1b[34m";
pub const DIM: &str = "\x1b[2m";

#[macro_export]
macro_rules! pass {
    ($($arg:tt)*) => {
        println!("{}PASS{}: {}", $crate::GREEN, $crate::RESET, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("{}WARN{}: {}", $crate::YELLOW, $crate::RESET, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {
        eprintln!("{}FAIL{}: {}", $crate::RED, $crate::RESET, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}INFO{}: {}", $crate::BLUE, $crate::RESET, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        eprintln!("{}LOG{}: {}", $crate::DIM, $crate::RESET, format!($($arg)*))
    };
}