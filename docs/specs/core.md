# lawkit-core API 仕様書

## 概要

lawkit-core は統計法則分析のRustライブラリ。単一エントリポイント `law()` 関数を通じて、すべての分析機能にアクセス。

## インストール

```toml
[dependencies]
lawkit-core = "0.4"
```

## 単一エントリポイント

```rust
use lawkit_core::{law, LawkitOptions, LawkitResult};
use serde_json::json;

// 基本使用
let data = json!([1, 2, 3, 10, 20, 30, 100, 200, 300]);
let results = law("benf", &data, None)?;

// オプション付き
let options = LawkitOptions::default();
let results = law("pareto", &data, Some(&options))?;
```

## サブコマンド

| サブコマンド | 説明 |
|-------------|------|
| `benf` / `benford` | ベンフォードの法則分析 |
| `pareto` | パレートの法則分析 |
| `zipf` | ジップの法則分析 |
| `normal` | 正規分布分析 |
| `poisson` | ポアソン分布分析 |
| `analyze` | 複数法則の統合分析 |
| `validate` | データ検証 |
| `diagnose` | 詳細診断 |
| `generate` | サンプルデータ生成 |

## 戻り値型

```rust
pub enum LawkitResult {
    BenfordAnalysis(String, BenfordData),
    ParetoAnalysis(String, ParetoData),
    ZipfAnalysis(String, ZipfData),
    NormalAnalysis(String, NormalData),
    PoissonAnalysis(String, PoissonData),
    IntegrationAnalysis(String, IntegrationData),
    ValidationResult(String, ValidationData),
    DiagnosticResult(String, DiagnosticData),
    GeneratedData(String, GeneratedDataInfo),
}
```

## データ型

### BenfordData

```rust
pub struct BenfordData {
    pub observed_distribution: [f64; 9],  // 桁1-9の観測頻度
    pub expected_distribution: [f64; 9],  // 桁1-9の期待頻度
    pub chi_square: f64,                  // カイ二乗統計量
    pub p_value: f64,                     // p値
    pub mad: f64,                         // 平均絶対偏差
    pub risk_level: String,               // "LOW", "MEDIUM", "HIGH"
    pub total_numbers: usize,             // 分析した数値の数
    pub analysis_summary: String,         // 分析要約
}
```

### ParetoData

```rust
pub struct ParetoData {
    pub top_20_percent_contribution: f64, // 上位20%の貢献率 (%)
    pub pareto_ratio: f64,                // パレート比率 (80/20基準で1.0が理想)
    pub concentration_index: f64,         // 集中度指数
    pub risk_level: String,               // "LOW", "MEDIUM", "HIGH"
    pub total_items: usize,               // 総項目数
    pub analysis_summary: String,
}
```

### ZipfData

```rust
pub struct ZipfData {
    pub zipf_coefficient: f64,        // ジップ係数 (理想は-1.0)
    pub correlation_coefficient: f64, // 相関係数
    pub deviation_score: f64,         // 理想からの偏差
    pub risk_level: String,
    pub total_items: usize,
    pub analysis_summary: String,
}
```

### NormalData

```rust
pub struct NormalData {
    pub mean: f64,              // 平均
    pub std_dev: f64,           // 標準偏差
    pub skewness: f64,          // 歪度
    pub kurtosis: f64,          // 尖度
    pub normality_test_p: f64,  // 正規性検定のp値
    pub risk_level: String,
    pub total_numbers: usize,
    pub analysis_summary: String,
}
```

### PoissonData

```rust
pub struct PoissonData {
    pub lambda: f64,         // λ (平均発生率)
    pub variance_ratio: f64, // 分散/平均 (ポアソンなら1.0)
    pub poisson_test_p: f64, // ポアソン検定のp値
    pub risk_level: String,
    pub total_events: usize,
    pub analysis_summary: String,
}
```

### IntegrationData

```rust
pub struct IntegrationData {
    pub laws_analyzed: Vec<String>,      // 分析した法則のリスト
    pub overall_risk: String,            // 総合リスクレベル
    pub conflicting_results: Vec<String>,// 矛盾する結果
    pub recommendations: Vec<String>,    // 推奨事項
    pub analysis_summary: String,
}
```

### ValidationData

```rust
pub struct ValidationData {
    pub validation_passed: bool,     // 検証成功したか
    pub issues_found: Vec<String>,   // 発見された問題
    pub data_quality_score: f64,     // データ品質スコア (0.0-1.0)
    pub analysis_summary: String,
}
```

### DiagnosticData

```rust
pub struct DiagnosticData {
    pub diagnostic_type: String,     // 診断タイプ
    pub findings: Vec<String>,       // 診断結果
    pub confidence_level: f64,       // 信頼水準
    pub analysis_summary: String,
}
```

### GeneratedDataInfo

```rust
pub struct GeneratedDataInfo {
    pub data_type: String,               // "benford", "normal", "poisson"
    pub count: usize,                    // 生成数
    pub parameters: HashMap<String, f64>,// パラメータ
    pub sample_data: Vec<f64>,           // 生成データ
}
```

## オプション

### LawkitOptions

```rust
pub struct LawkitOptions {
    pub ignore_keys_regex: Option<Regex>,      // 無視するキーのパターン
    pub path_filter: Option<String>,           // パスフィルタ
    pub output_format: Option<OutputFormat>,   // 出力形式
    pub show_details: Option<bool>,            // 詳細表示
    pub show_recommendations: Option<bool>,    // 推奨表示
    pub use_memory_optimization: Option<bool>, // メモリ最適化
    pub batch_size: Option<usize>,             // バッチサイズ
    pub lawkit_options: Option<LawkitSpecificOptions>,
}
```

### LawkitSpecificOptions

```rust
pub struct LawkitSpecificOptions {
    // リスク評価
    pub risk_threshold: Option<String>,     // "low", "medium", "high"
    pub confidence_level: Option<f64>,      // 0.0-1.0
    pub analysis_threshold: Option<f64>,
    pub significance_level: Option<f64>,
    pub min_sample_size: Option<usize>,
    pub enable_outlier_detection: Option<bool>,

    // ベンフォード固有
    pub benford_digits: Option<String>,     // "first", "second", "both"
    pub benford_base: Option<u32>,          // デフォルト10

    // パレート固有
    pub pareto_ratio: Option<f64>,          // デフォルト0.8
    pub pareto_category_limit: Option<usize>,

    // ジップ固有
    pub zipf_rank_limit: Option<usize>,
    pub zipf_frequency_cutoff: Option<f64>,

    // 生成固有
    pub generate_count: Option<usize>,
    pub generate_range_min: Option<f64>,
    pub generate_range_max: Option<f64>,
    pub generate_seed: Option<u64>,

    // 国際化
    pub enable_japanese_numerals: Option<bool>,
    pub enable_international_numerals: Option<bool>,

    // パフォーマンス
    pub enable_parallel_processing: Option<bool>,
    pub memory_limit_mb: Option<usize>,
}
```

### OutputFormat

```rust
pub enum OutputFormat {
    Lawkit,  // デフォルト
    Json,
    Yaml,
    Csv,
    Text,
}
```

## 入力形式

`law()` 関数は `serde_json::Value` を受け取る。数値は再帰的に抽出される：

```rust
// 配列
let data = json!([1.0, 2.0, 3.0, 10.0, 20.0]);

// オブジェクト (全ての値から再帰的に数値を抽出)
let data = json!({"a": 1, "b": {"c": 2}, "d": [3, 4]});
// → [1.0, 2.0, 3.0, 4.0] が抽出される

// 文字列も数値としてパース試行
let data = json!(["1.5", "2.5", "invalid"]);
// → [1.5, 2.5] が抽出される (invalidは無視)

// ネストした配列
let data = json!([[1, 2], [3, 4]]);
// → [1.0, 2.0, 3.0, 4.0] が抽出される
```

**抽出ルール:**
- `Number` → そのままf64に変換
- `String` → f64パース試行、失敗時は無視
- `Array` → 各要素を再帰処理
- `Object` → 全ての値を再帰処理
- `null`, `bool` → 無視

## リスクレベル判定

各分析関数は統計的基準に基づいてリスクレベルを判定：

| 分析 | LOW | MEDIUM | HIGH |
|------|-----|--------|------|
| Benford | p > 0.1 | 0.05 < p ≤ 0.1 | p ≤ 0.05 |
| Pareto | 貢献率 < 60% | 60-95% | > 95% |
| Zipf | 偏差 < 0.2 | 0.2-0.8 | > 0.8 |
| Normal | 正規性OK | 軽度逸脱 | 明確な非正規 |
| Poisson | 分散比≈1.0 | 軽度逸脱 | 明確な非ポアソン |

## 使用例

### ベンフォード分析

```rust
use lawkit_core::{law, LawkitResult};
use serde_json::json;

let data = json!([123, 456, 789, 1234, 5678]);
let results = law("benf", &data, None)?;

for result in results {
    if let LawkitResult::BenfordAnalysis(_, data) = result {
        println!("Chi-square: {}", data.chi_square);
        println!("p-value: {}", data.p_value);
        println!("Risk: {}", data.risk_level);
    }
}
```

### 統合分析

```rust
let data = json!([1, 2, 3, 10, 20, 30, 100, 200, 300]);
let results = law("analyze", &data, None)?;

for result in results {
    match result {
        LawkitResult::IntegrationAnalysis(_, data) => {
            println!("Laws analyzed: {:?}", data.laws_analyzed);
            println!("Overall risk: {}", data.overall_risk);
        }
        _ => {}
    }
}
```

### データ生成

```rust
// ベンフォード分布
let config = json!({"type": "benford", "count": 1000});

// パレート分布 (alpha: 形状パラメータ、デフォルト1.16)
let config = json!({"type": "pareto", "count": 1000, "alpha": 1.5});

// ジップ分布 (s: 指数パラメータ、デフォルト1.0)
let config = json!({"type": "zipf", "count": 100, "s": 1.0});

// 正規分布
let config = json!({"type": "normal", "count": 1000, "mean": 0.0, "std_dev": 1.0});

// ポアソン分布
let config = json!({"type": "poisson", "count": 1000, "lambda": 5.0});

let results = law("generate", &config, None)?;
if let Some(LawkitResult::GeneratedData(_, info)) = results.first() {
    println!("Generated {} values", info.sample_data.len());
}
```

**サポートする型:**
| type | パラメータ | デフォルト |
|------|-----------|-----------|
| `benford` / `benf` | - | - |
| `pareto` | `alpha` | 1.16 |
| `zipf` | `s` | 1.0 |
| `normal` | `mean`, `std_dev` | 0.0, 1.0 |
| `poisson` | `lambda` | 5.0 |

## パーサー関数

構造化データの解析用：

```rust
pub use parsers::{
    parse_csv,   // CSV解析
    parse_ini,   // INI解析
    parse_json,  // JSON解析
    parse_toml,  // TOML解析
    parse_xml,   // XML解析
    parse_yaml,  // YAML解析
};
```

## エラー処理

すべての関数は `anyhow::Result` を返す：

```rust
use anyhow::Result;

fn analyze() -> Result<()> {
    let data = json!([]);
    match law("benf", &data, None) {
        Ok(results) => { /* 処理 */ }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
```

## 高度な機能 (lawkit-cli経由)

以下の機能はCLI (`lawkit-cli`) でのみ利用可能：

- **クロスバリデーション**: `validate --cross-validation`
- **外れ値検出**: `normal --outliers`
- **品質管理分析**: `normal --quality-control`
- **時系列分析**: `normal --enable-timeseries`

これらはlawkit-core内の `laws::integration` モジュールで実装されている。
