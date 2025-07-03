# ZIPF_ARCHITECTURE.md - Zipf法則アーキテクチャ設計

## アーキテクチャ概要

Zipf法則実装は、lawkit統合プラットフォームの第三の統計法則として設計。ベンフォード・パレート法則の設計パターンを継承しつつ、テキスト分析・多言語対応・べき乗法則評価という独自機能を実装。

## モジュール構造

```
src/laws/zipf/
├── mod.rs              # モジュール定義・公開API
├── result.rs           # ZipfResult構造体・メトリクス計算
└── analysis.rs         # 分析ロジック・テキスト処理・多言語対応

src/subcommands/
└── zipf.rs             # CLI実装・出力フォーマット・テキストモード
```

## 核心データ構造

### ZipfResult構造体
```rust
#[derive(Debug, Clone)]
pub struct ZipfResult {
    // 基本情報
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub risk_level: RiskLevel,
    
    // Zipf分析メトリクス
    pub zipf_exponent: f64,           // Zipf指数（理論値1.0）
    pub correlation_coefficient: f64,  // 理論分布との相関
    pub distribution_quality: f64,     // 分布品質スコア（0-1）
    
    // 頻度分析
    pub total_observations: usize,     // 総観測数（全頻度の合計）
    pub unique_items: usize,          // ユニーク項目数
    pub top_item_frequency: f64,      // 最頻項目の出現率（%）
    pub rank_frequency_pairs: Vec<(usize, f64)>, // ランク-頻度ペア（上位20項目）
    
    // 分布特性
    pub concentration_index: f64,      // 集中度指数（Gini係数）
    pub diversity_index: f64,         // 多様性指数（Shannon entropy）
    pub power_law_fit: f64,           // べき乗法則適合度（KS統計）
}
```

## 核心アルゴリズム

### 1. Zipf指数計算（対数線形回帰）
```rust
fn calculate_zipf_exponent(frequencies: &[f64]) -> f64 {
    // 対数変換: log(frequency) = -α * log(rank) + c
    let mut log_ranks = Vec::new();
    let mut log_freqs = Vec::new();
    
    for (i, &freq) in frequencies.iter().enumerate() {
        if freq > 0.0 {
            log_ranks.push(((i + 1) as f64).ln());
            log_freqs.push(freq.ln());
        }
    }
    
    // 最小二乗法で傾き（Zipf指数）を計算
    let n = log_ranks.len() as f64;
    let sum_x: f64 = log_ranks.iter().sum();
    let sum_y: f64 = log_freqs.iter().sum();
    let sum_xy: f64 = log_ranks.iter().zip(log_freqs.iter()).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = log_ranks.iter().map(|x| x * x).sum();
    
    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    
    // Zipf指数は負の傾きの絶対値
    -slope
}
```

### 2. 理論分布との相関係数
```rust
fn calculate_correlation_with_theoretical_zipf(frequencies: &[f64]) -> f64 {
    let total: f64 = frequencies.iter().sum();
    
    // 理論Zipf分布生成（1/rank）
    let theoretical: Vec<f64> = (1..=frequencies.len())
        .map(|rank| 1.0 / rank as f64)
        .collect();
    
    // 正規化
    let theoretical_sum: f64 = theoretical.iter().sum();
    let theoretical_normalized: Vec<f64> = theoretical
        .iter()
        .map(|&x| x / theoretical_sum)
        .collect();
    
    let observed_normalized: Vec<f64> = frequencies
        .iter()
        .map(|&x| x / total)
        .collect();
    
    // ピアソン相関係数
    calculate_pearson_correlation(&observed_normalized, &theoretical_normalized)
}
```

### 3. べき乗法則適合度（KS統計簡易版）
```rust
fn calculate_power_law_fit(frequencies: &[f64]) -> f64 {
    let theoretical_zipf = calculate_zipf_exponent(frequencies);
    let total: f64 = frequencies.iter().sum();
    let mut max_diff = 0.0;
    let mut cumulative_observed = 0.0;
    let mut cumulative_theoretical = 0.0;
    
    for (i, &freq) in frequencies.iter().enumerate() {
        cumulative_observed += freq / total;
        
        let theoretical_freq = 1.0 / ((i + 1) as f64).powf(theoretical_zipf);
        cumulative_theoretical += theoretical_freq;
        
        let diff = (cumulative_observed - cumulative_theoretical).abs();
        max_diff = max_diff.max(diff);
    }
    
    // 適合度スコア（1.0 - KS統計量）
    (1.0 - max_diff).max(0.0)
}
```

## テキスト分析エンジン

### 多言語トークナイザー
```rust
fn tokenize_multilingual_text(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    
    for ch in text.chars() {
        match ch {
            // 英語・数字
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                current_token.push(ch);
            }
            // 日本語文字（ひらがな・カタカナ・漢字）
            '\u{3040}'..='\u{309F}' |  // ひらがな
            '\u{30A0}'..='\u{30FF}' |  // カタカナ
            '\u{4E00}'..='\u{9FAF}' => { // 漢字
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                tokens.push(ch.to_string()); // 1文字ずつトークン化
            }
            // 区切り文字
            ' ' | '\t' | '\n' | '\r' | ',' | '.' | '!' | '?' | ';' | ':' | 
            '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' => {
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
```

### 単語頻度抽出
```rust
fn extract_word_frequencies(text: &str) -> Vec<(String, usize)> {
    let mut word_counts = HashMap::new();
    
    // 多言語トークナイズ
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
```

## 分析API設計

### 公開インターフェース
```rust
// 数値データからZipf分析
pub fn analyze_zipf_distribution(frequencies: &[f64], dataset_name: &str) -> Result<ZipfResult>;

// テキストからZipf分析
pub fn analyze_text_zipf(text: &str, dataset_name: &str) -> Result<ZipfResult>;

// 数値データを頻度として扱うZipf分析
pub fn analyze_numeric_zipf(numbers: &[f64], dataset_name: &str) -> Result<ZipfResult>;

// 品質評価レポート生成
pub fn evaluate_zipf_quality(result: &ZipfResult) -> ZipfQualityReport;
```

### 品質評価システム
```rust
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
    match deviation {
        d if d < 0.1 => 1.0,
        d if d < 0.3 => 0.8,
        d if d < 0.5 => 0.6,
        d if d < 0.8 => 0.4,
        _ => 0.2,
    }
}
```

## CLI統合アーキテクチャ

### デュアルモード処理
```rust
pub fn run(matches: &ArgMatches) -> Result<()> {
    let is_text_mode = matches.get_flag("text");
    
    if let Some(input) = matches.get_one::<String>("input") {
        if is_text_mode {
            // テキスト分析モード
            let result = analyze_text_zipf(input, input)?;
            output_results(&matches, &result);
        } else {
            // 数値分析モード
            let numbers = parse_input_auto(input)?;
            let result = analyze_numbers_with_options(&matches, input.to_string(), &numbers)?;
            output_results(&matches, &result);
        }
    } else {
        // 標準入力処理（テキスト・数値両対応）
        handle_stdin_input(&matches)?;
    }
}
```

### 多言語出力システム
```rust
fn localized_text(key: &str, lang: &str) -> &'static str {
    match (lang, key) {
        // 日本語
        ("ja", "zipf_analysis_results") => "ジップの法則解析結果",
        ("ja", "zipf_exponent") => "ジップ指数",
        ("ja", "ideal_zipf") => "理想的なジップ分布 - ジップの法則に従っています",
        
        // 英語
        ("en", "zipf_analysis_results") => "Zipf's Law Analysis Results",
        ("en", "zipf_exponent") => "Zipf exponent",
        ("en", "ideal_zipf") => "Ideal Zipf distribution - follows Zipf's law",
        
        // 中国語
        ("zh", "zipf_analysis_results") => "齐普夫定律分析结果",
        ("zh", "zipf_exponent") => "齐普夫指数",
        ("zh", "ideal_zipf") => "理想的齐普夫分布 - 符合齐普夫定律",
        
        // デフォルト英語
        (_, key) => default_english_text(key),
    }
}
```

## パフォーマンス設計

### 計算複雑度分析
- **数値分析**: O(n log n) - ソート処理が支配的
- **テキスト分析**: O(m + v log v) - 文字数m + 語彙数vのソート
- **相関計算**: O(n) - 線形処理
- **総合複雑度**: O((m + n) log n) - 入力サイズ依存

### メモリ効率化
```rust
// 段階的メモリ管理
impl ZipfResult {
    pub fn new(dataset_name: String, frequencies: &[f64]) -> Result<Self> {
        // 1. 基本チェック（早期リターン）
        if frequencies.len() < 5 {
            return Err(BenfError::InsufficientData(frequencies.len()));
        }
        
        // 2. 必要な計算のみ実行
        let zipf_exponent = calculate_zipf_exponent(frequencies);
        let correlation_coefficient = calculate_correlation_with_theoretical_zipf(frequencies);
        
        // 3. 要求時計算（遅延評価）
        let rank_frequency_pairs = generate_rank_frequency_pairs(frequencies);
        
        // 4. 結果構築
        Ok(ZipfResult { /* ... */ })
    }
}
```

### 大規模データ対応
- **ストリーミング処理**: メモリ効率的な語彙カウント
- **Top-K最適化**: 上位頻度語のみ保持
- **並列処理準備**: 将来のマルチスレッド対応

## エラーハンドリング戦略

### エラー分類
```rust
pub enum ZipfError {
    InsufficientData(usize),           // データ点数不足
    InvalidFrequencyData,              // 無効な頻度データ
    TextProcessingError(String),       // テキスト処理エラー
    CalculationError(String),          // 数値計算エラー
    UnicodeError(String),              // 文字エンコーディングエラー
}
```

### 回復可能エラー処理
- **空文字列**: 警告付きで続行
- **無効文字**: ログ出力後スキップ
- **数値変換失敗**: 該当行スキップ
- **メモリ不足**: 部分処理・サンプリング

## テスト設計

### 単体テスト
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_perfect_zipf_distribution() {
        // 理想的なZipf分布（1/rank）
        let frequencies: Vec<f64> = (1..=20)
            .map(|rank| 1000.0 / rank as f64)
            .collect();
        
        let result = ZipfResult::new("test".to_string(), &frequencies).unwrap();
        assert!((result.zipf_exponent - 1.0).abs() < 0.1);
        assert!(result.correlation_coefficient > 0.9);
        assert!(matches!(result.risk_level, RiskLevel::Low));
    }
    
    #[test]
    fn test_multilingual_tokenization() {
        let text = "Hello 世界 测试 مرحبا नमस्ते";
        let tokens = tokenize_multilingual_text(text);
        
        assert!(tokens.contains(&"Hello".to_string()));
        assert!(tokens.contains(&"世".to_string()));
        assert!(tokens.contains(&"界".to_string()));
        assert!(tokens.len() > 5);
    }
    
    #[test]
    fn test_text_zipf_analysis() {
        let text = "the quick brown fox jumps over the lazy dog the fox is quick";
        let result = analyze_text_zipf(text, "sample_text").unwrap();
        
        assert!(result.numbers_analyzed > 0);
        assert!(result.unique_items > 0);
        assert!(result.zipf_exponent > 0.0);
    }
}
```

### 統合テスト
- **CLI引数組み合わせ**: 全オプション組み合わせテスト
- **出力フォーマット**: JSON/CSV/XML妥当性検証
- **多言語処理**: 各言語の正確な処理確認
- **大規模データ**: パフォーマンス・メモリ使用量測定

## 品質保証

### 静的解析
- **Rustfmt**: コード整形
- **Clippy**: パフォーマンス・安全性チェック
- **型安全性**: 強い型システム活用

### 動的検証
- **メモリ安全性**: Valgrind相当のチェック
- **データ競合**: スレッドセーフティ検証
- **数値精度**: 浮動小数点誤差対策

## 将来拡張アーキテクチャ

### 分析機能拡張
- **Zipf-Mandelbrot分布**: より一般化された分布モデル
- **動的分析**: 時系列でのZipf分布変化
- **比較分析**: 複数データセット間の分布比較

### 技術的拡張
- **並列処理**: rayon crateによるマルチスレッド対応
- **ストリーミング**: 大規模テキストのリアルタイム処理
- **機械学習統合**: 分布予測・異常検知

### 出力・可視化
- **グラフィカル出力**: SVG/PNG形式のグラフ生成
- **インタラクティブ**: Web UI連携
- **API化**: REST API・gRPC対応