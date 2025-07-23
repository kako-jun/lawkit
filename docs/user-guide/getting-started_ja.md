# はじめに

lawkitの基本的な使い方を学びましょう。このガイドでは、実際のサンプルデータを使って主要な機能を説明します。

## 基本的な使い方

### 1. ベンフォードの法則分析

会計データや自然データの不正検出に使用されます:

```bash
# 基本分析
lawkit benf data.csv

# 詳細出力
lawkit benf data.csv --verbose

# JSON形式で出力
lawkit benf data.csv --format json

# 指定した闾値で分析
lawkit benf data.csv --threshold high

# 監査レベルの分析（99%信頼度）
lawkit benf audit_data.csv --confidence 0.99 --verbose

# 大規模データセットの最適化
lawkit benf large_data.csv --sample-size 10000 --optimize

# 小さな値をフィルタ（ノイズ除去）
lawkit benf financial_data.csv --min-value 100
```

### 2. パレート分析（80/20の法則）

売上分析や在庫管理に使用されます:

```bash
# 基本的なパレート分析
lawkit pareto sales.csv

# 闾値を指定（80/20の代わりに90/10分析）
lawkit pareto sales.csv --concentration 0.9

# ジニ係数も計算
lawkit pareto sales.csv --gini-coefficient
```

### 3. ジップの法則分析

テキストデータの単語頻度分析:

```bash
# テキストファイルを分析
lawkit zipf document.txt

# 日本語テキスト分析
lawkit zipf japanese_text.txt

# 最小出現回数を指定
lawkit zipf text.txt --min-count 5
```

### 4. 正規分布分析

品質管理と外れ値検出:

```bash
# 正規性検定
lawkit normal measurements.csv

# 詳細な正規性検定
lawkit normal quality_data.csv --verbose

# 外れ値も検出
lawkit normal process_data.csv --outliers
```

### 5. ポアソン分布分析

イベント発生予測と稀なイベント分析:

```bash
# 基本的なポアソン分析
lawkit poisson events.csv

# 詳細なポアソン分析
lawkit poisson events.csv --verbose

# 高信頼度分析
lawkit poisson critical_events.csv --confidence 0.99 --verbose
```

### 6. 多法則比較

複数の統計法則を同時に適用した包括的分析:

```bash
# 多法則分析と推奨事項
lawkit analyze data.csv --laws benf,pareto,normal

# データ一貫性チェック
lawkit validate data.csv --laws all

# 詳細診断レポート
lawkit diagnose data.csv --focus conflict
```

## サンプルデータでの実践

### 会計データの不正検出

```bash
# サンプル会計データを作成
echo "TransactionID,Amount,Date
1,1234,2024-01-01
2,2345,2024-01-02
3,3456,2024-01-03" > accounting.csv

# ベンフォードの法則で分析
lawkit benf accounting.csv
```

### 売上データのパレート分析

```bash
# サンプル売上データを作成
echo "Product,Sales
Product A,80000
Product B,12000
Product C,5000
Product D,2000
Product E,1000" > sales.csv

# パレート分析を実行
lawkit pareto sales.csv --threshold 0.8
```

## 出力形式

lawkitは複数の出力形式をサポートしています:

```bash
# テキスト形式（デフォルト）
lawkit benf data.csv

# JSON形式
lawkit benf data.csv --format json

# CSV形式
lawkit benf data.csv --format csv

# YAML形式
lawkit benf data.csv --format yaml

# XML形式
lawkit benf data.csv --format xml
```

## 多言語サポート

lawkitは以下の言語をサポートしています:

```bash
# 英語出力（デフォルト統一）
lawkit benf data.csv

# 日本語数字は自動認識されます
echo "１２３４５６ ７８９０" | lawkit benf

# 中国語数字も自動認識されます
echo "一千二百三十四" | lawkit benf

# 繁体字中国語（古式）の金融数字もサポート
echo "壹萬貳仟參佰肆拾伍" | lawkit benf

# 日本語の漢数字も自動認識されます
echo "五万六千七百八十九" | lawkit benf
```

## 高度な機能

### フィルタリング

```bash
# 特定の範囲内のデータのみ分析
lawkit benf data.csv --filter ">=1000"

# 闾値設定
lawkit pareto sales.csv --concentration 0.95
```

### パフォーマンスチューニング

```bash
# 大きなファイルにはサンプリングを使用
lawkit benf large_data.csv --optimize

# 並列処理のスレッド数を指定
lawkit analyze data.csv --recommend
```

## 一般的なワークフロー

### 1. データ品質チェック
```bash
# 包括的なデータ品質評価
lawkit validate financial_data.csv --laws benf,normal
```

### 2. 不正検出パイプライン
```bash
# ベンフォードの法則で初期スクリーニング
lawkit benf transactions.csv --format json > results.json

# 異常の詳細分析
lawkit normal suspicious_data.csv --verbose
```

### 3. ビジネス分析
```bash
# 売上のパレート分析
lawkit pareto monthly_sales.csv --gini-coefficient

# 顧客分析
lawkit zipf customer_feedback.txt
```

## 次のステップ

- [Examples](examples_ja.md) - 実世界のユースケース
- [CLIリファレンス](../reference/cli-reference_ja.md) - すべてのコマンド詳細
- [Architecture](../guides/architecture_ja.md) - システム設計