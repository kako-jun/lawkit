# lawkit 動作実証

v2.4.1の実際の動作を示す簡潔な例集

## 基本統計法則

**Benford法則** [`benford-basic.txt`](outputs/benford-basic.txt)
```bash
lawkit benford data/financial_data.csv
```

**Pareto分析** [`pareto-basic.txt`](outputs/pareto-basic.txt) 
```bash
lawkit pareto data/sales_data.csv
```

**Zipf法則** [`zipf-basic.txt`](outputs/zipf-basic.txt)
```bash
lawkit zipf data/text_frequency.csv
```

## 高度分析

**統合分析** [`integrated-analysis.txt`](outputs/integrated-analysis.txt)
```bash
lawkit analyze data/comprehensive_data.csv --all-laws
```

**異常値検出** [`anomaly-detection.txt`](outputs/anomaly-detection.txt)
```bash
lawkit detect data/sensor_data.csv --method isolation-forest
```

**JSON出力** [`output-json.txt`](outputs/output-json.txt)
```bash
lawkit benford data/financial_data.csv --output json
```

## 詳細オプション

**信頼区間設定** [`confidence-interval.txt`](outputs/confidence-interval.txt)
```bash
lawkit benford data/financial_data.csv --confidence 0.99
```

**サンプルサイズ指定** [`sample-size.txt`](outputs/sample-size.txt)
```bash
lawkit pareto data/large_dataset.csv --sample-size 10000
```

## システム情報

**ヘルプ** [`help-output.txt`](outputs/help-output.txt)
**バージョン** [`version-info.txt`](outputs/version-info.txt)

---

全10例、5つの統計法則を網羅。lawkitは文書通りに機能します。