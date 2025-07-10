# 設定ガイド

lawkitの基本的な使用方法とコマンドラインオプションについて説明します。

## 基本的なオプション

lawkitでは以下の基本オプションが利用できます：

```bash
# 出力形式の指定
lawkit benf data.csv --output json
lawkit benf data.csv --output csv
lawkit benf data.csv --output yaml

# 詳細出力
lawkit benf data.csv --verbose

# 最適化モード（大きなファイル用）
lawkit benf data.csv --optimize
```

## 多言語数字認識

lawkitは以下の数字形式を自動認識します：

| 言語 | 数字認識例 |
|------|------------|
| 英語 | 123,456.78 |
| 日本語 | 123,456.78, １２３４５６, 五万六千 |
| 中国語 | 123,456.78, 壹貳參肆伍, 一万二千 |
| ヒンディー語 | 123,456.78, १२३४५६ |
| アラビア語 | 123,456.78, ١٢٣٤٥٦ |

```bash
# 国際数字フォーマットは自動認識されます
lawkit benf data.csv

# 出力は英語に統一されています
lawkit benf japanese_numbers.csv --verbose
```

## 入力データ形式

lawkitは以下の入力形式に対応しています：

```bash
# CSV ファイル
lawkit benf data.csv

# テキストファイル（ジップ法則用）
lawkit zipf document.txt

# 標準入力からのデータ
echo "1234 5678 9012" | lawkit benf

# 複数ファイルの処理
lawkit benf file1.csv file2.csv
```

## 出力形式

lawkitは複数の出力形式に対応しています：

```bash
# テキスト形式（デフォルト）
lawkit benf data.csv

# JSON形式
lawkit benf data.csv --output json

# CSV形式
lawkit benf data.csv --output csv

# YAML形式
lawkit benf data.csv --output yaml

# XML形式
lawkit benf data.csv --output xml

# TOML形式
lawkit benf data.csv --output toml
```

## 統計法則別オプション

### ベンフォード法則

```bash
# 基本分析
lawkit benf financial_data.csv

# 閾値指定
lawkit benf data.csv --threshold high

# 詳細出力
lawkit benf data.csv --verbose
```

### パレート法則

```bash
# 基本分析
lawkit pareto sales_data.csv

# 濃度指定（90/10分析）
lawkit pareto data.csv --concentration 0.9

# Gini係数計算
lawkit pareto data.csv --gini-coefficient

# パーセンタイル分析
lawkit pareto data.csv --percentiles 80,90,95
```

### ジップ法則

```bash
# 基本分析
lawkit zipf document.txt

# 最小出現回数指定
lawkit zipf text.txt --min-count 5

# 詳細出力
lawkit zipf document.txt --verbose
```

## パフォーマンス最適化

大きなファイルを処理する際のオプション：

```bash
# 最適化モード（サンプリング使用）
lawkit benf large_file.csv --optimize

# 詳細な進捗表示
lawkit benf big_data.csv --verbose

# 複数ファイルの並列処理
lawkit benf file1.csv file2.csv file3.csv --optimize
```

## 高度な分析機能

```bash
# 複数法則での統合分析
lawkit analyze data.csv --laws benf,pareto,normal

# データ検証
lawkit validate data.csv --laws all

# 診断レポート
lawkit diagnose data.csv --focus conflict

# 異常値検出
lawkit normal data.csv --outliers

# 時系列分析
lawkit normal timeseries.csv --time-series
```

## データ生成機能

lawkitでは検証用のサンプルデータを生成できます：

```bash
# ベンフォード分布に従うデータ生成
lawkit generate benf --size 1000

# パレート分布データ生成
lawkit generate pareto --size 500 --alpha 1.5

# 正規分布データ生成
lawkit generate normal --size 1000 --mean 50 --std 10

# ポアソン分布データ生成
lawkit generate poisson --size 100 --lambda 3.5

# ジップ分布データ生成
lawkit generate zipf --size 1000 --alpha 1.0
```

## 利用可能なコマンド一覧

```bash
# 全コマンドとオプションの表示
lawkit --help

# 特定のコマンドのヘルプ
lawkit benf --help
lawkit pareto --help
lawkit zipf --help
lawkit normal --help
lawkit poisson --help

# 利用可能な法則の一覧
lawkit list
```

## トラブルシューティング

### よくある問題と解決方法

1. **ファイルが読み込めない場合**：
   ```bash
   # ファイルの存在確認
   ls -la data.csv
   
   # 権限の確認
   file data.csv
   ```

2. **メモリ不足の場合**：
   ```bash
   # 最適化モードを使用
   lawkit benf large_file.csv --optimize
   ```

3. **予期しない結果の場合**：
   ```bash
   # 詳細出力で原因を確認
   lawkit benf data.csv --verbose
   ```

**注意**: 実際に使用できる機能については、`lawkit --help` および各サブコマンドの `--help` を参照してください。