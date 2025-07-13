use clap::{Arg, Command};
use lawkit_core::common::{memory::MemoryConfig, parallel::ParallelConfig};

/// 全サブコマンドで共通のオプションを定義
pub fn add_common_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("format")
            .long("format")
            .short('f')
            .value_name("FORMAT")
            .help("Output format: text, csv, json, yaml, toml, xml")
            .default_value("text"),
    )
    .arg(
        Arg::new("quiet")
            .long("quiet")
            .short('q')
            .help("Minimal output")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("verbose")
            .long("verbose")
            .short('v')
            .help("Detailed output")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("filter")
            .long("filter")
            .value_name("RANGE")
            .help("Filter numbers by range (e.g., >=100, <1000, 50-500)"),
    )
    .arg(
        Arg::new("min-count")
            .long("min-count")
            .short('c')
            .value_name("NUMBER")
            .help("Minimum number of data points required for analysis")
            .default_value("10"), // 統一されたデフォルト値
    )
}

/// input引数を追加（位置引数）
pub fn add_input_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("input")
            .help("Input data (file path, URL, or '-' for stdin)")
            .index(1),
    )
}

/// サブコマンド固有のオプション：ベンフォード法則
pub fn add_benf_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("threshold")
            .long("threshold")
            .short('t')
            .value_name("LEVEL")
            .help("Anomaly detection threshold: low, medium, high, critical")
            .default_value("auto"),
    )
    .arg(
        Arg::new("confidence")
            .long("confidence")
            .value_name("LEVEL")
            .help("Statistical confidence level for tests (0.01-0.99)")
            .default_value("0.95"),
    )
    .arg(
        Arg::new("sample-size")
            .long("sample-size")
            .value_name("NUMBER")
            .help("Maximum sample size for large datasets (improves performance)"),
    )
    .arg(
        Arg::new("min-value")
            .long("min-value")
            .value_name("VALUE")
            .help("Minimum value to include in analysis (filters small values that add noise)"),
    )
}

/// サブコマンド固有のオプション：パレート法則  
pub fn add_pareto_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("concentration")
            .long("concentration")
            .short('C')
            .value_name("THRESHOLD")
            .help("Concentration threshold (0.0-1.0)")
            .default_value("0.8"),
    )
    .arg(
        Arg::new("gini-coefficient")
            .long("gini-coefficient")
            .help("Calculate Gini coefficient for inequality measurement")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("percentiles")
            .long("percentiles")
            .value_name("PERCENTILES")
            .help("Custom percentiles to calculate (e.g., 70,80,90)"),
    )
    .arg(
        Arg::new("business-analysis")
            .long("business-analysis")
            .help("Enable business analysis insights")
            .action(clap::ArgAction::SetTrue),
    )
}

/// サブコマンド固有のオプション：Zipf法則
pub fn add_zipf_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("text")
            .long("text")
            .short('T')
            .help("Enable text analysis mode")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("words")
            .long("words")
            .short('w')
            .value_name("NUMBER")
            .help("Maximum number of words to analyze in text mode")
            .default_value("1000"),
    )
}

/// サブコマンド固有のオプション：正規分布
pub fn add_normal_options(cmd: Command) -> Command {
    cmd
        .arg(
            Arg::new("test")
                .long("test")
                .short('T')
                .value_name("METHOD")
                .help("Normality test method: shapiro, anderson, ks, all")
                .default_value("all"),
        )
        .arg(
            Arg::new("outliers")
                .long("outliers")
                .short('O')
                .help("Enable outlier detection")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("outlier-method")
                .long("outlier-method")
                .value_name("METHOD")
                .help("Outlier detection method: zscore, modified_zscore, iqr, lof, isolation, dbscan, ensemble")
                .default_value("zscore"),
        )
        .arg(
            Arg::new("quality-control")
                .long("quality-control")
                .short('Q')
                .help("Enable quality control analysis")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("spec-limits")
                .long("spec-limits")
                .value_name("LOWER,UPPER")
                .help("Specification limits for quality control (e.g., 9.5,10.5)"),
        )
        .arg(
            Arg::new("enable-timeseries")
                .long("enable-timeseries")
                .help("Enable time series analysis")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("timeseries-window")
                .long("timeseries-window")
                .value_name("SIZE")
                .help("Time series analysis window size")
                .default_value("10"),
        )
}

/// サブコマンド固有のオプション：ポアソン分布
pub fn add_poisson_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("test")
            .long("test")
            .short('T')
            .value_name("METHOD")
            .help("Goodness-of-fit test method: chi_square, ks, variance, all")
            .default_value("all"),
    )
    .arg(
        Arg::new("predict")
            .long("predict")
            .short('p')
            .help("Enable probability prediction")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("max-events")
            .long("max-events")
            .value_name("NUMBER")
            .help("Maximum number of events for analysis")
            .default_value("20"),
    )
    .arg(
        Arg::new("rare-events")
            .long("rare-events")
            .short('R')
            .help("Focus on rare event analysis")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("confidence")
            .long("confidence")
            .value_name("LEVEL")
            .help("Statistical confidence level for tests (0.01-0.99)")
            .default_value("0.95"),
    )
}

/// サブコマンド固有のオプション：データ生成
pub fn add_generate_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("samples")
            .long("samples")
            .short('s')
            .value_name("NUMBER")
            .help("Number of samples to generate")
            .default_value("1000"),
    )
    .arg(
        Arg::new("seed")
            .long("seed")
            .value_name("NUMBER")
            .help("Random seed for reproducible generation"),
    )
    .arg(
        Arg::new("output-file")
            .long("output-file")
            .short('o')
            .value_name("FILE")
            .help("Output file path (default: stdout)"),
    )
    .arg(
        Arg::new("fraud-rate")
            .long("fraud-rate")
            .value_name("RATE")
            .help("Fraud injection rate (0.0-1.0) for testing")
            .default_value("0.0"),
    )
}

/// Generate用法則固有オプション：ベンフォード法則
pub fn add_generate_benf_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("range")
            .long("range")
            .value_name("MIN,MAX")
            .help("Number range for generation (e.g., 1,10000)")
            .default_value("1,100000"),
    )
}

/// Generate用法則固有オプション：パレート法則
pub fn add_generate_pareto_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("concentration")
            .long("concentration")
            .short('C')
            .value_name("RATIO")
            .help("Concentration ratio (0.0-1.0, default: 0.8 for 80/20)")
            .default_value("0.8"),
    )
    .arg(
        Arg::new("scale")
            .long("scale")
            .value_name("NUMBER")
            .help("Scale parameter for Pareto distribution")
            .default_value("1.0"),
    )
}

/// Generate用法則固有オプション：Zipf法則  
pub fn add_generate_zipf_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("exponent")
            .long("exponent")
            .short('e')
            .value_name("NUMBER")
            .help("Zipf exponent (default: 1.0)")
            .default_value("1.0"),
    )
    .arg(
        Arg::new("vocabulary-size")
            .long("vocabulary-size")
            .short('V')
            .value_name("NUMBER")
            .help("Vocabulary size for text generation")
            .default_value("10000"),
    )
}

/// Generate用法則固有オプション：正規分布
pub fn add_generate_normal_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("mean")
            .long("mean")
            .short('m')
            .value_name("NUMBER")
            .help("Mean of normal distribution")
            .default_value("0.0"),
    )
    .arg(
        Arg::new("stddev")
            .long("stddev")
            .short('d')
            .value_name("NUMBER")
            .help("Standard deviation of normal distribution")
            .default_value("1.0"),
    )
}

/// Generate用法則固有オプション：ポアソン分布
pub fn add_generate_poisson_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("lambda")
            .long("lambda")
            .short('l')
            .value_name("NUMBER")
            .help("Lambda parameter (rate) for Poisson distribution")
            .default_value("2.0"),
    )
    .arg(
        Arg::new("time-series")
            .long("time-series")
            .short('T')
            .help("Generate time-series event data")
            .action(clap::ArgAction::SetTrue),
    )
}

/// サブコマンド固有のオプション：統合分析
pub fn add_integration_options(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("laws")
            .long("laws")
            .short('l') // compare専用で-lを使用
            .help("Laws to compare (benf,pareto,zipf,normal,poisson)")
            .value_name("LAWS"),
    )
    .arg(
        Arg::new("focus")
            .long("focus")
            .short('F') // -f から -F に変更（formatと区別）
            .help("Analysis focus area")
            .value_name("FOCUS")
            .value_parser(["quality", "concentration", "distribution", "anomaly"]),
    )
    .arg(
        Arg::new("threshold")
            .long("threshold")
            .short('t')
            .help("Conflict detection threshold (0.0-1.0)")
            .value_name("THRESHOLD")
            .value_parser(clap::value_parser!(f64))
            .default_value("0.5"),
    )
    .arg(
        Arg::new("recommend")
            .long("recommend")
            .short('r')
            .help("Enable recommendation mode")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("report")
            .long("report")
            .help("Integration report type")
            .value_name("TYPE")
            .value_parser(["summary", "detailed", "conflicting"])
            .default_value("summary"),
    )
    .arg(
        Arg::new("consistency-check")
            .long("consistency-check")
            .help("Enable consistency check")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("cross-validation")
            .long("cross-validation")
            .help("Enable cross-validation analysis")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("confidence-level")
            .long("confidence-level")
            .help("Confidence level")
            .value_name("LEVEL")
            .value_parser(clap::value_parser!(f64))
            .default_value("0.95"),
    )
    .arg(
        Arg::new("purpose")
            .long("purpose")
            .short('p')
            .help("Analysis purpose")
            .value_name("PURPOSE")
            .value_parser([
                "quality",
                "fraud",
                "concentration",
                "anomaly",
                "distribution",
                "general",
            ]),
    )
}

/// 自動最適化設定をセットアップ（データ特性に基づく）
pub fn setup_automatic_optimization_config() -> (ParallelConfig, MemoryConfig) {
    // 常に最適化設定を使用（自動最適化）
    let parallel_config = ParallelConfig {
        num_threads: 0, // auto-detect
        chunk_size: 1000,
        enable_parallel: true,
    };
    let memory_config = MemoryConfig {
        chunk_size: 10000,
        max_memory_mb: 512,
        enable_streaming: true,
        enable_compression: false,
    };
    (parallel_config, memory_config)
}

/// 最適化されたファイルリーダーを取得
pub fn get_optimized_reader(input: Option<&String>) -> Result<String, Box<dyn std::error::Error>> {
    // 自動最適化された読み込み方法を使用
    if let Some(input_path) = input {
        if input_path == "-" {
            use std::io::Read;
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        } else {
            std::fs::read_to_string(input_path).map_err(Into::into)
        }
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
