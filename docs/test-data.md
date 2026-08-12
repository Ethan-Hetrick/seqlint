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
```

## Downloads

- [PhiX:](https://www.ncbi.nlm.nih.gov/nuccore/NC_001422.1?report=fasta)


## Manual

Anything else not mentioned was edited in manually via VSCode or nano text editors.