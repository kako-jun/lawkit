# lawkit

[English](README.md)

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/lawkit.svg)](https://crates.io/crates/lawkit)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

統計法則分析ツールキット。ベンフォードの法則、パレートの法則、ジップの法則、正規分布、ポアソン分布でデータを分析。異常検知とデータ品質評価に。

## インストール

```bash
cargo install lawkit
```

## 対応法則

### ベンフォードの法則（不正検知）

```bash
$ lawkit benf financial_data.csv
Benford Law Analysis Results

Dataset: financial_data.csv
Numbers analyzed: 1000
[LOW] Dataset analysis

First Digit Distribution:
1: ███████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  30.1% (expected:  30.1%)
2: █████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  17.6% (expected:  17.6%)
3: ██████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  12.5% (expected:  12.5%)
...
```

### パレートの法則（80/20ルール）

```bash
$ lawkit pareto sales.csv
Pareto Principle (80/20 Rule) Analysis Results

Dataset: sales.csv
Numbers analyzed: 500
[LOW] Dataset analysis

Lorenz Curve (Cumulative Distribution):
 20%: ███████████████████████████████████████┃░░░░░░░░░░  79.2% cumulative (80/20 point)
 40%: █████████████████████████████████████████████░░░░░  91.5% cumulative
...

80/20 Rule: Top 20% owns 79.2% of total wealth (Ideal: 80.0%, Ratio: 0.99)
```

### ジップの法則（頻度分布）

```bash
$ lawkit zipf word_frequencies.csv
Zipf Law Analysis Results

Dataset: word_frequencies.csv
Numbers analyzed: 1000
[LOW] Dataset analysis

Rank-Frequency Distribution:
# 1: █████████████████████████████████████████████████┃  11.50% (expected: 11.50%)
# 2: █████████████████████████┃░░░░░░░░░░░░░░░░░░░░░░░   5.75% (expected: 5.75%)
# 3: █████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   3.83% (expected: 3.83%)
...

Zipf Exponent: 1.02 (ideal: 1.0), Correlation: 0.998
```

### 正規分布（品質管理）

```bash
$ lawkit normal measurements.csv
Normal Distribution Analysis Results

Dataset: measurements.csv
Numbers analyzed: 200
Quality Level: High

Distribution Histogram:
 -2.50- -1.89: ██████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  11.5%
 -1.89- -1.28: █████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  34.0%
 -1.28- -0.67: ███████████████████████████████████┃░░░░░░░░░░░░░░  69.8%
...

Distribution: μ=0.02, σ=1.01, Range: [-2.89, 3.12]
1σ: 68.5%, 2σ: 95.5%, 3σ: 99.7%
```

### ポアソン分布（稀な事象）

```bash
$ lawkit poisson events.csv
Poisson Distribution Analysis Results

Dataset: events.csv
Numbers analyzed: 100
Quality Level: High

Probability Distribution:
P(X= 0): ██████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0.095
P(X= 1): ███████████████████████████████████████████┃░░░░░░  0.224
P(X= 2): █████████████████████████████████████████████████┃  0.263
...

λ=2.35, Variance/Mean=1.02 (ideal: 1.0), Fit Score=0.95
```

## 使い方

```bash
# 単一法則の分析
lawkit benf data.csv          # ベンフォードの法則
lawkit pareto data.csv        # パレートの法則
lawkit zipf data.csv          # ジップの法則
lawkit normal data.csv        # 正規分布
lawkit poisson data.csv       # ポアソン分布

# 複数法則の同時分析
lawkit analyze data.csv       # 全法則を適用
lawkit validate data.csv      # データ検証
lawkit diagnose data.csv      # 詳細診断

# テストデータ生成
lawkit generate benf -s 1000
lawkit generate pareto -s 500

# ユーティリティ
lawkit list                   # 利用可能な法則一覧
lawkit selftest               # セルフテスト
```

## 入力ソース

```bash
lawkit benf data.csv                        # ファイル
lawkit benf https://example.com/data.json   # URL
cat data.csv | lawkit benf -                # 標準入力
```

フォーマット: CSV, JSON, YAML, プレーンテキスト（1行1数値）

## 主なオプション

```bash
-f, --format <FORMAT>      # 出力形式: text, csv, json, yaml, toml, xml
-q, --quiet                # 最小出力
-v, --verbose              # 詳細出力
--filter <RANGE>           # 数値フィルタ（例: >=100, <1000, 50-500）
-c, --min-count <N>        # 最小データ数（デフォルト: 10）
--no-color                 # 色なし出力
```

## リスクレベルと終了コード

| レベル | 意味 | 終了コード |
|--------|------|------------|
| LOW | 期待分布に適合 | 0 |
| MEDIUM | 軽微な逸脱、正常範囲 | 0 |
| HIGH | 有意な逸脱 (p ≤ 0.05) | 10 |
| CRITICAL | 深刻な異常 (p ≤ 0.01) | 11 |

## CI/CDでの活用

```bash
# 財務データの異常検知
if ! lawkit benf transactions.csv --quiet; then
  echo "異常を検知しました"
  lawkit benf transactions.csv --format json > report.json
fi

# 分布の検証
lawkit validate data.csv --cross-validation
```

## 単機能ツール

特定の法則だけを使いたい場合：
- [benf](https://crates.io/crates/benf) - ベンフォードの法則のみ
- [pareto](https://crates.io/crates/pareto) - パレートの法則のみ

## 実行例

[lawkit-cli/tests/cmd/](lawkit-cli/tests/cmd/) に詳細な例があります：

- [ベンフォードの法則](lawkit-cli/tests/cmd/benford.md)
- [パレートの法則](lawkit-cli/tests/cmd/pareto.md)
- [ジップの法則](lawkit-cli/tests/cmd/zipf.md)
- [正規分布](lawkit-cli/tests/cmd/normal.md)
- [ポアソン分布](lawkit-cli/tests/cmd/poisson.md)
- [出力形式](lawkit-cli/tests/cmd/output.md)
- [オプション](lawkit-cli/tests/cmd/options.md)
- [テストデータ生成](lawkit-cli/tests/cmd/generate.md)

## ドキュメント

- [CLI仕様書](docs/specs/cli.md)
- [Core API仕様書](docs/specs/core.md)

## ライセンス

MIT
