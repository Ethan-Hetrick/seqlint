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

use std::io::IsTerminal;
use std::sync::OnceLock;

static USE_COLOR: OnceLock<bool> = OnceLock::new();

pub fn use_color() -> bool {
    *USE_COLOR.get_or_init(|| std::io::stdout().is_terminal())
}

pub fn green() -> &'static str { if use_color() { "\x1b[32m" } else { "" } }
pub fn yellow() -> &'static str { if use_color() { "\x1b[33m" } else { "" } }
pub fn red() -> &'static str { if use_color() { "\x1b[31m" } else { "" } }
pub fn blue() -> &'static str { if use_color() { "\x1b[34m" } else { "" } }
pub fn dim() -> &'static str { if use_color() { "\x1b[2m" } else { "" } }
pub fn reset() -> &'static str { if use_color() { "\x1b[0m" } else { "" } }

#[macro_export]
macro_rules! pass {
    ($($arg:tt)*) => {
        println!("{}PASS{}: {}", $crate::green(), $crate::reset(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}WARN{}: {}", $crate::yellow(), $crate::reset(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {
        println!("{}FAIL{}: {}", $crate::red(), $crate::reset(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}INFO{}: {}", $crate::blue(), $crate::reset(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("{}LOG{}: {}", $crate::dim(), $crate::reset(), format!($($arg)*))
    };
}
