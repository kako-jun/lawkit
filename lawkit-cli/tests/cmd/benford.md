# Benford's Law Analysis

Detect anomalies in numerical data using Benford's Law (first digit distribution).

```console
$ lawkit benf tests/fixtures/benf_data.txt
Benford Law Analysis Results

Dataset: tests/fixtures/benf_data.txt
Numbers analyzed: 100
[LOW] Dataset analysis

First Digit Distribution:
1: ███████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  30.0% (expected:  30.1%)
2: █████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  18.0% (expected:  17.6%)
3: ██████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  12.0% (expected:  12.5%)
...

```

## Quiet Mode

```console
$ lawkit benf tests/fixtures/benf_data.txt -q
chi_square: 1.234
p_value: 0.987
risk_level: Low

```

## JSON Output

```console
$ lawkit benf tests/fixtures/benf_data.txt -f json
{
  "dataset_name": "tests/fixtures/benf_data.txt",
  "numbers_analyzed": 100,
  "risk_level": "Low",
  ...
}

```
