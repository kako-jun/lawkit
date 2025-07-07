# はじめに

lawkitの基本的な使用方法を学んでください。このガイドでは、主要な機能を実際のサンプルデータを使用して説明します。

## 基本的な使用方法

### 1. ベンフォード法則分析

会計データや自然データの不正検知に使用します：

```bash
# 基本的な分析
lawkit benf data.csv

# 詳細出力
lawkit benf data.csv --verbose

# JSON形式で出力
lawkit benf data.csv --output json

# 特定の列を分析
lawkit benf data.csv --columns "金額,売上"
```

### 2. パレート分析（80/20法則）

売上分析や在庫管理に使用します：

```bash
# 基本的なパレート分析
lawkit pareto sales.csv

# 閾値を指定（80/20ではなく90/10分析）
lawkit pareto sales.csv --threshold 0.9

# Gini係数も計算
lawkit pareto sales.csv --gini
```

### 3. ジップ法則分析

テキストデータの単語頻度分析：

```bash
# テキストファイルの分析
lawkit zipf document.txt

# テキスト分析
lawkit zipf japanese_text.txt

# 最小出現回数を指定
lawkit zipf text.txt --min-frequency 5
```

### 4. 正規分布分析

品質管理や異常値検出：

```bash
# 正規性検定
lawkit normal measurements.csv

# 管理図の作成
lawkit normal quality_data.csv --control-chart

# 工程能力評価
lawkit normal process_data.csv --capability
```

### 5. ポアソン分布分析

イベント発生予測や稀少事象分析：

```bash
# 基本的なポアソン分析
lawkit poisson events.csv

# 時間間隔を指定
lawkit poisson events.csv --interval hour

# 予測期間を指定
lawkit poisson events.csv --forecast 30
```

### 6. 複数法則比較

複数の統計法則を同時に適用して包括的な分析：

```bash
# 複数法則の比較
lawkit compare data.csv --laws benford,pareto,normal

# 矛盾検出
lawkit compare data.csv --laws all --detect-conflicts

# 推奨システム
lawkit compare data.csv --recommend
```

## サンプルデータでの実習

### 会計データの不正検知

```bash
# サンプル会計データを作成
echo "取引ID,金額,日付
1,1234,2024-01-01
2,2345,2024-01-02
3,3456,2024-01-03" > accounting.csv

# ベンフォード法則で分析
lawkit benf accounting.csv --columns 金額
```

### 売上データのパレート分析

```bash
# サンプル売上データを作成
echo "商品,売上
A商品,80000
B商品,12000
C商品,5000
D商品,2000
E商品,1000" > sales.csv

# パレート分析実行
lawkit pareto sales.csv --threshold 0.8
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
```

## 多言語対応

lawkitは以下の言語に対応しています：

```bash
# 英語出力（デフォルト統一）
lawkit benf data.csv

# 日本語数字は自動認識されます
echo "１２３４５６ ７８９０" | lawkit benf

# 中国語数字も自動認識されます
echo "一千二百三十四" | lawkit benf
```

## 高度な機能

### フィルタリング

```bash
# 特定の範囲のデータのみ分析
lawkit benf data.csv --min-value 1000 --max-value 100000

# 閾値設定
lawkit pareto sales.csv --threshold 0.95
```

### パフォーマンス調整

```bash
# 大きなファイルの場合、サンプリング
lawkit benf large_data.csv --sample-size 10000

# 並列処理のスレッド数指定
lawkit compare data.csv --threads 4
```

## よく使われるワークフロー

### 1. データ品質チェック
```bash
# 総合的なデータ品質評価
lawkit compare financial_data.csv --laws benford,normal --detect-conflicts
```

### 2. 不正検知パイプライン
```bash
# ベンフォード法則で初期スクリーニング
lawkit benf transactions.csv --output json > results.json

# 異常値を詳細分析
lawkit normal suspicious_data.csv --outliers
```

### 3. ビジネス分析
```bash
# 売上のパレート分析
lawkit pareto monthly_sales.csv --gini

# 顧客分析
lawkit zipf customer_feedback.txt
```

## 次のステップ

- [設定ガイド](configuration_ja.md) - カスタマイズと高度な設定
- [使用例](examples_ja.md) - 実際のユースケース
- [CLIリファレンス](../reference/cli-reference_ja.md) - 全コマンド詳細