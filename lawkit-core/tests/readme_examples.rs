use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::pareto::ParetoAnalysis;
use lawkit_core::laws::zipf::ZipfAnalysis;
use lawkit_core::laws::normal::NormalAnalysis;
use lawkit_core::laws::poisson::PoissonAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;
use std::collections::HashMap;

fn create_test_data() -> Vec<f64> {
    vec![1234.0, 5678.0, 9012.0, 3456.0, 7890.0, 2345.0, 6789.0, 1023.0, 4567.0, 8901.0]
}

fn create_sales_data() -> Vec<f64> {
    vec![1000.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7000.0, 8000.0, 9000.0, 10000.0]
}

fn create_frequency_data() -> Vec<f64> {
    vec![1000.0, 500.0, 333.0, 250.0, 200.0, 166.0, 142.0, 125.0, 111.0, 100.0]
}

#[test]
fn test_benf_basic_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 1: Core library Benford Law analysis
    let data = create_test_data();
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.numbers_analyzed > 0);
    assert!(result.first_digit_distribution.len() == 9);
    
    Ok(())
}

#[test]
fn test_pareto_sales_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 2: Core library Pareto analysis
    let data = create_sales_data();
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.numbers_analyzed > 0);
    assert!(result.pareto_ratio >= 0.0 && result.pareto_ratio <= 1.0);
    
    Ok(())
}

#[test]
fn test_analyze_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 3: Core library multi-law integration
    let data = create_test_data();
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benf");
    analysis.add_law("pareto");
    analysis.add_law("zipf");
    let result = analysis.analyze(&data)?;
    
    assert!(result.laws_executed > 0);
    assert!(result.overall_quality_score >= 0.0);
    
    Ok(())
}

#[test] 
fn test_zipf_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 4: Core library Zipf law analysis
    let data = create_frequency_data();
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.numbers_analyzed > 0);
    assert!(result.zipf_coefficient.is_some());
    
    Ok(())
}

#[test]
fn test_normal_distribution() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 5: Core library normal distribution
    let data = vec![10.0, 12.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0, 26.0, 28.0];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.numbers_analyzed > 0);
    assert!(result.mean.is_some());
    assert!(result.std_dev.is_some());
    
    Ok(())
}

#[test]
fn test_poisson_distribution() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 6: Core library Poisson distribution
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.numbers_analyzed > 0);
    assert!(result.lambda.is_some());
    
    Ok(())
}

#[test]
fn test_generate_benford_data() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 7: Core library data generation
    let generator = lawkit_core::generate::benford::BenfordGenerator::new();
    let data = generator.generate(100)?;
    
    assert_eq!(data.len(), 100);
    assert!(data.iter().all(|&x| x > 0.0));
    
    Ok(())
}

#[test]
fn test_data_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 8: Core library data validation
    let data = create_test_data();
    let validator = lawkit_core::common::statistics::DataValidator::new();
    let is_valid = validator.validate(&data)?;
    
    assert!(is_valid.is_finite);
    assert!(is_valid.has_positive_values);
    
    Ok(())
}

#[test]
fn test_outlier_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 9: Core library outlier detection
    let mut data = create_test_data();
    data.push(999999.0); // Add outlier
    
    let detector = lawkit_core::common::outliers::OutlierDetector::new();
    let outliers = detector.detect(&data)?;
    
    assert!(outliers.len() > 0);
    
    Ok(())
}

#[test]
fn test_risk_assessment() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 10: Core library risk assessment
    let data = create_test_data();
    let risk_analyzer = lawkit_core::common::risk::RiskAnalyzer::new();
    let risk_level = risk_analyzer.assess(&data)?;
    
    assert!(risk_level.score >= 0.0 && risk_level.score <= 1.0);
    
    Ok(())
}

#[test]
fn test_statistical_tests() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 11: Core library statistical tests
    let data = create_test_data();
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.chi_square_statistic.is_some());
    assert!(result.p_value.is_some());
    
    Ok(())
}

#[test]
fn test_confidence_intervals() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 12: Core library confidence intervals
    let data = create_test_data();
    let stats = lawkit_core::common::statistics::ConfidenceInterval::calculate(&data, 0.95)?;
    
    assert!(stats.lower_bound < stats.upper_bound);
    assert!(stats.confidence_level == 0.95);
    
    Ok(())
}

#[test]
fn test_mean_absolute_deviation() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 13: Core library MAD calculation
    let data = create_test_data();
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.mean_absolute_deviation.is_some());
    assert!(result.mean_absolute_deviation.unwrap() >= 0.0);
    
    Ok(())
}

#[test]
fn test_distribution_fitting() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 14: Core library distribution fitting
    let data = create_test_data();
    let fitter = lawkit_core::common::statistics::DistributionFitter::new();
    let fit_result = fitter.fit_benford(&data)?;
    
    assert!(fit_result.goodness_of_fit >= 0.0);
    
    Ok(())
}

#[test]
fn test_cumulative_distribution() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 15: Core library cumulative distribution
    let data = create_sales_data();
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.cumulative_percentages.len() > 0);
    
    Ok(())
}

#[test]
fn test_lorenz_curve() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 16: Core library Lorenz curve calculation
    let data = create_sales_data();
    let curve = lawkit_core::common::statistics::LorenzCurve::calculate(&data)?;
    
    assert!(curve.points.len() > 0);
    assert!(curve.gini_coefficient >= 0.0 && curve.gini_coefficient <= 1.0);
    
    Ok(())
}

#[test]
fn test_quality_metrics() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 17: Core library quality score calculation
    let data = create_test_data();
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benf");
    analysis.add_law("pareto");
    let result = analysis.analyze(&data)?;
    
    assert!(result.overall_quality_score >= 0.0 && result.overall_quality_score <= 1.0);
    
    Ok(())
}

#[test]
fn test_consistency_score() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 18: Core library consistency measurement
    let data = create_test_data();
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benf");
    analysis.add_law("pareto");
    let result = analysis.analyze(&data)?;
    
    assert!(result.consistency_score >= 0.0 && result.consistency_score <= 1.0);
    
    Ok(())
}

#[test]
fn test_conflict_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 19: Core library conflict detection
    let data = create_test_data();
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benf");
    analysis.add_law("normal");
    let result = analysis.analyze(&data)?;
    
    assert!(result.conflicts_detected >= 0);
    
    Ok(())
}

#[test]
fn test_recommendation_confidence() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 20: Core library recommendation confidence
    let data = create_test_data();
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benf");
    let result = analysis.analyze(&data)?;
    
    assert!(result.recommendation_confidence >= 0.0 && result.recommendation_confidence <= 1.0);
    
    Ok(())
}

#[test]
fn test_parallel_processing() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 21: Core library parallel processing
    let data = (0..10000).map(|i| i as f64).collect();
    let processor = lawkit_core::common::parallel::ParallelProcessor::new();
    let chunks = processor.split_data(&data, 4)?;
    
    assert_eq!(chunks.len(), 4);
    
    Ok(())
}

#[test]
fn test_memory_optimization() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 22: Core library memory optimization
    let data = create_test_data();
    let optimizer = lawkit_core::common::memory::MemoryOptimizer::new();
    let optimized = optimizer.optimize_analysis(&data)?;
    
    assert!(optimized.memory_usage < data.len() * 8); // Less than raw data size
    
    Ok(())
}

#[test]
fn test_streaming_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 23: Core library streaming I/O
    let data = create_test_data();
    let mut stream = lawkit_core::common::streaming_io::DataStream::new();
    
    for value in data {
        stream.add_value(value)?;
    }
    
    let result = stream.finalize()?;
    assert!(result.count > 0);
    
    Ok(())
}

#[test]
fn test_time_series_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 24: Core library time series analysis
    let data = create_test_data();
    let timestamps: Vec<u64> = (0..data.len()).map(|i| i as u64).collect();
    
    let analyzer = lawkit_core::common::timeseries::TimeSeriesAnalyzer::new();
    let result = analyzer.analyze(&data, &timestamps)?;
    
    assert!(result.trend.is_some());
    
    Ok(())
}

#[test]
fn test_international_parsing() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 25: Core library international number parsing
    let japanese_numbers = vec!["１２３４", "５６７８", "９０１２"];
    let parser = lawkit_core::common::international::NumberParser::new();
    let parsed = parser.parse_japanese(&japanese_numbers)?;
    
    assert_eq!(parsed.len(), 3);
    assert!(parsed.iter().all(|&x| x > 0.0));
    
    Ok(())
}

#[test]
fn test_filtering_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 26: Core library data filtering
    let mut data = create_test_data();
    data.push(0.0); // Add zero value
    data.push(-100.0); // Add negative value
    
    let filter = lawkit_core::common::filtering::DataFilter::new();
    let filtered = filter.remove_invalid(&data)?;
    
    assert!(filtered.len() < data.len());
    assert!(filtered.iter().all(|&x| x > 0.0));
    
    Ok(())
}

#[test]
fn test_format_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 27: Core library format detection
    let csv_sample = "value\n123\n456\n789";
    let detector = lawkit_core::common::input::file_detector::FormatDetector::new();
    let format = detector.detect_format(csv_sample.as_bytes())?;
    
    assert_eq!(format, lawkit_core::common::input::file_detector::FileFormat::CSV);
    
    Ok(())
}

#[test]
fn test_output_formatting() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 28: Core library output formatting
    let data = create_test_data();
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    let formatter = lawkit_core::common::output::formatter::OutputFormatter::new();
    let json_output = formatter.format_as_json(&result)?;
    
    assert!(json_output.contains("{"));
    assert!(json_output.contains("}"));
    
    Ok(())
}

#[test]
fn test_benchmark_performance() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 29: Core library performance benchmarking
    let large_data: Vec<f64> = (1..=10000).map(|i| i as f64).collect();
    
    let start = std::time::Instant::now();
    let mut analysis = BenfordAnalysis::new();
    let _result = analysis.analyze(&large_data)?;
    let duration = start.elapsed();
    
    // Should complete within reasonable time (less than 1 second)
    assert!(duration.as_secs() < 1);
    
    Ok(())
}