# seqlint

>[!WARNING] Work in progress! This project is under active development. Features, behavior, and the CLI interface are subject to change.

A linter for bioinformatics sequence files. `seqlint` performs integrity checks, format validation, and heuristic analysis on common sequence and alignment file formats.

## Disclaimer

This is the first Rust project I have attempted manually (minimal use of gen AI), and have decided that every commit will be performed by a human. I started this project on my Google Pixel 10 using [Rustroid](https://rustroid.is-a.dev/) in an attempt to replace doomscrolling with teaching myself Rust. Therefore, I'm only using genAI for code review and debugging. I currently only specify a flate2 dependency for gzip-decompression, all other code is implemented in-house.

## Usage

```
seqlint <fasta,fastq> <file1 file2 file3 ...>
```

## Features

### General file integrity checks

**Heuristics**

- [x] File path is valid and readable
- [x] File is not empty and has at least 4 bytes for processing
- [x] Decompressed data is not empty
- [x] Whitespace-only file detection
- [x] Compression format complete-header and truncation check

**Magic byte checks**

- [x] gzip
- [x] DEFLATE
- [x] bgzip
- [x] CRAM
- [x] Byte Order Marks (BOM) — [UTF8, UTF16_LE, UTF16_BE, UTF32_LE, UTF32_BE](https://learn.microsoft.com/en-us/windows/win32/intl/using-byte-order-marks)
- [x] BGZF End-of-file (EOF) mark
- [x] BGZF header — 13th/14th byte subfield ID `BC` / `DC` / `EC`
- [ ] Format versioning

**Byte-wise scanning**

- [x] ASCII compatibility
- [x] Offending ASCII bytes (control characters, NUL, CR, ...)

### File type pipelines

**FASTQ**

- [x] File extension is a standard extension
- [x] First character is `@`
- [x] 1st line of every record begins with `@`
- [x] 2nd line of every record begins with `+`
- [x] Sequence is IUPAC compatible
- [x] Detect paired-end read nomenclature in filenames (`<..>_R{1,2}_<..>`, `<..>_0{1,2}<.f..>`)
- [x] Report record count and parity (even/odd)
- [x] Sequence length equals quality length
- [ ] Phred quality encoding
- [ ] Sequence ID check
- [ ] Optional seqname after `+` must match the seqname following `@`

**FASTA**

- [x] File extension is a standard extension
- [x] First character is `>`
- [x] Empty record detection
- [x] NCBI seqID longer than 25 characters
- [x] NCBI seqID contains only accepted characters
- [x] Sequence is IUPAC compatible
- [x] Duplicate header detection
- [x] Report record count and parity (even/odd)

**SAM / BAM / CRAM**

- [ ] File extension is a standard extension
- [ ] Automatic detection / decompression of BAM/CRAM
- [ ] Header syntax
- [ ] 11 fields
- [ ] Valid delimiter
- [ ] Sequence length equals quality length

**VCF / BCF**

- [ ] File extension is a standard extension
- [ ] Automatic detection / decompression of BCF
- [ ] Accidental datetime formatting

**GFF**

- [ ] File extension is a standard extension

**CSV / TSV / other textual formats**

- [ ] Delimiter matches file extension
- [ ] File extension is a standard extension
- [ ] Properly quoted fields
- [ ] Rows have valid field count

**Index formats**

- [ ] Tabix index (.tbi) / Coordinate Sorted Index (.csi)

### Other checks

- [x] File ends with a valid newline character
- [x] gzip automatically detected and decompressed for file checks
- [x] Trailing whitespace check
- [x] Lines don't exceed 80 characters (may move to FASTA sequence check per NCBI spec)
- [x] gzip-compressed file contents not empty
- [ ] bgzip automatically detected and decompressed for file checks
- [ ] CRAM automatically detected and decompressed for file checks

## Roadmap

**User experience**

- [ ] Separate errors, warnings, compatibility, and informational messages
- [ ] Tabularized output ([tabled](https://docs.rs/crate/tabled/latest))

## Resources

- [MAQ FASTQ specification](https://maq.sourceforge.net/fastq.shtml)
- [UCSC Genome Browser FAQ: File Formats](https://genome.ucsc.edu/FAQ/FAQformat.html)
- [NCBI FASTA format specification](https://www.ncbi.nlm.nih.gov/genbank/fastaformat/)
- [NCBI FASTQ format specification](https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files)