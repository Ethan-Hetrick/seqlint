const BAM_MAGIC: &[u8; 4] = b"BAM\x01";

use seqlint::{info,log,warn};

// BAM specific file checks
pub struct Bam {
    bam_magic: bool,
    bam_header: Option<String>,
}

impl Bam {
    pub fn new(contents: &Vec<u8>) -> Bam {
        let bam_magic = contents.starts_with(BAM_MAGIC);

        let bam_header = if bam_magic {
            Bam::bam_header(&contents[5..])
        } else {
            None
        };

        let bam = Bam {
            bam_magic: bam_magic,
            bam_header: bam_header,
        };

        bam
    }

    fn bam_header(contents: &[u8]) -> Option<String> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut i = 0;

        for byte in contents.iter() {
            if *byte == b'\n' && contents.get(i + 1) != Some(&b'@') {
                return Some(String::from_utf8_lossy(&bytes)
                    .to_string()
                    .split_whitespace().collect::<Vec<_>>().join(" "));
            } else {
                bytes.push(*byte);
            }

            i += 1;
        }

        None
    }

    pub fn report(&self) {
        log!("== BAM checks ==");
        if self.bam_magic == true {
            info!("BAM magic string present")
        }

        match &self.bam_header {
            Some(header) => info!("BAM header: {header}"),
            _ => warn!("No BAM header returned")
        }
    }
}
