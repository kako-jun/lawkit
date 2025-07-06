use crate::common::risk::RiskLevel;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ParetoResult {
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub pareto_ratio: f64,                           // 80/20比率の実際値
    pub concentration_index: f64,                    // 集中度指標（ジニ係数）
    pub top_20_percent_share: f64,                   // 上位20%が占める割合
    pub cumulative_distribution: Vec<(f64, f64)>,    // ローレンツ曲線用データ
    pub custom_percentiles: Option<Vec<(f64, f64)>>, // カスタムパーセンタイル (パーセンタイル, シェア)
    pub risk_level: RiskLevel,
}

impl ParetoResult {
    pub fn new(dataset_name: String, numbers: &[f64]) -> Result<Self> {
        if numbers.is_empty() {
            return Err(crate::error::BenfError::NoNumbersFound);
        }

        if numbers.len() < 5 {
            return Err(crate::error::BenfError::InsufficientData(numbers.len()));
        }

        // データを降順にソート
        let mut sorted_numbers: Vec<f64> = numbers.to_vec();
        sorted_numbers.sort_by(|a, b| b.partial_cmp(a).unwrap());

        // 80/20原則の検証
        let total_sum: f64 = sorted_numbers.iter().sum();
        let top_20_percent_count = ((sorted_numbers.len() as f64) * 0.2).ceil() as usize;
        let top_20_percent_sum: f64 = sorted_numbers.iter().take(top_20_percent_count).sum();
        let top_20_percent_share = (top_20_percent_sum / total_sum) * 100.0;

        // ジニ係数計算（集中度指標）
        let concentration_index = calculate_gini_coefficient(&sorted_numbers);

        // ローレンツ曲線データ生成
        let cumulative_distribution = generate_lorenz_curve(&sorted_numbers);

        // パレート比率（実際の80/20からの偏差）
        let pareto_ratio = top_20_percent_share / 80.0; // 1.0に近いほど理想的

        // リスクレベル評価
        let risk_level = evaluate_pareto_risk(pareto_ratio, concentration_index);

        Ok(ParetoResult {
            dataset_name,
            numbers_analyzed: numbers.len(),
            pareto_ratio,
            concentration_index,
            top_20_percent_share,
            cumulative_distribution,
            custom_percentiles: None,
            risk_level,
        })
    }

    /// カスタムパーセンタイルを計算してセット
    pub fn with_custom_percentiles(mut self, percentiles: &[f64], numbers: &[f64]) -> Self {
        let mut sorted_numbers: Vec<f64> = numbers.to_vec();
        sorted_numbers.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let total_sum: f64 = sorted_numbers.iter().sum();
        let custom_percentiles = percentiles
            .iter()
            .map(|&p| {
                let top_percent_count =
                    ((sorted_numbers.len() as f64) * (p / 100.0)).ceil() as usize;
                let top_percent_sum: f64 = sorted_numbers.iter().take(top_percent_count).sum();
                let top_percent_share = (top_percent_sum / total_sum) * 100.0;
                (p, top_percent_share)
            })
            .collect();

        self.custom_percentiles = Some(custom_percentiles);
        self
    }
}

/// ジニ係数を計算（0=完全平等、1=完全不平等）
fn calculate_gini_coefficient(sorted_numbers: &[f64]) -> f64 {
    let n = sorted_numbers.len() as f64;
    let sum: f64 = sorted_numbers.iter().sum();

    if sum == 0.0 {
        return 0.0;
    }

    let mut gini_sum = 0.0;
    for (i, &value) in sorted_numbers.iter().enumerate() {
        gini_sum += (2.0 * (i as f64 + 1.0) - n - 1.0) * value;
    }

    gini_sum / (n * sum)
}

/// ローレンツ曲線のデータポイントを生成
fn generate_lorenz_curve(sorted_numbers: &[f64]) -> Vec<(f64, f64)> {
    let total_sum: f64 = sorted_numbers.iter().sum();
    #[allow(unused_assignments)]
    let mut cumulative_population = 0.0;
    let mut cumulative_wealth = 0.0;
    let mut curve_points = Vec::new();

    // 起点(0,0)を追加
    curve_points.push((0.0, 0.0));

    for (i, &value) in sorted_numbers.iter().enumerate() {
        cumulative_population = ((i + 1) as f64) / (sorted_numbers.len() as f64);
        cumulative_wealth += value / total_sum;
        curve_points.push((cumulative_population, cumulative_wealth));
    }

    curve_points
}

/// パレート分析に基づくリスクレベル評価
fn evaluate_pareto_risk(pareto_ratio: f64, gini_coefficient: f64) -> RiskLevel {
    // パレート比率が1.0（80/20）に近く、ジニ係数が適度なら低リスク
    let pareto_deviation = (pareto_ratio - 1.0).abs();

    match (pareto_deviation, gini_coefficient) {
        (dev, gini) if dev <= 0.1 && gini <= 0.4 => RiskLevel::Low, // 理想的なパレート分布
        (dev, gini) if dev <= 0.2 && gini <= 0.6 => RiskLevel::Medium, // 軽微な偏差
        (dev, gini) if dev <= 0.4 && gini <= 0.8 => RiskLevel::High, // 有意な偏差
        _ => RiskLevel::Critical,                                   // パレート原則から大きく逸脱
    }
}
