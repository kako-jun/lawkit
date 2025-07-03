# benf→lawkit移行計画

## 移行戦略概要

### 基本方針
1. **既存機能100%保持**: benfの全機能をlawkit benfで完全再現
2. **後方互換性維持**: 既存benfコマンドの継続サポート
3. **段階的移行**: リスクを最小化する段階的な構造変更
4. **品質保証**: 各段階でのテスト完全通過

### 移行アプローチ
- **Big Bang移行ではなく段階的移行**
- **既存テストケースの活用**: 28/28ユニットテストを移行指標に使用
- **機能テスト**: 実用例スクリプトでの動作確認
- **性能テスト**: ベンチマークによる性能劣化チェック

## Phase 1: 準備段階

### 1.1 バックアップ・ブランチ戦略
```bash
# 現在の安定版をバックアップ
git tag v1.0.0-benf-stable
git checkout -b feature/lawkit-migration
```

### 1.2 既存コード分析
**移行対象ファイル**:
```
src/
├── main.rs              ✅ → サブコマンドルーティングに変更
├── lib.rs               ✅ → 共通ライブラリエクスポートに変更
├── error.rs             ✅ → 共通エラー型として維持
├── core/
│   ├── benford.rs       🔄 → laws/benford/analysis.rs
│   ├── filtering.rs     🔄 → common/filtering.rs
│   ├── international.rs 🔄 → common/international.rs  
│   ├── japanese.rs      🔄 → laws/benford/japanese.rs
│   └── statistics.rs    🔄 → common/statistics.rs
├── input/               🔄 → common/input/
└── output/              🔄 → common/output/
```

### 1.3 依存関係分析
**外部クレート**: 変更なし（clap, tokio, reqwest, calamine等）
**内部モジュール**: 参照パスの更新が必要

## Phase 2: ディレクトリ構造移行

### 2.1 新ディレクトリ作成
```bash
mkdir -p src/common/input/formats
mkdir -p src/common/output
mkdir -p src/laws/benford
mkdir -p src/subcommands
```

### 2.2 ファイル移動・リネーム
```bash
# 共通機能移行
mv src/core/international.rs src/common/
mv src/core/filtering.rs src/common/
mv src/core/statistics.rs src/common/
mv src/input/* src/common/input/
mv src/output/* src/common/output/

# ベンフォード固有機能移行  
mv src/core/benford.rs src/laws/benford/analysis.rs
mv src/core/japanese.rs src/laws/benford/
```

### 2.3 モジュール定義ファイル作成
**src/common/mod.rs**:
```rust
pub mod international;
pub mod filtering;
pub mod statistics;
pub mod input;
pub mod output;
```

**src/laws/mod.rs**:
```rust
pub mod benford;
```

**src/laws/benford/mod.rs**:
```rust
pub mod analysis;
pub mod japanese;

pub use analysis::*;
```

**src/subcommands/mod.rs**:
```rust
pub mod benf;
```

## Phase 3: モジュール参照更新

### 3.1 import文の更新
**変更パターン**:
```rust
// Before
use crate::core::benford::*;
use crate::core::international::*;
use crate::input::parser::*;
use crate::output::formatter::*;

// After  
use crate::laws::benford::*;
use crate::common::international::*;
use crate::common::input::parser::*;
use crate::common::output::formatter::*;
```

### 3.2 自動化スクリプト
```bash
#!/bin/bash
# update_imports.sh

# coreモジュール参照の更新
find src -name "*.rs" -exec sed -i 's/use crate::core::international/use crate::common::international/g' {} \;
find src -name "*.rs" -exec sed -i 's/use crate::core::filtering/use crate::common::filtering/g' {} \;
find src -name "*.rs" -exec sed -i 's/use crate::core::statistics/use crate::common::statistics/g' {} \;
find src -name "*.rs" -exec sed -i 's/use crate::core::benford/use crate::laws::benford/g' {} \;

# input/outputモジュール参照の更新
find src -name "*.rs" -exec sed -i 's/use crate::input::/use crate::common::input::/g' {} \;
find src -name "*.rs" -exec sed -i 's/use crate::output::/use crate::common::output::/g' {} \;
```

## Phase 4: サブコマンドシステム実装

### 4.1 新main.rs実装
```rust
use clap::{command, Command, ArgMatches};

mod common;
mod laws;  
mod subcommands;
mod error;

fn main() {
    let matches = command!()
        .name("lawkit")
        .about("Statistical law analysis toolkit")
        .subcommand(
            Command::new("benf")
                .about("Benford's law analysis")
                // 既存引数定義をコピー
        )
        .subcommand(
            Command::new("list")
                .about("List available statistical laws")
        )
        .get_matches();

    let result = match matches.subcommand() {
        Some(("benf", sub_matches)) => subcommands::benf::run(sub_matches),
        Some(("list", _)) => list_laws(),
        _ => {
            show_help();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn list_laws() -> Result<(), error::LawkitError> {
    println!("Available statistical laws:");
    println!("  benf    - Benford's law analysis");
    Ok(())
}

fn show_help() {
    println!("lawkit - Statistical law analysis toolkit");
    println!("Usage: lawkit <SUBCOMMAND>");
    println!("Run 'lawkit --help' for more information.");
}
```

### 4.2 benf サブコマンド実装
**src/subcommands/benf.rs**:
```rust
use clap::ArgMatches;
use crate::common::input::parser::parse_input;
use crate::common::output::formatter::format_output;
use crate::laws::benford::analyze_benford;
use crate::error::LawkitError;

pub fn run(matches: &ArgMatches) -> Result<(), LawkitError> {
    // 既存main.rsの処理ロジックをそのまま移行
    // 引数解析、入力処理、解析、出力の流れを維持
    
    let input_data = parse_input(matches)?;
    let result = analyze_benford(&input_data, matches)?;
    format_output(&result, matches)?;
    
    Ok(())
}

pub fn run_standalone() -> Result<(), LawkitError> {
    // 個別benfコマンド用のエントリポイント
    // 既存main.rsの完全コピー
}
```

### 4.3 後方互換性確保
**src/bin/benf.rs** (新規作成):
```rust
use lawkit::subcommands::benf;

fn main() {
    if let Err(e) = benf::run_standalone() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

## Phase 5: テスト移行・検証

### 5.1 既存テストの移行
**テストファイル更新**:
```bash
# tests/unit/ ディレクトリのimport文更新
find tests -name "*.rs" -exec sed -i 's/use benf::/use lawkit::/g' {} \;
find tests -name "*.rs" -exec sed -i 's/use crate::core::/use crate::common::/g' {} \;
```

### 5.2 段階的テスト実行
```bash
# Step 1: コンパイルテスト
cargo check

# Step 2: ユニットテスト
cargo test --lib

# Step 3: 統合テスト  
cargo test --test '*'

# Step 4: CLIテスト
cargo build --release
./target/release/lawkit benf tests/fixtures/sample.csv
./target/release/benf tests/fixtures/sample.csv  # 後方互換性
```

### 5.3 機能回帰テスト
**既存実用例スクリプトの実行**:
```bash
# README記載例の実行確認
echo "1234 5678 9012" | ./target/release/lawkit benf
echo "1234 5678 9012" | ./target/release/benf  # 同じ結果確認

# 複雑な例
./target/release/lawkit benf tests/fixtures/financial_data.xlsx --format json
./target/release/benf tests/fixtures/financial_data.xlsx --format json
```

## Phase 6: パッケージ設定更新

### 6.1 Cargo.toml更新
```toml
[package]
name = "lawkit"           # benf → lawkit
version = "2.0.0"         # メジャーバージョンアップ
description = "Statistical law analysis toolkit with international number support"
keywords = ["statistics", "benford", "pareto", "zipf", "audit", "fraud-detection"]

[[bin]]
name = "lawkit"
path = "src/main.rs"

[[bin]]
name = "benf"             # 後方互換性
path = "src/bin/benf.rs"
```

### 6.2 README更新準備
- プロジェクト名をlawkitに変更
- インストール手順更新
- 使用例に`lawkit benf`を追加
- 後方互換性について説明

## Phase 7: 検証・完了

### 7.1 完了条件チェックリスト
- [ ] 全ユニットテスト通過 (28/28)
- [ ] 全統合テスト通過
- [ ] `lawkit benf` と `benf` の出力一致
- [ ] 性能劣化なし (±5%以内)
- [ ] メモリ使用量増加なし
- [ ] 全ファイル形式での動作確認
- [ ] 多言語出力の正常動作
- [ ] エラーハンドリングの一貫性

### 7.2 移行完了時の状態
```
benf → lawkit移行完了
├── lawkit benf: 新メインコマンド  
├── benf: 後方互換コマンド
├── 既存機能: 100%保持
├── テスト: 100%通過
└── 準備完了: pareto/zipf実装開始可能
```

## リスクと対策

### 主要リスク
1. **モジュール参照エラー**: import文更新ミス
2. **機能退化**: 移行過程での機能失失
3. **性能劣化**: 構造変更による性能影響
4. **テスト失敗**: 既存テストケースの不通過

### 対策
1. **自動化**: スクリプトによる一括更新
2. **段階的検証**: 各段階でのテスト実行
3. **ベンチマーク**: 性能監視の継続
4. **ロールバック計画**: 失敗時の復旧手順

### 緊急時対応
```bash
# 移行失敗時のロールバック
git checkout main
git reset --hard v1.0.0-benf-stable
```

## 移行スケジュール

### Day 1-2: 準備・構造移行
- ディレクトリ作成・ファイル移動
- モジュール定義作成
- import文更新

### Day 3-4: サブコマンド実装
- main.rs書き換え
- subcommands/benf.rs実装
- 後方互換性確保

### Day 5: テスト・検証
- 全テスト実行・修正
- 機能回帰テスト
- 性能ベンチマーク

### Day 6: 最終調整
- パッケージ設定更新
- ドキュメント更新
- 最終検証

**完了目標**: 1週間以内でbenf→lawkit移行完了、pareto実装開始準備完了