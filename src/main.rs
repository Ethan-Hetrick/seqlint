use std::{env,io};
use std::fs;

// FASTA reference: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/
// FASTQ reference: https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files

fn main() -> io::Result<()> {

    for path in env::args().skip(1) {

            // check: does exist
            assert!(fs::exists(&path).unwrap(), "\n\tERROR: Unable to access file {path}\n");

            // check: is not dir
            // TODO: better error message for no read access
            let metadata = fs::metadata(&path)?;

            assert!(!metadata.is_dir(), "\nERROR: {path} is a directory\n");

            // check: is not empty
            assert!(metadata.len() > 0, "\nERROR: {path} is empty\n");

            // Load file
            let contents: Vec<u8> = fs::read(&path)?;

            // check: does not have UTF BOM
            assert_ne!(&contents[0..3], [0xEF, 0xBB, 0xBF], "\n\nERROR: {path} contains UTF BOM. Remove it using:\n\n\t\tdos2unix --remove-bom {path}\n");

            // check: has gzip magic bytes
            let mut gzip: bool = false;
            if &contents[0..2] == [0x1F, 0x8B] {
                println!("\n{path} is gzip-compressed\n");
                gzip = true;
            }

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

            let mut counter: u32 = 0;
            let mut long: bool = false;
            for byte in contents.iter() {
                // check: all bytes are ASCII
                if !gzip { assert!(byte.is_ascii(), "\nERROR: {path} contains non-ASCII bytes.\n"); }

                // check: offending ASCII bytes
                if !gzip { assert!(!offenders.contains(byte)); }

                // count lines
                if *byte == 0x0A {
                    counter = 0;
                } else {
                    counter += 1;
                    if counter > 80 && !long {
                        println!("\nWARNING: {path} contains lines > 80 characters long\n");
                        long = true;
                    }
                }

            }

            // check: FASTA header
            if contents.starts_with(&[0x3E]) {
                println!("\n{path} contains FASTA header");
            // check: FASTQ header
            } else if contents.starts_with(&[0x40]) {
                println!("\n{path} contains FASTQ header");
            }

    }

    Ok(())

}