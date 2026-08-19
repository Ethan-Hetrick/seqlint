const BAM_HEADER: &[u8; 4] = b"BAM\x01";

use seqlint::{info,log};

// BAM specific file checks
pub struct Bam {
    bam_header: bool
}

impl Bam {
    pub fn new(contents: &Vec<u8>) -> Bam {
        let bam = Bam {
            bam_header: contents.starts_with(BAM_HEADER)
        };

        bam
    }

    pub fn report(&self) {
        log!("== BAM checks ==");
        if self.bam_header == true {
            info!("BAM header present")
        }
    }
}
