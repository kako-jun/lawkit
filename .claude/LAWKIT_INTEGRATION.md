# LAWKIT_INTEGRATION.md - 法則間統合仕様

## 統合プラットフォーム概要

lawkitは複数の統計法則を統一したCLIプラットフォームとして設計されている。各法則は独立性を保ちつつ、共通基盤・統一インターフェース・相互比較機能を提供する。

## 統合アーキテクチャ

### 階層構造
```
lawkit/
├── 共通基盤 (src/common/)          # 全法則共通機能
│   ├── input/                      # 入力処理・ファイル解析
│   ├── output/                     # 出力フォーマット統一
│   ├── risk.rs                     # リスク評価共通基準
│   └── filtering.rs                # データフィルタリング
├── 統計法則 (src/laws/)            # 各法則実装
│   ├── benford/                    # ベンフォード法則
│   ├── pareto/                     # パレート法則
│   └── zipf/                       # Zipf法則
├── サブコマンド (src/subcommands/)  # CLI実装
│   ├── benf.rs                     # ベンフォード CLI
│   ├── pareto.rs                   # パレート CLI
│   └── zipf.rs                     # Zipf CLI
└── 統合機能 (将来実装)
    ├── compare/                    # 法則間比較
    ├── combined/                   # 組み合わせ分析
    └── report/                     # 統合レポート生成
```

## 共通基盤設計

### 統一型システム
```rust
// 共通リスク評価
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,        // 理想的・正常
    Medium,     // 軽微な偏差・要監視
    High,       // 有意な偏差・対策推奨
    Critical,   // 重大な偏差・緊急対応
}

impl RiskLevel {
    pub fn exit_code(&self) -> i32 {
        match self {
            RiskLevel::Low => 0,
            RiskLevel::Medium => 1,
            RiskLevel::High => 2,
            RiskLevel::Critical => 3,
        }
    }
}

// 共通エラー型
pub enum BenfError {
    ParseError(String),
    InsufficientData(usize),
    InvalidInput(String),
    NetworkError(String),
    IoError(String),
}
```

### 統一入力処理
```rust
// 共通入力インターフェース
pub trait DataInput {
    fn parse_input_auto(input: &str) -> Result<Vec<f64>>;
    fn parse_text_input(text: &str) -> Result<Vec<f64>>;
    fn parse_file_input(path: &str) -> Result<Vec<f64>>;
}

// 多言語数字対応
pub fn extract_numbers_multilingual(text: &str) -> Vec<f64> {
    // 日本語数字・中国語金融数字・アラビア数字等を統一処理
}

// 統一フィルタリング
pub struct NumberFilter {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub range: Option<(f64, f64)>,
}
```

### 統一出力システム
```rust
// 全法則共通出力フォーマット
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Yaml,
    Toml,
    Xml,
}

// 多言語対応基盤
pub trait Localizable {
    fn localized_text(key: &str, lang: &str) -> &'static str;
    fn get_language(matches: &ArgMatches) -> &str;
}

// 統一CLI引数
pub trait CommonCliArgs {
    fn add_common_args(command: Command) -> Command {
        command
            .arg(format_arg())
            .arg(quiet_arg())
            .arg(verbose_arg())
            .arg(lang_arg())
            .arg(filter_arg())
            .arg(min_count_arg())
    }
}
```

## 法則別実装パターン

### Result構造体統一パターン
```rust
// 各法則のResult構造体は共通フィールドを持つ
pub trait StatisticalResult {
    fn dataset_name(&self) -> &str;
    fn numbers_analyzed(&self) -> usize;
    fn risk_level(&self) -> &RiskLevel;
    fn to_json(&self) -> serde_json::Value;
    fn to_csv_header() -> &'static str;
    fn to_csv_row(&self) -> String;
}

// 実装例
impl StatisticalResult for BenfordResult {
    fn dataset_name(&self) -> &str { &self.dataset_name }
    fn numbers_analyzed(&self) -> usize { self.numbers_analyzed }
    fn risk_level(&self) -> &RiskLevel { &self.risk_level }
    // ...
}
```

### 分析API統一パターン
```rust
// 各法則の主要分析関数は統一署名
pub trait StatisticalAnalysis {
    type Result: StatisticalResult;
    
    fn analyze(numbers: &[f64], dataset_name: &str) -> Result<Self::Result>;
    fn analyze_with_options(
        numbers: &[f64], 
        dataset_name: &str, 
        options: &AnalysisOptions
    ) -> Result<Self::Result>;
}

// 実装例
impl StatisticalAnalysis for BenfordAnalysis {
    type Result = BenfordResult;
    
    fn analyze(numbers: &[f64], dataset_name: &str) -> Result<BenfordResult> {
        BenfordResult::new(dataset_name.to_string(), numbers)
    }
}
```

## 法則間比較仕様

### 比較メトリクス
```rust
pub struct LawComparisonResult {
    pub dataset_name: String,
    pub benford_result: Option<BenfordResult>,
    pub pareto_result: Option<ParetoResult>,
    pub zipf_result: Option<ZipfResult>,
    
    // 比較メトリクス
    pub data_quality_score: f64,      // ベンフォード基準
    pub concentration_score: f64,      // パレート基準
    pub distribution_score: f64,       // Zipf基準
    
    // 統合評価
    pub overall_assessment: OverallAssessment,
    pub recommendations: Vec<String>,
}

pub enum OverallAssessment {
    HighQuality,        // 全法則で良好
    Mixed,              // 法則間で評価が分かれる
    Suspicious,         // 複数法則で問題検出
    Anomalous,          // 重大な異常パターン
}
```

### 統合分析ロジック
```rust
pub fn analyze_all_laws(numbers: &[f64], dataset_name: &str) -> LawComparisonResult {
    let benford_result = analyze_benford_law(numbers, dataset_name).ok();
    let pareto_result = analyze_pareto_distribution(numbers, dataset_name).ok();
    let zipf_result = analyze_numeric_zipf(numbers, dataset_name).ok();
    
    // 統合評価ロジック
    let overall_assessment = determine_overall_assessment(
        &benford_result,
        &pareto_result,
        &zipf_result
    );
    
    LawComparisonResult {
        dataset_name: dataset_name.to_string(),
        benford_result,
        pareto_result,
        zipf_result,
        overall_assessment,
        recommendations: generate_cross_law_recommendations(&benford_result, &pareto_result, &zipf_result),
    }
}
```

## CLI統合設計

### 統一コマンド構造
```rust
fn main() {
    let app = command!()
        .name("lawkit")
        .about("Statistical law analysis toolkit")
        .version(VERSION)
        .subcommand(benford_command())     // ベンフォード法則
        .subcommand(pareto_command())      // パレート法則
        .subcommand(zipf_command())        // Zipf法則
        .subcommand(compare_command())     // 比較分析（将来実装）
        .subcommand(list_command());       // 法則一覧
    
    match app.get_matches().subcommand() {
        Some(("benf", sub_matches)) => subcommands::benf::run(sub_matches),
        Some(("pareto", sub_matches)) => subcommands::pareto::run(sub_matches),
        Some(("zipf", sub_matches)) => subcommands::zipf::run(sub_matches),
        Some(("compare", sub_matches)) => subcommands::compare::run(sub_matches),
        Some(("list", _)) => list_all_laws(),
        _ => show_unified_help(),
    }
}
```

### 統一ヘルプシステム
```rust
fn list_all_laws() -> Result<(), LawkitError> {
    println!("Available statistical laws:");
    println!("  benf    - Benford's law analysis (fraud detection, data quality)");
    println!("  pareto  - Pareto principle (80/20 rule, concentration analysis)");
    println!("  zipf    - Zipf's law analysis (frequency distribution, text analysis)");
    println!();
    println!("Integration commands:");
    println!("  compare - Compare multiple laws on the same dataset");
    println!("  list    - Show this help");
    Ok(())
}
```

## 多言語統合対応

### 言語検出・統一
```rust
pub fn detect_language_unified(matches: &ArgMatches) -> &str {
    match matches.get_one::<String>("lang").map(|s| s.as_str()) {
        Some("auto") | None => {
            let lang = std::env::var("LANG").unwrap_or_default();
            match lang.as_str() {
                l if l.starts_with("ja") => "ja",
                l if l.starts_with("zh") => "zh",
                l if l.starts_with("hi") => "hi",
                l if l.starts_with("ar") => "ar",
                _ => "en",
            }
        },
        Some(lang) => match lang {
            "en" | "ja" | "zh" | "hi" | "ar" => lang,
            _ => "en",
        }
    }
}
```

### 統一メッセージシステム
```rust
pub struct MessageCatalog;

impl MessageCatalog {
    pub fn get(key: &str, lang: &str, law: Option<&str>) -> &'static str {
        // 法則固有メッセージ優先、フォールバック機構
        match law {
            Some("benf") => benford_messages::get(key, lang)
                .unwrap_or_else(|| common_messages::get(key, lang)),
            Some("pareto") => pareto_messages::get(key, lang)
                .unwrap_or_else(|| common_messages::get(key, lang)),
            Some("zipf") => zipf_messages::get(key, lang)
                .unwrap_or_else(|| common_messages::get(key, lang)),
            None => common_messages::get(key, lang),
        }
    }
}
```

## データ互換性設計

### 統一データ交換フォーマット
```rust
// 全法則間で共通のデータ交換フォーマット
#[derive(Serialize, Deserialize)]
pub struct UnifiedAnalysisResult {
    pub metadata: AnalysisMetadata,
    pub benford: Option<BenfordResult>,
    pub pareto: Option<ParetoResult>,
    pub zipf: Option<ZipfResult>,
    pub comparison: Option<ComparisonResult>,
}

#[derive(Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub dataset_name: String,
    pub timestamp: String,
    pub lawkit_version: String,
    pub input_source: String,
    pub analysis_options: HashMap<String, String>,
}
```

### 外部ツール連携
```rust
// 他の統計ツールとの連携インターフェース
pub trait ExternalIntegration {
    fn to_r_format(&self) -> String;
    fn to_python_dict(&self) -> String;
    fn to_excel_compatible(&self) -> Vec<Vec<String>>;
    fn to_tableau_extract(&self) -> String;
}
```

## パフォーマンス統合最適化

### 共通計算の最適化
```rust
// 複数法則で共通する計算を最適化
pub struct CommonCalculations {
    pub sorted_numbers: Vec<f64>,
    pub total_sum: f64,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
}

impl CommonCalculations {
    pub fn from_numbers(numbers: &[f64]) -> Self {
        let mut sorted = numbers.to_vec();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        
        let total_sum: f64 = sorted.iter().sum();
        let mean = total_sum / sorted.len() as f64;
        
        Self {
            sorted_numbers: sorted,
            total_sum,
            mean,
            median: calculate_median(&sorted),
            std_dev: calculate_std_dev(&sorted, mean),
        }
    }
}
```

### メモリ効率統合
```rust
// 大規模データでのメモリ効率化
pub struct MemoryEfficientAnalysis {
    sample_size: usize,
    streaming_threshold: usize,
}

impl MemoryEfficientAnalysis {
    pub fn analyze_large_dataset(
        &self,
        data_source: &mut dyn Iterator<Item = f64>
    ) -> Result<UnifiedAnalysisResult> {
        if self.should_use_streaming() {
            self.streaming_analysis(data_source)
        } else {
            self.full_memory_analysis(data_source)
        }
    }
}
```

## 拡張性設計

### 新法則追加インターフェース
```rust
// 新しい統計法則を追加するためのトレイト
pub trait StatisticalLaw {
    type Config;
    type Result: StatisticalResult;
    
    fn name() -> &'static str;
    fn description() -> &'static str;
    fn min_data_points() -> usize;
    
    fn analyze(
        numbers: &[f64], 
        dataset_name: &str, 
        config: Self::Config
    ) -> Result<Self::Result>;
    
    fn cli_command() -> Command;
    fn run_cli(matches: &ArgMatches) -> Result<()>;
}

// 実装例（将来のPoisson分布）
pub struct PoissonLaw;

impl StatisticalLaw for PoissonLaw {
    type Config = PoissonConfig;
    type Result = PoissonResult;
    
    fn name() -> &'static str { "poisson" }
    fn description() -> &'static str { "Poisson distribution analysis" }
    fn min_data_points() -> usize { 10 }
    
    // ... 実装
}
```

### プラグインシステム
```rust
// 動的プラグイン読み込み（将来実装）
pub trait LawkitPlugin {
    fn plugin_name(&self) -> &str;
    fn plugin_version(&self) -> &str;
    fn register_laws(&self) -> Vec<Box<dyn StatisticalLaw>>;
    fn register_commands(&self) -> Vec<Command>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn LawkitPlugin>>,
}
```

## 品質保証統合

### 統合テスト戦略
```rust
#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_all_laws_consistency() {
        // 同一データに対して全法則が一貫した品質評価を行うかテスト
        let test_data = generate_test_dataset();
        
        let benford = analyze_benford_law(&test_data, "test").unwrap();
        let pareto = analyze_pareto_distribution(&test_data, "test").unwrap();
        let zipf = analyze_numeric_zipf(&test_data, "test").unwrap();
        
        // リスク評価の一貫性チェック
        assert_risk_consistency(&benford.risk_level, &pareto.risk_level, &zipf.risk_level);
    }
    
    #[test]
    fn test_output_format_consistency() {
        // 全法則で出力フォーマットが統一されているかテスト
        for law in ["benf", "pareto", "zipf"] {
            for format in ["json", "csv", "yaml"] {
                let output = run_law_with_format(law, format);
                assert_valid_format(output, format);
            }
        }
    }
}
```

## 将来発展計画

### Phase 4: 統合機能実装
1. **比較分析コマンド**: `lawkit compare`
2. **統合レポート**: 複数法則の総合評価
3. **異常検知**: 法則間の矛盾検出

### Phase 5: 高度な分析機能
1. **時系列分析**: 分布変化の追跡
2. **機械学習統合**: パターン認識・予測
3. **可視化機能**: グラフ・チャート生成

### Phase 6: エンタープライズ機能
1. **API化**: REST/gRPC対応
2. **クラウド統合**: AWS/Azure/GCP対応
3. **大規模データ**: 分散処理・ストリーミング

この統合設計により、lawkitは単なるツール集合から**統合統計分析プラットフォーム**へと進化していく基盤が確立されている。