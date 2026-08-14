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
    pub empty_line: bool,
    pub line_count: usize,
}

impl ByteWiseCheck {
    pub fn report(&self) {
        println!("\nScan checks:");
        if !self.is_ascii {
            println!("- contains non-ASCII bytes");
        }
        if self.contains_offensive_bytes {
            println!("- contains unsupported ASCII bytes");
        }
        if self.trailing_whitespace {
            println!("- contains trailing whitespace");
        }
        if self.empty_line {
            println!("- contains empty lines");
        }
    }
}

pub struct FastQ {
    pub missing_header_character: bool,
    pub missing_delimiter: bool,
    pub bad_sequence: bool,
    pub record_count: usize,
    pub seq_qual_mismatch: bool,
    pub empty_plus_line: bool,
    pub phred_33_compatible: bool,
    pub phred_64_compatible: bool,
    pub solexa_compatible: bool,
}

impl FastQ {
    pub fn report(&self) {
        println!("- Counted {} records", self.record_count);
        if self.missing_header_character {
            println! {"- header line does not start with '@'"};
        }

        if self.missing_delimiter {
            println! {"- sequence line does not start with '+'"};
        }

        if self.bad_sequence {
            println!(
                "- sequence line contains invalid characters. \
                            Only IUPAC nucleotide symbols are allowed"
            );
        }

        if self.seq_qual_mismatch {
            println!("- record and sequence lengths differ");
        }

        if !self.empty_plus_line {
            println!("- title '+' line not empty");
        }
    }
}

pub struct FastA {
    pub missing_header_character: bool,
    pub valid_sequence: bool,
    pub record_count: usize,
    pub max_header_length: usize,
    pub valid_seq_id: bool,
    pub duplicate_header: bool,
    pub empty_record: bool,
    pub long_sequence: bool,
}

impl FastA {
    pub fn report(&self) {
        println!("\nFASTA checks:");

        if self.empty_record {
            println!("- contains empty record");
        }

        if self.long_sequence {
            println!("- sequence line is longer than 80 characters")
        }

        if !self.valid_seq_id {
            println! {"- seqID contains invalid characters.\n\t\
            Only letters, digits, hyphens (-), underscores (_), periods (.),\
            colons (:), asterisks (*), and number signs (#) are allowed" }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FastqState {
    Header,
    Sequence,
    Separator,
    Quality,
}

impl FastqState {
    fn next(self) -> Self {
        match self {
            FastqState::Header => FastqState::Sequence,
            FastqState::Sequence => FastqState::Separator,
            FastqState::Separator => FastqState::Quality,
            FastqState::Quality => FastqState::Header,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FastaState {
    Header,
    Sequence,
}

pub fn bytewise_checks(
    contents: &[u8],
    format: &str,
) -> (ByteWiseCheck, Option<FastQ>, Option<FastA>) {
    let mut line_length: usize = 0;
    let mut i: usize = 0;
    let mut trailing_whitespace: bool = false;
    let mut empty_line: bool = true;
    let mut is_ascii: bool = true;
    let mut contains_offensive_bytes: bool = false;
    let mut line_count: usize = 0usize;
    let mut header_len: usize = 0usize;
    let mut in_seq_id: bool = false;
    let mut fastq_record = FastQ {
        missing_header_character: false,
        missing_delimiter: false,
        bad_sequence: false,
        record_count: 0,
        seq_qual_mismatch: false,
        empty_plus_line: true,
        phred_33_compatible: true,
        phred_64_compatible: true,
        solexa_compatible: true,
    };
    let mut fasta_record = FastA {
        missing_header_character: false,
        valid_sequence: true,
        record_count: 0,
        max_header_length: 0,
        valid_seq_id: true,
        duplicate_header: false,
        empty_record: false,
        long_sequence: false,
    };
    let mut record_set = HashSet::new();
    let mut sequence_length: usize = 0;
    let mut quality_length: usize = 0;
    let mut fastq_state = FastqState::Header;
    let mut fasta_state = FastaState::Header;
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
        if *byte == b'\n' {
            line_count += 1;

            // TODO: also check if lines are all whitespace, not just two adjacent line endings
            // delcare a var only_whitespace, make it true as base case, make it false upon valid char
            // might replace the below line entirely.
            if i > 2 && contents[&i - 2] == b'\n' {
                empty_line = true;
            }

            // Check for duplicate FASTA headers
            if format == "fasta" && fasta_state == FastaState::Header {
                let record = &contents[((i - 1) - header_len)..(i - 1)];

                if !record_set.insert(record) {
                    fasta_record.duplicate_header = true;
                }
            }

            in_seq_id = false;
            line_length = 0;

            // check: trailing whitespace
            if i > 2 && is_whitespace(contents[i - 2]) && !trailing_whitespace {
                trailing_whitespace = true;
            }

            line_start = false;

            // Note: this sequence occurs at every newline
            if format == "fastq" {
                if fastq_state == FastqState::Quality {
                    fastq_record.record_count += 1;

                    if sequence_length != quality_length {
                        fastq_record.seq_qual_mismatch = true;
                    }
                    sequence_length = 0;
                    quality_length = 0;
                }

                fastq_state = fastq_state.next();
                line_start = true;
            } else if format == "fasta" {
                // Increment max header length if current header is larger
                if fasta_state == FastaState::Header && header_len > fasta_record.max_header_length
                {
                    fasta_record.max_header_length = header_len;
                }
                // Toggle to sequence state if last line was a header
                if fasta_state == FastaState::Header {
                    fasta_state = FastaState::Sequence;
                }
                header_len = 0;
                line_start = true;
            }
        } else {
            line_length += 1;

            if format == "fastq" {
                match fastq_state {
                    FastqState::Header => {
                        if line_start && *byte != b'@' {
                            fastq_record.missing_header_character = true;
                        }
                    }
                    FastqState::Sequence => {
                        sequence_length += 1;

                        if !is_iupac_byte(*byte) && !fastq_record.bad_sequence {
                            fastq_record.bad_sequence = true;
                        }
                    }

                    FastqState::Separator => {
                        if line_start && *byte != b'+' && !fastq_record.missing_delimiter {
                            fastq_record.missing_delimiter = true;
                        }

                        // check if + line is empty
                        // next char should be a line ending
                        // TODO: check if identical to header line
                        if line_start && *&contents[i] != b'\n' {
                            fastq_record.empty_plus_line = false;
                        }
                    }

                    FastqState::Quality => {
                        quality_length += 1;

                        if *byte < 64 || *byte > 126 {
                            fastq_record.phred_64_compatible = false;
                        }

                        if *byte < 33 || *byte > 126 {
                            fastq_record.phred_33_compatible = false;
                        }

                        if *byte < 59 || *byte > 126 {
                            fastq_record.solexa_compatible = false;
                        }
                    }
                }
            }

            if format == "fasta" {
                if fasta_state == FastaState::Sequence && line_start && *byte == b'>' {
                    fasta_state = FastaState::Header;
                    header_len = 0;
                    in_seq_id = true;
                }

                if fasta_state == FastaState::Sequence && line_length > 80 {
                    fasta_record.long_sequence = true;
                }

                match fasta_state {
                    FastaState::Header => {
                        header_len += 1;

                        // Check for empty FastA records
                        if header_len == 1 {
                            if i > 2 && contents[i - 2] == b'>' {
                                fasta_record.empty_record = true;
                            }
                        }

                        // check: missing header
                        if (*byte == b'>' && i == 1)
                            || (i >= 2 && *byte == b'>' && contents[i - 2] == b'\n')
                        {
                            fasta_record.record_count += 1;
                            in_seq_id = true;
                        } else if *byte != b'>' && i == 1 {
                            fasta_record.missing_header_character = true;
                        }

                        // toggle off seqID after the first space
                        if *byte == b' ' {
                            in_seq_id = false;
                        }

                        // The SeqID can only include letters, digits, hyphens (-),
                        // underscores (_), periods (.), colons (:), asterisks (*),
                        // and number signs (#)
                        if in_seq_id
                            && header_len > 1
                            && !(byte.is_ascii_alphanumeric()
                                || matches!(byte, b'-' | b'_' | b'.' | b':' | b'*' | b'#'))
                        {
                            fasta_record.valid_seq_id = false;
                        }
                    }

                    FastaState::Sequence => {
                        sequence_length += 1;

                        if !is_iupac_byte(*byte) {
                            if fasta_record.valid_sequence {
                                println! {"- sequence contains invalid characters. \
                                Only IUPAC nucleotide symbols are allowed"};
                            }
                            fasta_record.valid_sequence = false;
                        }
                    }
                }
            }

            line_start = false;
        }
    }

    if format == "fastq" {
        if sequence_length != quality_length {
            eprintln!("- Sequence and quality line lengths do not match");
        }

        if !fastq_record.phred_33_compatible {
            eprintln!("- quality score incompabile with PHRED +33");
        }

        if fastq_record.phred_64_compatible {
                eprintln!("- quality score compabile with PHRED +64");
            }

        if fastq_record.phred_64_compatible {
            eprintln!("- quality score compabile with Solexa +64");
        }

        if fastq_record.record_count == 0 {
            eprintln!("- Zero records found");
        } else if fastq_record.record_count % 2 != 0 {
            eprintln!("- Odd number of records: {}", fastq_record.record_count);
        } else {
            eprintln!("- Even number of records: {}", fastq_record.record_count);
        }
    } else if format == "fasta" {
        if fasta_record.max_header_length > 25 {
            let header_len = fasta_record.max_header_length.to_string();
            eprintln! {"- header length exceeds 25 characters.\n\t\
                Longest header is {header_len} characters long"
            }
        }
    }

    let bytewise_results = ByteWiseCheck {
        is_ascii,
        contains_offensive_bytes,
        trailing_whitespace,
        empty_line,
        line_count,
    };

    match format {
        "fastq" => (bytewise_results, Some(fastq_record), None),
        "fasta" => (bytewise_results, None, Some(fasta_record)),
        _ => (bytewise_results, None, None),
    }
}

pub fn decode_reader(bytes: &Vec<u8>) -> io::Result<Vec<u8>> {
    let mut gz: GzDecoder<&[u8]> = GzDecoder::new(&bytes[..]);
    let mut decompressed_contents = Vec::new();
    let _ = gz.read_to_end(&mut decompressed_contents);

    Ok(decompressed_contents)
}
