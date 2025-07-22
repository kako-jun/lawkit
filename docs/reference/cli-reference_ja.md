# CLIリファレンス

`lawkit`の完全なコマンドラインインターフェースドキュメント。

## グローバルコマンド

### `lawkit --help`
メインコマンドまたはサブコマンドのヘルプ情報を表示します。

### `lawkit --version`
バージョン情報を表示します。

### `lawkit list`
利用可能なすべての統計法則とその説明を一覧表示します。

```bash
$ lawkit list
Available statistical laws:
  benf    - Benford Law analysis
  pareto  - Pareto Principle (80/20 rule) analysis
  zipf    - Zipf Law analysis
  normal  - Normal Distribution analysis
  poisson - Poisson Distribution analysis

Integration commands:
  analyze  - Multi-law basic analysis and recommendations
  validate - Data validation and consistency checks  
  diagnose - Conflict detection and detailed diagnostics
```

## 統計法則コマンド

### `lawkit benf` - ベンフォードの法則分析

ベンフォードの法則を使用して異常を検出し、データ品質を評価します。

```bash
lawkit benf [OPTIONS] [INPUT]
```

#### オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小出力（数値のみ）
- `--verbose, -v` - 詳細な分析洞察を含む詳細デバッグ出力を有効化
- `--filter <RANGE>` - 範囲による数値フィルタリング（例: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数（デフォルト: 10）
- `--threshold <LEVEL>, -t` - 異常検出閾値: low, medium, high, critical（デフォルト: auto）
- `--confidence <LEVEL>` - 統計テストの信頼水準（0.01-0.99、デフォルト: 0.95）
- `--sample-size <NUMBER>` - 大規模データセットの最大サンプルサイズ（パフォーマンス向上）
- `--min-value <VALUE>` - 分析に含める最小値（ノイズを追加する小さな値をフィルタリング）

**注記**: `--optimize`オプションは非推奨になりました。最適化は自動的に適用されます。
#### 詳細出力
`--verbose`フラグは包括的なデバッグと分析情報を提供します：

**デバッグ情報:**
- 入力引数の検出と検証
- データ処理戦略（自動最適化、ストリーミング）
- フィルター適用と前後の統計
- データ収集と解析詳細

**パフォーマンス指標:**
- ミリ秒単位の処理時間
- MB単位のメモリ使用量
- 大規模データセットで処理されたチャンク数
- 処理されたアイテム数

**分析洞察:**
- 統計計算ステップ
- 信頼区間の詳細
- アルゴリズム選択の理由
- データ品質評価

詳細出力の例:
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

# 最小出力のための静音モード
lawkit benf data.csv --quiet

# 高闾値での大型トランザクションフィルタリング
lawkit benf accounts.csv --filter ">=1000" --threshold high

# 監査用の高信頼度分析（99%信頼水準）
lawkit benf audit_data.csv --confidence 0.99 --verbose

# 大規模データセットのパフォーマンス最適化
lawkit benf big_data.csv --sample-size 50000

# 分析にノイズを追加する小さな値を除外
lawkit benf financial_data.csv --min-value 100
```

### `lawkit pareto` - パレート原理分析

集中度を分析し、80/20の法則を適用します。

```bash
lawkit pareto [OPTIONS] [INPUT]
```

#### 共通オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小出力
- `--verbose, -v` - 詳細出力
- `--filter <RANGE>` - 範囲による数値フィルタリング（例: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数（デフォルト: 10）

#### 固有オプション
- `--concentration <THRESHOLD>, -C` - 集中度閾値 (0.0-1.0)（デフォルト: 0.8）
- `--gini-coefficient` - 不平等測定のためのジニ係数を計算
- `--percentiles <PERCENTILES>` - カスタムパーセンタイル計算（例: 70,80,90）
- `--business-analysis` - ビジネス分析インサイトを有効化

#### 例
```bash
# 基本的なパレート分析
lawkit pareto sales.csv

# カスタム閾値
lawkit pareto data.csv --concentration 0.9

# ジニ係数を含むビジネス分析
lawkit pareto customers.csv --business-analysis --gini-coefficient

# カスタムパーセンタイル
lawkit pareto revenue.csv --percentiles 70,80,90,95
```

### `lawkit zipf` - ジップの法則分析

頻度分布とランキングパターンを分析します。数値データとテキストデータの両方の分析をサポートします。

```bash
lawkit zipf [OPTIONS] [INPUT]
```

#### 共通オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小出力
- `--verbose, -v` - 詳細出力
- `--filter <RANGE>` - 範囲による数値フィルタリング（例: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数（デフォルト: 10）

#### 固有オプション
- `--text, -T` - テキスト分析モードを有効化
- `--words <NUMBER>, -w` - テキストモードで分析する最大単語数（デフォルト: 1000）

#### 例
```bash
# 基本的なジップ分析（数値データ）
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

正規性をテストし、外れ値を検出します。高度な統計分析機能を提供します。

```bash
lawkit normal [OPTIONS] [INPUT]
```

#### 共通オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小出力
- `--verbose, -v` - 詳細出力
- `--filter <RANGE>` - 範囲による数値フィルタリング（例: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数（デフォルト: 10）

#### 分析オプション
- `--test <METHOD>, -T` - 正規性テスト方法: shapiro, anderson, ks, all（デフォルト: all）
- `--outliers, -O` - 外れ値検出を有効化
- `--outlier-method <METHOD>` - 外れ値検出方法: zscore, modified_zscore, iqr, lof, isolation, dbscan, ensemble（デフォルト: zscore）
- `--quality-control, -Q` - 品質管理分析を有効化
- `--spec-limits <LOWER,UPPER>` - 品質管理の仕様限界（例: 9.5,10.5）
- `--enable-timeseries` - 時系列分析を有効化
- `--timeseries-window <SIZE>` - 時系列分析ウィンドウサイズ（デフォルト: 10）

#### 例
```bash
# 基本的な正規性テスト
lawkit normal data.csv

# 特定のテスト方法
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

イベント発生と稀なイベントを分析します。

```bash
lawkit poisson [OPTIONS] [INPUT]
```

#### 共通オプション
- `--format <FORMAT>, -f` - 出力形式: text, json, csv, yaml, toml, xml (デフォルト: text)
- `--quiet, -q` - 最小出力（数値のみ）
- `--verbose, -v` - 詳細な分析洞察を含む詳細デバッグ出力を有効化
- `--filter <RANGE>` - 範囲による数値フィルタリング（例: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析に必要な最小データポイント数（デフォルト: 10）
- `--confidence <LEVEL>` - 統計テストの信頼水準（0.01-0.99、デフォルト: 0.95）

#### 分析オプション
- `--test <METHOD>, -T` - 適合度テスト方法: chi_square, ks, variance, all（デフォルト: all）
- `--predict, -p` - 確率予測を有効化
- `--max-events <NUMBER>` - 分析用の最大イベント数（デフォルト: 20）
- `--rare-events, -R` - 稀なイベント分析に焦点を当てる

**注記**: `--optimize`オプションは非推奨になりました。最適化は自動的に適用されます。

#### 例
```bash
# 基本的なポアソン分析
lawkit poisson events.csv

# 特定のテスト方法
lawkit poisson data.csv --test chi_square

# 確率予測モード
lawkit poisson server_logs.csv --predict --max-events 50

# 稀なイベント分析
lawkit poisson rare_events.csv --rare-events

# 詳細出力
lawkit poisson incidents.csv --verbose

# JSON出力形式
lawkit poisson data.csv --format json

# 重要な分析のための高信頼度レベル
lawkit poisson server_errors.csv --confidence 0.99 --verbose
```

## 生成コマンド

### `lawkit generate` - サンプルデータ生成

テストと検証のために特定の統計法則に従うサンプルデータを生成します。

```bash
lawkit generate <LAW> [OPTIONS]
```

#### 利用可能な法則
- `benf` - ベンフォードの法則準拠データを生成
- `pareto` - パレート分布データを生成
- `zipf` - ジップの法則データを生成
- `normal` - 正規分布データを生成
- `poisson` - ポアソン分布データを生成

#### 共通生成オプション
- `--samples <NUMBER>` - 生成するサンプル数（デフォルト: 1000）
- `--seed <NUMBER>` - 再現可能な生成のためのランダムシード
- `--output-file <FILE>` - 出力ファイルパス（デフォルト: stdout）

#### 法則固有オプション

**ベンフォード生成:**
- `--fraud-rate <RATE>` - テスト用の不正注入率 (0.0-1.0)（デフォルト: 0.0）
- `--range <MIN,MAX>` - 生成用の数値範囲（例: 1,10000）（デフォルト: 1,100000）

#### 例
```bash
# ベンフォードの法則データを生成
lawkit generate benf --samples 5000

# 不正注入付きベンフォードデータを生成
lawkit generate benf --samples 2000 --fraud-rate 0.1

# カスタム範囲で再現可能なデータを生成
lawkit generate benf --samples 1000 --seed 42 --range 1,50000

# ファイルに生成して保存
lawkit generate normal --samples 1000 --output-file test_data.csv
```

## 統合コマンド

### `lawkit analyze` - 複数法則分析

包括的なデータ評価のための推奨事項を含む基本的な複数法則分析を実行します。

```bash
lawkit analyze [OPTIONS] [INPUT]
```

### `lawkit validate` - データ検証

複数の統計パターンにわたってデータの一貫性と品質を検証します。

```bash
lawkit validate [OPTIONS] [INPUT]
```

### `lawkit diagnose` - 競合検出

統計法則の結果間の競合を検出し、詳細な診断を提供します。

```bash
lawkit diagnose [OPTIONS] [INPUT]
```

#### オプション
- `--laws <LAWS>` - 分析する特定の法則: benf,pareto,zipf,normal,poisson
- `--focus <FOCUS>` - 分析の焦点: quality, concentration, distribution, anomaly
- `--purpose <PURPOSE>` - 分析の目的: quality, fraud, concentration, anomaly, distribution, general
- `--recommend` - 最適法則推奨モードを有効化
- `--threshold <THRESHOLD>` - 競合検出闾値 (0.0-1.0)（デフォルト: 0.5）
- `--report <TYPE>` - 統合レポートタイプ: summary, detailed, conflicting（デフォルト: summary）
- `--consistency-check` - 一貫性チェックを有効化
- `--cross-validation` - クロス検証分析を有効化
- `--confidence-level <LEVEL>` - 信頼度レベル（デフォルト: 0.95）

#### 例
```bash
# すべての法則を比較
lawkit analyze data.csv

# 不正検出に焦点を当てる
lawkit analyze transactions.csv --purpose fraud --recommend

# カスタム法則選択
lawkit analyze data.csv --laws benf,normal --focus quality

# JSON形式での詳細出力
lawkit analyze dataset.csv --verbose --format json
```

## 共通オプション

すべてのコマンドはこれらの共通オプションをサポートします：

### 入出力
- `[INPUT]` - 入力データ（ファイルパス、URL、またはstdinの場合は'-'）
- `--format <FORMAT>` - 出力形式: text, json, csv, yaml, toml, xml
- `--quiet, -q` - 最小出力
- `--verbose, -v` - 詳細出力
- `--no-color` - カラー出力を無効化

**注記**: 最適化は自動的に適用されるため、`--optimize`オプションは不要になりました。

### データ処理
- `--filter <RANGE>` - 数値フィルタリング (>=100, <1000, 50-500)
- `--min-count <NUMBER>` - 必要な最小データポイント数（デフォルト: 10）

## 入力形式

`lawkit`は複数の入力形式をサポートします：

- **テキストファイル** - 空白またはカンマで区切られた数値
- **CSV** - カンマ区切り値
- **JSON** - 構造化データ
- **YAML** - YAML設定ファイル
- **TOML** - TOML設定ファイル
- **XML** - XMLデータファイル

## 出力形式

### テキスト形式（デフォルト）
分析結果、解釈、推奨事項を含む人間が読める出力。

### JSON形式
APIや自動化のための機械読み取り可能な構造化出力：
```json
{
  "dataset": "data.csv",
  "numbers_analyzed": 1000,
  "risk_level": "Low",
  "analysis_results": {...}
}
```

### CSV形式
スプレッドシートインポート用の表形式：
```csv
dataset,numbers_analyzed,risk_level,score
data.csv,1000,Low,0.85
```

## 終了コード

- `0` - 成功、低リスク
- `10` - 中リスクを検出
- `11` - 高リスクを検出
- `12` - クリティカルリスクを検出
- `1` - 分析エラー
- `2` - 無効な引数
- `3` - ファイル/ネットワークエラー

## ユースケース別の例

### 不正検出
```bash
# 金融取引分析
lawkit benf transactions.csv --verbose

# 複数法則による不正検出
lawkit analyze suspicious_data.csv --purpose fraud --recommend
```

### データ品質評価
```bash
# 包括的な品質チェック
lawkit analyze dataset.csv --purpose quality --verbose

# 正規性に焦点を当てる
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