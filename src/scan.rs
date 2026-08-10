use flate2::read::GzDecoder;
use seqlint::{is_iupac_byte, is_offender, is_whitespace};
use std::collections::HashSet;
use std::io;
use std::io::Read;

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum FastqState {
    Header,
    Sequence,
    Separator,
    Quality
}

impl FastqState {
    fn next(self) -> Self {
        match self {
            FastqState::Header => FastqState::Sequence,
            FastqState::Sequence => FastqState::Separator,
            FastqState::Separator => FastqState::Quality,
            FastqState::Quality => FastqState::Header
        }
    }
}

pub fn bytewise_checks(contents: &[u8], pipeline: &str) -> ByteWiseCheck {
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
    let mut record_count: usize = 0;
    let mut fastq_record = FastQ {
        missing_header_character: false,
        missing_delimiter: false,
        bad_sequence: false,
    };
    let mut record_set = HashSet::new();
    let mut duplicate_header: bool = false;
    let mut sequence_length: usize = 0;
    let mut quality_length: usize = 0;
    let mut fastq_state = FastqState::Header;
    let mut line_start: bool = true;

    for byte in contents.iter() {
        // increase index
        i += 1;

        // check: all bytes are ASCII
        if !byte.is_ascii() {
            is_ascii = false;
        }
        // check: offending ASCII bytes
        if is_offender(*byte) {
            contains_offensive_bytes = true;
        }

        // count newlines
        if *byte == 0x0A {
            line_count += 1;
            if in_header {
                let record = &contents[((i - 1) - header_len)..(i - 1)];
                if record_set.insert(record) {
                    // New record
                } else if !duplicate_header {
                    duplicate_header = true;
                    println!("- duplicate headers")
                }
            }
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
            if i > 2 && is_whitespace(contents[i - 2]) && !trailing {
                trailing = true;
            }

            line_start = false;

            // Note: this sequence occurs at every newline
            if pipeline == "fastq" {
                if fastq_state == FastqState::Quality {
                    record_count += 1;

                    if sequence_length != quality_length {
                        println!("- record and sequence lengths differ")
                    }
                    sequence_length = 0;
                    quality_length = 0;
                }

                fastq_state = fastq_state.next();
                line_start = true;
            }
        } else {
            counter += 1;
            if counter > 80 && !long {
                long = true;
            }

            if pipeline == "fastq" {
                match fastq_state {
                    FastqState::Header => {
                        if line_start && *byte != b'@' && !fastq_record.missing_header_character {
                            println! {"- FastQ header line does not start with '@'"};
                            fastq_record.missing_header_character = true;
                        }
                    }
                    FastqState::Sequence => {
                        sequence_length += 1;

                        if !is_iupac_byte(*byte) && !fastq_record.bad_sequence {
                            println!("- FastQ sequence line contains invalid characters. \
                            Only IUPAC nucleotide symbols are allowed");
                            fastq_record.bad_sequence = true;
                        }
                    }

                    FastqState::Separator => {
                        if line_start && *byte !=b'+' && !fastq_record.missing_delimiter {
                            println! {"- FastQ sequence line does not start with '+'"};
                            fastq_record.missing_delimiter = true;
                        }
                    }

                    FastqState::Quality => {
                        quality_length += 1;
                    }
                }
            }

            if pipeline == "fasta" {
                if in_header {
                    header_len += 1;

                    // Check for empty FastA records
                    if header_len == 1 {
                        if !empty_record && i > 2 && contents[i - 2] == b'>' {
                            empty_record = true;
                        }
                    }

                    if (*byte == b'>' && i == 1)
                        || (i >= 2 && *byte == b'>' && contents[i - 2] == b'\n')
                    {
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
                    if in_seq_id
                        && (byte.is_ascii_alphanumeric()
                            || matches!(byte, b'-' | b'_' | b'.' | b':' | b'*' | b'#'))
                    {
                        // Valid chars
                    } else if !malformed_seq_id && in_seq_id {
                        malformed_seq_id = true;
                        println! {"- seqID contains invalid characters.\n\t\
                        Only letters, digits, hyphens (-), underscores (_), periods (.),\
                        colons (:), asterisks (*), and number signs (#) are allowed"}
                    }
                } else if *byte == b'>' {
                    in_header = true;
                    in_seq_id = true;
                    record_count += 1;
                } else {
                    if !is_iupac_byte(*byte) {
                        if !malformed_sequence {
                            println! {"- sequence contains invalid characters. \
                            Only IUPAC nucleotide symbols are allowed"};
                        }

                        malformed_sequence = true;
                    }
                }
            }

            if !is_whitespace(*byte) {
                emptyline = false;
            }
        }
    }

    if sequence_length != quality_length {
        println!("- Sequence and quality line lengths do not match");
    }

    if record_count == 0 {
        println!("- Zero records found");
    } else if record_count % 2 != 0 {
        println!("- Odd number of records: {}", record_count);
    } else {
        println!("- Even number of records: {}", record_count);
    }

    if max_header_len > 25 {
        println! {"- header length exceeds 25 characters.\n\t\
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
