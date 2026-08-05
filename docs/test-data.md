# Test data generation

```bash
printf '\xEF\xBB\xBFfoobar' > test/UTF-BOM.txt

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
```