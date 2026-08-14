# Test data generation

## Versions

GNU coreutils 8.30
bgzip (htslib) 1.23.1
samtools 1.23.1, Using htslib 1.23.1


## Commands
```bash
printf '\xEF\xBB\xBFfoobar' > test/fixtures/UTF8-BOM.txt
printf '\xFF\xFEfoobar' > test/fixtures/UTF16-LE-BOM.txt
printf '\xFE\xFFfoobar' > test/fixtures/UTF16-BE-BOM.txt
printf '\xFF\xFE\x00\x00foobar' > test/fixtures/UTF32-LE-BOM.txt
printf '\x00\x00\xFE\xFFfoobar' > test/fixtures/UTF32-BE-BOM.txt


cat > test/test.fasta <<'EOF'
>foobar_1|foo|BAR1451.1 hypothetical protein, complete cds [organism=Escherichia coli]
ATGCGATCGTAGCTAGCTAGCGTAGCTAGCATCGATCGATCGTAGCTAGCTAGCTAGCTA
GCTAGCTAGCGATCGATCGTAGCTAGCNNNNNNNNNCTAGCTAGCTAGCTAGCTAGCATC
GATCGATCGTAGCTAGCTAGCTAGCATCGATCGTAGCTAGCTAG
>foobar_2|foo|BAR1452.1 another sequence, partial
atgcatgcatgcatgcatgcatgcatgcgctagctagcyrwskmbdhvnATGCATGCATGC
ATGCATGCATGCATGCATGCATGC
>gi|61393989|gb|AY848686.1| Some virus complete genome
ATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGC
EOF

gzip -c test/test.fasta > test/test.fasta.gz

cat > test/test.fastq <<'EOF'
@SEQ_ID_1 length=36
GGGTGATGGCCGCTGCCGATGGCGTCAAATCCCACC
+
IIIIIIIIIIIIIIIIIIIIIIIIIIIIII9IG9IC
@SEQ_ID_2 length=36
GTTCAGGGATACGACGTTTGTATTTTAAGAATCTGA
+SEQ_ID_2 length=36
IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII6IBI
EOF

printf '>foobar\n' > test/long.fasta && printf 'A%.0s' {1..1000} >> test/long.fasta

printf '\x3E\x00\x00\x00' > test/null.fasta

printf '\x3E\x0D\x0D\x0D' > test/cr.fasta

head -c -1 test/test.fasta > test/no-last-newline.fasta

sed 's/$/\x20/' test/test.fastq > test/trailing.fastq

sed 's/$/\n   /' test/test.fasta > test/empty-lines.fasta

bgzip -c test/test.fastq > test/bgzf-test.fastq.gz

head -3 test.test.fastq > test/3-line.fastq

sed 's/@//g' test/test.fastq > test/missing-@.fastq

samtools import -0 test/fixtures/test.fastq -O 'SAM' > test/fixtures/test.fastq.sam

samtools import -0 test/fixtures/test.fastq -O 'BAM' > test/fixtures/test.fastq.bam

samtools import -0 test/fixtures/test.fastq -O 'CRAM' > test/fixtures/test.fastq.cram

sed 's|>|>\n>|g' test/fixtures/test.fasta > test/fixtures/test-empty-record.fasta

# Recycled synthetic illumina reads generated with bbtools randomreads.sh (version unsure)
zcat ~/assets/synthetic_reads/GCF-000007765-2-ASM776v2_1.fastq.gz | head -12 > test/fixtures/test_R1_001.fastq && printf '/n' >> test/fixture
s/test_R1_001.fastq
zcat ~/assets/synthetic_reads/GCF-000007765-2-ASM776v2_2.fastq.gz | head -12 > test/fixtures/test_R2_001.fastq && printf '/n' >> test/fixture
s/test_R2_001.fastq

cp test/fixtures/test_R1_001.fastq test/fixtures/test_01.fastq
cp test/fixtures/test_R2_001.fastq test/fixtures/test_02.fastq

head -8 test/fixtures/test.fastq > test/fixtures/2-records.fastq && printf '/n' >> test/fixtures/2-records.fastq

printf '>foobar\n>foobar\n' > test/fixtures/duplicate_header.fasta

# non UTF-8 byte in name
cp test.fasta $'test_\xff.fasta'

# phred 33 read
printf '@phred33\n%s\n+\n%s\n' "$(printf 'A%.0s' {1..94})" "$(printf '\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2a\x2b\x2c\x2d\x2e\x2f\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3a\x3b\x3c\x3d\x3e\x3f\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4a\x4b\x4c\x4d\x4e\x4f\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5a\x5b\x5c\x5d\x5e\x5f\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6a\x6b\x6c\x6d\x6e\x6f\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7a\x7b\x7c\x7d\x7e')" > test/fixtures/phred33.fastq


printf '@phred64\n%s\n+\n%s\n' "$(printf 'A%.0s' {1..63})" "$(printf '\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4a\x4b\x4c\x4d\x4e\x4f\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5a\x5b\x5c\x5d\x5e\x5f\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6a\x6b\x6c\x6d\x6e\x6f\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7a\x7b\x7c\x7d\x7e')" > test/fixtures/phred64.fastq

printf '@solexa\n%s\n+\n%s\n' "$(printf 'A%.0s' {1..68})" "$(printf '\x3b\x3c\x3d\x3e\x3f\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4a\x4b\x4c\x4d\x4e\x4f\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5a\x5b\x5c\x5d\x5e\x5f\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6a\x6b\x6c\x6d\x6e\x6f\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7a\x7b\x7c\x7d\x7e')" > test/fixtures/solexa.fastq
```

## Downloads

- [PhiX:](https://www.ncbi.nlm.nih.gov/nuccore/NC_001422.1?report=fasta)


## Manual

Anything else not mentioned was edited in manually via VSCode or nano text editors.