# 設定ガイド

lawkitの動作をカスタマイズするための設定方法を説明します。

## 設定ファイル

lawkitは以下の場所で設定ファイルを検索します：

1. `./lawkit.toml` (現在のディレクトリ)
2. `~/.config/lawkit/config.toml` (ユーザー設定)
3. `/etc/lawkit/config.toml` (システム設定)

### 基本的な設定例

```toml
# lawkit.toml

[general]
# デフォルト出力形式
default_output = "json"
# 注: 出力言語は英語で統一されています
# 並列処理のスレッド数
optimize = true

[benford]
# ベンフォード法則のデフォルト設定
confidence_level = 0.95
chi_squared_threshold = 0.05

[pareto]
# パレート法則のデフォルト設定
default_threshold = 0.8
include_gini = true

[zipf]
# ジップ法則のデフォルト設定
min_frequency = 2
max_words = 1000

[normal]
# 正規分布のデフォルト設定
normality_tests = ["shapiro", "anderson", "ks"]
outlier_method = "iqr"

[poisson]
# ポアソン分布のデフォルト設定
confidence_interval = 0.95
forecast_periods = 30

[integration]
# 統合分析のデフォルト設定
conflict_detection = true
auto_recommend = true
```

## 環境変数

設定は環境変数でも指定できます：

```bash
# 出力形式
export LAWKIT_OUTPUT=json

# 詳細出力
export LAWKIT_VERBOSE=true

# 詳細出力
export LAWKIT_VERBOSE=true

# 最適化モード
export LAWKIT_OPTIMIZE=true
```

## 言語設定

### 対応言語

| 言語コード | 言語名 | 数字認識 |
|------------|--------|----------|
| `en` | 英語 | 123,456.78 |
| `ja` | 日本語 | 123,456.78, １２３４５６ |
| `zh` | 中国語 | 123,456.78, 壹貳參 |
| `hi` | ヒンディー語 | 123,456.78, १२३४५६ |
| `ar` | アラビア語 | 123,456.78, ١٢٣٤٥٦ |

### 言語設定例

```bash
# コマンドラインで指定
# 国際数字フォーマットは自動認識されます
lawkit benf data.csv

# 設定ファイルで指定
# 出力は英語に統一されています

# 環境変数で指定
export LAWKIT_OPTIMIZE=true
```

## 入力設定

### ファイル形式の設定

```toml
[input]
# CSVの区切り文字
csv_delimiter = ","
# CSVのヘッダー行の有無
csv_has_header = true
# Excelの最大行数
excel_max_rows = 100000
# PDFのテキスト抽出方法
pdf_extraction_method = "advanced"
```

### 数字認識の設定

```toml
[parsing]
# 国際数字形式の認識
international_numbers = true
# 通貨記号の無視
ignore_currency = true
# パーセント記号の処理
handle_percentages = true
```

## 出力設定

### 出力形式のカスタマイズ

```toml
[output]
# デフォルト出力形式
default_format = "json"
# 精度（小数点以下の桁数）
precision = 6
# 科学的記数法の使用
scientific_notation = false

[output.json]
# JSON出力の整形
pretty_print = true
# null値の出力
include_nulls = false

[output.csv]
# CSV出力の区切り文字
delimiter = ","
# ヘッダー行の出力
include_header = true
```

## 統計法則別設定

### ベンフォード法則

```toml
[benford]
# 信頼度レベル
confidence_level = 0.95
# カイ二乗検定の閾値
chi_squared_threshold = 0.05
# 最小データ数
min_data_points = 30
# 日本語数字の認識
japanese_numbers = true
```

### パレート法則

```toml
[pareto]
# デフォルト閾値（80/20の80）
default_threshold = 0.8
# Gini係数の計算
include_gini = true
# ローレンツ曲線の描画
draw_lorenz_curve = false
```

### ジップ法則

```toml
[zipf]
# 最小出現頻度
min_frequency = 2
# 最大単語数
max_words = 1000
# ストップワードの除去
remove_stopwords = true
# 大文字小文字の区別
case_sensitive = false
```

## パフォーマンス設定

### メモリ使用量

```toml
[performance]
# 並列処理のスレッド数
optimize = true
# メモリ制限（MB）
memory_limit = 1024
# 大きなファイルの処理方法
optimize = true
```

### キャッシュ設定

```toml
[cache]
# キャッシュの有効化
enabled = true
# キャッシュディレクトリ
directory = "~/.cache/lawkit"
# キャッシュの有効期限（秒）
ttl = 3600
```

## ログ設定

```toml
[logging]
# ログレベル
level = "info"
# ログファイル
file = "/var/log/lawkit.log"
# ローテーション設定
rotate = true
max_size = "10MB"
```

## プロファイル設定

異なる用途に応じた設定プロファイルを作成できます：

```toml
[profiles.audit]
# 監査用設定
default_output = "json"
benford.confidence_level = 0.99
logging.level = "debug"

[profiles.quick]
# 高速分析用設定
performance.optimize = true
cache.enabled = true
benford.min_data_points = 10

[profiles.comprehensive]
# 包括分析用設定
integration.conflict_detection = true
integration.auto_recommend = true
normal.normality_tests = ["shapiro", "anderson", "ks", "jarque_bera"]
```

プロファイルの使用：

```bash
# 監査用設定で実行
lawkit benf data.csv --profile audit

# 高速分析用設定で実行
lawkit benf data.csv --profile quick
```

## 設定の確認

現在の設定を確認するには：

```bash
# 全設定を表示
lawkit config show

# 特定のセクションを表示
lawkit config show benford

# 設定ファイルの場所を表示
lawkit config path
```

## トラブルシューティング

### 設定が反映されない場合

1. 設定ファイルの構文確認：
   ```bash
   lawkit config validate
   ```

2. 設定の優先順位確認：
   ```bash
   lawkit config show --source
   ```

3. 権限の確認：
   ```bash
   ls -la ~/.config/lawkit/config.toml
   ```

**注意**: このドキュメントに記載されている設定機能の多くは、現在実装されていません。実際に使用できる機能については、`lawkit --help` および各サブコマンドの `--help` を参照してください。