# INTEGRATION_FEATURES.md - 統合機能仕様

## 概要

lawkit統合機能は、複数の統計法則を組み合わせて包括的なデータ分析を提供する機能群。個別の法則分析では発見できない複合的なパターンや矛盾点を検出し、データの品質・信頼性・特性を多角的に評価する。

## 核心機能

### 1. 複数法則比較分析 (`lawkit compare`)
**目的**: 同一データに対して全5法則を実行し、統合的な品質評価を提供

**実行法則**:
- **ベンフォード法則**: 自然性・品質評価
- **パレート法則**: 集中度・80/20分析
- **Zipf法則**: 頻度分布・ランキング特性
- **正規分布**: 正規性・異常値検出
- **ポアソン分布**: イベント発生・稀少事象

**統合評価メトリクス**:
- **総合品質スコア**: 全法則の評価を統合した信頼性指標
- **一貫性スコア**: 法則間の評価の一致度
- **矛盾検出**: 法則間で評価が大きく異なる場合の警告
- **推奨度**: 最適な法則の提案

### 2. 法則間矛盾検出機能
**矛盾パターン例**:
- ベンフォード法則: 高品質 ↔ 正規分布: 異常値多数
- パレート法則: 高集中 ↔ Zipf法則: 均等分布
- 正規分布: 正規性あり ↔ ポアソン分布: 離散性強

**検出アルゴリズム**:
```
矛盾度 = |法則A評価 - 法則B評価| / max(法則A評価, 法則B評価)
矛盾度 > 0.5 → 要注意
矛盾度 > 0.8 → 警告発出
```

### 3. 最適法則推奨システム
**推奨ロジック**:
1. **データ特性分析**: 連続/離散、範囲、分布形状
2. **適合度評価**: 各法則の適合度スコア比較
3. **用途別推奨**: 目的に応じた最適法則提案
4. **複数法則併用**: 補完的な法則組み合わせ

**推奨カテゴリ**:
- **品質監査**: ベンフォード + 正規分布
- **集中度分析**: パレート + Zipf
- **異常検知**: 正規分布 + ポアソン分布
- **総合評価**: 全法則統合

## CLI仕様

### 基本コマンド
```bash
lawkit compare [入力データ] [オプション]
```

### 主要オプション
- `--laws LAWS`: 比較対象法則指定 (benf,pareto,zipf,normal,poisson)
- `--focus FOCUS`: 重点分析項目 (quality,concentration,distribution,anomaly)
- `--threshold THRESHOLD`: 矛盾検出閾値 (0.0-1.0, デフォルト0.5)
- `--recommend`: 最適法則推奨モード
- `--report TYPE`: 統合レポート生成 (summary,detailed,conflicting)

### 統合固有オプション
- `--consistency-check`: 一貫性チェック実行
- `--cross-validation`: 相互検証分析
- `--confidence-level LEVEL`: 信頼度水準 (0.90,0.95,0.99)
- `--weight-scheme SCHEME`: 重み付けスキーム (equal,adaptive,expert)

## 使用例

### 基本統合分析
```bash
# 全法則による統合分析
lawkit compare financial_data.csv

# 特定法則組み合わせ
lawkit compare data.csv --laws benf,pareto,normal

# 品質重視の分析
lawkit compare transactions.csv --focus quality --recommend
```

### 矛盾検出分析
```bash
# 矛盾検出
lawkit compare suspicious_data.csv --consistency-check

# 詳細な矛盾分析
lawkit compare data.csv --report conflicting --threshold 0.3

# 相互検証
lawkit compare dataset.csv --cross-validation --confidence-level 0.95
```

### 推奨システム
```bash
# 最適法則推奨
echo "1 10 100 1000 10000" | lawkit compare --recommend

# 用途別推奨
lawkit compare sales_data.csv --focus concentration --recommend

# 統合レポート
lawkit compare audit_data.csv --report detailed --format json
```

## 出力形式

### 統合評価結果（テキスト）
```
統合分析結果

データセット: financial_data.csv
解析した数値数: 1000
実行法則: 5個 (benf, pareto, zipf, normal, poisson)

統合評価:
  総合品質スコア: 0.742
  一貫性スコア: 0.856
  矛盾検出: 1件
  推奨度: 高

法則別結果:
  ベンフォード法則: 品質=高 (0.834)
  パレート法則: 集中=中 (0.623)
  Zipf法則: 適合=良 (0.765)
  正規分布: 正規性=低 (0.234) ⚠️
  ポアソン分布: 適合=高 (0.891)

矛盾検出:
  ⚠️ 正規分布 vs 他法則: 評価が大きく異なります
     推定原因: 離散データに正規分布を適用
     推奨: ポアソン分布を主要法則として使用

推奨:
  🎯 主要法則: ポアソン分布 (イベント発生データ)
  🔍 補助法則: ベンフォード法則 (品質監査)
  📊 総合評価: 高品質、一部異常値あり
```

### JSON統合出力
```json
{
  "dataset": "financial_data.csv",
  "numbers_analyzed": 1000,
  "laws_executed": ["benf", "pareto", "zipf", "normal", "poisson"],
  "integration_metrics": {
    "overall_quality_score": 0.742,
    "consistency_score": 0.856,
    "conflicts_detected": 1,
    "recommendation_confidence": 0.824
  },
  "law_results": {
    "benf": {
      "quality_score": 0.834,
      "risk_level": "Low",
      "primary_metric": "first_digit_chi_square"
    },
    "pareto": {
      "concentration_score": 0.623,
      "risk_level": "Medium", 
      "primary_metric": "gini_coefficient"
    },
    "zipf": {
      "distribution_score": 0.765,
      "risk_level": "Low",
      "primary_metric": "zipf_coefficient"
    },
    "normal": {
      "normality_score": 0.234,
      "risk_level": "High",
      "primary_metric": "shapiro_wilk_p_value"
    },
    "poisson": {
      "fit_score": 0.891,
      "risk_level": "Low",
      "primary_metric": "variance_mean_ratio"
    }
  },
  "conflicts": [
    {
      "conflict_type": "distribution_mismatch",
      "laws_involved": ["normal", "poisson"],
      "conflict_score": 0.657,
      "description": "正規分布の評価が他法則と大きく異なる",
      "likely_cause": "離散データに連続分布を適用"
    }
  ],
  "recommendations": {
    "primary_law": "poisson",
    "secondary_laws": ["benf"],
    "confidence": 0.824,
    "rationale": "イベント発生データに最適、品質監査も推奨"
  }
}
```

## 統合アルゴリズム

### 1. 総合品質スコア計算
```
総合品質スコア = Σ(法則i品質スコア × 重みi) / Σ(重みi)

重み付けスキーム:
- equal: 全法則同等 (重み=1.0)
- adaptive: データ特性に応じて自動調整
- expert: 専門知識ベースの重み
```

### 2. 一貫性スコア計算
```
一貫性スコア = 1 - (評価分散 / 最大可能分散)

評価分散 = Σ(法則i評価 - 平均評価)² / 法則数
最大可能分散 = 1.0 (全法則が正反対の評価)
```

### 3. 矛盾検出アルゴリズム
```rust
fn detect_conflicts(results: &[LawResult]) -> Vec<Conflict> {
    let mut conflicts = Vec::new();
    
    for i in 0..results.len() {
        for j in i+1..results.len() {
            let score_diff = (results[i].score - results[j].score).abs();
            let max_score = results[i].score.max(results[j].score);
            
            if max_score > 0.0 {
                let conflict_ratio = score_diff / max_score;
                
                if conflict_ratio > CONFLICT_THRESHOLD {
                    conflicts.push(Conflict {
                        law_a: results[i].law_name.clone(),
                        law_b: results[j].law_name.clone(),
                        conflict_score: conflict_ratio,
                        likely_cause: diagnose_conflict_cause(&results[i], &results[j]),
                    });
                }
            }
        }
    }
    
    conflicts
}
```

### 4. 推奨システムロジック
```rust
fn recommend_optimal_laws(results: &[LawResult], data_characteristics: &DataCharacteristics) -> Recommendation {
    let mut scored_laws = Vec::new();
    
    for result in results {
        let base_score = result.quality_score;
        let compatibility_bonus = calculate_compatibility_bonus(result, data_characteristics);
        let purpose_bonus = calculate_purpose_bonus(result, data_characteristics.analysis_purpose);
        
        let total_score = base_score + compatibility_bonus + purpose_bonus;
        
        scored_laws.push(ScoredLaw {
            law_name: result.law_name.clone(),
            score: total_score,
            rationale: generate_rationale(result, data_characteristics),
        });
    }
    
    scored_laws.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    
    Recommendation {
        primary_law: scored_laws[0].law_name.clone(),
        secondary_laws: scored_laws.iter().skip(1).take(2).map(|s| s.law_name.clone()).collect(),
        confidence: calculate_confidence(&scored_laws),
        rationale: combine_rationales(&scored_laws),
    }
}
```

## データ特性分析

### 自動特性検出
```rust
pub struct DataCharacteristics {
    pub data_type: DataType,           // 連続/離散
    pub distribution_shape: DistributionShape,  // 正規/歪み/多峰性
    pub outlier_presence: OutlierLevel,         // 外れ値の程度
    pub scale_range: ScaleRange,                // 値の範囲
    pub seasonality: Option<SeasonalityPattern>, // 季節性パターン
    pub analysis_purpose: AnalysisPurpose,      // 分析目的
}

pub enum DataType {
    Continuous,     // 連続データ
    Discrete,       // 離散データ
    Mixed,          // 混合データ
}

pub enum DistributionShape {
    Normal,         // 正規分布様
    Skewed,         // 歪み分布
    Multimodal,     // 多峰性
    PowerLaw,       // べき乗分布
    Exponential,    // 指数分布
}

pub enum AnalysisPurpose {
    QualityAudit,   // 品質監査
    FraudDetection, // 不正検知
    ConcentrationAnalysis, // 集中度分析
    AnomalyDetection,      // 異常検知
    GeneralAnalysis,       // 一般分析
}
```

### 特性別推奨マトリクス
```
データ特性 → 推奨法則:
- 連続 + 正規分布様 → 正規分布 (主), ベンフォード (副)
- 離散 + 稀少事象 → ポアソン分布 (主), ベンフォード (副)
- 連続 + 歪み分布 → パレート法則 (主), Zipf法則 (副)
- 順序データ → Zipf法則 (主), パレート法則 (副)
- 品質監査目的 → ベンフォード法則 (主), 正規分布 (副)
```

## 重み付けスキーム

### 適応的重み付け
```rust
fn calculate_adaptive_weights(data_characteristics: &DataCharacteristics) -> HashMap<String, f64> {
    let mut weights = HashMap::new();
    
    // ベースライン重み
    weights.insert("benf".to_string(), 1.0);
    weights.insert("pareto".to_string(), 1.0);
    weights.insert("zipf".to_string(), 1.0);
    weights.insert("normal".to_string(), 1.0);
    weights.insert("poisson".to_string(), 1.0);
    
    // データ特性に応じた調整
    match data_characteristics.data_type {
        DataType::Continuous => {
            *weights.get_mut("normal").unwrap() *= 1.5;
            *weights.get_mut("poisson").unwrap() *= 0.5;
        },
        DataType::Discrete => {
            *weights.get_mut("poisson").unwrap() *= 1.5;
            *weights.get_mut("normal").unwrap() *= 0.5;
        },
        DataType::Mixed => {
            // 調整なし
        }
    }
    
    match data_characteristics.analysis_purpose {
        AnalysisPurpose::QualityAudit => {
            *weights.get_mut("benf").unwrap() *= 2.0;
        },
        AnalysisPurpose::ConcentrationAnalysis => {
            *weights.get_mut("pareto").unwrap() *= 2.0;
            *weights.get_mut("zipf").unwrap() *= 1.5;
        },
        _ => {}
    }
    
    weights
}
```

### 専門家重み付け
```rust
fn expert_weights(domain: &str) -> HashMap<String, f64> {
    match domain {
        "finance" => hashmap! {
            "benf" => 2.0,      // 財務監査重視
            "pareto" => 1.5,    // 集中度重要
            "zipf" => 1.0,
            "normal" => 1.2,
            "poisson" => 0.8,
        },
        "web_analytics" => hashmap! {
            "benf" => 1.0,
            "pareto" => 1.5,
            "zipf" => 2.0,      // 頻度分析重要
            "normal" => 0.8,
            "poisson" => 1.8,   // イベント発生重要
        },
        "quality_control" => hashmap! {
            "benf" => 1.5,
            "pareto" => 1.0,
            "zipf" => 0.8,
            "normal" => 2.0,    // 品質管理重要
            "poisson" => 1.0,
        },
        _ => equal_weights(),
    }
}
```

## 統合レポート生成

### サマリーレポート
- 各法則の評価を1行で要約
- 矛盾点の簡潔な指摘
- 推奨法則の提示
- 総合的な信頼度評価

### 詳細レポート
- 各法則の詳細分析結果
- 法則間の相関分析
- 統計的有意性検定
- 詳細な推奨理由

### 矛盾レポート
- 矛盾の詳細分析
- 推定原因の説明
- 対処法の提案
- 追加分析の推奨

## 性能最適化

### 並列実行
```rust
use rayon::prelude::*;

fn parallel_law_analysis(numbers: &[f64], dataset_name: &str) -> Vec<LawResult> {
    let laws = vec!["benf", "pareto", "zipf", "normal", "poisson"];
    
    laws.par_iter()
        .map(|&law| analyze_law(law, numbers, dataset_name))
        .collect()
}
```

### メモリ効率化
```rust
// 共通計算の再利用
pub struct CommonCalculations {
    pub sorted_numbers: Vec<f64>,
    pub basic_stats: BasicStats,
    pub percentiles: Vec<f64>,
}

impl CommonCalculations {
    pub fn new(numbers: &[f64]) -> Self {
        let mut sorted = numbers.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        Self {
            sorted_numbers: sorted.clone(),
            basic_stats: BasicStats::calculate(&sorted),
            percentiles: calculate_percentiles(&sorted),
        }
    }
}
```

## 拡張性

### 新法則統合
```rust
pub trait IntegrableLaw {
    fn integration_metadata(&self) -> IntegrationMetadata;
    fn compatibility_score(&self, other: &dyn IntegrableLaw) -> f64;
    fn conflict_detection_rules(&self) -> Vec<ConflictRule>;
}

pub struct IntegrationMetadata {
    pub law_name: String,
    pub data_type_compatibility: Vec<DataType>,
    pub optimal_data_ranges: Vec<(f64, f64)>,
    pub primary_metrics: Vec<String>,
}
```

### カスタム統合ルール
```rust
pub struct CustomIntegrationRule {
    pub rule_name: String,
    pub applicable_laws: Vec<String>,
    pub weight_modifier: fn(&DataCharacteristics) -> f64,
    pub conflict_threshold: f64,
}
```

この統合機能により、lawkitは単一法則分析から**多次元統合分析プラットフォーム**へと進化し、より精密で実用的なデータ分析を提供する。