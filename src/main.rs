use std::{env,io};
use std::fs;

// FASTA reference: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/
// FASTQ reference: https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files

fn main() -> io::Result<()> {

    for path in env::args().skip(1) {

            // Check if file exists
            assert!(fs::exists(&path).unwrap(), "\n\tERROR: Unable to access file {path}\n");

            // Make sure file is not a dir
            let metadata = fs::metadata(&path)?;
            assert!(!metadata.is_dir(), "\nERROR: {path} is a directory\n");

            // Make sure file is not empty
            assert!(metadata.len() > 0, "\nERROR: {path} is empty\n");

            // Load file
            let contents: Vec<u8> = fs::read(&path)?;

            // Check for UTF BOM
            assert_ne!(&contents[0..3], [0xEF, 0xBB, 0xBF], "\n\nERROR: {path} contains UTF BOM. Remove it using:\n\n\t\tdos2unix --remove-bom {path}\n");

            // Check for gzip magic bytes
            if &contents[0..2] == [0x1F, 0x8B] {
                println!("\n{path} is gzip-compressed\n");
            }

            // FASTA header check
            if contents.starts_with(&[0x3E]) {
                println!("\n{path} contains FASTA header");
            // FASTQ header check
            } else if contents.starts_with(&[0x40]) {
                println!("\n{path} contains FASTQ header");
            }


    }

    Ok(())

}