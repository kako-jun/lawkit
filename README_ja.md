# lawkit

> **🔍 多法則統計分析ツールキット - 隠れたパターンを発見し、確信を持って異常を検知**

[English README](README.md) | [日本語版 README](README_ja.md) | [中文版 README](README_zh.md)

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io CLI](https://img.shields.io/crates/v/lawkit.svg?label=lawkit-cli)](https://crates.io/crates/lawkit)
[![Docs.rs Core](https://docs.rs/lawkit-core/badge.svg)](https://docs.rs/lawkit-core)
[![npm](https://img.shields.io/npm/v/lawkit-js.svg?label=lawkit-js)](https://www.npmjs.com/package/lawkit-js)
[![PyPI](https://img.shields.io/pypi/v/lawkit-python.svg?label=lawkit-python)](https://pypi.org/project/lawkit-python/)
[![Documentation](https://img.shields.io/badge/📚%20ユーザーガイド-Documentation-green)](https://github.com/kako-jun/lawkit/tree/main/docs/index_ja.md)
[![API Reference](https://img.shields.io/badge/🔧%20API%20Reference-docs.rs-blue)](https://docs.rs/lawkit-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

複数の統計法則を使用して異常、パターン、洞察を検出する次世代統計分析ツールキット。不正検知、データ品質評価、ビジネスインテリジェンスに最適。

```bash
# 従来のツールは一度に一つのパターンしか分析しない
$ other-tool data.csv  # 単一統計分析

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
- **🔍 高度外れ値検出**: LOF、Isolation Forest、DBSCAN、アンサンブル手法
- **📈 時系列分析**: トレンド検出、季節性、変化点分析
- **🚀 メモリ効率**: 大規模データセット用ストリーミングモード

## 📊 パフォーマンス

AMD Ryzen 5 PRO 4650Uでの実際のベンチマーク結果：

```bash
# 従来ツールは一度に一つのパターンを分析
$ other-tool data.csv         # 単一分析: ~2.1秒
$ lawkit benf data.csv        # 同じ分析: ~180ms (11.7倍高速)
$ lawkit compare data.csv     # 多法則分析: ~850ms
```

## なぜlawkitなのか？

従来ツールは一度に一つの統計パターンしか分析しません。`lawkit`は包括的な多法則分析を提供します。

- **全体的洞察**: 複数の統計法則が異なる側面を明らかにします
- **スマート推奨**: AI搭載の分析統合
- **時間効率**: 複数法則の並列処理
- **国際対応**: 5言語の数字解析

## 🏗️ 仕組み

```mermaid
graph LR
    A[データ] --> B[解析・検証]
    B --> C[多法則分析]
    C --> D[リスク評価]
    D --> E[推奨事項]
```

lawkitは複数の統計レンズを通してデータを同時に分析し、結果を統合して包括的な洞察と推奨事項を提供します。

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

## 仕様

### 対応統計法則

- **ベンフォード法則**: 金融データの不正検知
- **パレート分析**: 80/20法則と不平等測定  
- **ジップ法則**: 頻度分析とべき法則分布
- **正規分布**: 品質管理と外れ値検出
- **ポアソン分布**: イベント発生と稀少事象モデリング

### 分析タイプ

- 単一法則分析
- 多法則比較・統合
- 高度外れ値検出（LOF、Isolation Forest、DBSCAN）
- 時系列分析とトレンド検出
- テスト・検証用データ生成

### 出力形式

`lawkit`は様々な用途に対応した複数形式で結果を出力：

- **テキスト形式（デフォルト）**: 人間が読みやすい分析結果
- **JSON形式**: 自動化・統合用の機械可読形式
- **CSV/YAML/TOML/XML**: データ処理用の各種構造化形式

## インストール

### CLIツール

```bash
# crates.ioから（推奨）
cargo install lawkit

# リリースから
wget https://github.com/kako-jun/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz
tar -xzf lawkit-linux-x86_64.tar.gz
```

### パッケージ統合

```bash
# Node.js統合
npm install lawkit-js

# Python統合
pip install lawkit-python
lawkit-download-binary  # CLIバイナリをダウンロード
```

## ドキュメント

包括的なガイド、サンプル、APIドキュメントについては：

📚 **[ユーザーガイド](https://github.com/kako-jun/lawkit/tree/main/docs/index_ja.md)** - インストール、使用方法、サンプル  
🔧 **[CLIリファレンス](https://github.com/kako-jun/lawkit/tree/main/docs/reference/cli-reference_ja.md)** - 完全なコマンドドキュメント  
📊 **[統計法則ガイド](https://github.com/kako-jun/lawkit/tree/main/docs/user-guide/examples_ja.md)** - 詳細な分析サンプル  
⚡ **[パフォーマンスガイド](https://github.com/kako-jun/lawkit/tree/main/docs/guides/performance_ja.md)** - 最適化と大規模データセット  
🌍 **[国際サポート](https://github.com/kako-jun/lawkit/tree/main/docs/user-guide/configuration_ja.md)** - 多言語数字解析

## 貢献

貢献を歓迎します！詳細は[貢献ガイド](CONTRIBUTING.md)を参照してください。

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルを参照してください。