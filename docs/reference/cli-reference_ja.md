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
| `--output` | `-o` | 出力形式 | `text` |
| `--language` | `-l` | 言語設定 | `en` |
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

### 対応言語

- `en` - 英語
- `ja` - 日本語
- `zh` - 中国語
- `hi` - ヒンディー語
- `ar` - アラビア語

## サブコマンド

### lawkit benf - ベンフォード法則分析

**用途**: 会計データ、選挙結果、自然データの不正検知

```bash
lawkit benf [ファイル] [オプション]
```

#### オプション

| オプション | 型 | 説明 | デフォルト |
|------------|-----|------|-----------|
| `--columns` | 文字列 | 分析対象列（カンマ区切り） | 全列 |
| `--confidence` | 数値 | 信頼度レベル（0.0-1.0） | 0.95 |
| `--min-value` | 数値 | 最小値フィルター | なし |
| `--max-value` | 数値 | 最大値フィルター | なし |
| `--sample-size` | 整数 | サンプルサイズ | 全データ |
| `--chi-squared` | - | カイ二乗検定を実行 | false |
| `--digits` | 整数 | 分析する桁数（1-4） | 1 |

#### 例

```bash
# 基本的な分析
lawkit benf data.csv

# 特定の列を分析
lawkit benf accounts.csv --columns "金額,売上"

# 高信頼度での分析
lawkit benf expenses.csv --confidence 0.99

# 範囲を指定して分析
lawkit benf transactions.csv --min-value 1000 --max-value 100000

# 日本語対応
lawkit benf 会計データ.csv --language ja

# JSON出力
lawkit benf data.csv --output json
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
| `--gini` | - | Gini係数を計算 | false |
| `--lorenz` | - | ローレンツ曲線データを出力 | false |
| `--top-n` | 整数 | 上位N項目を表示 | 10 |
| `--columns` | 文字列 | 分析対象列 | 全列 |

#### 例

```bash
# 基本的なパレート分析（80/20）
lawkit pareto sales.csv

# 90/10分析
lawkit pareto customers.csv --threshold 0.9

# Gini係数付き分析
lawkit pareto revenue.csv --gini

# 上位20項目の表示
lawkit pareto products.csv --top-n 20

# ローレンツ曲線データ出力
lawkit pareto income.csv --lorenz --output csv
```

### lawkit zipf - ジップ法則分析

**用途**: テキスト分析、単語頻度分析

```bash
lawkit zipf [ファイル] [オプション]
```

#### オプション

| オプション | 型 | 説明 | デフォルト |
|------------|-----|------|-----------|
| `--min-frequency` | 整数 | 最小出現頻度 | 2 |
| `--max-words` | 整数 | 最大単語数 | 1000 |
| `--case-sensitive` | - | 大文字小文字を区別 | false |
| `--remove-stopwords` | - | ストップワードを除去 | true |
| `--language` | 文字列 | テキスト言語 | auto |

#### 例

```bash
# 基本的なテキスト分析
lawkit zipf document.txt

# 日本語テキスト分析
lawkit zipf 文書.txt --language ja

# 高頻度単語のみ抽出
lawkit zipf text.txt --min-frequency 10

# 上位100単語
lawkit zipf large_text.txt --max-words 100

# 大文字小文字を区別
lawkit zipf code.txt --case-sensitive
```

### lawkit normal - 正規分布分析

**用途**: 品質管理、異常値検出、工程能力評価

```bash
lawkit normal [ファイル] [オプション]
```

#### オプション

| オプション | 型 | 説明 | デフォルト |
|------------|-----|------|-----------|
| `--normality-tests` | 文字列 | 正規性検定（shapiro,anderson,ks） | all |
| `--outliers` | - | 異常値検出 | false |
| `--outlier-method` | 文字列 | 異常値検出手法（iqr,zscore,isolation） | iqr |
| `--control-chart` | - | 管理図データ出力 | false |
| `--capability` | - | 工程能力評価 | false |
| `--spec-limits` | 文字列 | 規格限界（下限,上限） | なし |
| `--confidence` | 数値 | 信頼区間 | 0.95 |

#### 例

```bash
# 基本的な正規性分析
lawkit normal measurements.csv

# 異常値検出
lawkit normal quality_data.csv --outliers

# 工程能力評価
lawkit normal process.csv --capability --spec-limits 98.5,101.5

# 特定の正規性検定のみ
lawkit normal data.csv --normality-tests shapiro,anderson

# 管理図データ出力
lawkit normal control_data.csv --control-chart --output csv
```

### lawkit poisson - ポアソン分布分析

**用途**: イベント発生予測、稀少事象分析

```bash
lawkit poisson [ファイル] [オプション]
```

#### オプション

| オプション | 型 | 説明 | デフォルト |
|------------|-----|------|-----------|
| `--interval` | 文字列 | 時間間隔（hour,day,week,month） | day |
| `--forecast` | 整数 | 予測期間 | 30 |
| `--confidence` | 数値 | 信頼区間 | 0.95 |
| `--goodness-of-fit` | - | 適合度検定 | false |
| `--rate-estimation` | - | 発生率推定 | true |

#### 例

```bash
# 基本的なポアソン分析
lawkit poisson events.csv

# 時間毎の分析
lawkit poisson hourly_events.csv --interval hour

# 7日間の予測
lawkit poisson incidents.csv --forecast 7

# 適合度検定付き
lawkit poisson rare_events.csv --goodness-of-fit

# 高信頼度での分析
lawkit poisson defects.csv --confidence 0.99
```

### lawkit compare - 複数法則比較

**用途**: 複数法則の統合分析、矛盾検出、推奨システム

```bash
lawkit compare [ファイル] [オプション]
```

#### オプション

| オプション | 型 | 説明 | デフォルト |
|------------|-----|------|-----------|
| `--laws` | 文字列 | 適用法則（benford,pareto,zipf,normal,poisson,all） | all |
| `--detect-conflicts` | - | 矛盾検出 | false |
| `--recommend` | - | 推奨システム | false |
| `--weights` | 文字列 | 法則の重み（0.0-1.0,カンマ区切り） | 均等 |
| `--threshold` | 数値 | 矛盾検出閾値 | 0.1 |

#### 例

```bash
# 全法則での比較分析
lawkit compare data.csv --laws all

# 特定法則のみ
lawkit compare financial.csv --laws benford,normal

# 矛盾検出付き
lawkit compare audit_data.csv --detect-conflicts

# 推奨システム付き
lawkit compare dataset.csv --recommend

# 重み付き分析
lawkit compare data.csv --laws benford,pareto --weights 0.7,0.3
```

### lawkit list - 利用可能な法則一覧

```bash
lawkit list [オプション]
```

#### オプション

| オプション | 型 | 説明 |
|------------|-----|------|
| `--detailed` | - | 詳細説明を表示 |

#### 例

```bash
# 基本的な一覧表示
lawkit list

# 詳細説明付き
lawkit list --detailed
```

## 高度な使用例

### バッチ処理

```bash
# 複数ファイルの一括処理
for file in data/*.csv; do
    lawkit benf "$file" --output json > "results/$(basename "$file" .csv)_benf.json"
done
```

### パイプライン処理

```bash
# フィルタリング後の分析
cat large_data.csv | head -1000 | lawkit benf --columns "amount"

# 前処理との組み合わせ
preprocess_data.py input.xlsx | lawkit pareto --threshold 0.9
```

### 設定ファイルとの組み合わせ

```bash
# 設定ファイル使用
lawkit benf data.csv --config audit.toml

# プロファイル使用
lawkit compare data.csv --profile comprehensive
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
# → --min-data-points オプションで調整

# メモリ不足
lawkit benf huge_file.csv
# → --sample-size オプションでサンプリング
```

## パフォーマンスチューニング

### 大きなファイルの処理

```bash
# サンプリング
lawkit benf large_file.csv --sample-size 50000

# 並列処理
lawkit compare data.csv --threads 8

# メモリ制限
lawkit benf big_data.csv --memory-limit 1024
```

このリファレンスを使用して、lawkitの全機能を効果的に活用してください。