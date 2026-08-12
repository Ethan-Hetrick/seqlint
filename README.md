# seqlint

>WARNING!
>This project is under active development. Features, behavior, and the CLI interface are subject to change.

A linter for bioinformatics sequence files. `seqlint` performs integrity checks, format validation, and heuristic analysis on common sequence and alignment file formats.

## Disclaimer

This is the first Rust project I have attempted manually (minimal use of gen AI), and have decided that every commit will be performed by a human. I am also new to Rust, code quality qill improve as I do.

I'm only using genAI (mostly Claude Opus 4.8, no agents, just short web chat sessions) for code review, debugging and certain trivial formatting tasks, [all reviewed carefully](https://forge.rust-lang.org/policies/llm-usage.html).

> Fun fact: I started this project on my Google Pixel 10 using [Rustroid](https://rustroid.is-a.dev/) in an attempt to replace doomscrolling with teaching myself Rust

## Features

### General file integrity checks

**Heuristics**

- [x] File path exists and is readable
- [x] File path itself is exclusively UTF-8 compliant
- [x] File is not empty and has at least 4 bytes for processing
- [x] Decompressed data is not empty
- [x] Whitespace-only file detection

**Magic byte checks**

- [x] gzip
- [x] DEFLATE
- [x] BGZF (Blocked GNU Zip Format)
- [x] CRAM
- [x] Byte Order Marks (BOM) — [UTF8, UTF16_LE, UTF16_BE, UTF32_LE, UTF32_BE](https://learn.microsoft.com/en-us/windows/win32/intl/using-byte-order-marks)
- [x] BGZF End-of-file (EOF) mark
- [x] BGZF header — 13th/14th byte subfield ID `BC`
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

- [x] CLI
- [ ] Separate errors, warnings, compatibility, and informational messages
- [ ] Tabularized output ([tabled](https://docs.rs/crate/tabled/latest))

**QoL**

- [x] CLI skips duplicate input files by their canonical path

## Specifications and references

>WARNING: the below list was half AI-generated. This is for my review only as I develop this tool

### Core sequence formats

- [FASTQ format — Cock et al. (2010)](https://pubmed.ncbi.nlm.nih.gov/20015970/)
- [MAQ FASTQ documentation (historical)](https://maq.sourceforge.net/fastq.shtml)
- [NCBI FASTA format requirements](https://www.ncbi.nlm.nih.gov/genbank/fastaformat/)
- [NCBI SRA FASTQ format requirements](https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#fastq-files)

### Compression and container formats

- [RFC 1951 — DEFLATE Compressed Data Format Specification](https://www.rfc-editor.org/rfc/rfc1951)
- [RFC 1952 — GZIP File Format Specification](https://www.rfc-editor.org/rfc/rfc1952)
- [SAM/BAM and BGZF format specification](https://samtools.github.io/hts-specs/SAMv1.pdf)
- [CRAM 3.x format specification](https://samtools.github.io/hts-specs/CRAMv3.pdf)
- [CRAM codecs specification](https://samtools.github.io/hts-specs/CRAMcodecs.pdf)

### Alignment formats

- [GA4GH HTS format specifications](https://samtools.github.io/hts-specs/)
- [SAM/BAM format specification](https://samtools.github.io/hts-specs/SAMv1.pdf)
- [SAM/BAM/CRAM optional fields specification](https://samtools.github.io/hts-specs/SAMtags.pdf)
- [CRAM 3.x format specification](https://samtools.github.io/hts-specs/CRAMv3.pdf)
- [CRAM codecs specification](https://samtools.github.io/hts-specs/CRAMcodecs.pdf)

### Variant formats

- [VCF/BCF 4.5 format specification](https://samtools.github.io/hts-specs/VCFv4.5.pdf)
- [BCF 2 quick reference](https://samtools.github.io/hts-specs/BCFv2_qref.pdf)

### Index formats

- [BAI specification — SAM/BAM specification](https://samtools.github.io/hts-specs/SAMv1.pdf)
- [CSI v1 specification](https://samtools.github.io/hts-specs/CSIv1.pdf)
- [Tabix index specification](https://samtools.github.io/hts-specs/tabix.pdf)

### Genomic feature formats

- [GA4GH BED v1 specification](https://samtools.github.io/hts-specs/BEDv1.pdf)
- [Sequence Ontology GFF3 specification](https://github.com/The-Sequence-Ontology/Specifications/blob/master/gff3.md)
- [NCBI GFF3 conventions](https://www.ncbi.nlm.nih.gov/datasets/docs/v2/reference-docs/file-formats/annotation-files/about-ncbi-gff3/)
- [UCSC Genome Browser FAQ: File Formats](https://genome.ucsc.edu/FAQ/FAQformat.html)

### Delimited text formats

- [RFC 4180 — Common Format and MIME Type for CSV Files](https://www.rfc-editor.org/rfc/rfc4180)

### Sequence alphabets and identifiers

- [IUPAC-IUB nucleotide nomenclature](https://www.bioinformatics.org/sms/iupac.html)
- [NCBI FASTA sequence identifier requirements](https://www.ncbi.nlm.nih.gov/genbank/fastaformat/)

### Repository and archive compatibility

- [NCBI SRA File Format Guide](https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/)
- [NCBI SRA Submission Guide](https://www.ncbi.nlm.nih.gov/sra/docs/submit/)
- [ENA accepted read data formats and validation requirements](https://ena-docs.readthedocs.io/en/latest/fileprep/reads.html)
- [NCBI GFF3 conventions](https://www.ncbi.nlm.nih.gov/datasets/docs/v2/reference-docs/file-formats/annotation-files/about-ncbi-gff3/)

### Sequencing-platform conventions

- [Illumina BCL Convert FASTQ output conventions](https://support-docs.illumina.com/SW/BCL_Convert/Content/SW/BCLConvert/OutputFiles__swBCL_swBS_appBCL.htm)
- [PacBio BAM format specification](https://pacbiofileformats.readthedocs.io/en/latest/BAM.html)

### Encoding and text handling

- [Unicode Standard — Byte Order Mark](https://www.unicode.org/versions/latest/)
- [Microsoft: Using Byte Order Marks](https://learn.microsoft.com/en-us/windows/win32/intl/using-byte-order-marks)