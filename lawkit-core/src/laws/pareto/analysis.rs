use super::result::ParetoResult;
use crate::error::Result;

/// パレート法則（80/20原則）の分析を実行
pub fn analyze_pareto_distribution(numbers: &[f64], dataset_name: &str) -> Result<ParetoResult> {
    ParetoResult::new(dataset_name.to_string(), numbers)
}

/// ビジネスパレート分析を実行
pub fn analyze_business_pareto(
    numbers: &[f64],
    dataset_name: &str,
) -> Result<BusinessParetoAnalysis> {
    let pareto_result = analyze_pareto_distribution(numbers, dataset_name)?;

    let business_insights = generate_business_insights(&pareto_result);
    let action_recommendations = generate_action_recommendations(&pareto_result);

    Ok(BusinessParetoAnalysis {
        pareto_result,
        business_insights,
        action_recommendations,
    })
}

/// ビジネス洞察を生成
fn generate_business_insights(pareto_result: &ParetoResult) -> Vec<BusinessInsight> {
    let mut insights = Vec::new();

    // 80/20原則の適合度による洞察
    if pareto_result.pareto_ratio > 0.8 && pareto_result.pareto_ratio < 1.2 {
        insights.push(BusinessInsight {
            category: "Distribution".to_string(),
            message: "データは典型的なパレート分布を示しています".to_string(),
            impact_level: "High".to_string(),
        });
    }

    // 集中度による洞察
    if pareto_result.concentration_index > 0.6 {
        insights.push(BusinessInsight {
            category: "Concentration".to_string(),
            message: "高度な集中が見られます - 少数の要素が大きな影響を持っています".to_string(),
            impact_level: "Critical".to_string(),
        });
    }

    insights
}

/// アクション推奨を生成
fn generate_action_recommendations(pareto_result: &ParetoResult) -> Vec<ActionRecommendation> {
    let mut recommendations = Vec::new();

    // 上位20%の影響度に基づく推奨
    if pareto_result.top_20_percent_share > 80.0 {
        recommendations.push(ActionRecommendation {
            priority: "High".to_string(),
            action: "上位20%の要素に集中的にリソースを配分してください".to_string(),
            expected_impact: "効率的な成果向上が期待できます".to_string(),
        });
    }

    recommendations
}

/// ビジネス洞察
#[derive(Debug, Clone)]
pub struct BusinessInsight {
    pub category: String,
    pub message: String,
    pub impact_level: String,
}

/// アクション推奨
#[derive(Debug, Clone)]
pub struct ActionRecommendation {
    pub priority: String,
    pub action: String,
    pub expected_impact: String,
}

/// ビジネスパレート分析結果
#[derive(Debug, Clone)]
pub struct BusinessParetoAnalysis {
    pub pareto_result: ParetoResult,
    pub business_insights: Vec<BusinessInsight>,
    pub action_recommendations: Vec<ActionRecommendation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_pareto_distribution() {
        // 80/20分布に近いテストデータ
        let numbers = vec![
            100.0, 90.0, 80.0, 70.0, 60.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ];
        let result = analyze_pareto_distribution(&numbers, "test").unwrap();

        assert_eq!(result.numbers_analyzed, 15);
        assert!(result.top_20_percent_share > 50.0); // 上位20%がかなりの割合を占有
    }

    #[test]
    fn test_uniform_distribution() {
        // 均等分布（パレート原則に合わない）
        let numbers = vec![10.0; 20]; // 全て同じ値
        let result = analyze_pareto_distribution(&numbers, "uniform").unwrap();

        assert_eq!(result.numbers_analyzed, 20);
        assert!((result.top_20_percent_share - 20.0).abs() < 1.0); // 上位20%が約20%を占有
        assert!(matches!(
            result.risk_level,
            crate::common::risk::RiskLevel::Critical
        ));
    }

    #[test]
    fn test_business_analysis() {
        let numbers = vec![
            1000.0, 800.0, 600.0, 400.0, 200.0, 50.0, 40.0, 30.0, 20.0, 10.0,
        ];
        let result = analyze_pareto_distribution(&numbers, "sales").unwrap();

        assert_eq!(result.dataset_name, "sales");
        assert_eq!(result.numbers_analyzed, 10);
    }

    #[test]
    fn test_insufficient_data() {
        let numbers = vec![1.0, 2.0]; // 5個未満
        let result = analyze_pareto_distribution(&numbers, "test");

        assert!(result.is_err());
    }
}
