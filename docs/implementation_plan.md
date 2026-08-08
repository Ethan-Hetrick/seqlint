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
  - [x] UTF-8 Byte Order Mark (BOM)
  - [ ] BZGF End-of-file (EOF) mark
  - [] Other BOMs
  - [] format versioning

- Scanning (byte-wise) checks:
  - [x] ASCII compatible
  - [x] Offending ASCII-byes (e.g control characters, NUL, CR..)

- File type pipelines:
  - FASTQ:
    - [x] File extension is a standard extension
    - [x] First character is "@"
    - [] sequence is IUPAC compatible
    - [] paired-end checks (e.g. names match, same # records)
    - [] interleaved checks (e.g. even # records)
    - [] sequence len = qual len
  - FASTA:
    -  [x] File extension is a standard extension
    -  [x] First character is ">"
    -  [x] empty records
    -  [x] NCBI seqID > 25 characters
    -  [x] NCBI seqID contains only accepted characters
    -  [x] sequence is IUPAC compatible
    -  [ ] unique header / seqID
  - SAM | BAM | CRAM:
  - - [] File extension is a standard extension
    - [] Automatic detection / decompression of BAM/CRA
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
  - [] bgzip automatically detected and decompressed for file checks
  - [] CRAM automatically detected and decompressed for file checks

## User experience

- [] separate errors, warnings, compatibility or informational
- [] tabularize output: https://docs.rs/crate/tabled/latest