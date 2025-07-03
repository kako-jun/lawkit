# PARETO_ARCHITECTURE.md - パレート法則アーキテクチャ設計

## アーキテクチャ概要

パレート法則実装は、lawkit統合プラットフォームの第二の統計法則として設計されている。ベンフォード法則の実装パターンを踏襲しつつ、パレート分析特有の機能（Gini係数、累積分布分析）を追加実装。

## モジュール構造

```
src/laws/pareto/
├── mod.rs              # モジュール定義・公開API
├── result.rs           # ParetoResult構造体・メトリクス計算
└── analysis.rs         # 分析ロジック・ビジネス洞察生成

src/subcommands/
└── pareto.rs           # CLI実装・出力フォーマット
```

## 核心データ構造

### ParetoResult構造体
```rust
#[derive(Debug, Clone)]
pub struct ParetoResult {
    // 基本情報
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub risk_level: RiskLevel,
    
    // パレートメトリクス
    pub pareto_ratio: f64,              // パレート比率（0-1）
    pub concentration_index: f64,        // Gini係数（-1 to 1）
    pub top_20_percent_share: f64,       // 上位20%占有率（%）
    
    // 分布分析
    pub cumulative_distribution: Vec<f64>, // 累積分布データ
}
```

### 計算アルゴリズム

#### 1. パレート比率計算
```rust
// 上位20%が占める割合を計算
let top_20_percent_count = ((numbers.len() as f64) * 0.2).ceil() as usize;
let top_20_percent_sum: f64 = sorted_numbers.iter().take(top_20_percent_count).sum();
let pareto_ratio = top_20_percent_sum / total_sum;
```

#### 2. Gini係数計算（集中度指数）
```rust
// ローレンツ曲線からGini係数を算出
fn calculate_gini_coefficient(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let sum: f64 = values.iter().sum();
    
    let mut gini = 0.0;
    for (i, &value) in sorted_values.iter().enumerate() {
        gini += value * (2.0 * (i + 1) as f64 - n - 1.0);
    }
    
    gini / (n * sum)
}
```

#### 3. 累積分布曲線生成
```rust
// ローレンツ曲線データポイント生成
fn generate_cumulative_distribution(values: &[f64]) -> Vec<f64> {
    let total: f64 = values.iter().sum();
    let mut cumulative = 0.0;
    let mut distribution = Vec::new();
    
    for &value in values {
        cumulative += value;
        distribution.push(cumulative / total);
    }
    
    distribution
}
```

## リスク評価ロジック

### 判定基準
```rust
fn determine_risk_level(pareto_ratio: f64, concentration_index: f64) -> RiskLevel {
    match pareto_ratio {
        r if r >= 0.75 && r <= 0.85 => RiskLevel::Low,      // 理想的
        r if r >= 0.65 && r <= 0.90 => RiskLevel::Medium,   // 軽微偏差
        r if r >= 0.50 && r <= 0.95 => RiskLevel::High,     // 有意偏差
        _ => RiskLevel::Critical                             // 重大偏差
    }
}
```

### ビジネス洞察生成
```rust
pub struct BusinessParetoAnalysis {
    pub core_result: ParetoResult,
    pub business_insights: Vec<String>,        // ビジネス洞察
    pub action_recommendations: Vec<String>,   // アクション推奨
}

// 洞察生成ロジック
fn generate_business_insights(numbers: &[f64]) -> Vec<String> {
    let mut insights = Vec::new();
    
    // パレート比率評価
    if top_20_percent_share >= 75.0 && top_20_percent_share <= 85.0 {
        insights.push("理想的なパレート分布: 上位20%が約80%を占めています".to_string());
    } else if top_20_percent_share > 85.0 {
        insights.push("高集中分布: 極端な集中状態".to_string());
    }
    
    // 推奨アクション生成
    if top_20_percent_share >= 80.0 {
        insights.push("重点管理: 上位20%への集中投資を推奨".to_string());
        insights.push("リスク管理: 上位依存のリスクヘッジを検討".to_string());
    }
    
    insights
}
```

## CLI統合設計

### サブコマンド実装
```rust
// src/subcommands/pareto.rs
pub fn run(matches: &ArgMatches) -> Result<()> {
    // 1. 入力処理（共通基盤利用）
    let numbers = parse_input_with_filtering(matches)?;
    
    // 2. パレート分析実行
    let result = analyze_pareto_distribution(&numbers, dataset_name)?;
    
    // 3. 出力処理（フォーマット統一）
    output_results(&matches, &result);
    
    // 4. 終了コード（リスクレベル基準）
    std::process::exit(result.risk_level.exit_code());
}
```

### 多言語出力実装
```rust
fn localized_text(key: &str, lang: &str) -> &'static str {
    match (lang, key) {
        ("ja", "pareto_analysis_results") => "パレートの法則（80/20の法則）解析結果",
        ("en", "pareto_analysis_results") => "Pareto Principle (80/20 Rule) Analysis Results",
        ("zh", "pareto_analysis_results") => "帕累托法则（80/20定律）分析结果",
        // ... 他言語対応
    }
}
```

## 性能設計

### 計算複雑度
- **ソート処理**: O(n log n) - Rustの標準ソート使用
- **Gini係数**: O(n) - 一回のループで計算
- **累積分布**: O(n) - 線形スキャン
- **総合**: O(n log n) - ソートが支配的

### メモリ使用量
- **入力データ**: Vec<f64> - 8 * n bytes
- **ソート済み**: Vec<f64> - 8 * n bytes (一時的)
- **累積分布**: Vec<f64> - 8 * n bytes
- **総メモリ**: 約24 * n bytes + 固定コスト

### 最適化手法
- **早期リターン**: データ不足時の即座終了
- **インプレースソート**: 可能な場合のメモリ節約
- **遅延評価**: 累積分布は要求時のみ計算

## エラーハンドリング

### エラー分類
```rust
// 共通エラー型使用
use crate::error::{BenfError, Result};

// パレート固有エラー
pub enum ParetoError {
    InsufficientData(usize),        // データ不足
    InvalidDistribution,            // 無効な分布
    CalculationError(String),       // 計算エラー
}
```

### エラー処理戦略
- **データ不足**: 5個未満は分析不可
- **負の値**: 自動除外・警告表示
- **ゼロ値**: 除外・統計に含めない
- **NaN/Inf**: エラー終了

## テスト設計

### 単体テスト
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_perfect_pareto_distribution() {
        // 理想的な80/20分布のテスト
        let numbers = vec![100.0, 90.0, 80.0, 70.0, 60.0, 10.0, 9.0, 8.0, 7.0, 6.0];
        let result = analyze_pareto_distribution(&numbers, "test").unwrap();
        assert!(result.top_20_percent_share > 75.0);
        assert!(matches!(result.risk_level, RiskLevel::Low));
    }
    
    #[test]
    fn test_uniform_distribution() {
        // 均等分布（パレート原則に合わない）のテスト
        let numbers = vec![10.0; 20];
        let result = analyze_pareto_distribution(&numbers, "uniform").unwrap();
        assert!((result.top_20_percent_share - 20.0).abs() < 1.0);
        assert!(matches!(result.risk_level, RiskLevel::Critical));
    }
}
```

### 統合テスト
- CLI引数の組み合わせテスト
- 出力フォーマット検証
- 多言語出力確認
- エラーケース処理

## 品質保証

### コード品質
- **Rustfmt**: 自動フォーマット適用
- **Clippy**: 静的解析・パフォーマンス改善
- **型安全性**: 強い型システム活用
- **エラーハンドリング**: Result型の一貫使用

### ドキュメント
- **Rustdoc**: API仕様の自動生成
- **使用例**: 実践的なサンプルコード
- **アルゴリズム**: 数学的背景の説明

## 将来拡張計画

### 追加メトリクス
- **80/20比率**: カスタマイズ可能な比率設定
- **多段階分析**: 10/50/90パーセンタイル分析
- **時系列分析**: パレート分布の時間変化

### ビジネス機能
- **業界ベンチマーク**: 業界標準との比較
- **予測分析**: 分布変化の予測
- **最適化提案**: 具体的改善案の生成

### 技術拡張
- **並列処理**: 大規模データ対応
- **ストリーミング**: リアルタイム分析
- **可視化**: グラフ出力機能