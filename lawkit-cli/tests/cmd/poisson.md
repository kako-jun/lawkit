# Poisson Distribution Analysis

Analyze rare event counts (defects, arrivals, incidents).

```console
$ lawkit poisson tests/fixtures/poisson_data.txt
Poisson Distribution Analysis Results

Dataset: tests/fixtures/poisson_data.txt
Numbers analyzed: 100
Quality Level: High

Probability Distribution:
P(X= 0): ██████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0.095
P(X= 1): ███████████████████████████████████████████┃░░░░░░  0.224
P(X= 2): █████████████████████████████████████████████████┃  0.263
...

λ=2.35, Variance/Mean=1.02 (ideal: 1.0), Fit Score=0.95

```

## Expected Lambda

```console
$ lawkit poisson tests/fixtures/poisson_data.txt --lambda 2.0
...
Expected λ: 2.0, Observed λ: 2.35
Deviation: 17.5%

```
