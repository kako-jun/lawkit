# lawkit Test: statistical/benford_basic

**Description:** Verify Benford's law analysis produces correct first digit distribution with appropriate visualization and statistical assessment

**Command:** `lawkit benf tests/fixtures/sample_data.csv`

**Generated:** Mon Jul 14 03:30:00 PM JST 2025
**Version:** v2.4.1

## Command Output

```
Benford Law Analysis Results

Dataset: sample_data.csv
Numbers analyzed: 100
[LOW] Dataset analysis

First Digit Distribution:
1: ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░  36.0% (expected:  30.1%)
2: ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  16.0% (expected:  17.6%)
3: █████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  12.0% (expected:  12.5%)
4: ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  10.0% (expected:   9.7%)
5: ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   5.0% (expected:   7.9%)
6: ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   7.0% (expected:   6.7%)
7: ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   4.0% (expected:   5.8%)
8: ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   6.0% (expected:   5.1%)
9: ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   4.0% (expected:   4.6%)
```

**Exit Code:** 0

✅ **Status:** SUCCESS

---