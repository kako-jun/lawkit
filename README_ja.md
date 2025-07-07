# lawkit

> **🔍 多法則統計分析ツールキット - 隠れたパターンを発見し、確信を持って異常を検知**

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io CLI](https://img.shields.io/crates/v/lawkit.svg?label=lawkit-cli)](https://crates.io/crates/lawkit)
[![Docs.rs Core](https://docs.rs/lawkit-core/badge.svg)](https://docs.rs/lawkit-core)
[![Documentation](https://img.shields.io/badge/📚%20ユーザーガイド-Documentation-green)](https://github.com/kako-jun/lawkit/tree/main/docs/index_ja.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

複数の統計法則を使用して異常、パターン、洞察を検出する次世代統計分析ツールキット。不正検知、データ品質評価、ビジネスインテリジェンスに最適。

```bash
# 従来のツールは一度に一つのパターンしか分析しない
$ benf data.csv  # ベンフォード法則のみ

# lawkitは包括的な多法則分析を提供
$ lawkit compare --laws all data.csv
📊 ベンフォード法則: ⚠️  中リスク (カイ二乗: 15.2)
📈 パレート分析: ✅ 正常分布 (Gini: 0.31)
📉 ジップ法則: ❌ 高リスク (相関: 0.45)
🔔 正規分布: ✅ ガウス分布 (p値: 0.12)
🎯 ポアソン分布: ⚠️  中リスク (λ=2.3)
🧠 推奨: ジップ分析に注目 - 異常な頻度パターンを検出
```

## ✨ 主な機能

- **🎯 多法則分析**: ベンフォード、パレート、ジップ、正規、ポアソン分布
- **🌍 国際入力**: 英語、日本語、中国語、ヒンディー語、アラビア語形式の数字を解析
- **🤖 スマート統合**: 包括的洞察のための複数法則比較
- **⚡ 高性能**: 並列処理によるRust製
- **📊 豊富な出力**: テキスト、JSON、CSV、YAML、TOML、XML形式
- **🔗 メタチェーン**: 時系列での統計パターン傾向分析

## 📊 パフォーマンスベンチマーク

```bash
# 100Kデータポイントでのベンチマーク
# 従来ツールは1つのパターンを個別に分析
benf data.csv                 # 単一統計分析: ~2.1秒
pareto data.csv               # 別の個別分析: ~2.1秒

# lawkitは複数パターンを効率的に分析
lawkit benf data.csv          # 単一法則: ~180ms (11.7倍高速)
lawkit compare data.csv       # 多法則分析: ~850ms (逐次実行より2.5倍高速)
```

| データセットサイズ | 単一法則 | 多法則 | メモリ使用量 |
|-------------------|---------|-------|-------------|
| 1Kポイント         | 8ms     | 25ms  | 2.1MB       |
| 10Kポイント        | 45ms    | 180ms | 8.4MB       |
| 100Kポイント       | 180ms   | 850ms | 32MB        |
| 1Mポイント         | 2.1秒   | 9.2秒 | 128MB       |

## 🚀 クイックスタート

### インストール

```bash
# 統計分析ツールキット（推奨）
cargo install lawkit

# または事前ビルドバイナリをダウンロード
wget https://github.com/kako-jun/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz
tar -xzf lawkit-linux-x86_64.tar.gz
```

### 基本的な使用方法

```bash
# 単一法則分析
lawkit benf data.csv
lawkit pareto sales.csv
lawkit normal measurements.csv

# 多法則比較（推奨）
lawkit compare --laws benf,pareto data.csv
lawkit compare --laws all financial_data.csv

# フィルタリング付き高度分析
lawkit compare --laws all --filter ">=1000" --format json data.csv
```

## 🔍 対応統計法則

### 1. ベンフォード法則
**用途**: 財務データの不正検知
```bash
lawkit benf transactions.csv --threshold high
```
データ操作を示す可能性がある不自然な数字分布を検出します。

### 2. パレート分析（80/20法則）
**用途**: ビジネス優先順位付けと不平等測定
```bash
lawkit pareto customer_revenue.csv --verbose
```
結果の大部分を駆動する重要な少数を特定します。

### 3. ジップ法則
**用途**: 頻度分析とテキストマイニング
```bash
lawkit zipf --text document.txt
lawkit zipf website_traffic.csv
```
ランキングと頻度のべき法則分布を分析します。

### 4. 正規分布
**用途**: 品質管理と外れ値検出
```bash
lawkit normal --quality-control --spec-limits 9.5,10.5 production.csv
lawkit normal --outliers process_data.csv
```
統計的工程管理と異常検知。

### 5. ポアソン分布
**用途**: イベント発生と稀少事象モデリング
```bash
lawkit poisson --predict --rare-events incident_data.csv
```
離散的イベント発生をモデル化・予測します。

## 🎲 データ生成・テスト機能

lawkitには教育、テスト、デモンストレーション用の強力なデータ生成機能が含まれています。

### サンプルデータの生成

特定の法則に従う統計的に正確なサンプルデータを生成：

```bash
# ベンフォードの法則に従う1000サンプルを生成
lawkit generate benf --samples 1000

# パレート分布（80/20の法則）を生成
lawkit generate pareto --samples 5000 --concentration 0.8

# カスタムパラメータでZipf分布を生成
lawkit generate zipf --samples 2000 --exponent 1.0 --vocabulary-size 1000

# 正規分布データを生成
lawkit generate normal --samples 1000 --mean 100 --stddev 15

# ポアソンイベントデータを生成
lawkit generate poisson --samples 500 --lambda 2.5
```

### 生成→分析パイプライン

生成と分析を組み合わせてテストと検証を実行：

```bash
# ベンフォードの法則検出をテスト
lawkit generate benf --samples 10000 | lawkit benf --format json

# 生成データでパレートの法則を検証
lawkit generate pareto --samples 5000 | lawkit pareto --verbose

# 統計手法を検証
lawkit generate normal --samples 1000 --mean 50 --stddev 10 | lawkit normal --outliers

# 不正注入でのテスト
lawkit generate benf --samples 5000 --fraud-rate 0.2 | lawkit benf --threshold critical
```

### セルフテスト

lawkit機能の包括的なセルフテストを実行：

```bash
# 全セルフテストを実行
lawkit selftest

# 特定機能をテスト
lawkit generate benf --samples 100 | lawkit benf --quiet
```

### 教育用途

統計概念の教育に最適：

```bash
# 中心極限定理の実演
for i in {1..5}; do
  lawkit generate normal --samples 1000 --mean 100 --stddev 15 | 
  lawkit normal --verbose
done

# パレートの法則の実際の動作を表示
lawkit generate pareto --samples 10000 --concentration 0.8 | 
lawkit pareto --format json | jq '.concentration_ratio'

# 異なる分布の比較
lawkit generate benf --samples 1000 > benf_data.txt
lawkit generate normal --samples 1000 > normal_data.txt
lawkit compare --laws benf,normal benf_data.txt
lawkit compare --laws benf,normal normal_data.txt
```

## 国際数字サポート

### 対応数字形式

#### 1. 全角数字
```bash
echo "１２３４５６ ７８９０１２" | lawkit benf
```

#### 2. 漢数字（基本）
```bash
echo "一二三四五六七八九" | lawkit benf
```

#### 3. 漢数字（位取り）
```bash
echo "一千二百三十四 五千六百七十八 九万一千二百" | lawkit benf
```

#### 4. 混在パターン
```bash
echo "売上123万円 経費45万6千円 利益78万９千円" | lawkit benf
```

### 変換ルール

| 漢字 | 数字 | 備考 |
|------|------|------|
| 一 | 1 | 基本数字 |
| 十 | 10 | 十の位 |
| 百 | 100 | 百の位 |
| 千 | 1000 | 千の位 |
| 万 | 10000 | 万の位 |
| 一千二百三十四 | 1234 | 位取り記法 |

### ヒンディー語数字（हिन्दी अंक）
```bash
# デーヴァナーガリー数字
echo "१२३४५६ ७८९०१२" | lawkit benf --lang hi
```

### アラビア語数字（الأرقام العربية）
```bash  
# 東アラビア・インド数字
echo "١٢٣٤٥٦ ٧٨٩٠١٢" | lawkit benf --lang ar
```

## ドキュメント

包括的なドキュメントについては以下を参照してください：
- 📖 **[日本語ドキュメント](docs/index_ja.md)** - 完全なユーザーガイドとAPIリファレンス

## 開発・貢献

[CONTRIBUTING.md](CONTRIBUTING.md)を参照してください。

## ライセンス

MIT ライセンス - [LICENSE](LICENSE)ファイルを参照

## 参考資料

- [ベンフォードの法則 - Wikipedia](https://ja.wikipedia.org/wiki/ベンフォードの法則)
- [不正検知におけるベンフォードの法則の活用](https://example.com/benford-fraud-jp)