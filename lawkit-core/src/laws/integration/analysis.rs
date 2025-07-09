use super::result::*;
use crate::error::Result;
use crate::laws::benford::analyze_benford_law;
use crate::laws::normal::analyze_normal_distribution;
use crate::laws::pareto::analyze_pareto_distribution;
use crate::laws::poisson::analyze_poisson_distribution;
use crate::laws::zipf::analyze_numeric_zipf;
use diffx_core::{diff, DiffResult};
use rayon::prelude::*;
use std::collections::HashSet;

/// 統合分析実行
pub fn analyze_all_laws(numbers: &[f64], dataset_name: &str) -> Result<IntegrationResult> {
    let mut result = IntegrationResult::new(dataset_name.to_string(), numbers);

    // 全法則を並列実行
    let law_results = execute_laws_parallel(numbers, dataset_name);

    // 結果を統合
    for (law_name, law_result) in law_results {
        if let Ok(res) = law_result {
            result.add_law_result(&law_name, res);
        }
    }

    // 統合分析実行
    result.finalize_analysis();

    Ok(result)
}

/// 指定した法則のみで統合分析
pub fn analyze_selected_laws(
    numbers: &[f64],
    dataset_name: &str,
    selected_laws: &[String],
) -> Result<IntegrationResult> {
    let mut result = IntegrationResult::new(dataset_name.to_string(), numbers);

    // 指定法則のみ実行
    let law_results = execute_selected_laws_parallel(numbers, dataset_name, selected_laws);

    // 結果を統合
    for (law_name, law_result) in law_results {
        if let Ok(res) = law_result {
            result.add_law_result(&law_name, res);
        }
    }

    // 統合分析実行
    result.finalize_analysis();

    Ok(result)
}

/// 法則間比較分析
pub fn compare_laws(
    numbers: &[f64],
    dataset_name: &str,
    focus: Option<&str>,
) -> Result<IntegrationResult> {
    let mut result = analyze_all_laws(numbers, dataset_name)?;

    // フォーカス分析
    if let Some(focus_area) = focus {
        apply_focus_analysis(&mut result, focus_area);
    }

    Ok(result)
}

/// クロスバリデーション分析
pub fn cross_validate_laws(
    numbers: &[f64],
    dataset_name: &str,
    confidence_level: f64,
) -> Result<CrossValidationResult> {
    // データを分割してクロスバリデーション実行
    let chunk_size = numbers.len() / 5; // 5-fold クロスバリデーション
    let mut validation_results = Vec::new();

    for i in 0..5 {
        let start = i * chunk_size;
        let end = if i == 4 {
            numbers.len()
        } else {
            (i + 1) * chunk_size
        };

        let test_data = &numbers[start..end];
        let train_data: Vec<f64> = numbers[..start]
            .iter()
            .chain(numbers[end..].iter())
            .cloned()
            .collect();

        if !train_data.is_empty() && !test_data.is_empty() {
            let train_result = analyze_all_laws(&train_data, &format!("{dataset_name}_train_{i}"))?;
            let test_result = analyze_all_laws(test_data, &format!("{dataset_name}_test_{i}"))?;

            validation_results.push(ValidationFold {
                fold_number: i + 1,
                train_result: train_result.clone(),
                test_result: test_result.clone(),
                consistency_score: calculate_fold_consistency(&train_result, &test_result),
            });
        }
    }

    Ok(CrossValidationResult {
        dataset_name: dataset_name.to_string(),
        confidence_level,
        validation_folds: validation_results.clone(),
        overall_stability: calculate_overall_stability(&validation_results),
        stability_assessment: assess_stability(&validation_results),
    })
}

/// 矛盾検出に特化した分析
pub fn detect_conflicts_detailed(
    numbers: &[f64],
    dataset_name: &str,
    threshold: f64,
) -> Result<ConflictAnalysisResult> {
    let integration_result = analyze_all_laws(numbers, dataset_name)?;

    let detailed_conflicts = analyze_conflicts_in_depth(&integration_result, threshold);
    let conflict_patterns = identify_conflict_patterns(&detailed_conflicts);
    let resolution_strategies = generate_resolution_strategies(&detailed_conflicts);

    Ok(ConflictAnalysisResult {
        dataset_name: dataset_name.to_string(),
        threshold,
        integration_result,
        detailed_conflicts: detailed_conflicts.clone(),
        conflict_patterns,
        resolution_strategies,
        conflict_severity: assess_conflict_severity(&detailed_conflicts),
    })
}

/// 推奨システム詳細分析
pub fn generate_detailed_recommendations(
    numbers: &[f64],
    dataset_name: &str,
    analysis_purpose: AnalysisPurpose,
) -> Result<DetailedRecommendationResult> {
    let mut integration_result = analyze_all_laws(numbers, dataset_name)?;

    // 分析目的を設定
    integration_result.data_characteristics.analysis_purpose = analysis_purpose.clone();
    integration_result.finalize_analysis();

    let purpose_specific_recommendations = generate_purpose_recommendations(&integration_result);
    let combination_analysis = analyze_law_combinations(&integration_result);
    let effectiveness_scores = calculate_effectiveness_scores(&integration_result);

    Ok(DetailedRecommendationResult {
        dataset_name: dataset_name.to_string(),
        analysis_purpose,
        integration_result: integration_result.clone(),
        purpose_specific_recommendations,
        combination_analysis,
        effectiveness_scores,
        implementation_guidance: generate_implementation_guidance(&integration_result),
    })
}

// ヘルパー関数群

fn execute_laws_parallel(numbers: &[f64], dataset_name: &str) -> Vec<(String, Result<LawResult>)> {
    let laws = vec!["benf", "pareto", "zipf", "normal", "poisson"];

    laws.par_iter()
        .map(|&law| {
            let result = match law {
                "benf" => analyze_benford_law(numbers, dataset_name).map(LawResult::Benford),
                "pareto" => {
                    analyze_pareto_distribution(numbers, dataset_name).map(LawResult::Pareto)
                }
                "zipf" => analyze_numeric_zipf(numbers, dataset_name).map(LawResult::Zipf),
                "normal" => {
                    analyze_normal_distribution(numbers, dataset_name).map(LawResult::Normal)
                }
                "poisson" => {
                    analyze_poisson_distribution(numbers, dataset_name).map(LawResult::Poisson)
                }
                _ => Err(crate::error::BenfError::InvalidInput(format!(
                    "Unknown law: {law}"
                ))),
            };
            (law.to_string(), result)
        })
        .collect()
}

fn execute_selected_laws_parallel(
    numbers: &[f64],
    dataset_name: &str,
    selected_laws: &[String],
) -> Vec<(String, Result<LawResult>)> {
    let available_laws: HashSet<&str> = ["benf", "pareto", "zipf", "normal", "poisson"]
        .iter()
        .cloned()
        .collect();

    selected_laws
        .par_iter()
        .filter(|law| available_laws.contains(law.as_str()))
        .map(|law| {
            let result = match law.as_str() {
                "benf" => analyze_benford_law(numbers, dataset_name).map(LawResult::Benford),
                "pareto" => {
                    analyze_pareto_distribution(numbers, dataset_name).map(LawResult::Pareto)
                }
                "zipf" => analyze_numeric_zipf(numbers, dataset_name).map(LawResult::Zipf),
                "normal" => {
                    analyze_normal_distribution(numbers, dataset_name).map(LawResult::Normal)
                }
                "poisson" => {
                    analyze_poisson_distribution(numbers, dataset_name).map(LawResult::Poisson)
                }
                _ => Err(crate::error::BenfError::InvalidInput(format!(
                    "Unknown law: {law}"
                ))),
            };
            (law.clone(), result)
        })
        .collect()
}

pub fn apply_focus_analysis(result: &mut IntegrationResult, focus: &str) {
    // Set the focus field
    result.focus = Some(focus.to_string());

    match focus {
        "quality" => {
            // 品質重視の重み調整
            if let Some(benf_score) = result.law_scores.get("benf") {
                result
                    .law_scores
                    .insert("benf".to_string(), benf_score * 1.5);
            }
            if let Some(normal_score) = result.law_scores.get("normal") {
                result
                    .law_scores
                    .insert("normal".to_string(), normal_score * 1.3);
            }
        }
        "concentration" => {
            // 集中度重視の重み調整
            if let Some(pareto_score) = result.law_scores.get("pareto") {
                result
                    .law_scores
                    .insert("pareto".to_string(), pareto_score * 1.5);
            }
            if let Some(zipf_score) = result.law_scores.get("zipf") {
                result
                    .law_scores
                    .insert("zipf".to_string(), zipf_score * 1.3);
            }
        }
        "distribution" => {
            // 分布適合重視
            if let Some(normal_score) = result.law_scores.get("normal") {
                result
                    .law_scores
                    .insert("normal".to_string(), normal_score * 1.4);
            }
            if let Some(poisson_score) = result.law_scores.get("poisson") {
                result
                    .law_scores
                    .insert("poisson".to_string(), poisson_score * 1.4);
            }
        }
        "anomaly" => {
            // 異常検知重視
            if let Some(normal_score) = result.law_scores.get("normal") {
                result
                    .law_scores
                    .insert("normal".to_string(), normal_score * 1.6);
            }
            if let Some(poisson_score) = result.law_scores.get("poisson") {
                result
                    .law_scores
                    .insert("poisson".to_string(), poisson_score * 1.4);
            }
        }
        _ => {}
    }

    // 重み調整後に再計算
    result.finalize_analysis();
}

fn calculate_fold_consistency(
    train_result: &IntegrationResult,
    test_result: &IntegrationResult,
) -> f64 {
    // diffx-coreを使用した一貫性分析
    calculate_enhanced_consistency_with_diffx(train_result, test_result)
}

/// diffx-coreを使用した一貫性計算
fn calculate_enhanced_consistency_with_diffx(
    train_result: &IntegrationResult,
    test_result: &IntegrationResult,
) -> f64 {
    // HashMap<String, f64>をJSONに変換してdiffx-coreで比較
    let train_json = serde_json::to_value(&train_result.law_scores).unwrap_or_default();
    let test_json = serde_json::to_value(&test_result.law_scores).unwrap_or_default();

    // diffx-coreで構造的差分を検出
    let diff_results = diff(&train_json, &test_json, None, Some(0.01), None);

    if diff_results.is_empty() {
        return 1.0; // 完全一致
    }

    // 差分の種類と重要度に基づいて一貫性スコアを計算
    let total_laws = train_result
        .law_scores
        .len()
        .max(test_result.law_scores.len()) as f64;
    if total_laws == 0.0 {
        return 0.0;
    }

    let mut total_diff_impact = 0.0;

    for diff_result in &diff_results {
        let impact = match diff_result {
            DiffResult::Added(_, _) => 0.5,   // 追加は中程度の影響
            DiffResult::Removed(_, _) => 0.5, // 削除は中程度の影響
            DiffResult::Modified(_, old_val, new_val) => {
                // 数値の変更は差分の大きさに応じて影響度を計算
                if let (Some(old_num), Some(new_num)) = (old_val.as_f64(), new_val.as_f64()) {
                    let diff_ratio = (old_num - new_num).abs() / old_num.max(new_num).max(0.01);
                    diff_ratio.min(1.0)
                } else {
                    1.0 // 非数値の変更は最大影響
                }
            }
            DiffResult::TypeChanged(_, _, _) => 1.0, // 型変更は最大影響
        };
        total_diff_impact += impact;
    }

    // 一貫性スコア = 1 - (平均影響度)
    let average_impact = total_diff_impact / diff_results.len() as f64;
    (1.0 - average_impact).max(0.0)
}

fn calculate_overall_stability(validation_results: &[ValidationFold]) -> f64 {
    if validation_results.is_empty() {
        return 0.0;
    }

    let total_consistency: f64 = validation_results
        .iter()
        .map(|fold| fold.consistency_score)
        .sum();

    total_consistency / validation_results.len() as f64
}

fn assess_stability(validation_results: &[ValidationFold]) -> StabilityAssessment {
    let overall_stability = calculate_overall_stability(validation_results);

    match overall_stability {
        s if s > 0.9 => StabilityAssessment::VeryStable,
        s if s > 0.8 => StabilityAssessment::Stable,
        s if s > 0.7 => StabilityAssessment::ModeratelyStable,
        s if s > 0.6 => StabilityAssessment::Unstable,
        _ => StabilityAssessment::VeryUnstable,
    }
}

fn analyze_conflicts_in_depth(result: &IntegrationResult, threshold: f64) -> Vec<DetailedConflict> {
    let mut detailed_conflicts = Vec::new();

    for conflict in &result.conflicts {
        if conflict.conflict_score >= threshold {
            let statistical_significance = calculate_statistical_significance(conflict, result);
            let impact_assessment = assess_conflict_impact(conflict, result);
            let root_cause_analysis = perform_root_cause_analysis(conflict, result);

            detailed_conflicts.push(DetailedConflict {
                base_conflict: conflict.clone(),
                statistical_significance,
                impact_assessment,
                root_cause_analysis,
                confidence_interval: calculate_conflict_confidence_interval(conflict, result),
            });
        }
    }

    detailed_conflicts
}

fn identify_conflict_patterns(detailed_conflicts: &[DetailedConflict]) -> Vec<ConflictPattern> {
    let mut patterns = Vec::new();

    // 頻出する矛盾タイプを特定
    let mut type_counts = std::collections::HashMap::new();
    for conflict in detailed_conflicts {
        *type_counts
            .entry(conflict.base_conflict.conflict_type.clone())
            .or_insert(0) += 1;
    }

    for (conflict_type, count) in type_counts {
        if count > 1 {
            patterns.push(ConflictPattern {
                pattern_type: conflict_type.clone(),
                frequency: count,
                severity: calculate_pattern_severity(detailed_conflicts, &conflict_type),
                description: describe_conflict_pattern(&conflict_type),
            });
        }
    }

    patterns
}

fn generate_resolution_strategies(
    detailed_conflicts: &[DetailedConflict],
) -> Vec<ResolutionStrategy> {
    let mut strategies = Vec::new();

    for conflict in detailed_conflicts {
        let strategy = match conflict.base_conflict.conflict_type {
            ConflictType::DistributionMismatch => ResolutionStrategy {
                strategy_name: "Distribution Type Optimization".to_string(),
                priority: Priority::High,
                steps: vec![
                    "Check data type (continuous/discrete)".to_string(),
                    "Select optimal distribution law".to_string(),
                    "Exclude inappropriate law results".to_string(),
                ],
                expected_outcome: "Improved distribution compatibility".to_string(),
                confidence: 0.85,
            },
            ConflictType::QualityDisagreement => ResolutionStrategy {
                strategy_name: "Quality Assessment Integration".to_string(),
                priority: Priority::Medium,
                steps: vec![
                    "Use Benford's Law as quality assessment baseline".to_string(),
                    "Utilize other laws as supplementary evaluation".to_string(),
                    "Make final decision with comprehensive quality score".to_string(),
                ],
                expected_outcome: "Consistent quality assessment".to_string(),
                confidence: 0.75,
            },
            _ => ResolutionStrategy {
                strategy_name: "Comprehensive Evaluation Focus".to_string(),
                priority: Priority::Low,
                steps: vec![
                    "Judge results from multiple laws comprehensively".to_string(),
                    "Utilize contradictory points as complementary information".to_string(),
                ],
                expected_outcome: "Comprehensive analysis results".to_string(),
                confidence: 0.6,
            },
        };

        strategies.push(strategy);
    }

    strategies
}

fn assess_conflict_severity(detailed_conflicts: &[DetailedConflict]) -> ConflictSeverity {
    if detailed_conflicts.is_empty() {
        return ConflictSeverity::None;
    }

    let max_score = detailed_conflicts
        .iter()
        .map(|c| c.base_conflict.conflict_score)
        .fold(0.0, f64::max);

    let high_severity_count = detailed_conflicts
        .iter()
        .filter(|c| c.base_conflict.conflict_score > 0.8)
        .count();

    match (max_score, high_severity_count) {
        (s, _) if s > 0.9 => ConflictSeverity::Critical,
        (s, c) if s > 0.7 && c > 2 => ConflictSeverity::High,
        (s, c) if s > 0.5 && c > 0 => ConflictSeverity::Medium,
        (s, _) if s > 0.3 => ConflictSeverity::Low,
        _ => ConflictSeverity::None,
    }
}

fn generate_purpose_recommendations(result: &IntegrationResult) -> Vec<PurposeRecommendation> {
    let mut recommendations = Vec::new();

    match result.data_characteristics.analysis_purpose {
        AnalysisPurpose::QualityAudit => {
            recommendations.push(PurposeRecommendation {
                purpose: AnalysisPurpose::QualityAudit,
                recommended_laws: vec!["benf".to_string(), "normal".to_string()],
                rationale: "品質監査にはベンフォード法則での自然性チェックと正規分布での統計的品質評価が最適".to_string(),
                effectiveness: 0.9,
                implementation_priority: Priority::High,
            });
        }
        AnalysisPurpose::ConcentrationAnalysis => {
            recommendations.push(PurposeRecommendation {
                purpose: AnalysisPurpose::ConcentrationAnalysis,
                recommended_laws: vec!["pareto".to_string(), "zipf".to_string()],
                rationale: "集中度分析にはパレート法則での80/20分析とZipf法則での順位分布が有効"
                    .to_string(),
                effectiveness: 0.85,
                implementation_priority: Priority::High,
            });
        }
        AnalysisPurpose::AnomalyDetection => {
            recommendations.push(PurposeRecommendation {
                purpose: AnalysisPurpose::AnomalyDetection,
                recommended_laws: vec!["normal".to_string(), "poisson".to_string()],
                rationale:
                    "異常検知には正規分布での外れ値検出とポアソン分布での稀少事象検出が適用可能"
                        .to_string(),
                effectiveness: 0.8,
                implementation_priority: Priority::Medium,
            });
        }
        _ => {
            recommendations.push(PurposeRecommendation {
                purpose: AnalysisPurpose::GeneralAnalysis,
                recommended_laws: result.laws_executed.clone(),
                rationale: "総合分析では全法則を活用して多角的な評価を実施".to_string(),
                effectiveness: 0.7,
                implementation_priority: Priority::Medium,
            });
        }
    }

    recommendations
}

fn analyze_law_combinations(result: &IntegrationResult) -> Vec<CombinationAnalysis> {
    let mut combinations = Vec::new();

    let laws: Vec<String> = result.law_scores.keys().cloned().collect();

    // 2法則組み合わせ分析
    for i in 0..laws.len() {
        for j in i + 1..laws.len() {
            let law_a = &laws[i];
            let law_b = &laws[j];

            let synergy_score = calculate_synergy_score(law_a, law_b, result);
            let complementarity = assess_complementarity(law_a, law_b);

            combinations.push(CombinationAnalysis {
                laws: vec![law_a.clone(), law_b.clone()],
                synergy_score,
                complementarity,
                use_cases: generate_combination_use_cases(law_a, law_b),
                effectiveness_rating: rate_combination_effectiveness(
                    synergy_score,
                    complementarity,
                ),
            });
        }
    }

    combinations.sort_by(|a, b| b.synergy_score.partial_cmp(&a.synergy_score).unwrap());
    combinations
}

fn calculate_effectiveness_scores(
    result: &IntegrationResult,
) -> HashMap<String, EffectivenessScore> {
    let mut scores = HashMap::new();

    for (law, &base_score) in &result.law_scores {
        let data_compatibility =
            calculate_data_compatibility_score(law, &result.data_characteristics);
        let purpose_alignment =
            calculate_purpose_alignment_score(law, &result.data_characteristics.analysis_purpose);
        let reliability = calculate_reliability_score(law, result);

        let overall_effectiveness =
            (base_score + data_compatibility + purpose_alignment + reliability) / 4.0;

        scores.insert(
            law.clone(),
            EffectivenessScore {
                base_score,
                data_compatibility,
                purpose_alignment,
                reliability,
                overall_effectiveness,
            },
        );
    }

    scores
}

fn generate_implementation_guidance(result: &IntegrationResult) -> ImplementationGuidance {
    let primary_law = &result.recommendations.primary_law;
    let setup_steps = generate_setup_steps(primary_law);
    let validation_criteria = generate_validation_criteria(primary_law);
    let monitoring_recommendations = generate_monitoring_recommendations(result);

    ImplementationGuidance {
        primary_law: primary_law.clone(),
        setup_steps,
        validation_criteria,
        monitoring_recommendations,
        estimated_effort: estimate_implementation_effort(result),
        success_indicators: generate_success_indicators(result),
    }
}

// 以下、詳細なヘルパー関数の実装は省略...
// （実際の実装では、上記の各関数の詳細な実装が必要）

// プレースホルダー実装
fn calculate_statistical_significance(_conflict: &Conflict, _result: &IntegrationResult) -> f64 {
    0.5
}
fn assess_conflict_impact(_conflict: &Conflict, _result: &IntegrationResult) -> ImpactLevel {
    ImpactLevel::Medium
}
fn perform_root_cause_analysis(_conflict: &Conflict, _result: &IntegrationResult) -> String {
    "Under analysis".to_string()
}
fn calculate_conflict_confidence_interval(
    _conflict: &Conflict,
    _result: &IntegrationResult,
) -> (f64, f64) {
    (0.0, 1.0)
}
fn calculate_pattern_severity(
    _conflicts: &[DetailedConflict],
    _conflict_type: &ConflictType,
) -> f64 {
    0.5
}
fn describe_conflict_pattern(_conflict_type: &ConflictType) -> String {
    "Pattern analysis in progress".to_string()
}
fn calculate_synergy_score(_law_a: &str, _law_b: &str, _result: &IntegrationResult) -> f64 {
    0.5
}
fn assess_complementarity(_law_a: &str, _law_b: &str) -> f64 {
    0.5
}
fn generate_combination_use_cases(_law_a: &str, _law_b: &str) -> Vec<String> {
    vec!["一般分析".to_string()]
}
fn rate_combination_effectiveness(_synergy: f64, _complementarity: f64) -> f64 {
    0.5
}
fn calculate_data_compatibility_score(_law: &str, _characteristics: &DataCharacteristics) -> f64 {
    0.5
}
fn calculate_purpose_alignment_score(_law: &str, _purpose: &AnalysisPurpose) -> f64 {
    0.5
}
fn calculate_reliability_score(_law: &str, _result: &IntegrationResult) -> f64 {
    0.5
}
fn generate_setup_steps(_law: &str) -> Vec<String> {
    vec!["セットアップ中".to_string()]
}
fn generate_validation_criteria(_law: &str) -> Vec<String> {
    vec!["検証基準設定中".to_string()]
}
fn generate_monitoring_recommendations(_result: &IntegrationResult) -> Vec<String> {
    vec!["監視設定中".to_string()]
}
fn estimate_implementation_effort(_result: &IntegrationResult) -> String {
    "中程度".to_string()
}
fn generate_success_indicators(_result: &IntegrationResult) -> Vec<String> {
    vec!["成功指標設定中".to_string()]
}

// 追加のデータ構造
#[derive(Debug, Clone)]
pub struct CrossValidationResult {
    pub dataset_name: String,
    pub confidence_level: f64,
    pub validation_folds: Vec<ValidationFold>,
    pub overall_stability: f64,
    pub stability_assessment: StabilityAssessment,
}

#[derive(Debug, Clone)]
pub struct ValidationFold {
    pub fold_number: usize,
    pub train_result: IntegrationResult,
    pub test_result: IntegrationResult,
    pub consistency_score: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StabilityAssessment {
    VeryStable,
    Stable,
    ModeratelyStable,
    Unstable,
    VeryUnstable,
}

#[derive(Debug, Clone)]
pub struct ConflictAnalysisResult {
    pub dataset_name: String,
    pub threshold: f64,
    pub integration_result: IntegrationResult,
    pub detailed_conflicts: Vec<DetailedConflict>,
    pub conflict_patterns: Vec<ConflictPattern>,
    pub resolution_strategies: Vec<ResolutionStrategy>,
    pub conflict_severity: ConflictSeverity,
}

#[derive(Debug, Clone)]
pub struct DetailedConflict {
    pub base_conflict: Conflict,
    pub statistical_significance: f64,
    pub impact_assessment: ImpactLevel,
    pub root_cause_analysis: String,
    pub confidence_interval: (f64, f64),
}

#[derive(Debug, Clone)]
pub struct ConflictPattern {
    pub pattern_type: ConflictType,
    pub frequency: usize,
    pub severity: f64,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ResolutionStrategy {
    pub strategy_name: String,
    pub priority: Priority,
    pub steps: Vec<String>,
    pub expected_outcome: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImpactLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictSeverity {
    Critical,
    High,
    Medium,
    Low,
    None,
}

#[derive(Debug, Clone)]
pub struct DetailedRecommendationResult {
    pub dataset_name: String,
    pub analysis_purpose: AnalysisPurpose,
    pub integration_result: IntegrationResult,
    pub purpose_specific_recommendations: Vec<PurposeRecommendation>,
    pub combination_analysis: Vec<CombinationAnalysis>,
    pub effectiveness_scores: HashMap<String, EffectivenessScore>,
    pub implementation_guidance: ImplementationGuidance,
}

#[derive(Debug, Clone)]
pub struct PurposeRecommendation {
    pub purpose: AnalysisPurpose,
    pub recommended_laws: Vec<String>,
    pub rationale: String,
    pub effectiveness: f64,
    pub implementation_priority: Priority,
}

#[derive(Debug, Clone)]
pub struct CombinationAnalysis {
    pub laws: Vec<String>,
    pub synergy_score: f64,
    pub complementarity: f64,
    pub use_cases: Vec<String>,
    pub effectiveness_rating: f64,
}

#[derive(Debug, Clone)]
pub struct EffectivenessScore {
    pub base_score: f64,
    pub data_compatibility: f64,
    pub purpose_alignment: f64,
    pub reliability: f64,
    pub overall_effectiveness: f64,
}

#[derive(Debug, Clone)]
pub struct ImplementationGuidance {
    pub primary_law: String,
    pub setup_steps: Vec<String>,
    pub validation_criteria: Vec<String>,
    pub monitoring_recommendations: Vec<String>,
    pub estimated_effort: String,
    pub success_indicators: Vec<String>,
}

use std::collections::HashMap;
