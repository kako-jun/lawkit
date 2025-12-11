# Test Data Generation

Generate synthetic data following statistical laws for testing.

## Benford Data

```console
$ lawkit generate benf -s 1000
12345.67
234.56
...

```

## Pareto Data

```console
$ lawkit generate pareto -s 500
1.02
47.89
...

```

## Zipf Data

```console
$ lawkit generate zipf -s 1000
1150
575
383
...

```

## Normal Data

```console
$ lawkit generate normal -s 200
0.234
-1.456
...

```

## Poisson Data

```console
$ lawkit generate poisson -s 100
2
0
3
...

```

## Output to File

```console
$ lawkit generate benf -s 1000 > test_data.txt

```
