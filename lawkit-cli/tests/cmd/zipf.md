# Zipf's Law Analysis

Analyze rank-frequency distributions (word frequencies, city populations, etc.).

```console
$ lawkit zipf tests/fixtures/zipf_data.txt
Zipf Law Analysis Results

Dataset: tests/fixtures/zipf_data.txt
Numbers analyzed: 100
[LOW] Dataset analysis

Rank-Frequency Distribution:
# 1: █████████████████████████████████████████████████┃  11.50%
# 2: █████████████████████████┃░░░░░░░░░░░░░░░░░░░░░░░   5.75%
# 3: █████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   3.83%
...

Zipf Exponent: 1.02 (ideal: 1.0), Correlation: 0.998

```

## Verbose Output

```console
$ lawkit zipf tests/fixtures/zipf_data.txt -v
...
Top 10 by rank:
  #1: 1150 (11.50%)
  #2: 575 (5.75%)
  ...

```
