use crate::common::risk::RiskLevel;
use crate::laws::benford::BenfordResult;
use crate::laws::normal::NormalResult;
use crate::laws::pareto::ParetoResult;
use crate::laws::poisson::PoissonResult;
use crate::laws::zipf::ZipfResult;
use diffx_core::{diff, DiffResult};
use std::collections::HashMap;

/// 法則名を短縮形からフルネーム（アポストロフィなし）に変換
fn get_law_display_name(law: &str) -> &str {
    match law {
        "benf" => "Benford Law",
        "pareto" => "Pareto Principle",
        "zipf" => "Zipf Law",
        "normal" => "Normal Distribution",
        "poisson" => "Poisson Distribution",
        _ => law,
    }
}

/// 統合分析結果
#[derive(Debug, Clone)]
pub struct IntegrationResult {
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub laws_executed: Vec<String>,

    // 統合評価メトリクス
    pub overall_quality_score: f64,     // 総合品質スコア (0-1)
    pub consistency_score: f64,         // 一貫性スコア (0-1)
    pub conflicts_detected: usize,      // 矛盾検出数
    pub recommendation_confidence: f64, // 推奨信頼度 (0-1)

    // 個別法則結果
    pub benford_result: Option<BenfordResult>,
    pub pareto_result: Option<ParetoResult>,
    pub zipf_result: Option<ZipfResult>,
    pub normal_result: Option<NormalResult>,
    pub poisson_result: Option<PoissonResult>,

    // 統合分析
    pub law_scores: HashMap<String, f64>, // 法則別スコア
    pub conflicts: Vec<Conflict>,         // 検出された矛盾
    pub recommendations: Recommendation,  // 推奨法則
    pub data_characteristics: DataCharacteristics, // データ特性

    // 統合評価
    pub overall_assessment: OverallAssessment,
    pub risk_level: RiskLevel,

    // 分析フォーカス
    pub focus: Option<String>, // 分析フォーカス (quality, concentration, etc.)
}

/// 法則間矛盾
#[derive(Debug, Clone)]
pub struct Conflict {
    pub conflict_type: ConflictType,
    pub laws_involved: Vec<String>,
    pub conflict_score: f64, // 矛盾の強さ (0-1)
    pub description: String,
    pub likely_cause: String,
    pub resolution_suggestion: String,
}

/// 矛盾タイプ
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConflictType {
    DistributionMismatch,   // 分布適合性の不一致
    QualityDisagreement,    // 品質評価の不一致
    RiskLevelConflict,      // リスクレベルの不一致
    ScaleIncompatibility,   // スケール不適合
    MethodologicalConflict, // 手法論的矛盾
    ScoreDeviation,         // スコア乖離（diffx-core検出）
    UnexpectedConsistency,  // 異常な一致（diffx-core検出）
}

/// 推奨システム結果
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub primary_law: String,                           // 主要推奨法則
    pub secondary_laws: Vec<String>,                   // 補助推奨法則
    pub confidence: f64,                               // 推奨信頼度
    pub rationale: String,                             // 推奨理由
    pub alternative_combinations: Vec<LawCombination>, // 代替組み合わせ
}

/// 法則組み合わせ
#[derive(Debug, Clone)]
pub struct LawCombination {
    pub laws: Vec<String>,
    pub purpose: String,
    pub effectiveness_score: f64,
    pub description: String,
}

/// データ特性
#[derive(Debug, Clone)]
pub struct DataCharacteristics {
    pub data_type: DataType,
    pub distribution_shape: DistributionShape,
    pub outlier_presence: OutlierLevel,
    pub scale_range: ScaleRange,
    pub analysis_purpose: AnalysisPurpose,
    pub sample_size_category: SampleSizeCategory,
}

/// データタイプ
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Continuous, // 連続データ
    Discrete,   // 離散データ
    Mixed,      // 混合データ
    Integer,    // 整数データ
    Unknown,    // 不明
}

/// 分布形状
#[derive(Debug, Clone, PartialEq)]
pub enum DistributionShape {
    Normal,      // 正規分布様
    Skewed,      // 歪み分布
    Multimodal,  // 多峰性
    PowerLaw,    // べき乗分布
    Exponential, // 指数分布
    Uniform,     // 一様分布
    Unknown,     // 不明
}

/// 外れ値レベル
#[derive(Debug, Clone, PartialEq)]
pub enum OutlierLevel {
    None,     // 外れ値なし
    Low,      // 軽微
    Moderate, // 中程度
    High,     // 高
    Extreme,  // 極端
}

/// スケール範囲
#[derive(Debug, Clone, PartialEq)]
pub enum ScaleRange {
    Narrow, // 狭い範囲 (1-2桁)
    Medium, // 中程度 (3-4桁)
    Wide,   // 広い範囲 (5桁以上)
    Mixed,  // 混合スケール
}

/// 分析目的
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisPurpose {
    QualityAudit,          // 品質監査
    FraudDetection,        // 不正検知
    ConcentrationAnalysis, // 集中度分析
    AnomalyDetection,      // 異常検知
    DistributionFitting,   // 分布適合
    GeneralAnalysis,       // 一般分析
}

/// サンプルサイズカテゴリ
#[derive(Debug, Clone, PartialEq)]
pub enum SampleSizeCategory {
    Small,     // < 30
    Medium,    // 30-300
    Large,     // 300-3000
    VeryLarge, // > 3000
}

/// 総合評価
#[derive(Debug, Clone, PartialEq)]
pub enum OverallAssessment {
    Excellent,   // 優秀 - 全法則で一貫して高評価
    Good,        // 良好 - 大部分の法則で良評価
    Mixed,       // 混合 - 法則間で評価が分かれる
    Concerning,  // 懸念 - 複数法則で問題検出
    Problematic, // 問題 - 重大な矛盾や異常
}

impl IntegrationResult {
    pub fn new(dataset_name: String, numbers: &[f64]) -> Self {
        Self {
            dataset_name,
            numbers_analyzed: numbers.len(),
            laws_executed: Vec::new(),
            overall_quality_score: 0.0,
            consistency_score: 0.0,
            conflicts_detected: 0,
            recommendation_confidence: 0.0,
            benford_result: None,
            pareto_result: None,
            zipf_result: None,
            normal_result: None,
            poisson_result: None,
            law_scores: HashMap::new(),
            conflicts: Vec::new(),
            recommendations: Recommendation::empty(),
            data_characteristics: DataCharacteristics::analyze(numbers),
            overall_assessment: OverallAssessment::Mixed,
            risk_level: RiskLevel::Medium,
            focus: None,
        }
    }

    /// 法則結果を追加
    pub fn add_law_result(&mut self, law_name: &str, result: LawResult) {
        match law_name {
            "benf" => {
                if let LawResult::Benford(r) = result {
                    // r.conformity_score の代わりに、適切なスコアを使用
                    // 例えば、MAD (Mean Absolute Deviation) を逆転させたものや、p_value を使用
                    // ここでは MAD を使用し、値が小さいほど良いので 1.0 - MAD/100.0 のように変換
                    let score = 1.0 - (r.mean_absolute_deviation / 100.0);
                    self.law_scores.insert("benf".to_string(), score);
                    self.benford_result = Some(r);
                }
            }
            "pareto" => {
                if let LawResult::Pareto(r) = result {
                    self.law_scores
                        .insert("pareto".to_string(), r.concentration_index);
                    self.pareto_result = Some(r);
                }
            }
            "zipf" => {
                if let LawResult::Zipf(r) = result {
                    self.law_scores
                        .insert("zipf".to_string(), r.distribution_quality);
                    self.zipf_result = Some(r);
                }
            }
            "normal" => {
                if let LawResult::Normal(r) = result {
                    self.law_scores
                        .insert("normal".to_string(), r.normality_score);
                    self.normal_result = Some(r);
                }
            }
            "poisson" => {
                if let LawResult::Poisson(r) = result {
                    self.law_scores
                        .insert("poisson".to_string(), r.goodness_of_fit_score);
                    self.poisson_result = Some(r);
                }
            }
            _ => {}
        }

        if !self.laws_executed.contains(&law_name.to_string()) {
            self.laws_executed.push(law_name.to_string());
        }
    }

    /// 統合分析実行
    pub fn finalize_analysis(&mut self) {
        self.calculate_overall_quality_score();
        self.calculate_consistency_score();
        self.detect_conflicts();
        self.generate_recommendations();
        self.assess_overall_quality();
        self.determine_risk_level();
    }

    fn calculate_overall_quality_score(&mut self) {
        if self.law_scores.is_empty() {
            self.overall_quality_score = 0.0;
            return;
        }

        let weights = self.get_adaptive_weights();
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for (law, score) in &self.law_scores {
            if let Some(&weight) = weights.get(law) {
                weighted_sum += score * weight;
                total_weight += weight;
            }
        }

        self.overall_quality_score = if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        };
    }

    fn calculate_consistency_score(&mut self) {
        if self.law_scores.len() < 2 {
            self.consistency_score = 1.0;
            return;
        }

        let scores: Vec<f64> = self.law_scores.values().cloned().collect();
        let mean_score: f64 = scores.iter().sum::<f64>() / scores.len() as f64;

        let variance: f64 = scores
            .iter()
            .map(|score| (score - mean_score).powi(2))
            .sum::<f64>()
            / scores.len() as f64;

        // 最大可能分散は1.0（全法則が正反対の評価）
        let max_variance = 1.0;
        self.consistency_score = 1.0 - (variance / max_variance).min(1.0);
    }

    fn detect_conflicts(&mut self) {
        self.conflicts.clear();

        // diffx-coreを使用してより詳細な矛盾分析を実行
        self.detect_conflicts_with_diffx();

        // 従来の手法も併用（スコア差分の詳細分析）
        self.detect_score_conflicts();

        self.conflicts_detected = self.conflicts.len();
    }

    /// diffx-coreを使用した構造的矛盾検出
    fn detect_conflicts_with_diffx(&mut self) {
        if self.law_scores.is_empty() {
            return;
        }

        // 期待されるスコア分布（平均値ベース）と実際のスコア分布を比較
        let average_score: f64 =
            self.law_scores.values().sum::<f64>() / self.law_scores.len() as f64;
        let mut expected_scores = HashMap::new();

        for law in self.law_scores.keys() {
            expected_scores.insert(law.clone(), average_score);
        }

        // JSONに変換してdiffx-coreで比較
        let expected_json = serde_json::to_value(&expected_scores).unwrap_or_default();
        let actual_json = serde_json::to_value(&self.law_scores).unwrap_or_default();

        // diffx-coreで構造的差分を分析
        let diff_results = diff(&expected_json, &actual_json, None, Some(0.01), None);

        if diff_results.is_empty() {
            // 全てのスコアが期待値と一致（疑わしい一致）
            if self.law_scores.len() > 1 {
                let conflict = Conflict {
                    conflict_type: ConflictType::UnexpectedConsistency,
                    laws_involved: self.law_scores.keys().cloned().collect(),
                    conflict_score: 0.6,
                    description:
                        "All statistical laws show identical scores, indicating potential data or analysis issues"
                            .to_string(),
                    likely_cause: "Insufficient data diversity or analysis algorithm issues".to_string(),
                    resolution_suggestion: "Please review data quality and analysis methods".to_string(),
                };
                self.conflicts.push(conflict);
            }
        } else {
            // 差分が検出された場合の詳細分析
            for diff_result in &diff_results {
                match diff_result {
                    DiffResult::Modified(path, expected_val, actual_val) => {
                        if let (Some(expected), Some(actual)) =
                            (expected_val.as_f64(), actual_val.as_f64())
                        {
                            let deviation = (actual - expected).abs() / expected.max(0.01);

                            if deviation > 0.3 {
                                // 30%以上の偏差を異常とする
                                let law_name = path.trim_start_matches('"').trim_end_matches('"');
                                let conflict = Conflict {
                                    conflict_type: ConflictType::ScoreDeviation,
                                    laws_involved: vec![law_name.to_string()],
                                    conflict_score: deviation.min(1.0),
                                    description: format!(
                                        "{} score {:.3} significantly deviates from expected {:.3} - deviation {:.1}%",
                                        get_law_display_name(law_name), actual, expected, deviation * 100.0
                                    ),
                                    likely_cause: format!(
                                        "{} may not be compatible with the data pattern",
                                        get_law_display_name(law_name)
                                    ),
                                    resolution_suggestion: format!(
                                        "Please review application conditions and data quality for {}",
                                        get_law_display_name(law_name)
                                    ),
                                };
                                self.conflicts.push(conflict);
                            }
                        }
                    }
                    DiffResult::Added(path, _val) | DiffResult::Removed(path, _val) => {
                        // 予期しない法則の追加・削除
                        let law_name = path.trim_start_matches('"').trim_end_matches('"');
                        let conflict = Conflict {
                            conflict_type: ConflictType::MethodologicalConflict,
                            laws_involved: vec![law_name.to_string()],
                            conflict_score: 0.5,
                            description: format!(
                                "Unexpected change detected for {}",
                                get_law_display_name(law_name)
                            ),
                            likely_cause: "Analysis configuration or law selection inconsistency"
                                .to_string(),
                            resolution_suggestion: "Please verify the analysis target law settings"
                                .to_string(),
                        };
                        self.conflicts.push(conflict);
                    }
                    DiffResult::TypeChanged(path, _old, _new) => {
                        // スコアの型変更（通常は発生しないはず）
                        let law_name = path.trim_start_matches('"').trim_end_matches('"');
                        let conflict = Conflict {
                            conflict_type: ConflictType::MethodologicalConflict,
                            laws_involved: vec![law_name.to_string()],
                            conflict_score: 0.8,
                            description: format!(
                                "Score type changed for {}",
                                get_law_display_name(law_name)
                            ),
                            likely_cause: "Internal analysis error or data corruption".to_string(),
                            resolution_suggestion: "Please re-run the analysis".to_string(),
                        };
                        self.conflicts.push(conflict);
                    }
                }
            }
        }
    }

    /// diffx-core強化版スコア矛盾検出
    fn detect_score_conflicts(&mut self) {
        let laws: Vec<String> = self.law_scores.keys().cloned().collect();

        // diffx-coreを使用した構造化比較用のJSONオブジェクト作成
        for i in 0..laws.len() {
            for j in i + 1..laws.len() {
                let law_a = &laws[i];
                let law_b = &laws[j];

                if let (Some(&score_a), Some(&score_b)) =
                    (self.law_scores.get(law_a), self.law_scores.get(law_b))
                {
                    // 法則Aの詳細構造
                    let law_a_profile = serde_json::json!({
                        "law_name": law_a,
                        "score": score_a,
                        "confidence_level": self.get_confidence_level(score_a),
                        "score_category": self.categorize_score(score_a),
                        "relative_rank": self.get_relative_rank(law_a)
                    });

                    // 法則Bの詳細構造
                    let law_b_profile = serde_json::json!({
                        "law_name": law_b,
                        "score": score_b,
                        "confidence_level": self.get_confidence_level(score_b),
                        "score_category": self.categorize_score(score_b),
                        "relative_rank": self.get_relative_rank(law_b)
                    });

                    // diffx-coreで構造的差分を検出
                    let diff_results = diff(&law_a_profile, &law_b_profile, None, Some(0.1), None);

                    // 従来の単純差分計算
                    let score_diff = (score_a - score_b).abs();
                    let max_score = score_a.max(score_b);

                    if max_score > 0.0 {
                        let conflict_ratio = score_diff / max_score;

                        // diffx-coreの結果と組み合わせた強化判定
                        let has_structural_conflict = !diff_results.is_empty()
                            && diff_results.iter().any(|result| {
                                if let DiffResult::Modified(path, _old_val, _new_val) = result {
                                    if path.contains("confidence_level")
                                        || path.contains("score_category")
                                    {
                                        return true;
                                    }
                                }
                                false
                            });

                        if conflict_ratio > 0.5 || has_structural_conflict {
                            let enhanced_conflict_score = if has_structural_conflict {
                                conflict_ratio * 1.5 // 構造的矛盾があれば重みを増加
                            } else {
                                conflict_ratio
                            };

                            let conflict = self.create_enhanced_conflict(
                                law_a.clone(),
                                law_b.clone(),
                                enhanced_conflict_score.min(1.0),
                                score_a,
                                score_b,
                                &diff_results,
                            );
                            self.conflicts.push(conflict);
                        }
                    }
                }
            }
        }
    }

    /// diffx-core結果を含む強化版矛盾オブジェクト作成
    fn create_enhanced_conflict(
        &self,
        law_a: String,
        law_b: String,
        conflict_score: f64,
        score_a: f64,
        score_b: f64,
        diff_results: &[DiffResult],
    ) -> Conflict {
        let conflict_type = self.classify_conflict_type(&law_a, &law_b);

        // diffx-coreの差分情報から詳細な説明を生成
        let mut detailed_description = format!(
            "{} and {} show significantly different evaluations (difference: {:.3})",
            get_law_display_name(&law_a),
            get_law_display_name(&law_b),
            (score_a - score_b).abs()
        );

        if !diff_results.is_empty() {
            detailed_description.push_str(" with structural differences in: ");
            let diff_details: Vec<String> = diff_results
                .iter()
                .filter_map(|result| {
                    if let DiffResult::Modified(path, old_val, new_val) = result {
                        Some(format!("{path} ({old_val:?} → {new_val:?})"))
                    } else {
                        None
                    }
                })
                .collect();
            detailed_description.push_str(&diff_details.join(", "));
        }

        let likely_cause =
            self.diagnose_enhanced_conflict_cause(&law_a, &law_b, score_a, score_b, diff_results);
        let resolution_suggestion =
            self.suggest_enhanced_conflict_resolution(&law_a, &law_b, &conflict_type, diff_results);

        Conflict {
            conflict_type,
            laws_involved: vec![law_a, law_b],
            conflict_score,
            description: detailed_description,
            likely_cause,
            resolution_suggestion,
        }
    }

    /// ヘルパーメソッド: 信頼度レベル計算
    fn get_confidence_level(&self, score: f64) -> String {
        match score {
            s if s >= 0.8 => "high".to_string(),
            s if s >= 0.6 => "medium".to_string(),
            s if s >= 0.4 => "low".to_string(),
            _ => "very_low".to_string(),
        }
    }

    /// ヘルパーメソッド: スコア分類
    fn categorize_score(&self, score: f64) -> String {
        match score {
            s if s >= 0.9 => "excellent".to_string(),
            s if s >= 0.7 => "good".to_string(),
            s if s >= 0.5 => "fair".to_string(),
            s if s >= 0.3 => "poor".to_string(),
            _ => "very_poor".to_string(),
        }
    }

    /// ヘルパーメソッド: 相対順位取得
    fn get_relative_rank(&self, law_name: &str) -> usize {
        let mut scores: Vec<(String, f64)> = self
            .law_scores
            .iter()
            .map(|(name, &score)| (name.clone(), score))
            .collect();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scores
            .iter()
            .position(|(name, _)| name == law_name)
            .unwrap_or(0)
            + 1
    }

    /// 強化版原因診断
    fn diagnose_enhanced_conflict_cause(
        &self,
        law_a: &str,
        law_b: &str,
        score_a: f64,
        score_b: f64,
        diff_results: &[DiffResult],
    ) -> String {
        let mut cause = self.diagnose_conflict_cause(law_a, law_b, score_a, score_b);

        if !diff_results.is_empty() {
            cause.push_str(" Additionally, structural analysis reveals: ");
            let structural_issues: Vec<String> = diff_results
                .iter()
                .filter_map(|result| {
                    if let DiffResult::Modified(path, _, _) = result {
                        if path.contains("confidence_level") {
                            Some("confidence level mismatch".to_string())
                        } else if path.contains("score_category") {
                            Some("score category divergence".to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            cause.push_str(&structural_issues.join(", "));
        }

        cause
    }

    /// 強化版解決策提案
    fn suggest_enhanced_conflict_resolution(
        &self,
        law_a: &str,
        law_b: &str,
        conflict_type: &ConflictType,
        diff_results: &[DiffResult],
    ) -> String {
        let mut suggestion = self.suggest_conflict_resolution(law_a, law_b, conflict_type);

        if !diff_results.is_empty() {
            suggestion.push_str(" Consider deep structural analysis of data characteristics affecting confidence levels and score categories.");
        }

        suggestion
    }

    #[allow(dead_code)]
    fn create_conflict(
        &self,
        law_a: String,
        law_b: String,
        conflict_score: f64,
        score_a: f64,
        score_b: f64,
    ) -> Conflict {
        let conflict_type = self.classify_conflict_type(&law_a, &law_b);
        let description = format!(
            "{} and {} show significantly different evaluations (difference: {:.3})",
            get_law_display_name(&law_a),
            get_law_display_name(&law_b),
            (score_a - score_b).abs()
        );
        let likely_cause = self.diagnose_conflict_cause(&law_a, &law_b, score_a, score_b);
        let resolution_suggestion =
            self.suggest_conflict_resolution(&law_a, &law_b, &conflict_type);

        Conflict {
            conflict_type,
            laws_involved: vec![law_a, law_b],
            conflict_score,
            description,
            likely_cause,
            resolution_suggestion,
        }
    }

    fn classify_conflict_type(&self, law_a: &str, law_b: &str) -> ConflictType {
        match (law_a, law_b) {
            ("normal", "poisson") | ("poisson", "normal") => ConflictType::DistributionMismatch,
            ("benf", _) | (_, "benf") => ConflictType::QualityDisagreement,
            ("pareto", "zipf") | ("zipf", "pareto") => ConflictType::ScaleIncompatibility,
            _ => ConflictType::MethodologicalConflict,
        }
    }

    fn diagnose_conflict_cause(
        &self,
        law_a: &str,
        law_b: &str,
        score_a: f64,
        score_b: f64,
    ) -> String {
        match (&self.data_characteristics.data_type, law_a, law_b) {
            (DataType::Discrete, "normal", "poisson") if score_a < score_b => {
                "Normal distribution applied to discrete data".to_string()
            }
            (DataType::Continuous, "poisson", "normal") if score_a < score_b => {
                "Poisson distribution applied to continuous data".to_string()
            }
            (_, "benf", _) if score_a > score_b => {
                "Data shows naturalness but different distribution characteristics".to_string()
            }
            _ => "Laws have different applicability ranges due to complex data characteristics"
                .to_string(),
        }
    }

    fn suggest_conflict_resolution(
        &self,
        _law_a: &str,
        _law_b: &str,
        conflict_type: &ConflictType,
    ) -> String {
        match conflict_type {
            ConflictType::DistributionMismatch => {
                "Select the optimal distribution for your data type".to_string()
            }
            ConflictType::QualityDisagreement => {
                "For quality auditing, prioritize Benford's Law".to_string()
            }
            ConflictType::ScaleIncompatibility => {
                "Check the scale characteristics of your data".to_string()
            }
            _ => "Use multiple laws in combination for comprehensive analysis".to_string(),
        }
    }

    fn generate_recommendations(&mut self) {
        let scored_laws = self.score_laws_for_recommendation();

        if scored_laws.is_empty() {
            self.recommendations = Recommendation::empty();
            self.recommendation_confidence = 0.0;
            return;
        }

        let primary_law = scored_laws[0].0.clone();
        let secondary_laws: Vec<String> = scored_laws
            .iter()
            .skip(1)
            .take(2)
            .map(|(law, _)| law.clone())
            .collect();

        let confidence = self.calculate_recommendation_confidence(&scored_laws);
        let rationale = self.generate_recommendation_rationale(&primary_law, &secondary_laws);
        let alternatives = self.generate_alternative_combinations();

        self.recommendations = Recommendation {
            primary_law,
            secondary_laws,
            confidence,
            rationale,
            alternative_combinations: alternatives,
        };

        self.recommendation_confidence = confidence;
    }

    fn score_laws_for_recommendation(&self) -> Vec<(String, f64)> {
        let mut scored_laws = Vec::new();
        let weights = self.get_adaptive_weights();

        for (law, &base_score) in &self.law_scores {
            let weight = weights.get(law).unwrap_or(&1.0);
            let compatibility_bonus = self.calculate_compatibility_bonus(law);
            let purpose_bonus = self.calculate_purpose_bonus(law);

            let total_score = base_score * weight + compatibility_bonus + purpose_bonus;
            scored_laws.push((law.clone(), total_score));
        }

        scored_laws.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scored_laws
    }

    fn get_adaptive_weights(&self) -> HashMap<String, f64> {
        let mut weights = HashMap::new();

        // ベースライン重み
        weights.insert("benf".to_string(), 1.0);
        weights.insert("pareto".to_string(), 1.0);
        weights.insert("zipf".to_string(), 1.0);
        weights.insert("normal".to_string(), 1.0);
        weights.insert("poisson".to_string(), 1.0);

        // データ特性に応じた調整
        match self.data_characteristics.data_type {
            DataType::Continuous => {
                weights.insert("normal".to_string(), 1.5);
                weights.insert("poisson".to_string(), 0.5);
            }
            DataType::Discrete => {
                weights.insert("poisson".to_string(), 1.5);
                weights.insert("normal".to_string(), 0.5);
            }
            DataType::Integer => {
                weights.insert("poisson".to_string(), 1.3);
                weights.insert("normal".to_string(), 0.7);
            }
            _ => {}
        }

        // 分析目的に応じた調整
        match self.data_characteristics.analysis_purpose {
            AnalysisPurpose::QualityAudit | AnalysisPurpose::FraudDetection => {
                weights.insert("benf".to_string(), 2.0);
            }
            AnalysisPurpose::ConcentrationAnalysis => {
                weights.insert("pareto".to_string(), 2.0);
                weights.insert("zipf".to_string(), 1.5);
            }
            AnalysisPurpose::AnomalyDetection => {
                weights.insert("normal".to_string(), 1.8);
                weights.insert("poisson".to_string(), 1.5);
            }
            _ => {}
        }

        weights
    }

    fn calculate_compatibility_bonus(&self, law: &str) -> f64 {
        match (law, &self.data_characteristics.data_type) {
            ("normal", DataType::Continuous) => 0.2,
            ("poisson", DataType::Discrete) => 0.2,
            ("poisson", DataType::Integer) => 0.15,
            ("benf", _) => 0.1, // ベンフォード法則は汎用的
            _ => 0.0,
        }
    }

    fn calculate_purpose_bonus(&self, law: &str) -> f64 {
        match (law, &self.data_characteristics.analysis_purpose) {
            ("benf", AnalysisPurpose::QualityAudit) => 0.3,
            ("benf", AnalysisPurpose::FraudDetection) => 0.3,
            ("pareto", AnalysisPurpose::ConcentrationAnalysis) => 0.25,
            ("normal", AnalysisPurpose::AnomalyDetection) => 0.25,
            ("poisson", AnalysisPurpose::AnomalyDetection) => 0.2,
            _ => 0.0,
        }
    }

    fn calculate_recommendation_confidence(&self, scored_laws: &[(String, f64)]) -> f64 {
        if scored_laws.len() < 2 {
            return 0.5;
        }

        let top_score = scored_laws[0].1;
        let second_score = scored_laws[1].1;

        let score_gap = top_score - second_score;
        let consistency_factor = self.consistency_score;
        let conflict_penalty = self.conflicts_detected as f64 * 0.1;

        ((score_gap + consistency_factor) / 2.0 - conflict_penalty).clamp(0.1, 1.0)
    }

    fn generate_recommendation_rationale(&self, primary: &str, secondary: &[String]) -> String {
        let primary_reason = match primary {
            "benf" => "excellent data naturalness and quality",
            "pareto" => "optimal for concentration analysis",
            "zipf" => "good fit for frequency distribution characteristics",
            "normal" => "normality confirmed",
            "poisson" => "matches event occurrence patterns",
            _ => "high overall compatibility",
        };

        let secondary_reason = if !secondary.is_empty() {
            format!(
                ", complementary analysis possible with {}",
                secondary.join(" and ")
            )
        } else {
            String::new()
        };

        format!("{primary_reason}{secondary_reason}")
    }

    fn generate_alternative_combinations(&self) -> Vec<LawCombination> {
        let mut combinations = Vec::new();

        // Quality audit combination
        if self.law_scores.contains_key("benf") && self.law_scores.contains_key("normal") {
            combinations.push(LawCombination {
                laws: vec!["benf".to_string(), "normal".to_string()],
                purpose: "Quality Audit".to_string(),
                effectiveness_score: 0.85,
                description: "Benford's Law for naturalness, Normal distribution for statistical quality assessment".to_string(),
            });
        }

        // Concentration analysis combination
        if self.law_scores.contains_key("pareto") && self.law_scores.contains_key("zipf") {
            combinations.push(LawCombination {
                laws: vec!["pareto".to_string(), "zipf".to_string()],
                purpose: "Concentration Analysis".to_string(),
                effectiveness_score: 0.8,
                description:
                    "Pareto principle for 80/20 rule, Zipf's Law for rank distribution analysis"
                        .to_string(),
            });
        }

        // Anomaly detection combination
        if self.law_scores.contains_key("normal") && self.law_scores.contains_key("poisson") {
            combinations.push(LawCombination {
                laws: vec!["normal".to_string(), "poisson".to_string()],
                purpose: "Anomaly Detection".to_string(),
                effectiveness_score: 0.75,
                description: "Normal distribution for outliers, Poisson distribution for rare event detection".to_string(),
            });
        }

        combinations
    }

    fn assess_overall_quality(&mut self) {
        let high_quality_count = self
            .law_scores
            .values()
            .filter(|&&score| score > 0.8)
            .count();

        let low_quality_count = self
            .law_scores
            .values()
            .filter(|&&score| score < 0.4)
            .count();

        let total_laws = self.law_scores.len();

        self.overall_assessment = match (high_quality_count, low_quality_count, total_laws) {
            (h, 0, t) if h == t => OverallAssessment::Excellent,
            (h, l, t) if h >= t * 2 / 3 && l == 0 => OverallAssessment::Good,
            (_, l, t) if l >= t / 2 => OverallAssessment::Problematic,
            (_, l, _) if l > 0 && self.conflicts_detected > 2 => OverallAssessment::Concerning,
            _ => OverallAssessment::Mixed,
        };
    }

    fn determine_risk_level(&mut self) {
        self.risk_level = match self.overall_assessment {
            OverallAssessment::Excellent => RiskLevel::Low,
            OverallAssessment::Good => RiskLevel::Low,
            OverallAssessment::Mixed => RiskLevel::Medium,
            OverallAssessment::Concerning => RiskLevel::High,
            OverallAssessment::Problematic => RiskLevel::Critical,
        };
    }
}

impl Recommendation {
    pub fn empty() -> Self {
        Self {
            primary_law: String::new(),
            secondary_laws: Vec::new(),
            confidence: 0.0,
            rationale: String::new(),
            alternative_combinations: Vec::new(),
        }
    }
}

impl DataCharacteristics {
    pub fn analyze(numbers: &[f64]) -> Self {
        let data_type = detect_data_type(numbers);
        let distribution_shape = detect_distribution_shape(numbers);
        let outlier_presence = detect_outliers(numbers);
        let scale_range = detect_scale_range(numbers);
        let sample_size_category = categorize_sample_size(numbers.len());

        Self {
            data_type,
            distribution_shape,
            outlier_presence,
            scale_range,
            analysis_purpose: AnalysisPurpose::GeneralAnalysis, // デフォルト
            sample_size_category,
        }
    }
}

/// ラップ型 - 各法則の結果を統一的に扱う
#[derive(Debug, Clone)]
pub enum LawResult {
    Benford(BenfordResult),
    Pareto(ParetoResult),
    Zipf(ZipfResult),
    Normal(NormalResult),
    Poisson(PoissonResult),
}

// ヘルパー関数群

fn detect_data_type(numbers: &[f64]) -> DataType {
    let all_integers = numbers.iter().all(|&x| x.fract() == 0.0);
    let all_non_negative = numbers.iter().all(|&x| x >= 0.0);

    if all_integers && all_non_negative {
        DataType::Integer
    } else if all_integers {
        DataType::Discrete
    } else {
        DataType::Continuous
    }
}

fn detect_distribution_shape(numbers: &[f64]) -> DistributionShape {
    if numbers.len() < 10 {
        return DistributionShape::Unknown;
    }

    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let variance =
        numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (numbers.len() - 1) as f64;

    // 簡易的な分布形状判定
    let skewness = calculate_skewness(numbers, mean, variance.sqrt());

    if skewness.abs() < 0.5 {
        DistributionShape::Normal
    } else if skewness > 1.0 {
        DistributionShape::Skewed
    } else {
        DistributionShape::Unknown
    }
}

fn calculate_skewness(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    if std_dev == 0.0 {
        return 0.0;
    }

    let n = numbers.len() as f64;
    let sum_cubed_deviations = numbers
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>();

    sum_cubed_deviations / n
}

fn detect_outliers(numbers: &[f64]) -> OutlierLevel {
    if numbers.len() < 10 {
        return OutlierLevel::None;
    }

    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let q1_idx = sorted_numbers.len() / 4;
    let q3_idx = (sorted_numbers.len() * 3) / 4;

    let q1 = sorted_numbers[q1_idx];
    let q3 = sorted_numbers[q3_idx];
    let iqr = q3 - q1;

    let lower_bound = q1 - 1.5 * iqr;
    let upper_bound = q3 + 1.5 * iqr;

    let outlier_count = numbers
        .iter()
        .filter(|&&x| x < lower_bound || x > upper_bound)
        .count();

    let outlier_ratio = outlier_count as f64 / numbers.len() as f64;

    match outlier_ratio {
        0.0 => OutlierLevel::None,
        r if r < 0.05 => OutlierLevel::Low,
        r if r < 0.1 => OutlierLevel::Moderate,
        r if r < 0.2 => OutlierLevel::High,
        _ => OutlierLevel::Extreme,
    }
}

fn detect_scale_range(numbers: &[f64]) -> ScaleRange {
    if numbers.is_empty() {
        return ScaleRange::Narrow;
    }

    let min_val = numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val = numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    if min_val <= 0.0 || max_val <= 0.0 {
        return ScaleRange::Mixed;
    }

    let range_ratio = max_val / min_val;

    match range_ratio {
        r if r < 100.0 => ScaleRange::Narrow,   // 2桁以下
        r if r < 10000.0 => ScaleRange::Medium, // 4桁以下
        _ => ScaleRange::Wide,                  // 5桁以上
    }
}

fn categorize_sample_size(size: usize) -> SampleSizeCategory {
    match size {
        0..=29 => SampleSizeCategory::Small,
        30..=299 => SampleSizeCategory::Medium,
        300..=2999 => SampleSizeCategory::Large,
        _ => SampleSizeCategory::VeryLarge,
    }
}
