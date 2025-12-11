# Pareto Principle Analysis

Analyze wealth/value concentration using the 80/20 rule.

```console
$ lawkit pareto tests/fixtures/pareto_data.txt
Pareto Principle (80/20 Rule) Analysis Results

Dataset: tests/fixtures/pareto_data.txt
Numbers analyzed: 100
[LOW] Dataset analysis

Lorenz Curve (Cumulative Distribution):
 20%: ███████████████████████████████████████┃░░░░░░░░░░  79.2% cumulative (80/20 point)
 40%: █████████████████████████████████████████████░░░░░  91.5% cumulative
...

80/20 Rule: Top 20% owns 79.2% of total wealth (Ideal: 80.0%, Ratio: 0.99)

```

## Custom Percentiles

```console
$ lawkit pareto tests/fixtures/pareto_data.txt --percentile 10,50,90
...
Custom Percentiles:
  10%: 45.2%
  50%: 87.3%
  90%: 98.1%

```
