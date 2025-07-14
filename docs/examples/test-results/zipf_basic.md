# lawkit Test: statistical/zipf_basic

**Description:** Verify Zipf's law analysis produces correct rank-frequency distribution with exponent and correlation calculations

**Command:** `lawkit zipf tests/fixtures/sample_data.csv`

**Generated:** Mon Jul 14 03:30:00 PM JST 2025
**Version:** v2.4.1

## Command Output

```
Zipf Law Analysis Results

Dataset: sample_data.csv
Numbers analyzed: 100
[CRITICAL] Dataset analysis

Rank-Frequency Distribution:
# 1: ██████████████████████████████████████████████████  10.42% (freq: 9060)
# 2: ███████████████████████████████████████████████░░░   9.89% (freq: 8594)
# 3: ███████████████████████████████████████░░░░░░░░░░░   8.19% (freq: 7117)
# 4: █████████████████████████████████░░░░░░░░░░░░░░░░░   6.84% (freq: 5944)
# 5: ██████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░   5.50% (freq: 4780)
# 6: ████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░   4.90% (freq: 4259)
# 7: ███████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░   4.77% (freq: 4145)
# 8: ███████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   3.90% (freq: 3387)
# 9: ████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   3.24% (freq: 2820)
#10: ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   2.89% (freq: 2511)

Zipf Exponent: 2.598 (ideal: 1.0), Correlation: 0.881
```

**Exit Code:** 0

✅ **Status:** SUCCESS

---