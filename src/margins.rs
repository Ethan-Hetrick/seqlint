const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
const UTF16_LE_BOM: [u8; 2] = [0xFF, 0xFE];
const UTF16_BE_BOM: [u8; 2] = [0xFE, 0xFF];
const UTF32_LE_BOM: [u8; 4] = [0xFF, 0xFE, 0x00, 0x00];
const UTF32_BE_BOM: [u8; 4] = [0x00, 0x00, 0xFE, 0xFF];

#[derive(Debug)]
pub struct Header {
    utf_bom: bool,
    pub gzip_magic: bool,
    deflate: bool,
    cram_magic: bool,
    bgzf_subfield: Option<&'static str>,
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
        contents.starts_with(&[31, 139])
    }

    fn is_deflate(contents: &Vec<u8>) -> bool {
        // 3rd byte set to 8 for DEFLATE]
        contents[2] == 8
    }

    fn cram_magic(contents: &Vec<u8>) -> bool {
        contents.starts_with(&[67, 82, 65, 77])
    }

    fn bgzf_header(gzip_magic: bool, contents: &Vec<u8>) -> Option<&'static str> {
        // 13th and 14th byte check
        if !gzip_magic {
            return None
        }

        // TODO: make skip verbose eventually
        match contents.get(12..14) {
            Some(b"BC") => Some("BC"),
            _ => None
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
        };

        header
    }

    pub fn report(&self) {
        println!("\nFile header checks:");
        // Error if BOM exists
        if self.utf_bom {
            eprintln!("- WARNING!: contains UTF BOM");
        }

        // Print if file is gzipped
        if self.gzip_magic {
            println!("- gzip-compressed");
        }

        if self.deflate {
            println! {"- compressed with DEFLATE"};
        }

        if self.cram_magic {
            println! {"- is a CRAM file"};
        }

        if let Some(subfield) = self.bgzf_subfield {
            eprintln!("- contains BGZF subfield: {}", subfield)
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
        contents[*&size - 1] == 0x0A
    }

    pub fn new(contents: &Vec<u8>, size: &usize) -> Footer {
        let footer = Footer {
            newline: Footer::check_final_newline(&contents, &size),
            bgzf_eof: Footer::bgzf_eof(&contents, &size),
        };

        footer
    }

    pub fn report(&self) {
        println!("\nFooter checks:");
        if self.bgzf_eof {
            println!("- contains valid BGZF EOF bytes");
        }
        if self.newline {
            println!("- contains final newline");
        }
    }
}
