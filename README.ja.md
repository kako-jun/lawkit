# lawkit

[English](README.md)

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/lawkit.svg)](https://crates.io/crates/lawkit)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

統計法則分析ツールキット。ベンフォードの法則、パレートの法則、ジップの法則などを使ってデータを分析。異常検知とデータ品質評価に。

## なぜ lawkit？

従来のツールは一度に1つのパターンしか分析できません。lawkitは複数の統計法則を同時に分析：

```bash
$ lawkit benf financial_data.csv
Benford Law Analysis Results

Dataset: financial_data.csv
Numbers analyzed: 1000
[LOW] ベンフォードの法則に適合

First Digit Distribution:
1: ██████████████████████████████  30.1% (expected: 30.1%)
2: ██████████████████              17.8% (expected: 17.6%)
3: ████████████                    12.5% (expected: 12.5%)
...

$ lawkit pareto sales.csv
80/20 Rule: Top 20% owns 79.2% of total
```

## インストール

```bash
# CLIツールとして
cargo install lawkit

# ライブラリとして（Cargo.toml）
[dependencies]
lawkit-core = "2.5"
```

## 使い方

```bash
# 分析コマンド
lawkit benf data.csv          # ベンフォードの法則（不正検知）
lawkit pareto data.csv        # パレートの法則（80/20ルール）
lawkit zipf data.csv          # ジップの法則（頻度分布）
lawkit normal data.csv        # 正規分布（品質管理）
lawkit poisson data.csv       # ポアソン分布（稀な事象）

# 統合コマンド
lawkit analyze data.csv       # 複数法則の同時分析
lawkit validate data.csv      # データ検証
lawkit diagnose data.csv      # 詳細診断

# テストデータ生成
lawkit generate benf -s 1000
lawkit generate pareto -s 500

# ユーティリティ
lawkit list                   # 利用可能な法則一覧
lawkit selftest               # セルフテスト実行
```

## 対応入力

- ファイル: `lawkit benf data.csv`
- URL: `lawkit benf https://example.com/data.json`
- 標準入力: `cat data.csv | lawkit benf -`

フォーマット: CSV, JSON, YAML, プレーンテキスト（1行1数値）

## 主なオプション

```bash
-f, --format <FORMAT>      # 出力形式: text, csv, json, yaml, toml, xml
-q, --quiet                # 最小出力
-v, --verbose              # 詳細出力
--filter <RANGE>           # 数値フィルタ（例: >=100, <1000, 50-500）
-c, --min-count <N>        # 最小データ数（デフォルト: 10）
--no-color                 # 色なし出力
-t, --threshold <LEVEL>    # 検出閾値: low, medium, high, critical
--confidence <LEVEL>       # 信頼水準（0.01-0.99, デフォルト: 0.95）
--sample-size <N>          # 大規模データ用の最大サンプルサイズ
--min-value <VALUE>        # 含める最小値
```

## リスクレベル

| レベル | 意味 | 終了コード |
|--------|------|------------|
| LOW | 期待分布に適合 | 0 |
| MEDIUM | 軽微な逸脱、正常範囲 | 0 |
| HIGH | 有意な逸脱 (p ≤ 0.05) | 10 |
| CRITICAL | 深刻な異常 (p ≤ 0.01) | 11 |

その他の終了コード: 1（一般エラー）, 2（引数エラー）

## CI/CDでの活用

```bash
# 財務データの異常検知
if ! lawkit benf transactions.csv --quiet; then
  echo "異常を検知しました"
  lawkit benf transactions.csv --format json > report.json
fi

# クロスバリデーションでデータ品質を検証
lawkit validate data.csv --cross-validation
```

## ドキュメント

- [CLI仕様書](docs/specs/cli.md)
- [Core API仕様書](docs/specs/core.md)
- [テスト例](lawkit-cli/tests/)

## ライセンス

MIT
