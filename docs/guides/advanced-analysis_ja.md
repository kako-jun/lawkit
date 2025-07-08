# 高度分析ガイド

このガイドでは、lawkit 2.0で利用可能な高度な統計分析機能について説明します。

## 目次

- [異常値検出](#異常値検出)
- [時系列分析](#時系列分析)
- [並列処理](#並列処理)
- [メモリ効率的な処理](#メモリ効率的な処理)
- [統計法則との統合](#統計法則との統合)
- [パフォーマンス最適化](#パフォーマンス最適化)

## 異常値検出

lawkitは従来のZスコア法を超える高度な異常値検出アルゴリズムを提供します。

### Local Outlier Factor (LOF)

LOFは近隣点と比較した局所密度に基づいて異常値を検出します。

```bash
# 基本的なLOF異常値検出
lawkit benf financial_data.csv --outlier-method lof

# 近隣数を設定
lawkit benf data.csv --outlier-method lof --outlier-k 10

# 特定の信頼閾値でLOF
lawkit benf transactions.csv --outlier-method lof --outlier-k 5 --outlier-threshold 1.5
```

**適用場面:**
- 密度が変化するクラスターデータ
- 金融詐欺検出
- ネットワーク異常検出

### Isolation Score

Isolation Forestの原理に基づき、高次元データに効果的です。

```bash
# 基本的な分離ベース検出
lawkit benf dataset.csv --outlier-method isolation

# 木の深さを設定
lawkit benf data.csv --outlier-method isolation --outlier-depth 10

# 検出感度を調整
lawkit benf logs.csv --outlier-method isolation --outlier-threshold 0.7
```

**適用場面:**
- 高次元データセット
- 大規模異常検出
- リアルタイム処理シナリオ

### DBSCAN風検出

不規則なパターンに対する密度ベース空間クラスタリングアプローチ。

```bash
# 基本的なDBSCAN異常値検出
lawkit benf spatial_data.csv --outlier-method dbscan

# 近傍パラメータを設定
lawkit benf data.csv --outlier-method dbscan --outlier-eps 0.5 --outlier-min-pts 3

# 自動パラメータ調整
lawkit benf dataset.csv --outlier-method dbscan --auto-tune
```

**適用場面:**
- 空間データ分析
- 不規則なクラスター形状
- ノイズフィルタリング

### アンサンブル手法（推奨）

複数のアルゴリズムを組み合わせて堅牢な異常値検出を行います。

```bash
# 全手法のアンサンブルを使用
lawkit benf data.csv --outlier-method ensemble

# 合意要件のあるアンサンブル
lawkit benf financial.csv --outlier-method ensemble --min-consensus 2

# 詳細なアンサンブル結果
lawkit benf data.csv --outlier-method ensemble --show-method-scores
```

**利点:**
- 合意による高精度
- パラメータ選択に対する堅牢性
- 信頼度スコアリング
- 手法比較の洞察

### 異常値分析の例

```bash
# 金融詐欺検出
lawkit benf transactions.csv \
  --outlier-method ensemble \
  --columns "amount,frequency" \
  --min-consensus 2 \
  --format json

# 製造業の品質管理
lawkit normal measurements.csv \
  --outlier-method lof \
  --outlier-k 7 \
  --statistical-tests \
  --confidence 0.99

# ネットワークセキュリティ分析
lawkit benf network_logs.csv \
  --outlier-method isolation \
  --outlier-depth 12 \
   \
  --real-time
```

## 時系列分析

予測機能を備えた高度な時間ベースパターン分析。

### 基本時系列分析

```bash
# 時間ベースデータの分析
lawkit benf sales_data.csv \
  --enable-timeseries \
  --timestamp-column "date" \
  --value-column "revenue"

# 自動タイムスタンプ検出
lawkit benf time_data.csv --enable-timeseries --auto-detect-timestamp
```

### トレンド分析

データのトレンドを検出・定量化します。

```bash
# 線形トレンド分析
lawkit benf stock_prices.csv \
  --enable-timeseries \
  --trend-analysis \
  --trend-method linear

# 多項式トレンドフィッティング
lawkit benf complex_data.csv \
  --enable-timeseries \
  --trend-analysis \
  --trend-method polynomial \
  --trend-degree 3

# トレンド有意性検定
lawkit benf metrics.csv \
  --enable-timeseries \
  --trend-analysis \
  --test-significance \
  --confidence 0.95
```

### 季節性検出

データの周期的パターンを特定します。

```bash
# 自動季節性検出
lawkit benf monthly_sales.csv \
  --enable-timeseries \
  --detect-seasonality

# 既知の周期を指定
lawkit benf daily_data.csv \
  --enable-timeseries \
  --detect-seasonality \
  --period 7

# 複数の季節性パターン
lawkit benf hourly_data.csv \
  --enable-timeseries \
  --detect-seasonality \
  --multiple-periods 24,168,8760
```

### 変化点検出

データパターンの重要な変化を発見します。

```bash
# 基本変化点検出
lawkit benf process_data.csv \
  --enable-timeseries \
  --detect-changepoints

# 敏感な変化点検出
lawkit benf metrics.csv \
  --enable-timeseries \
  --detect-changepoints \
  --changepoint-threshold 2.0

# 変化点タイプ分析
lawkit benf data.csv \
  --enable-timeseries \
  --detect-changepoints \
  --analyze-change-types
```

### 予測

信頼区間付きの予測を生成します。

```bash
# 基本予測
lawkit benf sales_history.csv \
  --enable-timeseries \
  --forecast-steps 12

# 信頼区間付き予測
lawkit benf revenue_data.csv \
  --enable-timeseries \
  --forecast-steps 6 \
  --confidence-interval 0.95

# トレンドと季節性を含む高度予測
lawkit benf complex_timeseries.csv \
  --enable-timeseries \
  --forecast-steps 24 \
  --include-trend \
  --include-seasonality \
  --uncertainty-quantification
```

### 時系列分析の例

```bash
# 金融市場分析
lawkit benf stock_data.csv \
  --enable-timeseries \
  --timestamp-column "date" \
  --trend-analysis \
  --detect-changepoints \
  --forecast-steps 30 \
  --format json

# IoTセンサーモニタリング
lawkit normal sensor_readings.csv \
  --enable-timeseries \
  --detect-seasonality \
  --period 24 \
  --anomaly-detection \
  --real-time-alerts

# ビジネスメトリクス追跡
lawkit analyze monthly_kpis.csv \
  --enable-timeseries \
  --trend-analysis \
  --forecast-steps 6 \
  --confidence-interval 0.90 \
  --dashboard-output
```

## 並列処理

大規模データセットの高速分析のためのマルチコア処理を活用します。

### 自動並列処理

```bash
# 自動並列化を有効化
lawkit analyze large_dataset.csv --enable-parallel

# lawkitに最適スレッド数を決定させる
lawkit benf huge_file.csv --enable-parallel --auto-threads
```

### 手動スレッド設定

```bash
# スレッド数を指定
lawkit analyze data.csv 

# 利用可能な全コアを使用
lawkit benf data.csv 

# 他のプロセス用にいくつかのコアを残す
lawkit analyze data.csv 
```

### チャンクベース処理

```bash
# メモリ効率のためのチャンクサイズ設定
lawkit benf large_data.csv \
  --enable-parallel \
  

# 適応的チャンクサイジング
lawkit analyze massive_file.csv \
  --enable-parallel \
  --adaptive-chunks \
  
```

### パフォーマンス監視

```bash
# 並列パフォーマンスをベンチマーク
lawkit analyze data.csv \
  --enable-parallel \
  --benchmark-parallel \
  --show-speedup

# 詳細なパフォーマンスメトリクス
lawkit benf data.csv \
  --enable-parallel \
  --performance-report \
  
```

### 並列処理の例

```bash
# 大規模詐欺検出
lawkit benf transactions.csv \
  --enable-parallel \
  \
  --outlier-method ensemble \
  

# マルチファイル比較分析
lawkit analyze datasets/*.csv \
  --enable-parallel \
  \
   \
  --output-format json

# リアルタイムデータ処理
lawkit benf data.csv \
  --optimize \
  --quiet
```

## メモリ効率的な処理

ストリーミングと増分アルゴリズムを使用して、利用可能RAMより大きなデータセットを処理します。

### ストリーミングモード

```bash
# 大ファイルの基本ストリーミング
lawkit benf massive_dataset.csv 

# カスタムチャンクサイズでのストリーミング
lawkit benf huge_file.csv  

# メモリ制限ストリーミング
lawkit benf data.csv  
```

### 増分統計

```bash
# Welfordのオンラインアルゴリズムを使用
lawkit benf large_data.csv --incremental-stats

# 定期的結果での増分処理
lawkit analyze data.csv \
  --incremental-stats \
  --progress-interval 10000

# メモリ監視
lawkit benf data.csv \
  --incremental-stats \
  --monitor-memory \
  --memory-alerts
```

### リソース管理

```bash
# メモリ制限を設定
lawkit benf data.csv 

# リソース監視
lawkit analyze large_files/*.csv \
  --monitor-resources \
   \
  --cpu-limit 80

# ガベージコレクション最適化
lawkit benf data.csv \
   \
  --optimize-gc \
  
```

### メモリ効率的な例

```bash
# 4GB RAMで10GB以上のデータセットを処理
lawkit benf massive_financial_data.csv \
   \
   \
   \
  --incremental-stats \
  --progress-reporting

# 継続的データ処理
lawkit benf continuous_data.csv \
   \
  --real-time \
   \
  --buffer-size 100 \
  --live-updates

# マルチギガバイト比較
lawkit analyze huge_datasets/*.csv \
   \
  --enable-parallel \
   \
  --incremental-stats \
  --summary-only
```

## 統計法則との統合

高度機能をlawkitの統計法則分析と組み合わせます。

### 強化ベンフォード分析

```bash
# 高度異常値検出付きベンフォード法則
lawkit benf financial_data.csv \
  --outlier-method ensemble \
  --time-series-analysis \
  

# マルチスケールベンフォード分析
lawkit benf data.csv \
  --benford-scales 1,2,3 \
  --outlier-method lof \
  --confidence-intervals
```

### 時系列付きパレート分析

```bash
# 時間を通じた動的パレート分析
lawkit pareto sales_data.csv \
  --enable-timeseries \
  --track-pareto-changes \
  --forecast-pareto-shifts

# 季節パレートパターン
lawkit pareto seasonal_data.csv \
  --enable-timeseries \
  --detect-seasonality \
  --pareto-by-season
```

### 正規分布の強化

```bash
# 高度正規性検定
lawkit normal data.csv \
  --multiple-tests \
  --outlier-removal \
  --robust-estimation

# 時変正規性
lawkit normal time_data.csv \
  --enable-timeseries \
  --sliding-window-tests \
  --normality-tracking
```

### マルチ法則比較

```bash
# 包括的統計分析
lawkit analyze complex_data.csv \
  --all-laws \
  --advanced-outliers \
  --time-series-analysis \
   \
  --detailed-report

# 合意ベース分析
lawkit analyze data.csv \
  --law-ensemble \
  --outlier-consensus \
  --confidence-aggregation \
  --uncertainty-quantification
```

## パフォーマンス最適化

特定のユースケースに基づいて分析パフォーマンスを最適化します。

### データセットサイズガイドライン

**小規模データセット (< 10Kレコード):**
```bash
lawkit benf data.csv  
```

**中規模データセット (10K - 1Mレコード):**
```bash
lawkit benf data.csv \
  --enable-parallel \
  \
  
```

**大規模データセット (1M+レコード):**
```bash
lawkit benf data.csv \
   \
  --enable-parallel \
  \
   \
  --incremental-stats
```

### ユースケース最適化

**リアルタイム分析:**
```bash
lawkit benf data.csv \
   \
  --real-time \
  --outlier-method isolation \
  --fast-mode
```

**バッチ処理:**
```bash
lawkit analyze datasets/*.csv \
  --enable-parallel \
  \
  --batch-mode \
  --summary-only
```

**インタラクティブ分析:**
```bash
lawkit benf data.csv \
  --interactive \
  --progressive-results \
  --visualization \
  --dashboard-mode
```

### ハードウェア最適化

**CPU集約的ワークロード:**
```bash
lawkit analyze data.csv \
  --enable-parallel \
  --cpu-intensive \
  \
  --no-io-limit
```

**メモリ制約システム:**
```bash
lawkit benf data.csv \
   \
   \
  --minimal-cache \
  --compress-intermediate
```

**ストレージ最適化:**
```bash
lawkit benf data.csv \
   \
  --direct-io \
  --minimal-memory \
  --compress-output
```

これらの高度機能を使用して、特定の要件と制約に合わせた高度な統計分析を実行してください。