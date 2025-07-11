use super::result::ZipfResult;
use crate::error::Result;
use std::collections::HashMap;

/// ジップの法則（Zipf's law）の分析を実行
pub fn analyze_zipf_distribution(frequencies: &[f64], dataset_name: &str) -> Result<ZipfResult> {
    ZipfResult::new(dataset_name.to_string(), frequencies)
}

/// テキストデータからZipf分析を実行
pub fn analyze_text_zipf(text: &str, dataset_name: &str) -> Result<ZipfResult> {
    let word_frequencies = extract_word_frequencies(text);
    let frequencies: Vec<f64> = word_frequencies
        .into_iter()
        .map(|(_, freq)| freq as f64)
        .collect();
    analyze_zipf_distribution(&frequencies, dataset_name)
}

/// 頻度データからZipf分析を実行
pub fn analyze_text_zipf_from_frequencies(
    frequencies: &[(String, usize)],
    dataset_name: &str,
) -> Result<ZipfResult> {
    let freq_values: Vec<f64> = frequencies.iter().map(|(_, freq)| *freq as f64).collect();
    analyze_zipf_distribution(&freq_values, dataset_name)
}

/// テキストから単語頻度を抽出
fn extract_word_frequencies(text: &str) -> Vec<(String, usize)> {
    let mut word_counts = HashMap::new();

    // 単語分割（日本語・英語・中国語対応）
    let words = tokenize_multilingual_text(text);

    for word in words {
        if !word.is_empty() && word.len() > 1 {
            *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }

    // 頻度順にソート
    let mut frequencies: Vec<(String, usize)> = word_counts.into_iter().collect();
    frequencies.sort_by(|a, b| b.1.cmp(&a.1));

    frequencies
}

/// 多言語テキストのトークン化
fn tokenize_multilingual_text(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for ch in text.chars() {
        match ch {
            // 英語・数字の処理
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                current_token.push(ch);
            }
            // 日本語文字の処理
            '\u{3040}'..='\u{309F}' |  // ひらがな
            '\u{30A0}'..='\u{30FF}' |  // カタカナ
            '\u{4E00}'..='\u{9FAF}' => { // 漢字
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                tokens.push(ch.to_string());
            }
            // 区切り文字
            ' ' | '\t' | '\n' | '\r' | ',' | '.' | '!' | '?' | ';' | ':' |
            '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' |
            '|' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '+' | '=' |
            '<' | '>' | '~' | '`' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

/// 数値データからZipf分析（頻度分布として扱う）
pub fn analyze_numeric_zipf(numbers: &[f64], dataset_name: &str) -> Result<ZipfResult> {
    // 数値を頻度として扱い、降順にソート
    let mut frequencies = numbers.to_vec();
    frequencies.sort_by(|a, b| b.partial_cmp(a).unwrap());

    // 負の値を除去
    frequencies.retain(|&x| x > 0.0);

    analyze_zipf_distribution(&frequencies, dataset_name)
}

/// 複数データセットの統合Zipf分析
pub fn analyze_combined_zipf(datasets: &[(&str, &[f64])]) -> Result<Vec<ZipfResult>> {
    let mut results = Vec::new();

    for (name, data) in datasets {
        let result = analyze_zipf_distribution(data, name)?;
        results.push(result);
    }

    Ok(results)
}

/// Zipf分布の品質評価
pub fn evaluate_zipf_quality(zipf_result: &ZipfResult) -> ZipfQualityReport {
    let mut quality_metrics = Vec::new();

    // 指数の理想値からの偏差
    let exponent_score = calculate_exponent_score(zipf_result.zipf_exponent);
    quality_metrics.push(QualityMetric {
        name: "Exponent Quality".to_string(),
        score: exponent_score,
        description: format!("指数値: {:.3} (理想値: 1.0)", zipf_result.zipf_exponent),
    });

    // 相関係数の評価
    let correlation_score = zipf_result.correlation_coefficient;
    quality_metrics.push(QualityMetric {
        name: "Correlation".to_string(),
        score: correlation_score,
        description: format!("相関係数: {correlation_score:.3}"),
    });

    // 全体品質スコア
    let overall_score = (exponent_score + correlation_score) / 2.0;

    ZipfQualityReport {
        overall_score,
        quality_metrics,
        compliance_level: determine_compliance_level(overall_score),
    }
}

/// 指数品質スコアを計算
fn calculate_exponent_score(exponent: f64) -> f64 {
    // 理想的なZipf指数は1.0
    let deviation = (exponent - 1.0).abs();

    // 偏差に基づくスコア計算
    if deviation <= 0.1 {
        1.0
    } else if deviation <= 0.3 {
        0.8
    } else if deviation <= 0.5 {
        0.6
    } else if deviation <= 0.7 {
        0.4
    } else if deviation <= 1.0 {
        0.2
    } else {
        0.0
    }
}

/// 遵守レベルを判定
fn determine_compliance_level(score: f64) -> String {
    if score >= 0.8 {
        "Excellent".to_string()
    } else if score >= 0.6 {
        "Good".to_string()
    } else if score >= 0.4 {
        "Fair".to_string()
    } else if score >= 0.2 {
        "Poor".to_string()
    } else {
        "Very Poor".to_string()
    }
}

/// 品質メトリック
#[derive(Debug, Clone)]
pub struct QualityMetric {
    pub name: String,
    pub score: f64,
    pub description: String,
}

/// Zipf品質レポート
#[derive(Debug, Clone)]
pub struct ZipfQualityReport {
    pub overall_score: f64,
    pub quality_metrics: Vec<QualityMetric>,
    pub compliance_level: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_zipf_analysis() {
        let text = "the quick brown fox jumps over the lazy dog the fox is quick";
        let result = analyze_text_zipf(text, "sample_text").unwrap();

        assert!(result.numbers_analyzed > 0);
        assert!(result.unique_items > 0);
        assert!(result.total_observations > 0);
    }

    #[test]
    fn test_numeric_zipf_analysis() {
        let numbers = vec![
            1000.0, 500.0, 333.0, 250.0, 200.0, 166.0, 142.0, 125.0, 111.0, 100.0,
        ];
        let result = analyze_numeric_zipf(&numbers, "numeric_test").unwrap();

        assert_eq!(result.numbers_analyzed, 10);
        assert!(result.zipf_exponent > 0.0);
    }

    #[test]
    fn test_multilingual_tokenization() {
        let text = "Hello 世界 测试 مرحبا";
        let tokens = tokenize_multilingual_text(text);

        assert!(!tokens.is_empty());
        assert!(tokens.contains(&"Hello".to_string()));
        assert!(tokens.contains(&"世".to_string()));
        assert!(tokens.contains(&"界".to_string()));
    }

    #[test]
    fn test_zipf_quality_evaluation() {
        let frequencies = vec![100.0, 50.0, 33.0, 25.0, 20.0];
        let result = analyze_zipf_distribution(&frequencies, "test").unwrap();
        let quality_report = evaluate_zipf_quality(&result);

        assert!(quality_report.overall_score >= 0.0);
        assert!(quality_report.overall_score <= 1.0);
    }

    #[test]
    fn test_combined_zipf_analysis() {
        let dataset1 = vec![100.0, 50.0, 33.0, 25.0, 20.0];
        let dataset2 = vec![200.0, 100.0, 66.0, 50.0, 40.0];
        let datasets = vec![
            ("dataset1", dataset1.as_slice()),
            ("dataset2", dataset2.as_slice()),
        ];

        let results = analyze_combined_zipf(&datasets).unwrap();
        assert_eq!(results.len(), 2);
    }
}
