# benf実装アーキテクチャ

## 現在のプロジェクト構造
```
src/
├── main.rs              # エントリポイント
├── lib.rs               # ライブラリルート
├── cli.rs               # CLI引数解析（clap）
├── core/
│   ├── mod.rs
│   ├── benford.rs       # ベンフォード法則計算
│   ├── japanese.rs      # 日本語数字変換
│   └── statistics.rs    # 統計計算
├── input/
│   ├── mod.rs
│   ├── parser.rs        # ファイル形式判定・解析
│   ├── network.rs       # HTTP取得
│   └── formats/         # 各ファイル形式パーサー
│       ├── excel.rs
│       ├── pdf.rs
│       ├── word.rs
│       ├── powerpoint.rs
│       ├── csv.rs
│       ├── html.rs
│       └── structured.rs
├── output/
│   ├── mod.rs
│   ├── formatter.rs     # 出力フォーマット
│   └── display.rs       # ASCII グラフ生成
└── error.rs             # エラー型定義
```

## benf固有の型定義
```rust
#[derive(Debug, Clone)]
pub struct BenfordResult {
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub digit_distribution: [f64; 9],
    pub expected_distribution: [f64; 9],
    pub chi_square: f64,
    pub p_value: f64,
    pub mean_absolute_deviation: f64,
    pub risk_level: RiskLevel,
    pub confidence_interval: Option<(f64, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,      // p > 0.1
    Medium,   // 0.05 < p ≤ 0.1  
    High,     // 0.01 < p ≤ 0.05
    Critical, // p ≤ 0.01
}

#[derive(Debug, Clone)]
pub struct FilterOptions {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub range: Option<(f64, f64)>,
    pub comparison: Option<ComparisonFilter>,
}

#[derive(Debug, Clone)]
pub struct HttpOptions {
    pub proxy: Option<String>,
    pub insecure: bool,
    pub timeout: Duration,
    pub user_agent: String,
}
```

## benf固有のエラー型
```rust
#[derive(Debug, thiserror::Error)]
pub enum BenfError {
    #[error("ファイル読み込みエラー: {0}")]
    FileRead(String),
    #[error("ネットワークエラー: {0}")]
    Network(String),
    #[error("数値抽出エラー: {0}")]
    NumberExtraction(String),
    #[error("統計計算エラー: {0}")]
    Statistics(String),
    #[error("データ不足エラー: {minimum}個以上の数値が必要ですが、{actual}個しかありません")]
    InsufficientData { minimum: usize, actual: usize },
    #[error("フィルタエラー: {0}")]
    FilterError(String),
    #[error("HTTP設定エラー: {0}")]
    HttpConfigError(String),
}
```

## 技術スタック
### 依存ライブラリ
- **CLI**: clap 4.x (引数解析)
- **非同期**: tokio 1.x (HTTP取得)
- **HTTP**: reqwest 0.11 (URL取得、プロキシ対応)
- **ファイル解析**:
  - calamine (Excel: .xlsx, .xls, .xlsb, .ods)
  - pdf-extract (PDF解析)
  - docx-rs (Word: .docx解析)
  - zip (PowerPoint: .pptx解析)
- **HTML解析**: scraper (HTMLタグ除去)
- **出力**: serde_json, serde_yaml, toml (構造化出力)
- **国際化**: 正規表現ベースの5文字体系統一処理

### パフォーマンス特性
- **メモリ効率**: 大容量ファイル対応、ストリーミング処理
- **処理速度**: 9999個数値を10秒以内処理
- **軽量バイナリ**: 依存関係最適化、高速起動

## 国際数字処理エンジン
### 対応文字体系
1. **日本語**: 全角数字・漢数字（基本・位取り）・混在パターン
2. **中国語**: 基本漢数字・金融文字（壹貳參等）・繁体字
3. **ヒンディー語**: デーヴァナーガリー文字（०१२等）
4. **アラビア語**: アラビア・インド数字（٠١٢等）
5. **ラテン**: 標準アラビア数字（0123456789）

### 統合処理関数
```rust
pub fn extract_numbers_international(text: &str) -> Vec<f64> {
    let text = convert_japanese_numerals(text);
    let text = convert_chinese_numerals(&text);
    let text = convert_hindi_numerals(&text);
    let text = convert_arabic_numerals(&text);
    extract_numbers(&text)
}
```

## テスト構造
### ユニットテスト
- tests/unit/japanese_numerals.rs (94テストケース)
- tests/unit/benford_calculations.rs (85テストケース)
- tests/unit/number_extraction.rs (67テストケース)
- tests/unit/risk_assessment.rs (78テストケース)

### 統合テスト
- tests/integration/cli_tests.rs (45テストケース)
- tests/integration/file_format_tests.rs (28テストケース)
- tests/integration/filtering_integration_tests.rs (6テストケース)
- tests/integration/http_options_tests.rs (8テストケース)

### テストフィクスチャ
- tests/fixtures/ - CSV/JSON/YAML/TOML/Excel/Word/PDF実ファイル
- tests/common/ - 共通テストユーティリティ

## 終了コード設計
- **0**: Low/Medium (正常終了)
- **10**: High (高注意レベル)
- **11**: Critical (致命的注意レベル)

## CLI引数設計
### 基本オプション
- `--format <FORMAT>`: 出力形式 (text/json/csv/yaml/toml/xml)
- `--lang <LANG>`: 出力言語 (en/ja/zh/hi/ar)
- `--quiet`: 最小出力
- `--verbose`: 詳細出力

### データ処理オプション
- `--filter <RANGE>`: 数値フィルタ (">=100", "100-500", "=200")
- `--threshold <LEVEL>`: 閾値設定 (auto/low/medium/high/critical/0.05)
- `--min-count <N>`: 最小数値数要件 (デフォルト: 5)

### HTTPオプション
- `--url <URL>`: HTTP/HTTPS取得
- `--proxy <URL>`: プロキシサーバー
- `--insecure`: SSL証明書検証スキップ
- `--timeout <SECONDS>`: タイムアウト設定 (1-3600秒)
- `--user-agent <STRING>`: カスタムUser-Agent