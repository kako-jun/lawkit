# Output Formats

## Text (Default)

Human-readable output with visual bars.

```console
$ lawkit benf tests/fixtures/benf_data.txt
Benford Law Analysis Results
...

```

## JSON

Machine-readable JSON for automation.

```console
$ lawkit benf tests/fixtures/benf_data.txt -f json
{
  "dataset_name": "tests/fixtures/benf_data.txt",
  "numbers_analyzed": 100,
  "risk_level": "Low",
  "chi_square": 1.234,
  "p_value": 0.987,
  ...
}

```

## CSV

Tabular output for spreadsheets.

```console
$ lawkit benf tests/fixtures/benf_data.txt -f csv
dataset,numbers_analyzed,risk_level,chi_square,p_value,mad
tests/fixtures/benf_data.txt,100,Low,1.234,0.987,0.002

```

## YAML

```console
$ lawkit benf tests/fixtures/benf_data.txt -f yaml
dataset_name: tests/fixtures/benf_data.txt
numbers_analyzed: 100
risk_level: Low
...

```

## TOML

```console
$ lawkit benf tests/fixtures/benf_data.txt -f toml
dataset_name = "tests/fixtures/benf_data.txt"
numbers_analyzed = 100
risk_level = "Low"
...

```

## XML

```console
$ lawkit benf tests/fixtures/benf_data.txt -f xml
<?xml version="1.0" encoding="UTF-8"?>
<benford_analysis>
  <dataset>tests/fixtures/benf_data.txt</dataset>
  ...
</benford_analysis>

```
