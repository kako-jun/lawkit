use lawkit_core::common::output::{create_output_writer, OutputConfig};
use lawkit_core::error::Result;
use lawkit_core::laws::integration::{
    analyze_all_laws, analyze_selected_laws, compare_laws, 
    cross_validate_laws, detect_conflicts_detailed, 
    generate_detailed_recommendations, AnalysisPurpose
};
use clap::{ArgMatches, Command, Arg, value_parser};
use std::io::Write;

pub fn command() -> Command {
    Command::new("compare")
        .about("複数の統計法則を比較・統合分析")
        .arg(
            Arg::new("input")
                .help("入力データ（ファイルパス、URL、または '-' で標準入力）")
                .value_name("INPUT")
                .index(1)
        )
        .arg(
            Arg::new("laws")
                .long("laws")
                .short('l')
                .help("比較対象法則 (benf,pareto,zipf,normal,poisson)")
                .value_name("LAWS")
        )
        .arg(
            Arg::new("focus")
                .long("focus")
                .short('f')
                .help("重点分析項目")
                .value_name("FOCUS")
                .value_parser(["quality", "concentration", "distribution", "anomaly"])
        )
        .arg(
            Arg::new("threshold")
                .long("threshold")
                .short('t')
                .help("矛盾検出閾値 (0.0-1.0)")
                .value_name("THRESHOLD")
                .value_parser(value_parser!(f64))
                .default_value("0.5")
        )
        .arg(
            Arg::new("recommend")
                .long("recommend")
                .short('r')
                .help("最適法則推奨モード")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("report")
                .long("report")
                .help("統合レポート生成")
                .value_name("TYPE")
                .value_parser(["summary", "detailed", "conflicting"])
                .default_value("summary")
        )
        .arg(
            Arg::new("consistency-check")
                .long("consistency-check")
                .help("一貫性チェック実行")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("cross-validation")
                .long("cross-validation")
                .help("相互検証分析")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("confidence-level")
                .long("confidence-level")
                .help("信頼度水準")
                .value_name("LEVEL")
                .value_parser(value_parser!(f64))
                .default_value("0.95")
        )
        .arg(
            Arg::new("purpose")
                .long("purpose")
                .short('p')
                .help("分析目的")
                .value_name("PURPOSE")
                .value_parser(["quality", "fraud", "concentration", "anomaly", "distribution", "general"])
        )
        // 共通オプション
        .arg(
            Arg::new("format")
                .long("format")
                .help("出力形式")
                .value_name("FORMAT")
                .value_parser(["text", "json", "csv", "yaml", "toml", "xml"])
                .default_value("text")
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .help("最小出力")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("詳細出力")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .help("出力言語")
                .value_name("LANG")
                .value_parser(["en", "ja", "zh", "hi", "ar", "auto"])
                .default_value("auto")
        )
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    // 分析モード判定
    if matches.get_flag("cross-validation") {
        return run_cross_validation_mode(matches);
    }
    
    if matches.get_flag("consistency-check") {
        return run_consistency_check_mode(matches);
    }
    
    if matches.get_flag("recommend") {
        return run_recommendation_mode(matches);
    }
    
    let report_type = matches.get_one::<String>("report").unwrap();
    match report_type.as_str() {
        "conflicting" => run_conflict_analysis_mode(matches),
        "detailed" => run_detailed_analysis_mode(matches),
        _ => run_summary_analysis_mode(matches),
    }
}

fn run_summary_analysis_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = lawkit_core::common::input::parse_input_auto(matches.get_one::<String>("input").unwrap())?;
    let dataset_name = get_dataset_name(matches);
    
    let result = if let Some(laws_str) = matches.get_one::<String>("laws") {
        let selected_laws: Vec<String> = laws_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        analyze_selected_laws(&numbers, &dataset_name, &selected_laws)?
    } else if let Some(focus) = matches.get_one::<String>("focus") {
        compare_laws(&numbers, &dataset_name, Some(focus))?
    } else {
        analyze_all_laws(&numbers, &dataset_name)?
    };
    
    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);
    
    output_integration_result(&mut writer, &result, &output_config)?;
    
    std::process::exit(result.risk_level.exit_code());
}

fn run_detailed_analysis_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = lawkit_core::common::input::parse_input_auto(matches.get_one::<String>("input").unwrap())?;
    let dataset_name = get_dataset_name(matches);
    
    let result = analyze_all_laws(&numbers, &dataset_name)?;
    
    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);
    
    output_detailed_integration_result(&mut writer, &result, &output_config)?;
    
    std::process::exit(result.risk_level.exit_code());
}

fn run_conflict_analysis_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = lawkit_core::common::input::parse_input_auto(matches.get_one::<String>("input").unwrap())?;
    let dataset_name = get_dataset_name(matches);
    let threshold = *matches.get_one::<f64>("threshold").unwrap();
    
    let conflict_result = detect_conflicts_detailed(&numbers, &dataset_name, threshold)?;
    
    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);
    
    output_conflict_analysis_result(&mut writer, &conflict_result, &output_config)?;
    
    std::process::exit(conflict_result.integration_result.risk_level.exit_code());
}

fn run_cross_validation_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = lawkit_core::common::input::parse_input_auto(matches.get_one::<String>("input").unwrap())?;
    let dataset_name = get_dataset_name(matches);
    let confidence_level = *matches.get_one::<f64>("confidence-level").unwrap();
    
    let cv_result = cross_validate_laws(&numbers, &dataset_name, confidence_level)?;
    
    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);
    
    output_cross_validation_result(&mut writer, &cv_result, &output_config)?;
    
    Ok(())
}

fn run_consistency_check_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = lawkit_core::common::input::parse_input_auto(matches.get_one::<String>("input").unwrap())?;
    let dataset_name = get_dataset_name(matches);
    let threshold = *matches.get_one::<f64>("threshold").unwrap();
    
    let result = analyze_all_laws(&numbers, &dataset_name)?;
    
    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);
    
    output_consistency_check_result(&mut writer, &result, threshold, &output_config)?;
    
    std::process::exit(result.risk_level.exit_code());
}

fn run_recommendation_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = lawkit_core::common::input::parse_input_auto(matches.get_one::<String>("input").unwrap())?;
    let dataset_name = get_dataset_name(matches);
    
    let analysis_purpose = matches.get_one::<String>("purpose")
        .map(|p| parse_analysis_purpose(p))
        .unwrap_or(AnalysisPurpose::GeneralAnalysis);
    
    let recommendation_result = generate_detailed_recommendations(&numbers, &dataset_name, analysis_purpose)?;
    
    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);
    
    output_recommendation_result(&mut writer, &recommendation_result, &output_config)?;
    
    std::process::exit(recommendation_result.integration_result.risk_level.exit_code());
}

fn get_dataset_name(matches: &ArgMatches) -> String {
    matches.get_one::<String>("input")
        .map(|s| s.clone())
        .unwrap_or_else(|| "stdin".to_string())
}

fn parse_analysis_purpose(purpose_str: &str) -> AnalysisPurpose {
    match purpose_str {
        "quality" => AnalysisPurpose::QualityAudit,
        "fraud" => AnalysisPurpose::FraudDetection,
        "concentration" => AnalysisPurpose::ConcentrationAnalysis,
        "anomaly" => AnalysisPurpose::AnomalyDetection,
        "distribution" => AnalysisPurpose::DistributionFitting,
        _ => AnalysisPurpose::GeneralAnalysis,
    }
}

// 出力関数群

fn output_integration_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    config: &OutputConfig,
) -> Result<()> {
    match config.format.as_str() {
        "json" => output_integration_json(writer, result),
        "csv" => output_integration_csv(writer, result),
        "yaml" => output_integration_yaml(writer, result),
        _ => output_integration_text(writer, result, config),
    }
}

fn output_integration_text(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    config: &OutputConfig,
) -> Result<()> {
    if config.quiet {
        writeln!(writer, "{:.3}", result.overall_quality_score)?;
        return Ok(());
    }

    let lang = &config.language;
    
    writeln!(writer, "{}", get_text("integration_title", lang))?;
    writeln!(writer)?;
    
    writeln!(writer, "{}: {}", get_text("dataset", lang), result.dataset_name)?;
    writeln!(writer, "{}: {}", get_text("numbers_analyzed", lang), result.numbers_analyzed)?;
    writeln!(writer, "{}: {} ({})", 
        get_text("laws_executed", lang), 
        result.laws_executed.len(),
        result.laws_executed.join(", ")
    )?;
    writeln!(writer)?;
    
    writeln!(writer, "{}:", get_text("integration_metrics", lang))?;
    writeln!(writer, "  {}: {:.3}", get_text("overall_quality", lang), result.overall_quality_score)?;
    writeln!(writer, "  {}: {:.3}", get_text("consistency", lang), result.consistency_score)?;
    writeln!(writer, "  {}: {}", get_text("conflicts_detected", lang), result.conflicts_detected)?;
    writeln!(writer, "  {}: {:.3}", get_text("recommendation_confidence", lang), result.recommendation_confidence)?;
    writeln!(writer)?;
    
    writeln!(writer, "{}:", get_text("law_results", lang))?;
    for (law, score) in &result.law_scores {
        let law_name = get_law_name(law, lang);
        writeln!(writer, "  {}: {:.3}", law_name, score)?;
    }
    writeln!(writer)?;
    
    if !result.conflicts.is_empty() {
        writeln!(writer, "{}:", get_text("conflicts", lang))?;
        for conflict in &result.conflicts {
            writeln!(writer, "  ⚠️ {}", conflict.description)?;
            writeln!(writer, "     {}: {}", get_text("cause", lang), conflict.likely_cause)?;
            writeln!(writer, "     {}: {}", get_text("suggestion", lang), conflict.resolution_suggestion)?;
        }
        writeln!(writer)?;
    }
    
    writeln!(writer, "{}:", get_text("recommendations", lang))?;
    writeln!(writer, "  🎯 {}: {}", 
        get_text("primary_law", lang), 
        get_law_name(&result.recommendations.primary_law, lang)
    )?;
    
    if !result.recommendations.secondary_laws.is_empty() {
        let secondary_names: Vec<String> = result.recommendations.secondary_laws
            .iter()
            .map(|law| get_law_name(law, lang))
            .collect();
        writeln!(writer, "  🔍 {}: {}", 
            get_text("secondary_laws", lang), 
            secondary_names.join(", ")
        )?;
    }
    
    writeln!(writer, "  📊 {}: {}", get_text("rationale", lang), result.recommendations.rationale)?;
    writeln!(writer)?;
    
    if config.verbose {
        output_verbose_integration_details(writer, result, lang)?;
    }
    
    Ok(())
}

fn output_integration_json(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
) -> Result<()> {
    let json_value = serde_json::json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "laws_executed": result.laws_executed,
        "integration_metrics": {
            "overall_quality_score": result.overall_quality_score,
            "consistency_score": result.consistency_score,
            "conflicts_detected": result.conflicts_detected,
            "recommendation_confidence": result.recommendation_confidence
        },
        "law_scores": result.law_scores,
        "conflicts": result.conflicts.iter().map(|c| {
            serde_json::json!({
                "type": format!("{:?}", c.conflict_type),
                "laws_involved": c.laws_involved,
                "conflict_score": c.conflict_score,
                "description": c.description,
                "likely_cause": c.likely_cause,
                "resolution_suggestion": c.resolution_suggestion
            })
        }).collect::<Vec<_>>(),
        "recommendations": {
            "primary_law": result.recommendations.primary_law,
            "secondary_laws": result.recommendations.secondary_laws,
            "confidence": result.recommendations.confidence,
            "rationale": result.recommendations.rationale
        },
        "overall_assessment": format!("{:?}", result.overall_assessment),
        "risk_level": format!("{:?}", result.risk_level)
    });
    
    writeln!(writer, "{}", serde_json::to_string_pretty(&json_value)?)?;
    Ok(())
}

fn output_integration_csv(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
) -> Result<()> {
    writeln!(writer, "dataset,numbers_analyzed,laws_executed,overall_quality_score,consistency_score,conflicts_detected,primary_law,overall_assessment,risk_level")?;
    writeln!(writer, "{},{},{},{:.3},{:.3},{},{},{:?},{:?}",
        result.dataset_name,
        result.numbers_analyzed,
        result.laws_executed.len(),
        result.overall_quality_score,
        result.consistency_score,
        result.conflicts_detected,
        result.recommendations.primary_law,
        result.overall_assessment,
        result.risk_level
    )?;
    Ok(())
}

fn output_integration_yaml(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
) -> Result<()> {
    writeln!(writer, "dataset: \"{}\"", result.dataset_name)?;
    writeln!(writer, "numbers_analyzed: {}", result.numbers_analyzed)?;
    writeln!(writer, "laws_executed:")?;
    for law in &result.laws_executed {
        writeln!(writer, "  - \"{}\"", law)?;
    }
    writeln!(writer, "integration_metrics:")?;
    writeln!(writer, "  overall_quality_score: {:.3}", result.overall_quality_score)?;
    writeln!(writer, "  consistency_score: {:.3}", result.consistency_score)?;
    writeln!(writer, "  conflicts_detected: {}", result.conflicts_detected)?;
    writeln!(writer, "law_scores:")?;
    for (law, score) in &result.law_scores {
        writeln!(writer, "  {}: {:.3}", law, score)?;
    }
    writeln!(writer, "recommendations:")?;
    writeln!(writer, "  primary_law: \"{}\"", result.recommendations.primary_law)?;
    writeln!(writer, "  confidence: {:.3}", result.recommendations.confidence)?;
    Ok(())
}

fn output_detailed_integration_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    config: &OutputConfig,
) -> Result<()> {
    output_integration_result(writer, result, config)?;
    
    if config.format == "text" {
        writeln!(writer)?;
        writeln!(writer, "=== {} ===", get_text("detailed_analysis", &config.language))?;
        
        output_detailed_law_results(writer, result, &config.language)?;
        output_data_characteristics(writer, result, &config.language)?;
        output_alternative_combinations(writer, result, &config.language)?;
    }
    
    Ok(())
}

fn output_conflict_analysis_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::ConflictAnalysisResult,
    config: &OutputConfig,
) -> Result<()> {
    let lang = &config.language;
    
    writeln!(writer, "{}", get_text("conflict_analysis_title", lang))?;
    writeln!(writer)?;
    writeln!(writer, "{}: {}", get_text("dataset", lang), result.dataset_name)?;
    writeln!(writer, "{}: {:.3}", get_text("threshold", lang), result.threshold)?;
    writeln!(writer, "{}: {:?}", get_text("conflict_severity", lang), result.conflict_severity)?;
    writeln!(writer)?;
    
    if !result.detailed_conflicts.is_empty() {
        writeln!(writer, "{}:", get_text("detailed_conflicts", lang))?;
        for (i, conflict) in result.detailed_conflicts.iter().enumerate() {
            writeln!(writer, "{}. {}", i + 1, conflict.base_conflict.description)?;
            writeln!(writer, "   {}: {:.3}", get_text("significance", lang), conflict.statistical_significance)?;
            writeln!(writer, "   {}: {:?}", get_text("impact", lang), conflict.impact_assessment)?;
            writeln!(writer, "   {}: {}", get_text("root_cause", lang), conflict.root_cause_analysis)?;
            writeln!(writer)?;
        }
    }
    
    if !result.resolution_strategies.is_empty() {
        writeln!(writer, "{}:", get_text("resolution_strategies", lang))?;
        for strategy in &result.resolution_strategies {
            writeln!(writer, "• {} ({:?})", strategy.strategy_name, strategy.priority)?;
            writeln!(writer, "  {}: {}", get_text("expected_outcome", lang), strategy.expected_outcome)?;
            writeln!(writer, "  {}: {:.3}", get_text("confidence", lang), strategy.confidence)?;
            writeln!(writer)?;
        }
    }
    
    Ok(())
}

fn output_cross_validation_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::CrossValidationResult,
    config: &OutputConfig,
) -> Result<()> {
    let lang = &config.language;
    
    writeln!(writer, "{}", get_text("cross_validation_title", lang))?;
    writeln!(writer)?;
    writeln!(writer, "{}: {}", get_text("dataset", lang), result.dataset_name)?;
    writeln!(writer, "{}: {:.3}", get_text("confidence_level", lang), result.confidence_level)?;
    writeln!(writer, "{}: {:.3}", get_text("overall_stability", lang), result.overall_stability)?;
    writeln!(writer, "{}: {:?}", get_text("stability_assessment", lang), result.stability_assessment)?;
    writeln!(writer)?;
    
    writeln!(writer, "{}:", get_text("validation_folds", lang))?;
    for fold in &result.validation_folds {
        writeln!(writer, "  {}: {:.3}", 
            format!("{} {}", get_text("fold", lang), fold.fold_number),
            fold.consistency_score
        )?;
    }
    
    Ok(())
}

fn output_consistency_check_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    threshold: f64,
    config: &OutputConfig,
) -> Result<()> {
    let lang = &config.language;
    
    writeln!(writer, "{}", get_text("consistency_check_title", lang))?;
    writeln!(writer)?;
    
    output_integration_result(writer, result, config)?;
    
    writeln!(writer)?;
    writeln!(writer, "=== {} ===", get_text("consistency_analysis", lang))?;
    writeln!(writer, "{}: {:.3}", get_text("threshold", lang), threshold)?;
    writeln!(writer, "{}: {:.3}", get_text("consistency_score", lang), result.consistency_score)?;
    
    if result.consistency_score >= threshold {
        writeln!(writer, "✅ {}", get_text("consistent_results", lang))?;
    } else {
        writeln!(writer, "⚠️ {}", get_text("inconsistent_results", lang))?;
    }
    
    Ok(())
}

fn output_recommendation_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::DetailedRecommendationResult,
    config: &OutputConfig,
) -> Result<()> {
    let lang = &config.language;
    
    writeln!(writer, "{}", get_text("recommendation_title", lang))?;
    writeln!(writer)?;
    writeln!(writer, "{}: {}", get_text("dataset", lang), result.dataset_name)?;
    writeln!(writer, "{}: {:?}", get_text("analysis_purpose", lang), result.analysis_purpose)?;
    writeln!(writer)?;
    
    writeln!(writer, "{}:", get_text("purpose_recommendations", lang))?;
    for rec in &result.purpose_specific_recommendations {
        writeln!(writer, "• {}: {}", 
            format!("{:?}", rec.purpose),
            rec.recommended_laws.join(", ")
        )?;
        writeln!(writer, "  {}: {}", get_text("rationale", lang), rec.rationale)?;
        writeln!(writer, "  {}: {:.3}", get_text("effectiveness", lang), rec.effectiveness)?;
        writeln!(writer)?;
    }
    
    if !result.combination_analysis.is_empty() {
        writeln!(writer, "{}:", get_text("combination_analysis", lang))?;
        for combo in result.combination_analysis.iter().take(3) {
            writeln!(writer, "• {}: {:.3}", 
                combo.laws.join(" + "), 
                combo.synergy_score
            )?;
        }
        writeln!(writer)?;
    }
    
    Ok(())
}

// 詳細出力ヘルパー関数

fn output_verbose_integration_details(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    lang: &str,
) -> Result<()> {
    writeln!(writer, "=== {} ===", get_text("detailed_metrics", lang))?;
    
    output_data_characteristics(writer, result, lang)?;
    
    if !result.recommendations.alternative_combinations.is_empty() {
        output_alternative_combinations(writer, result, lang)?;
    }
    
    Ok(())
}

fn output_detailed_law_results(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    lang: &str,
) -> Result<()> {
    writeln!(writer, "{}:", get_text("individual_law_results", lang))?;
    
    if let Some(ref benf_result) = result.benford_result {
        writeln!(writer, "• {}: {:.3} ({})", 
            get_law_name("benf", lang),
            1.0 - (benf_result.mean_absolute_deviation / 100.0),
            format!("{:?}", benf_result.risk_level)
        )?;
    }
    
    if let Some(ref pareto_result) = result.pareto_result {
        writeln!(writer, "• {}: {:.3} ({})", 
            get_law_name("pareto", lang),
            pareto_result.concentration_index,
            format!("{:?}", pareto_result.risk_level)
        )?;
    }
    
    if let Some(ref zipf_result) = result.zipf_result {
        writeln!(writer, "• {}: {:.3} ({})", 
            get_law_name("zipf", lang),
            zipf_result.distribution_quality,
            format!("{:?}", zipf_result.risk_level)
        )?;
    }
    
    if let Some(ref normal_result) = result.normal_result {
        writeln!(writer, "• {}: {:.3} ({})", 
            get_law_name("normal", lang),
            normal_result.normality_score,
            format!("{:?}", normal_result.risk_level)
        )?;
    }
    
    if let Some(ref poisson_result) = result.poisson_result {
        writeln!(writer, "• {}: {:.3} ({})", 
            get_law_name("poisson", lang),
            poisson_result.goodness_of_fit_score,
            format!("{:?}", poisson_result.risk_level)
        )?;
    }
    
    writeln!(writer)?;
    Ok(())
}

fn output_data_characteristics(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    lang: &str,
) -> Result<()> {
    let chars = &result.data_characteristics;
    
    writeln!(writer, "{}:", get_text("data_characteristics", lang))?;
    writeln!(writer, "  {}: {:?}", get_text("data_type", lang), chars.data_type)?;
    writeln!(writer, "  {}: {:?}", get_text("distribution_shape", lang), chars.distribution_shape)?;
    writeln!(writer, "  {}: {:?}", get_text("outlier_presence", lang), chars.outlier_presence)?;
    writeln!(writer, "  {}: {:?}", get_text("scale_range", lang), chars.scale_range)?;
    writeln!(writer, "  {}: {:?}", get_text("sample_size_category", lang), chars.sample_size_category)?;
    writeln!(writer)?;
    
    Ok(())
}

fn output_alternative_combinations(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    lang: &str,
) -> Result<()> {
    writeln!(writer, "{}:", get_text("alternative_combinations", lang))?;
    
    for combo in &result.recommendations.alternative_combinations {
        writeln!(writer, "• {} ({})", combo.purpose, combo.laws.join(" + "))?;
        writeln!(writer, "  {}: {:.3}", get_text("effectiveness", lang), combo.effectiveness_score)?;
        writeln!(writer, "  {}: {}", get_text("description", lang), combo.description)?;
        writeln!(writer)?;
    }
    
    Ok(())
}

// 多言語対応ヘルパー関数

fn get_text(key: &str, lang: &str) -> String {
    match (key, lang) {
        ("integration_title", "ja") => "統合分析結果".to_string(),
        ("integration_title", _) => "Integration Analysis Result".to_string(),
        ("dataset", "ja") => "データセット".to_string(),
        ("dataset", _) => "Dataset".to_string(),
        ("numbers_analyzed", "ja") => "解析した数値数".to_string(),
        ("numbers_analyzed", _) => "Numbers Analyzed".to_string(),
        ("laws_executed", "ja") => "実行法則".to_string(),
        ("laws_executed", _) => "Laws Executed".to_string(),
        ("integration_metrics", "ja") => "統合評価".to_string(),
        ("integration_metrics", _) => "Integration Metrics".to_string(),
        ("overall_quality", "ja") => "総合品質スコア".to_string(),
        ("overall_quality", _) => "Overall Quality Score".to_string(),
        ("consistency", "ja") => "一貫性スコア".to_string(),
        ("consistency", _) => "Consistency Score".to_string(),
        ("conflicts_detected", "ja") => "矛盾検出".to_string(),
        ("conflicts_detected", _) => "Conflicts Detected".to_string(),
        ("recommendation_confidence", "ja") => "推奨信頼度".to_string(),
        ("recommendation_confidence", _) => "Recommendation Confidence".to_string(),
        ("law_results", "ja") => "法則別結果".to_string(),
        ("law_results", _) => "Law Results".to_string(),
        ("conflicts", "ja") => "矛盾検出".to_string(),
        ("conflicts", _) => "Conflicts".to_string(),
        ("cause", "ja") => "推定原因".to_string(),
        ("cause", _) => "Likely Cause".to_string(),
        ("suggestion", "ja") => "推奨対策".to_string(),
        ("suggestion", _) => "Suggestion".to_string(),
        ("recommendations", "ja") => "推奨".to_string(),
        ("recommendations", _) => "Recommendations".to_string(),
        ("primary_law", "ja") => "主要法則".to_string(),
        ("primary_law", _) => "Primary Law".to_string(),
        ("secondary_laws", "ja") => "補助法則".to_string(),
        ("secondary_laws", _) => "Secondary Laws".to_string(),
        ("rationale", "ja") => "推奨理由".to_string(),
        ("rationale", _) => "Rationale".to_string(),
        _ => key.to_string(),
    }
}

fn get_law_name(law: &str, lang: &str) -> String {
    match (law, lang) {
        ("benf", "ja") => "ベンフォード法則".to_string(),
        ("benf", _) => "Benford's Law".to_string(),
        ("pareto", "ja") => "パレート法則".to_string(),
        ("pareto", _) => "Pareto Principle".to_string(),
        ("zipf", "ja") => "Zipf法則".to_string(),
        ("zipf", _) => "Zipf's Law".to_string(),
        ("normal", "ja") => "正規分布".to_string(),
        ("normal", _) => "Normal Distribution".to_string(),
        ("poisson", "ja") => "ポアソン分布".to_string(),
        ("poisson", _) => "Poisson Distribution".to_string(),
        _ => law.to_string(),
    }
}