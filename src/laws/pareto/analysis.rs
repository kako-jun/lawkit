use super::result::ParetoResult;
use crate::error::Result;

/// パレート法則（80/20原則）の分析を実行
pub fn analyze_pareto_distribution(numbers: &[f64], dataset_name: &str) -> Result<ParetoResult> {
    ParetoResult::new(dataset_name.to_string(), numbers)
}

/// ビジネス分析向けのパレート分析
pub fn analyze_business_pareto(numbers: &[f64], dataset_name: &str) -> Result<BusinessParetoAnalysis> {
    let result = ParetoResult::new(dataset_name.to_string(), numbers)?;
    
    Ok(BusinessParetoAnalysis {
        core_result: result,
        business_insights: generate_business_insights(&numbers),
        action_recommendations: generate_action_recommendations(&numbers),
    })
}

#[derive(Debug, Clone)]
pub struct BusinessParetoAnalysis {
    pub core_result: ParetoResult,
    pub business_insights: Vec<String>,
    pub action_recommendations: Vec<String>,
}

/// ビジネス向けの洞察を生成
fn generate_business_insights(numbers: &[f64]) -> Vec<String> {
    let mut insights = Vec::new();
    
    // データを降順にソート
    let mut sorted_numbers: Vec<f64> = numbers.iter().cloned().collect();
    sorted_numbers.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
    let total_sum: f64 = sorted_numbers.iter().sum();
    let top_20_percent_count = ((sorted_numbers.len() as f64) * 0.2).ceil() as usize;
    let top_20_percent_sum: f64 = sorted_numbers.iter().take(top_20_percent_count).sum();
    let top_20_percent_share = (top_20_percent_sum / total_sum) * 100.0;
    
    // パレート比率の評価
    if top_20_percent_share >= 75.0 && top_20_percent_share <= 85.0 {
        insights.push("理想的なパレート分布: 上位20%が約80%を占めています".to_string());
    } else if top_20_percent_share > 85.0 {
        insights.push(format!("高集中分布: 上位20%が{:.1}%を占める極端な集中状態", top_20_percent_share));
    } else {
        insights.push(format!("分散分布: 上位20%が{:.1}%のみ占める比較的均等な分布", top_20_percent_share));
    }
    
    // トップ10%の分析
    let top_10_percent_count = ((sorted_numbers.len() as f64) * 0.1).ceil() as usize;
    let top_10_percent_sum: f64 = sorted_numbers.iter().take(top_10_percent_count).sum();
    let top_10_percent_share = (top_10_percent_sum / total_sum) * 100.0;
    insights.push(format!("上位10%集中度: {:.1}%の価値を占有", top_10_percent_share));
    
    // 分布の特徴
    let largest_value = sorted_numbers[0];
    let median_index = sorted_numbers.len() / 2;
    let median_value = sorted_numbers[median_index];
    let ratio = largest_value / median_value;
    
    if ratio > 10.0 {
        insights.push("極端な格差: 最大値が中央値の10倍以上".to_string());
    } else if ratio > 5.0 {
        insights.push("顕著な格差: 最大値が中央値の5倍以上".to_string());
    } else {
        insights.push("比較的均等: 最大値と中央値の差が小さい".to_string());
    }
    
    insights
}

/// ビジネス向けのアクション推奨を生成
fn generate_action_recommendations(numbers: &[f64]) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    let mut sorted_numbers: Vec<f64> = numbers.iter().cloned().collect();
    sorted_numbers.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
    let total_sum: f64 = sorted_numbers.iter().sum();
    let top_20_percent_count = ((sorted_numbers.len() as f64) * 0.2).ceil() as usize;
    let top_20_percent_sum: f64 = sorted_numbers.iter().take(top_20_percent_count).sum();
    let top_20_percent_share = (top_20_percent_sum / total_sum) * 100.0;
    
    if top_20_percent_share >= 80.0 {
        recommendations.push("重点管理: 上位20%の要素に集中的にリソースを投入".to_string());
        recommendations.push("リスク管理: 上位要素への依存度が高いため代替案を検討".to_string());
    } else if top_20_percent_share < 60.0 {
        recommendations.push("効率化機会: より集中的な戦略でパフォーマンス向上の余地".to_string());
        recommendations.push("優先順位付け: 重要な要素を特定し集中投資を検討".to_string());
    } else {
        recommendations.push("バランス維持: 現在の分布バランスを維持しつつ最適化".to_string());
    }
    
    // 下位80%への言及
    let bottom_80_percent_share = 100.0 - top_20_percent_share;
    if bottom_80_percent_share < 25.0 {
        recommendations.push("下位要素見直し: 価値の低い80%の要素の削減や改善を検討".to_string());
    }
    
    recommendations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_pareto_distribution() {
        // 80/20分布に近いテストデータ
        let numbers = vec![100.0, 90.0, 80.0, 70.0, 60.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let result = analyze_pareto_distribution(&numbers, "test").unwrap();
        
        assert_eq!(result.numbers_analyzed, 15);
        assert!(result.top_20_percent_share > 70.0); // 上位20%が70%以上を占有
    }

    #[test]
    fn test_uniform_distribution() {
        // 均等分布（パレート原則に合わない）
        let numbers = vec![10.0; 20]; // 全て同じ値
        let result = analyze_pareto_distribution(&numbers, "uniform").unwrap();
        
        assert_eq!(result.numbers_analyzed, 20);
        assert!((result.top_20_percent_share - 20.0).abs() < 1.0); // 上位20%が約20%を占有
        assert!(matches!(result.risk_level, crate::common::risk::RiskLevel::Critical));
    }

    #[test]
    fn test_business_analysis() {
        let numbers = vec![1000.0, 800.0, 600.0, 400.0, 200.0, 50.0, 40.0, 30.0, 20.0, 10.0];
        let result = analyze_business_pareto(&numbers, "sales").unwrap();
        
        assert!(!result.business_insights.is_empty());
        assert!(!result.action_recommendations.is_empty());
    }

    #[test]
    fn test_insufficient_data() {
        let numbers = vec![1.0, 2.0]; // 5個未満
        let result = analyze_pareto_distribution(&numbers, "test");
        
        assert!(result.is_err());
    }
}