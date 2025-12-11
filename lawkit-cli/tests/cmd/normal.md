# Normal Distribution Analysis

Analyze data for normality (quality control, measurements).

```console
$ lawkit normal tests/fixtures/normal_data.txt
Normal Distribution Analysis Results

Dataset: tests/fixtures/normal_data.txt
Numbers analyzed: 100
Quality Level: High

Distribution Histogram:
 -2.50- -1.89: ██████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  11.5%
 -1.89- -1.28: █████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  34.0%
...

Distribution: μ=0.02, σ=1.01, Range: [-2.89, 3.12]
1σ: 68.5%, 2σ: 95.5%, 3σ: 99.7%

```

## Sigma Coverage Check

```console
$ lawkit normal tests/fixtures/normal_data.txt -v
...
Sigma Coverage:
  Within 1σ: 68.5% (expected: 68.27%)
  Within 2σ: 95.5% (expected: 95.45%)
  Within 3σ: 99.7% (expected: 99.73%)

```
