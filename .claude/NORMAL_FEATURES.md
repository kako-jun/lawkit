# NORMAL_FEATURES.md - 正規分布機能仕様

## 概要
正規分布（Normal Distribution）の分析機能仕様。統計学で最も重要な連続確率分布である正規分布への適合性を検証し、データの品質評価・異常値検出・工程管理に活用する包括的分析ツール。

## 核心機能

### 1. 正規分布分析 (`lawkit normal`)
**目的**: データが正規分布に従うかを多角的に検証し、統計的品質管理に必要な情報を提供

**入力形式**:
- 数値データ（連続値・離散値両対応）
- ファイル形式: CSV, JSON, Excel, テキスト
- 標準入力対応
- 国際数字対応（日本語数字、中国語金融数字等）

**出力メトリクス**:
- **分布パラメータ**: 平均・標準偏差・分散・歪度・尖度
- **正規性検定**: Shapiro-Wilk、Anderson-Darling、Kolmogorov-Smirnov
- **適合度評価**: Q-Q相関・正規性スコア・分布品質
- **信頼区間**: 平均の95%信頼区間・予測区間・3σ限界

### 2. 正規性検定機能
**3つの主要検定手法**:
- **Shapiro-Wilk検定**: 最も強力、小〜中規模データに最適
- **Anderson-Darling検定**: 裾部分に敏感、中程度の検出力
- **Kolmogorov-Smirnov検定**: 大規模データ対応、汎用性高

**検定モード**:
```bash
lawkit normal --test shapiro    # Shapiro-Wilk検定のみ
lawkit normal --test anderson   # Anderson-Darling検定のみ  
lawkit normal --test ks         # Kolmogorov-Smirnov検定のみ
lawkit normal --test all        # 全検定の統合結果
```

### 3. 異常値検出機能
**3つの検出手法**:
- **Z-score法**: 標準的手法、閾値2.5σ
- **修正Z-score法**: 中央値ベース、外れ値に頑健、閾値3.5
- **IQR法**: 四分位範囲、ノンパラメトリック

**異常値検出モード**:
```bash
lawkit normal --outliers                    # Z-scoreデフォルト
lawkit normal --outliers --outlier-method zscore    # Z-score法
lawkit normal --outliers --outlier-method modified # 修正Z-score法
lawkit normal --outliers --outlier-method iqr      # IQR法
```

### 4. 品質管理分析
**工程能力指数**:
- **Cp指数**: 工程のばらつき能力
- **Cpk指数**: 工程の中心とばらつき総合評価
- **工程能力評価**: Excellent(≥1.33) / Adequate(≥1.0) / Poor(≥0.67) / Inadequate(<0.67)

**管理図機能**:
- **3σ限界**: 管理限界線
- **Western Electric Rules**: 管理図違反パターン検出
- **連続点検出**: 7点連続傾向の検出

**品質管理モード**:
```bash
lawkit normal --quality-control                      # 基本QC分析
lawkit normal --quality-control --spec-limits 8,12   # 規格限界指定
```

### 5. 多言語対応
- **日本語**: 正規分布解析、正規性検定、品質管理
- **英語**: Normal Distribution Analysis, Normality Tests
- **中国語**: 正态分布分析、正态性检验
- **ヒンディー語**: सामान्य वितरण विश्लेषण
- **アラビア語**: تحليل التوزيع الطبيعي

## CLI仕様

### 基本コマンド
```bash
lawkit normal [入力データ] [オプション]
```

### 主要オプション
- `--format FORMAT`: 出力形式 (text, json, csv, yaml, toml, xml)
- `--quiet/-q`: 最小出力（パラメータのみ）
- `--verbose/-v`: 詳細分析・解釈付き
- `--lang/-l LANGUAGE`: 出力言語 (en, ja, zh, hi, ar, auto)
- `--filter RANGE`: 数値範囲フィルタ
- `--min-count NUMBER`: 最小データ点数（デフォルト8）

### 専用モードオプション
- `--test TYPE`: 正規性検定モード (shapiro, anderson, ks, all)
- `--outliers`: 異常値検出モード
- `--outlier-method METHOD`: 異常値検出手法 (zscore, modified, iqr)
- `--quality-control`: 品質管理分析モード
- `--spec-limits LOWER,UPPER`: 規格限界 (例: 8.0,12.0)

### 使用例

#### 基本分析
```bash
# 基本正規分布分析
echo "10.1 9.8 10.2 9.9 10.0 10.3 9.7 10.1 9.9 10.0" | lawkit normal

# 詳細分析（日本語）
lawkit normal quality_data.csv --verbose --lang ja

# CSVファイルの分析
lawkit normal measurements.csv --format json
```

#### 正規性検定
```bash
# Shapiro-Wilk検定
lawkit normal data.csv --test shapiro

# 全検定の実行
echo "1 2 3 4 5" | lawkit normal --test all --lang en

# 特定検定のJSON出力
lawkit normal sample.txt --test anderson --format json
```

#### 異常値検出
```bash
# Z-score法による異常値検出
lawkit normal dataset.csv --outliers

# 修正Z-score法
echo "1 2 3 4 5 100" | lawkit normal --outliers --outlier-method modified

# IQR法での検出
lawkit normal data.txt --outliers --outlier-method iqr --verbose
```

#### 品質管理
```bash
# 工程能力分析
lawkit normal production_data.csv --quality-control --spec-limits 9.5,10.5

# 管理図分析
echo "10.1 9.8 10.2" | lawkit normal --quality-control --verbose

# QC結果のCSV出力
lawkit normal qc_data.csv --quality-control --format csv
```

## 出力形式

### テキスト出力（デフォルト）
```
正規分布解析結果

データセット: quality_data.csv
解析した数値数: 50
品質レベル: Low

分布パラメータ:
  平均: 10.05
  標準偏差: 0.12
  分散: 0.014
  歪度: -0.15
  尖度: 0.32

正規性検定:
  Shapiro-Wilk: W=0.987, p=0.856
  Anderson-Darling: A²=0.234, p=0.654
  Kolmogorov-Smirnov: D=0.067, p=0.743

適合度評価:
  正規性スコア: 0.875
  Q-Q相関: 0.994
  分布品質: 0.903

σ範囲カバー率:
  1σ: 68.0%
  2σ: 95.0%
  3σ: 99.0%

解釈:
✅ 優れた正規分布適合
   データは正規分布に良く従っています
```

### JSON出力
```json
{
  "dataset": "quality_data.csv",
  "numbers_analyzed": 50,
  "risk_level": "Low",
  "mean": 10.05,
  "std_dev": 0.12,
  "variance": 0.014,
  "skewness": -0.15,
  "kurtosis": 0.32,
  "shapiro_wilk": {
    "statistic": 0.987,
    "p_value": 0.856
  },
  "anderson_darling": {
    "statistic": 0.234,
    "p_value": 0.654
  },
  "kolmogorov_smirnov": {
    "statistic": 0.067,
    "p_value": 0.743
  },
  "normality_score": 0.875,
  "qq_correlation": 0.994,
  "distribution_quality": 0.903,
  "outliers": {
    "z_score_count": 0,
    "modified_z_count": 0,
    "iqr_count": 0
  },
  "confidence_intervals": {
    "mean_95": [10.01, 10.09],
    "prediction_95": [9.81, 10.29],
    "three_sigma": [9.69, 10.41]
  },
  "sigma_coverage": {
    "within_1_sigma": 68.0,
    "within_2_sigma": 95.0,
    "within_3_sigma": 99.0
  }
}
```

### CSV出力
```csv
dataset,numbers_analyzed,risk_level,mean,std_dev,variance,skewness,kurtosis,normality_score
quality_data.csv,50,Low,10.050,0.120,0.014,-0.150,0.320,0.875
```

## 品質評価基準

### Quality Level（品質レベル）
- **Low (優秀)**: 正規性スコア > 0.8、外れ値 < 5%、歪度・尖度が小
- **Medium (良好)**: 正規性スコア > 0.6、外れ値 < 10%、軽微な偏差
- **High (要注意)**: 正規性スコア > 0.3、外れ値 < 20%、有意な偏差  
- **Critical (問題)**: 正規性スコア ≤ 0.3、多数の外れ値、重大な偏差

### 正規性検定判定基準
- **p値 > 0.05**: 正規分布に従う（帰無仮説採択）
- **p値 ≤ 0.05**: 正規分布に従わない（帰無仮説棄却）
- **統合判定**: 複数検定の総合評価による最終判断

### 工程能力評価
- **Excellent (Cpk ≥ 1.33)**: 優秀な工程、シックスシグマレベル
- **Adequate (1.0 ≤ Cpk < 1.33)**: 適切な工程、改善余地あり
- **Poor (0.67 ≤ Cpk < 1.0)**: 不十分な工程、改善必要
- **Inadequate (Cpk < 0.67)**: 不適切な工程、緊急改善要

## 応用分野

### 製造業・品質管理
- **工程管理**: 製造ばらつきの評価・管理図作成
- **品質保証**: 製品仕様への適合性評価
- **検査データ**: 測定値の正規性確認
- **Six Sigma**: データドリブン品質改善

### 金融・リスク管理
- **リターン分析**: 投資収益率の分布分析
- **VaR計算**: リスク値算定の前提確認
- **信用リスク**: デフォルト率の分布モデリング
- **市場リスク**: 価格変動の正規性検証

### 医学・生物統計
- **臨床試験**: 測定値の分布確認
- **疫学研究**: 生体指標の正規性検定
- **薬効評価**: 治療効果の統計的検定
- **バイオマーカー**: 正常値範囲の設定

### 心理学・教育
- **テスト得点**: 試験結果の分布分析
- **尺度開発**: 心理尺度の妥当性検証
- **アンケート**: 評定値の分布特性確認
- **能力測定**: 認知能力の正規性評価

### IT・データサイエンス
- **A/Bテスト**: 実験結果の前提確認
- **機械学習**: 特徴量の正規化判定
- **ログ分析**: システム性能の分布評価
- **異常検知**: ベースライン分布の確立

## 技術的制約

### データ要件
- **最小データ点数**: 8個（正規性検定の最低要件）
- **推奨サイズ**: 30-10000データ点（統計的検出力確保）
- **データ型**: 連続値推奨、離散値も対応

### 検定手法の適用条件
- **Shapiro-Wilk**: n ≤ 5000、最も強力だが計算負荷大
- **Anderson-Darling**: 中程度のサンプルサイズに適用
- **Kolmogorov-Smirnov**: 大サンプル対応、保守的

### 性能特性
- **処理速度**: O(n log n) - ソート処理が支配的
- **メモリ使用量**: O(n) - データサイズに比例
- **精度**: 浮動小数点64bit、数値計算安定性確保

### 制限事項
- 極端に小さなサンプル（n < 8）は分析不可
- 同一値のみのデータは標準偏差0でエラー
- 欠損値は事前除去が必要
- 極端な外れ値は検定結果に影響

## アルゴリズム詳細

### 正規性検定アルゴリズム
```
Shapiro-Wilk:
W = (Σ(ai * x(i)))² / Σ(xi - x̄)²
where ai: 順序統計量の係数

Anderson-Darling:  
A² = -n - (1/n)Σ(2i-1)[ln(F(x(i))) + ln(1-F(x(n+1-i)))]
where F: 標準正規分布の累積分布関数

Kolmogorov-Smirnov:
D = max|F̂n(x) - F0(x)|
where F̂n: 経験分布関数, F0: 理論分布関数
```

### Q-Q相関計算
```
1. 観測値を昇順ソート
2. 理論分位点計算: Φ⁻¹((i-0.5)/n)
3. ピアソン相関係数計算
4. 相関値が1に近いほど正規分布に適合
```

### 異常値検出アルゴリズム
```
Z-score法:
z = (x - μ) / σ
|z| > 2.5 で異常値判定

修正Z-score法:
Modified Z = 0.6745 * (x - median) / MAD
|Modified Z| > 3.5 で異常値判定

IQR法:
Lower bound = Q1 - 1.5 * IQR
Upper bound = Q3 + 1.5 * IQR
範囲外を異常値判定
```

## 統合機能

### 他法則との比較
- **ベンフォード法則**: データ品質 vs 分布適合性
- **パレート法則**: 集中度 vs 正規性
- **Zipf法則**: べき乗分布 vs 正規分布
- **将来統合**: 分布型自動判定・最適法則推奨

### 出力連携
- 全形式で統一されたスキーマ
- 統計ソフトウェア（R/Python/SAS）連携
- BIツール（Tableau/Power BI）対応

### 将来拡張
- 他の分布との適合度比較（対数正規・指数・ガンマ分布）
- ベイズ統計への拡張
- 多変量正規性検定
- リアルタイム品質監視