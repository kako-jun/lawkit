# CLIリファレンス

`lawkit`のコマンドライン・インターフェース完全ドキュメント。

## グローバルコマンド

### `lawkit --help`
メインコマンドまたはサブコマンドのヘルプ情報を表示。

### `lawkit --version`
バージョン情報を表示。

### `lawkit list`
利用可能なすべての統計法則とその説明をリスト表示。

```bash
$ lawkit list
利用可能な統計法則:
  benf    - ベンフォード法則分析
  pareto  - パレート原理 (80/20ルール) 分析
  zipf    - ジップ法則分析
  normal  - 正規分布分析
  poisson - ポアソン分布分析

統合コマンド:
  analyze  - 多法則基本分析と推奨
  validate - データ検証と整合性チェック
  diagnose - 矛盾検出と詳細診断
```

## 統計法則コマンド

### `lawkit benf` - ベンフォード法則分析

ベンフォード法則を使用して異常を検出し、データ品質を評価。

```bash
lawkit benf [OPTIONS] [INPUT]
```

#### オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小限の出力（数値のみ）
- `--verbose, -v` - 詳細なデバッグ出力と分析洞察を有効化
- `--filter <RANGE>` - 数値を範囲でフィルタ (例: >=100, <1000, 50-500)
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数 (デフォルト: 10)
- `--threshold <LEVEL>, -t` - 異常検出閾値: low, medium, high, critical (デフォルト: auto)
- `--confidence <LEVEL>` - 統計的検定の信頼度レベル (0.01-0.99, デフォルト: 0.95)
- `--sample-size <NUMBER>` - 大規模データセットの最大サンプルサイズ（性能向上）
- `--min-value <VALUE>` - 分析に含める最小値（ノイズとなる小さな値をフィルタ）

**注意**: `--optimize`オプションは廃止されました。最適化は自動的に適用されます。

#### 詳細出力
`--verbose`フラグは包括的なデバッグと分析情報を提供します：

**デバッグ情報:**
- 入力引数の検出と検証
- データ処理戦略（自動最適化、ストリーミング）
- フィルタ適用の前後統計
- データ収集と解析の詳細

**パフォーマンス指標:**
- 処理時間（ミリ秒）
- メモリ使用量（MB）
- 大規模データセットの処理チャンク数
- 処理されたアイテム数

**分析洞察:**
- 統計計算ステップ
- 信頼区間の詳細
- アルゴリズム選択の理由
- データ品質評価

詳細出力例:
```bash
$ echo "123 456 789" | lawkit benf --verbose
Debug: input argument = None
Debug: Reading from stdin, using automatic optimization
Debug: Using automatic optimization (streaming + incremental + memory efficiency)
Debug: Collected 3 numbers from stream
Debug: Streaming analysis successful - 3 items processed
Debug: Processed 3 numbers in 1 chunks
Debug: Memory used: 0.00 MB
Debug: Processing time: 1 ms

# 標準分析出力が続きます...
```

#### 例
```bash
# 基本分析
lawkit benf data.csv

# JSON形式での詳細出力
lawkit benf transactions.json --verbose --format json

# 最小限出力のためのクワイエットモード
lawkit benf data.csv --quiet

# 高閾値での大規模取引フィルタ
lawkit benf accounts.csv --filter ">=1000" --threshold high

# 監査用の高信頼度分析（99%信頼度レベル）
lawkit benf audit_data.csv --confidence 0.99 --verbose

# 大規模データセットのパフォーマンス最適化
lawkit benf big_data.csv --sample-size 50000

# 分析にノイズを加える小さな値をフィルタ
lawkit benf financial_data.csv --min-value 100
```

### `lawkit pareto` - パレート原理分析

集中度を分析し、80/20ルールを適用。

```bash
lawkit pareto [OPTIONS] [INPUT]
```

#### 共通オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小限の出力
- `--verbose, -v` - 詳細出力
- `--filter <RANGE>` - 数値を範囲でフィルタ (例: >=100, <1000, 50-500)
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数 (デフォルト: 10)

#### 固有オプション
- `--concentration <THRESHOLD>, -C` - 集中度閾値 (0.0-1.0) (デフォルト: 0.8)
- `--gini-coefficient` - 不平等測定のためのジニ係数を計算
- `--percentiles <PERCENTILES>` - 計算するカスタムパーセンタイル (例: 70,80,90)
- `--business-analysis` - ビジネス分析洞察を有効化

#### 例
```bash
# 基本パレート分析
lawkit pareto sales.csv

# カスタム閾値
lawkit pareto data.csv --concentration 0.9

# ジニ係数付きビジネス分析
lawkit pareto customers.csv --business-analysis --gini-coefficient

# カスタムパーセンタイル
lawkit pareto revenue.csv --percentiles 70,80,90,95
```

### `lawkit zipf` - ジップ法則分析

頻度分布とランキングパターンを分析。数値データとテキストデータの両方に対応。

```bash
lawkit zipf [OPTIONS] [INPUT]
```

#### 共通オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小限の出力
- `--verbose, -v` - 詳細出力
- `--filter <RANGE>` - 数値を範囲でフィルタ (例: >=100, <1000, 50-500)
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数 (デフォルト: 10)

#### 固有オプション
- `--text, -T` - テキスト分析モードを有効化
- `--words <NUMBER>, -w` - テキストモードで分析する最大単語数 (デフォルト: 1000)

#### 詳細出力
`--verbose`フラグは包括的なデバッグと分析情報を提供します：

**デバッグ情報:**
- 入力引数の検出と検証
- テキストモードvs数値モードの判定
- データ処理戦略（ストリーミング分析）
- データ収集と解析の詳細

**パフォーマンス指標:**
- 処理時間（ミリ秒）
- メモリ使用量（MB）
- 処理チャンク数
- 処理されたアイテム数

詳細出力例:
```bash
$ echo "1 2 3 4 5" | lawkit zipf --verbose
Debug: input argument = None
Debug: text mode = false
Debug: Reading from stdin, using automatic optimization
Debug: Collected 5 numbers from input

# 標準分析出力が続きます...
```

#### 例
```bash
# 基本ジップ分析（数値データ）
lawkit zipf frequency_data.csv

# テキスト分析モード
lawkit zipf text_document.txt --text

# 単語数制限付きテキスト分析
lawkit zipf large_text.txt --text --words 500

# 詳細出力
lawkit zipf rankings.csv --verbose

# JSON出力形式
lawkit zipf data.csv --format json
```

### `lawkit normal` - 正規分布分析

正規性をテストし、外れ値を検出。高度な統計分析機能を提供。

```bash
lawkit normal [OPTIONS] [INPUT]
```

#### 共通オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小限の出力
- `--verbose, -v` - 詳細出力
- `--filter <RANGE>` - 数値を範囲でフィルタ (例: >=100, <1000, 50-500)
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数 (デフォルト: 10)

#### 分析オプション
- `--test <METHOD>, -T` - 正規性検定方法: shapiro, anderson, ks, all (デフォルト: all)
- `--outliers, -O` - 外れ値検出を有効化
- `--outlier-method <METHOD>` - 外れ値検出方法: zscore, modified_zscore, iqr, lof, isolation, dbscan, ensemble (デフォルト: zscore)
- `--quality-control, -Q` - 品質管理分析を有効化
- `--spec-limits <LOWER,UPPER>` - 品質管理用規格限界 (例: 9.5,10.5)
- `--enable-timeseries` - 時系列分析を有効化
- `--timeseries-window <SIZE>` - 時系列分析ウィンドウサイズ (デフォルト: 10)

#### 詳細出力
`--verbose`フラグは包括的なデバッグと分析情報を提供します：

**デバッグ情報:**
- 入力引数の検出と検証
- データ処理戦略（自動最適化、ストリーミング）
- データ収集と解析の詳細
- ストリーミング分析メトリクス

**パフォーマンス指標:**
- 処理時間（ミリ秒）
- メモリ使用量（MB）
- 大規模データセットの処理チャンク数
- 処理されたアイテム数

詳細出力例:
```bash
$ echo "50 51 49 52 48" | lawkit normal --verbose
Debug: input argument = None
Debug: Reading from stdin, using automatic optimization
Debug: Collected 5 numbers from stream
Debug: Memory used: 0.00 MB

# 標準分析出力が続きます...
```

#### 例
```bash
# 基本正規性テスト
lawkit normal data.csv

# 特定の検定方法
lawkit normal data.csv --test shapiro

# 外れ値検出
lawkit normal data.csv --outliers --outlier-method lof

# 品質管理分析
lawkit normal production_data.csv --quality-control --spec-limits 9.5,10.5

# 時系列分析
lawkit normal timeseries_data.csv --enable-timeseries --timeseries-window 20

# 詳細出力
lawkit normal measurements.csv --verbose

# JSON出力形式
lawkit normal quality_data.csv --format json
```

### `lawkit poisson` - ポアソン分布分析

イベント発生と稀な事象を分析。

```bash
lawkit poisson [OPTIONS] [INPUT]
```

#### 共通オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小限の出力（数値のみ）
- `--verbose, -v` - 詳細なデバッグ出力と分析洞察を有効化
- `--filter <RANGE>` - 数値を範囲でフィルタ (例: >=100, <1000, 50-500)
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数 (デフォルト: 10)
- `--confidence <LEVEL>` - 統計的検定の信頼度レベル (0.01-0.99, デフォルト: 0.95)

#### 分析オプション
- `--test <METHOD>, -T` - 適合度検定方法: chi_square, ks, variance, all (デフォルト: all)
- `--predict, -p` - 確率予測を有効化
- `--max-events <NUMBER>` - 分析対象の最大イベント数 (デフォルト: 20)
- `--rare-events, -R` - 稀な事象分析に特化

**注意**: `--optimize`オプションは廃止されました。最適化は自動的に適用されます。

#### 例
```bash
# 基本ポアソン分析
lawkit poisson events.csv

# 詳細出力
lawkit poisson incidents.csv --verbose

# JSON出力形式
lawkit poisson data.csv --format json

# 重要分析用の高信頼度レベル
lawkit poisson server_errors.csv --confidence 0.99 --verbose
```

## 生成コマンド

### `lawkit generate` - サンプルデータ生成

テストと検証のために特定の統計法則に従うサンプルデータを生成。

```bash
lawkit generate <LAW> [OPTIONS]
```

#### 利用可能な法則
- `benf` - ベンフォード法則準拠データを生成
- `pareto` - パレート分布データを生成
- `zipf` - ジップ法則データを生成
- `normal` - 正規分布データを生成
- `poisson` - ポアソン分布データを生成

#### 共通生成オプション
- `--samples <NUMBER>` - 生成するサンプル数 (デフォルト: 1000)
- `--seed <NUMBER>` - 再現可能な生成のためのランダムシード
- `--output-file <FILE>` - 出力ファイルパス (デフォルト: stdout)

#### 法則固有オプション

**ベンフォード生成:**
- `--fraud-rate <RATE>` - テスト用の不正注入率 (0.0-1.0) (デフォルト: 0.0)
- `--range <MIN,MAX>` - 生成のための数値範囲 (例: 1,10000) (デフォルト: 1,100000)

#### 例
```bash
# ベンフォード法則データの生成
lawkit generate benf --samples 5000

# 不正注入付きベンフォードデータの生成
lawkit generate benf --samples 2000 --fraud-rate 0.1

# カスタム範囲での再現可能なデータ生成
lawkit generate benf --samples 1000 --seed 42 --range 1,50000

# 生成してファイルに保存
lawkit generate normal --samples 1000 --output-file test_data.csv
```

## 統合コマンド

### `lawkit analyze` - 多法則分析

包括的なデータ評価のための推奨付き基本多法則分析を実行。

```bash
lawkit analyze [OPTIONS] [INPUT]
```

### `lawkit validate` - データ検証

複数の統計パターンでデータの整合性と品質を検証。

```bash
lawkit validate [OPTIONS] [INPUT]
```

### `lawkit diagnose` - 矛盾検出

統計法則結果間の矛盾を検出し、詳細診断を提供。

```bash
lawkit diagnose [OPTIONS] [INPUT]
```

#### オプション
- `--laws <LAWS>` - 分析する特定の法則: benf,pareto,zipf,normal,poisson
- `--focus <FOCUS>` - 分析焦点: quality, concentration, distribution, anomaly
- `--purpose <PURPOSE>` - 分析目的: quality, fraud, concentration, anomaly, distribution, general
- `--recommend` - 最適法則推奨モードを有効化
- `--threshold <THRESHOLD>` - 矛盾検出閾値 (0.0-1.0) (デフォルト: 0.5)
- `--report <TYPE>` - 統合レポートタイプ: summary, detailed, conflicting (デフォルト: summary)
- `--consistency-check` - 整合性チェックを有効化
- `--cross-validation` - クロス検証分析を有効化
- `--confidence-level <LEVEL>` - 信頼度レベル (デフォルト: 0.95)

#### 例
```bash
# すべての法則を比較
lawkit analyze data.csv

# 不正検出に焦点
lawkit analyze transactions.csv --purpose fraud --recommend

# カスタム法則選択
lawkit analyze data.csv --laws benf,normal --focus quality

# JSON形式での詳細出力
lawkit analyze dataset.csv --verbose --format json
```

## 共通オプション

すべてのコマンドは以下の共通オプションをサポート:

### 入出力
- `[INPUT]` - 入力データ（ファイルパス、URL、または標準入力の場合は '-'）
- `--format <FORMAT>` - 出力形式: text, json, csv, yaml, toml, xml
- `--quiet, -q` - 最小限の出力
- `--verbose, -v` - 詳細出力
- `--optimize` - 大規模データセットのメモリと処理の最適化を有効化

### データ処理
- `--filter <RANGE>` - 数値フィルタリング (>=100, <1000, 50-500)
- `--min-count <NUMBER>` - 必要な最小データポイント数 (デフォルト: 10)

## 入力形式

`lawkit`は複数の入力形式をサポート:

- **テキストファイル** - 空白/カンマで区切られた数値
- **CSV** - カンマ区切り値
- **JSON** - 構造化データ
- **YAML** - YAML設定ファイル
- **TOML** - TOML設定ファイル
- **XML** - XMLデータファイル

## 出力形式

### テキスト形式（デフォルト）
分析結果、解釈、推奨を含む人間が読みやすい出力。

### JSON形式
APIと自動化のための機械可読構造化出力:
```json
{
  "dataset": "data.csv",
  "numbers_analyzed": 1000,
  "risk_level": "Low",
  "analysis_results": {...}
}
```

### CSV形式
スプレッドシートインポート用の表形式:
```csv
dataset,numbers_analyzed,risk_level,score
data.csv,1000,Low,0.85
```

## 終了コード

- `0` - 成功、低リスク
- `10` - 中リスク検出
- `11` - 高リスク検出
- `12` - 危険リスク検出
- `1` - 分析エラー
- `2` - 無効な引数
- `3` - ファイル/ネットワークエラー

## ユースケース別の例

### 不正検出
```bash
# 金融取引分析
lawkit benf transactions.csv --verbose

# 多法則不正検出
lawkit analyze suspicious_data.csv --purpose fraud --recommend
```

### データ品質評価
```bash
# 包括的品質チェック
lawkit analyze dataset.csv --purpose quality --verbose

# 正規性に焦点
lawkit normal dataset.csv --verbose
```

### ビジネスインテリジェンス
```bash
# 80/20分析
lawkit pareto sales.csv --threshold 0.8

# 顧客分析
lawkit zipf customer_frequency.csv --verbose
```

### 異常検出
```bash
# 正規性と外れ値分析
lawkit normal data.csv --verbose

# イベント分析
lawkit poisson incidents.csv --verbose
```