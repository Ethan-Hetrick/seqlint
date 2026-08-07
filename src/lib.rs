use std::fs;
use std::io::Read;
use std::io;
use flate2::read::GzDecoder;

#[derive(Debug)]
pub struct Header {
    utf_bom: bool,
    gzip_magic: bool,
    deflate: bool,
    cram_magic: bool,
}

impl Header {
    fn utf_bom(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0xEF, 0xBB, 0xBF])
    }

    fn gzip_magic(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0x1F, 0x8B])
    }

    fn is_deflate(contents: &Vec<u8>) -> bool {
        // 3rd byte set to 8 for DEFLATE]
        contents[2] == 8
    }

    fn cram_magic(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[0x43, 0x52, 0x41, 0x4d])
    }

    pub fn new (contents: &Vec<u8>, path: &String) -> bool {
        // check: headers
        println!("\nHeader checks:");
        let header = Header {
            utf_bom: Header::utf_bom(&contents),
            gzip_magic: Header::gzip_magic(&contents),
            deflate: Header::is_deflate(&contents),
            cram_magic: Header::cram_magic(&contents),
        };

        // Error if BOM exists
        assert!(!header.utf_bom, "\n\nERROR: file contains UTF BOM. Remove it using:\n\n\t\tdos2unix --remove-bom {path}\n");

        // Print if file is gzipped
        if header.gzip_magic { println!("- gzip-compressed"); }

        if header.deflate { println!{"- compressed with DEFLATE"} }

        if header.cram_magic { println!{"- is a CRAM file"} }

        header.gzip_magic
    }
}

pub struct Footer {
    pub newline: bool,
    pub bgzf_eof: bool,
}

impl Footer {
    fn bgzf_eof(contents: &Vec<u8>, size: &usize) -> bool {

        let eof = vec![0x1f, 0x8b, 0x08, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
                       0x06, 0x00, 0x42, 0x43, 0x02, 0x00, 0x1b, 0x00, 0x03, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];

        *size >= eof.len() && contents[*size - eof.len()..] == eof
    }

    fn check_final_newline (contents: &[u8], size: &usize) -> bool {
        contents[*&size - 1] == 0x0A
    }

    pub fn new (contents: &Vec<u8>, size: &usize) -> Footer {
        let footer = Footer {
            newline: Footer::check_final_newline(&contents, &size),
            bgzf_eof: Footer::bgzf_eof(&contents, &size),
        };

        footer
    }
}

#[derive(Debug)]
pub struct ByteWiseCheck {
    pub is_ascii: bool,
    pub contains_offensive_bytes: bool,
    pub trailing_whitespace: bool,
    pub long_lines: bool,
    pub empty_lines: bool,
    pub line_count: usize,
    pub empty_record: bool,
}

pub fn bytewise_checks(contents: &[u8], pipeline: &str) -> ByteWiseCheck {

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
    let mut line_count: usize = 0usize;
    let mut max_header_len: usize = 0usize;
    let mut header_len: usize = 0usize;
    let mut in_header: bool = false;
    let mut in_seq_id: bool = false;
    let mut malformed_seq_id: bool = false;
    let mut malformed_sequence: bool = false;
    let mut empty_record: bool = false;

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
            line_count += 1;
            header_len = 0;
            in_header = false;
            in_seq_id = false;

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

            if pipeline == "fasta"  {
                if in_header {
                    header_len += 1;

                    // Check for empty FastA records
                    if header_len == 1 {
                        if !empty_record && i > 2 && contents[i - 2] == b'>'  {
                            empty_record = true;
                        }
                    }

                    if *byte == 0x20 {
                        in_seq_id = false;
                    }

                    if header_len > max_header_len {
                        max_header_len = header_len;
                    }

                    // The SeqID can only include letters, digits, hyphens (-),
                    // underscores (_), periods (.), colons (:), asterisks (*),
                    // and number signs (#)
                    if in_seq_id && (byte.is_ascii_alphanumeric()
                        || matches!(byte, b'-' | b'_' | b'.' | b':' | b'*' | b'#'))
                    {
                        // Valid chars
                    } else if !malformed_seq_id && in_seq_id {
                        malformed_seq_id = true;
                        println!{"- seqID contains invalid characters.\n\t\
                        Only letters, digits, hyphens (-), underscores (_), periods (.),\
                         colons (:), asterisks (*), and number signs (#) are allowed"}
                    }
                } else if *byte == 0x3E {
                    in_header = true;
                    in_seq_id = true;
                } else {
                    if !(byte.is_ascii_alphabetic() || matches!(byte, b'-' | b'.')) {
                        if !malformed_sequence {
                            println!{"- sequence contains invalid characters. \
                            Only IUPAC nucleotide symbols are allowed"};
                        }

                        malformed_sequence = true;
                    }
                }

            }

            if !whitespaces.contains(byte) {
                emptyline = false;
            }
        }
    }

    if max_header_len > 25 {
        println!{"- header length exceeds 25 characters.\n\t\
        Longest header is {max_header_len} characters long"}
    }

    ByteWiseCheck {
        is_ascii: is_ascii,
        contains_offensive_bytes: contains_offensive_bytes,
        trailing_whitespace: trailing,
        long_lines: long,
        empty_lines: emptyline_check,
        line_count: line_count,
        empty_record: empty_record,
    }
}

pub fn integrity_checks(path: &String) -> usize {
    // check: does exist
    assert!(fs::exists(&path).unwrap(), "\n\tERROR: Unable to access file {path}\n");
    let metadata = fs::metadata(&path).expect("REASON");

    // check: is not dir
    // TODO: better error message for no read access
    assert!(!metadata.is_dir(), "\nERROR: {path} is a directory\n");

    // check: is not empty
    let size = metadata.len() as usize;
    assert!(*&size > 0, "\nERROR: {path} is empty\n");

    // check: has more than 3 bytes
    assert!(*&size >= 4, "\nERROR: {path} < 3 bytes, unable to process\n");

    size
}

pub fn decode_reader(bytes: &Vec<u8>) -> io::Result<Vec<u8>> {
   let mut gz: GzDecoder<&[u8]> = GzDecoder::new(&bytes[..]);
   let mut decompressed_contents = Vec::new();
   let _ = gz.read_to_end(&mut decompressed_contents);

   Ok(decompressed_contents)
}