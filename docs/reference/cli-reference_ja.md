# CLIリファレンス

lawkitの全コマンドとオプションの詳細リファレンスです。

## 基本構文

```bash
lawkit [サブコマンド] [ファイル] [オプション]
```

## グローバルオプション

すべてのサブコマンドで使用可能なオプション：

| オプション | 短縮形 | 説明 | デフォルト |
|------------|--------|------|-----------|
| `--format` | `-f` | 出力形式 | `text` |
| `--verbose` | `-v` | 詳細出力 | `false` |
| `--quiet` | `-q` | 簡潔出力 | `false` |
| `--help` | `-h` | ヘルプ表示 | - |
| `--version` | `-V` | バージョン表示 | - |

### 出力形式

- `text` - 人間が読みやすいテキスト形式
- `json` - JSON形式
- `csv` - CSV形式
- `yaml` - YAML形式
- `toml` - TOML形式
- `xml` - XML形式


## サブコマンド

### lawkit benf - ベンフォード法則分析

**用途**: 会計データ、選挙結果、自然データの不正検知

```bash
lawkit benf [ファイル] [オプション]
```

#### オプション

共通オプション（--format, --verbose, --quiet）に加えて、特別なオプションはありません。

#### 例

```bash
# 基本的な分析
lawkit benf data.csv

# 詳細出力
lawkit benf transactions.json --verbose

# JSON出力
lawkit benf data.csv --format json

# 簡潔出力
lawkit benf accounts.csv --quiet

# 高額取引をフィルタリングし、高い闾値で分析
lawkit benf accounts.csv --filter ">=1000" --threshold high
```

### lawkit pareto - パレート法則分析

**用途**: 80/20分析、売上分析、在庫管理

```bash
lawkit pareto [ファイル] [オプション]
```

#### オプション

| オプション | 型 | 説明 | デフォルト |
|------------|-----|------|-----------|
| `--threshold` | 数値 | パレート閾値（0.0-1.0） | 0.8 |

#### 例

```bash
# 基本的なパレート分析（80/20）
lawkit pareto sales.csv

# 90/10分析
lawkit pareto customers.csv --concentration 0.9

# Gini係数付きビジネス分析
lawkit pareto revenue.csv --business-analysis --gini-coefficient

# カスタム百分位数
lawkit pareto products.csv --percentiles 70,80,90,95
```

### lawkit zipf - ジップ法則分析

**用途**: テキスト分析、単語頻度分析

```bash
lawkit zipf [ファイル] [オプション]
```

#### オプション

共通オプション（--format, --verbose, --quiet）に加えて、特別なオプションはありません。

#### 例

```bash
# 基本的な分析
lawkit zipf frequency_data.csv

# 詳細出力
lawkit zipf rankings.csv --verbose

# JSON出力
lawkit zipf data.csv --format json
```

### lawkit normal - 正規分布分析

**用途**: 品質管理、異常値検出、工程能力評価

```bash
lawkit normal [ファイル] [オプション]
```

#### オプション

共通オプション（--format, --verbose, --quiet）に加えて、特別なオプションはありません。

#### 例

```bash
# 基本的な正規性分析
lawkit normal measurements.csv

# 詳細出力
lawkit normal quality_data.csv --verbose

# JSON出力
lawkit normal data.csv --format json
```

### lawkit poisson - ポアソン分布分析

**用途**: イベント発生予測、稀少事象分析

```bash
lawkit poisson [ファイル] [オプション]
```

#### オプション

共通オプション（--format, --verbose, --quiet）に加えて、特別なオプションはありません。

#### 例

```bash
# 基本的なポアソン分析
lawkit poisson events.csv

# 詳細出力
lawkit poisson incidents.csv --verbose

# JSON出力
lawkit poisson defects.csv --format json
```

### lawkit generate - サンプルデータ生成

**用途**: テスト用データ生成、法則検証、ベンチマーク

```bash
lawkit generate [法則] [オプション]
```

#### 利用可能な法則
- `benf` - ベンフォード法則準拠データ生成
- `pareto` - パレート分布データ生成
- `zipf` - Zipf法則データ生成
- `normal` - 正規分布データ生成
- `poisson` - ポアソン分布データ生成

#### 共通オプション

| オプション | 型 | 説明 | デフォルト |
|------------|-----|------|-----------|
| `--samples` | 整数 | 生成サンプル数 | 1000 |

#### 例

```bash
# ベンフォード法則データ生成
lawkit generate benf --samples 5000

# 不正注入付きベンフォードデータ生成
lawkit generate benf --samples 2000 --fraud-rate 0.1

# カスタム範囲の再現可能なデータ生成
lawkit generate benf --samples 1000 --seed 42 --range 1,50000

# ファイルに保存
lawkit generate normal --samples 1000 --output-file test_data.csv
```

### lawkit analyze - 複数法則比較

**用途**: 複数法則の統合分析、矛盾検出、推奨システム

```bash
lawkit analyze [ファイル] [オプション]
```

#### オプション

| オプション | 型 | 説明 | デフォルト |
|------------|-----|------|-----------|
| `--laws` | 文字列 | 適用法則（benf,pareto,zipf,normal,poisson） | all |
| `--focus` | 文字列 | 分析フォーカス（quality,concentration,distribution,anomaly） | general |
| `--purpose` | 文字列 | 分析目的（quality,fraud,concentration,anomaly,distribution,general） | general |
| `--recommend` | - | 推奨システム | false |

#### 例

```bash
# 全法則での比較分析
lawkit analyze data.csv

# 特定法則のみ
lawkit analyze financial.csv --laws benf,normal

# 推奨システム付き
lawkit analyze dataset.csv --recommend

# 矛盾検出と一貫性チェック
lawkit analyze data.csv --consistency-check --report conflicting

# 詳細出力
lawkit analyze data.csv --verbose --format json
```

### lawkit validate - データ検証

**用途**: データの一貫性と品質を複数の統計パターンで検証

```bash
lawkit validate [ファイル] [オプション]
```

#### 例

```bash
# 基本的な検証
lawkit validate data.csv

# 詳細出力
lawkit validate dataset.csv --verbose
```

### lawkit diagnose - 診断

**用途**: 統計法則間の矛盾検出と詳細診断

```bash
lawkit diagnose [ファイル] [オプション]
```

#### 例

```bash
# 基本的な診断
lawkit diagnose data.csv

# 詳細出力
lawkit diagnose dataset.csv --verbose
```

### lawkit list - 利用可能な法則一覧

```bash
lawkit list
```

#### 例

```bash
# 基本的な一覧表示
lawkit list
```

## 高度な使用例

### バッチ処理

```bash
# 複数ファイルの一括処理
for file in data/*.csv; do
    lawkit benf "$file" --format json > "results/$(basename "$file" .csv)_benf.json"
done
```

### パイプライン処理

```bash
# フィルタリング後の分析
cat large_data.csv | head -1000 | lawkit benf

# 前処理との組み合わせ
preprocess_data.py input.xlsx | lawkit pareto --threshold 0.9
```

## エラーハンドリング

### 終了コード

- `0` - 成功
- `1` - 一般エラー
- `2` - ファイル読み込みエラー
- `3` - データ形式エラー
- `4` - 統計計算エラー
- `5` - 出力エラー

### よくあるエラーと対処法

```bash
# ファイルが見つからない
lawkit benf nonexistent.csv
# → ファイルパスを確認

# データが不足
lawkit benf small_data.csv
# → より多くのデータを提供

# メモリ不足
lawkit benf huge_file.csv
# → 入力データのサイズを小さくする
```

## パフォーマンスチューニング

### 大きなファイルの処理

```bash
# 基本的な分析
lawkit benf large_file.csv

# 詳細出力
lawkit analyze data.csv --verbose

# 複数法則の並列処理
lawkit analyze data.csv --laws benf,pareto,normal
```

このリファレンスを使用して、lawkitの全機能を効果的に活用してください。