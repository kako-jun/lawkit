# lawkit CLI 仕様書

## 概要

統計法則分析ツール。データがベンフォードの法則、パレートの法則、ジップの法則などに従っているかを分析し、異常検知や品質管理に活用。

## サブコマンド

### 分析コマンド

| コマンド | 説明 |
|----------|------|
| `benf` | ベンフォードの法則分析（先頭桁の分布） |
| `pareto` | パレートの法則分析（80/20ルール、集中度） |
| `zipf` | ジップの法則分析（頻度分布、テキスト分析可） |
| `normal` | 正規分布分析（外れ値検出、品質管理、時系列分析） |
| `poisson` | ポアソン分布分析（稀事象分析） |

### 統合コマンド

| コマンド | 説明 |
|----------|------|
| `analyze` | 複数法則の統合分析と推奨 |
| `validate` | データ検証・整合性チェック |
| `diagnose` | 詳細診断・衝突検出 |

### ユーティリティ

| コマンド | 説明 |
|----------|------|
| `generate <law>` | サンプルデータ生成（benf, pareto, zipf, normal, poisson） |
| `list` | 利用可能な法則一覧 |
| `selftest` | セルフテスト実行 |

## 入力

- ファイルパス
- URL（HTTP/HTTPS）
- `-` または省略で標準入力

数値データを1行1つで受け付ける。

## 共通オプション

| オプション | 説明 | デフォルト |
|------------|------|------------|
| `-f, --format <FORMAT>` | 出力形式: text, csv, json, yaml, toml, xml | text |
| `-q, --quiet` | 最小出力（分布のみ） | - |
| `-v, --verbose` | 詳細出力 | - |
| `--filter <RANGE>` | 数値フィルタ | - |
| `-c, --min-count <N>` | 分析に必要な最小データ数 | 10 |
| `--no-color` | 色付け無効化 | - |

### フィルタ構文

- `>=100` - 100以上
- `<1000` - 1000未満
- `50-500` - 50以上500以下

## サブコマンド別オプション

### benf

| オプション | 説明 | デフォルト |
|------------|------|------------|
| `-t, --threshold <LEVEL>` | 異常検知閾値: low, medium, high, critical, auto | auto |
| `--confidence <LEVEL>` | 統計的信頼水準 (0.01-0.99) | 0.95 |
| `--sample-size <N>` | 大規模データ用サンプルサイズ | - |
| `--min-value <VALUE>` | 最小値フィルタ | - |

### pareto

| オプション | 説明 | デフォルト |
|------------|------|------------|
| `-C, --concentration <N>` | 集中度閾値 (0.0-1.0) | 0.8 |
| `--gini-coefficient` | ジニ係数を計算 | - |
| `--percentiles <LIST>` | カスタムパーセンタイル (例: 70,80,90) | - |
| `--business-analysis` | ビジネス分析インサイト | - |

### zipf

| オプション | 説明 | デフォルト |
|------------|------|------------|
| `-T, --text` | テキスト分析モード（単語頻度） | - |
| `-w, --words <N>` | 分析する最大単語数 | 1000 |

### normal

| オプション | 説明 | デフォルト |
|------------|------|------------|
| `-T, --test <METHOD>` | 正規性検定: shapiro, anderson, ks, all | all |
| `-O, --outliers` | 外れ値検出有効化 | - |
| `--outlier-method <METHOD>` | 外れ値検出方法: zscore, modified_zscore, iqr, lof, isolation, dbscan, ensemble | zscore |
| `-Q, --quality-control` | 品質管理分析有効化 | - |
| `--spec-limits <LOWER,UPPER>` | 規格限界 (例: 9.5,10.5) | - |
| `--enable-timeseries` | 時系列分析有効化 | - |
| `--timeseries-window <SIZE>` | 時系列ウィンドウサイズ | 10 |

### poisson

| オプション | 説明 | デフォルト |
|------------|------|------------|
| `-T, --test <METHOD>` | 適合度検定: chi_square, ks, variance, all | all |
| `-p, --predict` | 確率予測有効化 | - |
| `--max-events <N>` | 最大イベント数 | 20 |
| `-R, --rare-events` | 稀事象分析フォーカス | - |
| `--confidence <LEVEL>` | 信頼水準 | 0.95 |

### analyze / validate / diagnose

| オプション | 説明 | デフォルト |
|------------|------|------------|
| `-l, --laws <LAWS>` | 分析する法則 (例: benf,pareto,zipf) | 全法則 |
| `-F, --focus <FOCUS>` | フォーカス: quality, concentration, distribution, anomaly | - |
| `-t, --threshold <N>` | 異常検知閾値 (0.0-1.0) | 0.5 |
| `-r, --recommend` | 推奨モード有効化 | - |
| `--report <TYPE>` | レポートタイプ: summary, detailed, anomalies | summary |
| `--consistency-check` | 整合性チェック有効化 | - |
| `--cross-validation` | クロスバリデーション有効化 | - |
| `--confidence-level <LEVEL>` | 信頼水準 | 0.95 |
| `-p, --purpose <PURPOSE>` | 分析目的: quality, fraud, concentration, anomaly, distribution, general | - |

### generate <law>

| オプション | 説明 | デフォルト |
|------------|------|------------|
| `-s, --samples <N>` | 生成するサンプル数 | 1000 |
| `--seed <N>` | 乱数シード（再現性用） | - |
| `-o, --output-file <FILE>` | 出力ファイル | stdout |
| `--fraud-rate <RATE>` | 不正注入率 (0.0-1.0、benfのみ) | 0.0 |
| `--range <MIN,MAX>` | 数値範囲 (benfのみ) | 1,100000 |

**注**: generateコマンドは`--quiet`, `--verbose`, `--no-color`のみ受け付け、`--format`や`--filter`は受け付けない。出力は常に1行1数値のプレーンテキスト形式で、分析コマンドへのパイプを想定した設計。

## 終了コード

### 標準終了コード

| コード | 意味 |
|--------|------|
| 0 | 正常終了 (LOW または MEDIUM リスク) |
| 1 | 一般エラー |
| 2 | 引数エラー |
| 10 | HIGH リスク検出 (p ≤ 0.05) |
| 11 | CRITICAL リスク検出 (p ≤ 0.01) |

### 品質管理終了コード (normal --quality-control)

| コード | 意味 |
|--------|------|
| 0 | Excellent (優良) |
| 1 | Adequate (適正) |
| 2 | Poor (不良) |
| 3 | Inadequate (不適) |

## リスクレベル

| レベル | p値の範囲 | 終了コード | 説明 |
|--------|-----------|------------|------|
| LOW | p > 0.1 | 0 | 統計的に正常な分布 |
| MEDIUM | 0.05 < p ≤ 0.1 | 0 | 軽度の逸脱、要注意 |
| HIGH | 0.01 < p ≤ 0.05 | 10 | 有意な逸脱、詳細調査推奨 |
| CRITICAL | p ≤ 0.01 | 11 | 重大な逸脱、即時対応推奨 |

## 出力例

### benf (テキスト形式)

```
Benford Law Analysis Results

Dataset: stdin
Numbers analyzed: 1000
[LOW] Dataset analysis

First Digit Distribution:
1: ██████████████████████████████  30.1% (expected: 30.1%)
2: ██████████████████              17.8% (expected: 17.6%)
3: ████████████                    12.5% (expected: 12.5%)
...
```

### JSON出力 (-f json)

```json
{
  "chi_square": 5.23,
  "dataset": "stdin",
  "mean_absolute_deviation": 0.82,
  "numbers_analyzed": 1000,
  "p_value": 0.73,
  "risk_level": "Low"
}
```

### pareto出力

```
Pareto Principle (80/20 Rule) Analysis Results

Dataset: stdin
Numbers analyzed: 100

Lorenz Curve (Cumulative Distribution):
 10%: ██████████████████████████████████████████████████  50.0% cumulative
 20%: ████████████████████████████████████████████████████████████████  80.0% cumulative (80/20 point)
...

80/20 Rule: Top 20% owns 80.0% of total wealth
```

### normal --outliers 出力

```
Method: Z-Score
Outliers found: 3

Outlier Details:
  Index: 5 (Value: 100.000)
  Index: 23 (Value: -50.000)
  Index: 47 (Value: 150.000)
```

### normal --quality-control 出力

```
Quality Control Analysis
Mean: 9.990
Standard Deviation: 0.179
Cp: 0.930
Cpk: 0.911
Process Capability: Poor
Within Specification: 100.0%
```
