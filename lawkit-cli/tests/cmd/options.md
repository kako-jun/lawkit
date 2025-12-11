# Common Options

## Filter Numbers

Filter input data by range before analysis.

```console
$ lawkit benf data.csv --filter ">=100"
...

$ lawkit benf data.csv --filter "<1000"
...

$ lawkit benf data.csv --filter "50-500"
...

```

## Minimum Count

Require minimum data points for analysis.

```console
$ lawkit benf small_data.txt -c 50
Error: Insufficient data (10 < 50 minimum)

```

## Quiet Mode

Minimal output (key metrics only).

```console
$ lawkit benf data.txt -q
chi_square: 1.234
p_value: 0.987
risk_level: Low

```

## Verbose Mode

Detailed output with additional statistics.

```console
$ lawkit benf data.txt -v
...
Additional Statistics:
  Mean Absolute Deviation: 0.002
  Kolmogorov-Smirnov: 0.045
  ...

```

## No Color

Disable ANSI color output.

```console
$ lawkit benf data.txt --no-color
...

```

## Version

```console
$ lawkit --version
lawkit 2.5.16

```
