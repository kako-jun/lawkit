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
    let frequencies: Vec<f64> = word_frequencies.into_iter().map(|(_, freq)| freq as f64).collect();
    analyze_zipf_distribution(&frequencies, dataset_name)
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

/// 複数データソースからの統合Zipf分析
pub fn analyze_combined_zipf(datasets: &[(&str, &[f64])]) -> Result<Vec<ZipfResult>> {
    let mut results = Vec::new();
    
    for (name, data) in datasets {
        let result = analyze_zipf_distribution(data, name)?;
        results.push(result);
    }
    
    Ok(results)
}

/// Zipf分析の品質評価
pub fn evaluate_zipf_quality(result: &ZipfResult) -> ZipfQualityReport {
    let exponent_quality = evaluate_exponent_quality(result.zipf_exponent);
    let correlation_quality = evaluate_correlation_quality(result.correlation_coefficient);
    let distribution_quality = evaluate_distribution_quality(result.distribution_quality);
    
    ZipfQualityReport {
        overall_score: (exponent_quality + correlation_quality + distribution_quality) / 3.0,
        exponent_quality,
        correlation_quality,
        distribution_quality,
        recommendations: generate_recommendations(result),
    }
}

#[derive(Debug, Clone)]
pub struct ZipfQualityReport {
    pub overall_score: f64,
    pub exponent_quality: f64,
    pub correlation_quality: f64,
    pub distribution_quality: f64,
    pub recommendations: Vec<String>,
}

fn evaluate_exponent_quality(exponent: f64) -> f64 {
    // 理想的なZipf指数は1.0
    let deviation = (exponent - 1.0).abs();
    if deviation < 0.1 {
        1.0
    } else if deviation < 0.3 {
        0.8
    } else if deviation < 0.5 {
        0.6
    } else if deviation < 0.8 {
        0.4
    } else {
        0.2
    }
}

fn evaluate_correlation_quality(correlation: f64) -> f64 {
    correlation.abs()
}

fn evaluate_distribution_quality(quality: f64) -> f64 {
    quality
}

fn generate_recommendations(result: &ZipfResult) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    // Zipf指数に基づく推奨
    if result.zipf_exponent > 1.5 {
        recommendations.push("急激な集中: 上位項目への過度な集中が見られます".to_string());
    } else if result.zipf_exponent < 0.5 {
        recommendations.push("分散傾向: より集中的な分布が期待されます".to_string());
    }
    
    // 相関係数に基づく推奨
    if result.correlation_coefficient < 0.5 {
        recommendations.push("Zipf法則からの逸脱: 分布パターンの見直しが必要です".to_string());
    }
    
    // 多様性指数に基づく推奨
    if result.diversity_index < 1.0 {
        recommendations.push("低多様性: 項目の多様性向上を検討してください".to_string());
    } else if result.diversity_index > 4.0 {
        recommendations.push("高多様性: 重要項目への集中を検討してください".to_string());
    }
    
    recommendations
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
        let numbers = vec![1000.0, 500.0, 333.0, 250.0, 200.0, 166.0, 142.0, 125.0, 111.0, 100.0];
        let result = analyze_numeric_zipf(&numbers, "numeric_test").unwrap();
        
        assert_eq!(result.numbers_analyzed, 10);
        assert!(result.zipf_exponent > 0.0);
    }

    #[test]
    fn test_multilingual_tokenization() {
        let text = "Hello 世界 测试 مرحبا";
        let tokens = tokenize_multilingual_text(text);
        
        assert!(tokens.len() > 0);
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
        let datasets = vec![("dataset1", dataset1.as_slice()), ("dataset2", dataset2.as_slice())];
        
        let results = analyze_combined_zipf(&datasets).unwrap();
        assert_eq!(results.len(), 2);
    }
}