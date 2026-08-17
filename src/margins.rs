const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
const UTF16_LE_BOM: [u8; 2] = [0xFF, 0xFE];
const UTF16_BE_BOM: [u8; 2] = [0xFE, 0xFF];
const UTF32_LE_BOM: [u8; 4] = [0xFF, 0xFE, 0x00, 0x00];
const UTF32_BE_BOM: [u8; 4] = [0x00, 0x00, 0xFE, 0xFF];

use seqlint::{info, log, warn, fail};

#[derive(Debug)]
pub struct Header {
    utf_bom: bool,
    pub gzip_magic: bool,
    deflate: bool,
    cram_magic: bool,
    bgzf_subfield: Option<&'static str>,
    fourth_and_fifth_bytes_set: bool,
    xlen: Option<u16>,
    slen: Option<u16>,
    bsize: Option<u16>,
    //isize: u32,
}

impl Header {
    fn utf_bom(contents: &Vec<u8>) -> bool {
        contents.starts_with(&UTF8_BOM)
            || contents.starts_with(&UTF16_LE_BOM)
            || contents.starts_with(&UTF16_BE_BOM)
            || contents.starts_with(&UTF32_LE_BOM)
            || contents.starts_with(&UTF32_BE_BOM)
    }

    pub fn gzip_magic(contents: &Vec<u8>) -> bool {
        contents.get(0) == Some(&31) && contents.get(1) == Some(&139)
    }

    fn is_deflate(contents: &Vec<u8>) -> bool {
        contents.get(3) == Some(&8)
    }

    fn fourth_and_fifth_bytes_set(contents: &[u8]) -> bool {
        contents.get(3).is_some_and(|&b| b != 0) && contents.get(4).is_some_and(|&b| b != 0)
    }

    fn xlen(contents: &Vec<u8>) -> Option<u16> {
        let bytes: [u8; 2] = contents.get(10..12)?.try_into().ok()?;
        Some(u16::from_le_bytes(bytes))
    }

    fn slen(contents: &Vec<u8>) -> Option<u16> {
        let bytes: [u8; 2] = contents.get(14..16)?.try_into().ok()?;
        Some(u16::from_le_bytes(bytes))
    }

    // TODO: implement isize calculation
    // fn isize(contents: &Vec<u8>) -> u32 {
    //     u32::from_le_bytes([contents[23], contents[24], contents[25], contents[26]])
    // }

    fn cram_magic(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[67, 82, 65, 77])
    }

    fn bsize(contents: &Vec<u8>) -> Option<u16> {
        let bytes: [u8; 2] = contents.get(16..18)?.try_into().ok()?;
        Some(u16::from_le_bytes(bytes) + 1)
    }

    fn bgzf_header(gzip_magic: bool, contents: &Vec<u8>) -> Option<&'static str> {
        // 13th and 14th byte check
        if !gzip_magic {
            return None;
        }

        // TODO: make skip verbose eventually
        match contents.get(12..14) {
            Some(b"BC") => Some("BC"),
            _ => None,
        }
    }

    pub fn new(contents: &Vec<u8>) -> Header {
        // check: headers
        let gzip_magic = Header::gzip_magic(&contents);

        let header = Header {
            utf_bom: Header::utf_bom(&contents),
            gzip_magic,
            deflate: Header::is_deflate(&contents),
            cram_magic: Header::cram_magic(&contents),
            bgzf_subfield: Header::bgzf_header(gzip_magic, &contents),
            fourth_and_fifth_bytes_set: Header::fourth_and_fifth_bytes_set(&contents),
            xlen: Header::xlen(&contents),
            slen: Header::slen(&contents),
            bsize: Header::bsize(&contents),
            //isize: Header::isize(&contents),
        };

        header
    }

    pub fn report(&self) {
        let mut bgzf_subfield_valid: bool = false;

        log!("== File header checks ==");
        // Error if BOM exists
        if self.utf_bom {
            fail!("- contains UTF BOM");
        }

        //let _isize = dbg!(self.isize);

        // Print if file is gzipped
        if self.gzip_magic {
            info!("gzip-compressed");
        }

        if self.deflate {
            info! {"compressed with DEFLATE"};
        }

        if self.cram_magic {
            info! {"is a CRAM file"};
        }

        if let Some(_subfield) = self.bgzf_subfield {
            info!("contains BGZF subfield 'BC'");
            bgzf_subfield_valid = true;
        }

        if self.fourth_and_fifth_bytes_set {
            info!("bytes 4-5 are set");
        }

        if self.xlen == Some(6) && self.slen == Some(2) {
            info!("xlen and slen bytes are BGZF compatible");

            if self.bsize == Some(0) {
                warn!("BGZF block size = 0")
            }
        }

        if self.xlen == Some(6)
            && self.slen == Some(2)
            && bgzf_subfield_valid
            && self.deflate
            && self.gzip_magic
        {
            info!("BAM file detected");
        }


    }
}

pub struct Footer {
    pub newline: bool,
    pub bgzf_eof: bool,
}

impl Footer {
    fn bgzf_eof(contents: &Vec<u8>, size: &usize) -> bool {
        let eof = vec![
            0x1f, 0x8b, 0x08, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x06, 0x00, 0x42, 0x43,
            0x02, 0x00, 0x1b, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        *size >= eof.len() && contents[*size - eof.len()..] == eof
    }

    fn check_final_newline(contents: &[u8], size: &usize) -> bool {
        contents[*&size - 1] == b'\n'
    }

    pub fn new(contents: &Vec<u8>, size: &usize) -> Footer {
        let footer = Footer {
            newline: Footer::check_final_newline(&contents, &size),
            bgzf_eof: Footer::bgzf_eof(&contents, &size),
        };

        footer
    }

    pub fn report(&self) {
        log!("== Footer checks ==");
        if self.bgzf_eof {
            info!("contains valid BGZF EOF bytes");
        }
        if !self.newline && !self.bgzf_eof {
            warn!("missing final newline/return character")
        }
    }
}
