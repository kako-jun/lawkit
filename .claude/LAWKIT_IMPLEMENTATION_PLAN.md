# lawkit実装計画

## 現状分析

### 現在のbenf構造
```
src/
├── main.rs              # CLI エントリポイント
├── lib.rs               # ライブラリルート
├── error.rs             # エラー型定義
├── core/                # コア機能
│   ├── benford.rs       # ベンフォード法則計算
│   ├── filtering.rs     # データフィルタリング
│   ├── international.rs # 国際数字処理
│   ├── japanese.rs      # 日本語数字変換
│   └── statistics.rs    # 統計計算
├── input/               # 入力処理
│   ├── file_detector.rs # ファイル形式検出
│   ├── parser.rs        # パーサー統合
│   └── formats/         # 形式別パーサー
└── output/              # 出力処理
    └── formatter.rs     # フォーマッター
```

### 移行必要な要素
- ✅ **再利用可能**: core/, input/, output/ → common/
- ✅ **法則固有**: core/benford.rs → laws/benford/
- 🔄 **新規作成**: subcommands/, laws/pareto/, laws/zipf/

## Phase 1: サブコマンドアーキテクチャ実装

### 1.1 プロジェクト構造変更

**目標構造**:
```
src/
├── main.rs              # サブコマンドルーティング
├── lib.rs               # 共通ライブラリエクスポート
├── common/              # 全法則共通機能
│   ├── mod.rs
│   ├── international.rs # 国際数字処理エンジン
│   ├── filtering.rs     # データフィルタリング
│   ├── statistics.rs    # 基本統計関数
│   ├── input/           # 入力処理（移動）
│   │   ├── file_detector.rs
│   │   ├── parser.rs
│   │   └── formats/
│   └── output/          # 出力処理（移動）
│       └── formatter.rs
├── laws/                # 法則固有実装
│   ├── mod.rs
│   ├── benford/         # ベンフォード法則
│   │   ├── mod.rs
│   │   ├── analysis.rs  # 分析ロジック（既存benford.rs）
│   │   └── japanese.rs  # 日本語数字（benf固有）
│   ├── pareto/          # パレート法則（新規）
│   │   ├── mod.rs
│   │   └── analysis.rs
│   └── zipf/            # ジップ法則（新規）
│       ├── mod.rs
│       └── analysis.rs
├── subcommands/         # サブコマンド実装
│   ├── mod.rs
│   ├── benf.rs          # lawkit benf
│   ├── pareto.rs        # lawkit pareto
│   └── zipf.rs          # lawkit zipf
└── error.rs             # エラー型（共通）
```

### 1.2 trait-based統合設計

**StatisticalLaw trait**:
```rust
pub trait StatisticalLaw {
    type Result: Serialize + Clone + Debug;
    type Options: Clone + Debug;
    
    fn analyze(&self, data: &[f64], options: Self::Options) 
        -> Result<Self::Result, crate::error::LawkitError>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn supported_arguments(&self) -> Vec<&'static str>;
}
```

**各法則の実装**:
```rust
// laws/benford/mod.rs
pub struct BenfordLaw;
impl StatisticalLaw for BenfordLaw {
    type Result = BenfordResult;
    type Options = BenfordOptions;
    
    fn analyze(&self, data: &[f64], options: Self::Options) 
        -> Result<Self::Result, LawkitError> {
        // 既存ロジック活用
    }
}

// laws/pareto/mod.rs  
pub struct ParetoLaw;
impl StatisticalLaw for ParetoLaw {
    type Result = ParetoResult;
    type Options = ParetoOptions;
    // ...
}
```

### 1.3 サブコマンド統合システム

**メインエントリポイント (main.rs)**:
```rust
use clap::{Command, Arg, ArgMatches, command, value_parser};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("benf")
                .about("Benford's law analysis")
                // 既存CLI引数を継承
        )
        .subcommand(
            Command::new("pareto")  
                .about("Pareto principle analysis")
        )
        .subcommand(
            Command::new("zipf")
                .about("Zipf's law analysis") 
        )
        .subcommand(
            Command::new("list")
                .about("List available statistical laws")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("benf", sub_matches)) => subcommands::benf::run(sub_matches),
        Some(("pareto", sub_matches)) => subcommands::pareto::run(sub_matches),
        Some(("zipf", sub_matches)) => subcommands::zipf::run(sub_matches),
        Some(("list", _)) => list_laws(),
        _ => show_help(),
    }
}
```

## Phase 2: 既存コード移行手順

### 2.1 共通機能の移行

**Step 1**: ディレクトリ作成・ファイル移動
```bash
mkdir -p src/common/input/formats
mkdir -p src/common/output  
mkdir -p src/laws/benford
mkdir -p src/subcommands

# 共通機能移動
mv src/core/international.rs src/common/
mv src/core/filtering.rs src/common/
mv src/core/statistics.rs src/common/
mv src/input/* src/common/input/
mv src/output/* src/common/output/

# ベンフォード固有機能移行
mv src/core/benford.rs src/laws/benford/analysis.rs
mv src/core/japanese.rs src/laws/benford/
```

**Step 2**: モジュール参照の更新
- `use crate::core::` → `use crate::common::`
- `use crate::input::` → `use crate::common::input::`
- `use crate::output::` → `use crate::common::output::`

### 2.2 後方互換性の確保

**個別benfコマンド維持**:
```rust
// src/bin/benf.rs (新規作成)
fn main() {
    // 既存main.rsロジックそのまま
    // または lawkit benf へのプロキシ
    lawkit::subcommands::benf::run_standalone();
}
```

**Cargo.toml更新**:
```toml
[package]
name = "lawkit"  # benf → lawkit

[[bin]]
name = "lawkit"
path = "src/main.rs"

[[bin]]  
name = "benf"    # 後方互換性
path = "src/bin/benf.rs"
```

## Phase 3: 新法則実装

### 3.1 パレート法則実装計画

**laws/pareto/analysis.rs**:
```rust
pub struct ParetoResult {
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub pareto_ratio: f64,           // 80/20比率
    pub concentration_index: f64,     // 集中度指標
    pub cumulative_distribution: Vec<(f64, f64)>,
    pub risk_level: RiskLevel,
}

pub fn analyze_pareto_distribution(numbers: &[f64]) -> ParetoResult {
    // 80/20原則の検証
    // ジニ係数計算
    // ローレンツ曲線生成
}
```

### 3.2 ジップ法則実装計画

**laws/zipf/analysis.rs**:
```rust
pub struct ZipfResult {
    pub dataset_name: String,
    pub words_analyzed: usize,
    pub zipf_coefficient: f64,        // ジップ係数
    pub rank_frequency_pairs: Vec<(usize, usize)>,
    pub goodness_of_fit: f64,
    pub risk_level: RiskLevel,
}

pub fn analyze_zipf_distribution(text: &str) -> ZipfResult {
    // 単語頻度分析
    // ランク-頻度分布計算
    // べき乗則フィッティング
}
```

## Phase 4: 統合機能実装

### 4.1 法則間比較機能

```rust
// subcommands/compare.rs
pub fn compare_laws(data: &[f64], laws: Vec<Box<dyn StatisticalLaw>>) 
    -> ComparisonResult {
    // 複数法則による多角的分析
    // 結果の統計的比較
    // 統合リスク評価
}
```

### 4.2 統合出力形式

```rust
#[derive(Serialize)]
pub struct LawkitResult {
    pub law: String,
    pub dataset_name: String,
    pub analysis_timestamp: String,
    pub result: serde_json::Value,  // 法則固有結果
    pub metadata: LawkitMetadata,
}
```

## 移行スケジュール

### Week 1: インフラ構築
- [x] ドキュメント構造整備 ✅
- [ ] プロジェクト構造変更
- [ ] trait定義・共通インターフェース

### Week 2: benf移行
- [ ] 既存コード移行
- [ ] `lawkit benf` 実装
- [ ] 後方互換性確保・テスト

### Week 3: pareto実装
- [ ] パレート法則分析ロジック
- [ ] `lawkit pareto` サブコマンド
- [ ] 統合テスト

### Week 4: zipf実装・統合機能
- [ ] ジップ法則分析ロジック
- [ ] `lawkit zipf` サブコマンド
- [ ] 法則間比較機能
- [ ] 最終統合テスト

## リスク管理

### 技術的リスク
- **後方互換性**: 既存benfユーザーへの影響最小化
- **パフォーマンス**: 共通基盤による性能劣化回避
- **複雑性**: trait抽象化による保守性確保

### 対策
- 段階的移行・徹底的テスト
- benchmarkによる性能監視
- 明確なモジュール分離・ドキュメント整備