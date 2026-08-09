use std::io::Read;
use std::io;
use flate2::read::GzDecoder;
use seqlint::IupacByte;

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

pub struct FastQ {
    pub missing_header_character: bool,
    pub missing_delimiter: bool,
    pub bad_sequence: bool,
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
    let mut last_byte: bool = false;
    let mut record_count: usize = 0;
    let mut fastq_record = FastQ {
        missing_header_character: false,
        missing_delimiter: false,
        bad_sequence: false,
    };

    for byte in contents.iter() {
        // increase index
        i += 1;

        if i == contents.len() {
            last_byte = true;
        }

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

            if pipeline == "fastq" {
                // FastQ files are organized in four line entries
                if !last_byte {
                    if line_count > 1 && line_count % 4 == 0 {   // 1/4 - Header line
                        if &contents[i] != &b'@' && fastq_record.missing_header_character == false {
                            println!{"- FastQ header line does not start with '@'"};
                            fastq_record.missing_header_character = true;
                        }
                    } else if (line_count + 2) % 4 == 0 {               // 2/4 - Sequence line
                        if &contents[i] != &b'+' && fastq_record.missing_delimiter == false {
                            println!{"- FastQ sequence line does not start with '+'"};
                            fastq_record.missing_delimiter = true;
                        }
                    }
                }
            }

        } else {
            counter += 1;
            if counter > 80 && !long {
                long = true;
            }

            if pipeline == "fastq" {
                if (*byte == b'@' && i == 1)
                    || (i >= 2 && *byte == b'@' && contents[i-2] == b'\n') {
                    record_count += 1;
                }
                if (line_count + 3) % 4 == 0 { // Sequence line
                        if !byte.is_iupac_byte() {
                            if !fastq_record.bad_sequence {
                                println!("- FastQ sequence line contains invalid characters. Only IUPAC nucleotide symbols are allowed");
                            }
                            fastq_record.bad_sequence = true;
                        }
                    }
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

                    if (*byte == b'>' && i == 1)
                    || (i >= 2 && *byte == b'>' && contents[i-2] == b'\n') {
                        record_count += 1;
                    }

                    if *byte == b' ' {
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
                } else if *byte == b'>' {
                    in_header = true;
                    in_seq_id = true;
                    record_count += 1;
                } else {
                    if !(byte.is_iupac_byte()) {
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

    if record_count == 0 {
        println!("- Zero records found");
    } else if record_count % 2 != 0 {
        println!("- Odd number of records: {}", record_count);
    } else {
        println!("- Even number of records: {}", record_count);
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

pub fn decode_reader(bytes: &Vec<u8>) -> io::Result<Vec<u8>> {
   let mut gz: GzDecoder<&[u8]> = GzDecoder::new(&bytes[..]);
   let mut decompressed_contents = Vec::new();
   let _ = gz.read_to_end(&mut decompressed_contents);

   Ok(decompressed_contents)
}