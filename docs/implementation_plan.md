# Implementation plan

Doc to keep track of features I've added and want to add:

## General file integrity checks

- General heuristics:
  - File path is valid and readable
  - File is not empty and has at least 4 bytes for processing purposes
  - That decompressed data is not empty
  - Whitespace-only files
  - compression formats complete header and truncation check

- Magic byte checks
  - [x] gzip
  - [x] DEFLATE
  - [x] bgzip
  - [x] CRAM
  - [x] Byte Order Marks (BOM)
        - [UTF8, UTF16_LE, UTF16_BE, UTF32_LE, UTF32_BE](https://learn.microsoft.com/en-us/windows/win32/intl/using-byte-order-marks)
  - [x] BZGF End-of-file (EOF) mark
  - [x] BGZF header. 13th/14th byte subfield ID BC || DC || EC
  - [] format versioning

- Scanning (byte-wise) checks:
  - [x] ASCII compatible
  - [x] Offending ASCII-byes (e.g control characters, NUL, CR..)

- File type pipelines:
  - FASTQ:
    - [x] File extension is a standard extension
    - [x] First character is "@"
    - [x] 1st line of every record begins with "@"
    - [x] 2nd line of every record begins with "+"
    - [x] sequence is IUPAC compatible
    - [x] detect paired-end read nomenclature in file names "<..>_R{1,2}_<..>", "<..>_0{1,2}<.f..>"
    - [x] report record count and if it is even or odd
    - [x] sequence len = qual len
  - FASTA:
    - [x] File extension is a standard extension
    - [x] First character is ">"
    - [x] empty records
    - [x] NCBI seqID > 25 characters
    - [x] NCBI seqID contains only accepted characters
    - [x] sequence is IUPAC compatible
    - [x] duplicate header
    - [x] report record count and if it is even or odd
  - SAM | BAM | CRAM:
  - - [] File extension is a standard extension
    - [] Automatic detection / decompression of BAM/CRAM
    - [] Header syntax
    - [] 11 fields
    - [] valid delimiter
    - [] sequence len = qual len
  - VCF | BCF
  - - [] File extension is a standard extension
    - [] automatic detection / decompression of BCF
    - [] accidental datetime formatting
  - GFF
    - File extension is a standard extension
  - CSV | TSV | other textual data formats
    - Delimiter matches file extension
    - File extension is a standard extension
    - Properly quoted fields
    - rows have valid # fields

- Other:
  - [x] File ends with valid newline character
  - [x] gzip automatically detected and decompressed for file checks
  - [x] trailing whitespace check
  - [x] lines don't exceed 80 characters (might move to FASTA sequence check per NCBI specification)
  - [x] gzip-compressed file contents not empty
  - [] bgzip automatically detected and decompressed for file checks
  - [] CRAM automatically detected and decompressed for file checks

## User experience

- [] separate errors, warnings, compatibility or informational
- [] tabularize output: https://docs.rs/crate/tabled/latest