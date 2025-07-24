# API リファレンス - lawkit-core

統計法則分析機能を提供する `lawkit-core` Rust クレートの完全な API ドキュメント。

## 概要

`lawkit-core` クレートは lawkit エコシステムの中核であり、データ品質評価と不正検出のための包括的な統計法則分析を提供します。他の Rust アプリケーションに組み込んで、統計分析機能を追加できます。

**統一API設計**: コアAPIは、すべての分析操作に対して単一のメイン関数 `law()` のみを公開します。すべての機能はサブコマンドとオプションパラメータを使用してこの統一インターフェースからアクセスできます。この設計により、すべてのユースケースにおいて一貫性とシンプルさが保証されます。

## インストール

`Cargo.toml` に `lawkit-core` を追加：

```toml
[dependencies]
lawkit-core = "0.2.0"
```

### フィーチャーフラグ

```toml
[dependencies]
lawkit-core = { version = "0.2.0", features = ["all-laws"] }
```

利用可能なフィーチャー：
- `benford` (デフォルト) - ベンフォードの法則分析
- `pareto` (デフォルト) - パレート分布分析
- `zipf` (デフォルト) - ジップの法則分析
- `normal` - 正規分布テスト
- `poisson` - ポアソン分布分析
- `all-laws` - すべての統計法則を有効化

## パブリック API

### コアタイプ

#### `LawkitResult`

統計法則分析の結果を表します。

```rust
#[derive(Debug, PartialEq, Serialize)]
pub enum LawkitResult {
    // 分析結果
    BenfordAnalysis(String, BenfordData),
    ParetoAnalysis(String, ParetoData),
    ZipfAnalysis(String, ZipfData),
    NormalAnalysis(String, NormalData),
    PoissonAnalysis(String, PoissonData),
    
    // ユーティリティ結果
    ValidationResult(bool, Vec<String>),
    DiagnosticInfo(DiagnosticData),
    GeneratedData(Vec<Value>),
    LawList(Vec<LawInfo>),
    SelfTestResult(TestResults),
    
    // 多法則分析
    MultiLawAnalysis(Vec<(String, AnalysisResult)>),
}
```

#### 分析データ構造体

```rust
pub struct BenfordData {
    pub digit_frequencies: HashMap<u8, f64>,
    pub expected_frequencies: HashMap<u8, f64>,
    pub chi_square_statistic: f64,
    pub p_value: f64,
    pub compliant: bool,
    pub sample_size: usize,
}

pub struct ParetoData {
    pub ratio: (f64, f64),  // 例：(80.0, 20.0)
    pub gini_coefficient: f64,
    pub lorenz_curve: Vec<(f64, f64)>,
    pub compliant: bool,
}
```

### コア関数

#### `law()`

統計法則分析とユーティリティ操作のための主要関数。これはすべての lawkit 操作のための統一APIエントリーポイントです。

```rust
pub fn law(
    subcommand: &str,
    data_or_config: &Value,
    options: Option<&LawkitOptions>,
) -> Result<LawkitResult, Error>
```

**パラメータ:**
- `subcommand`: 実行する分析タイプまたはユーティリティコマンド
- `data_or_config`: サブコマンドに応じた入力データまたは設定
- `options`: 分析のためのオプション設定

**戻り値:** 分析結果を表す `Result<LawkitResult, Error>`

#### LawkitOptions 構造体

```rust
pub struct LawkitOptions {
    // 出力制御
    pub format: Option<String>,
    pub quiet: Option<bool>,
    pub verbose: Option<bool>,
    pub no_color: Option<bool>,
    
    // 分析パラメータ
    pub min_count: Option<usize>,
    pub confidence_level: Option<f64>,
    pub significance_level: Option<f64>,
    
    // 分析フィルター
    pub filter: Option<String>,
    pub laws: Option<Vec<String>>,
    pub focus: Option<String>,
    
    // 閾値
    pub threshold: Option<f64>,
    pub tolerance: Option<f64>,
    
    // 高度なオプション
    pub statistical_tests: Option<Vec<String>>,
    pub include_metadata: Option<bool>,
    pub detailed_report: Option<bool>,
}
```

**例:**
```rust
use lawkit_core::{law, LawkitOptions, LawkitResult};
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 不正検出のための金融取引データ
    let transactions = json!([10234, 2341, 45632, 1234, 8765]);
    
    let options = LawkitOptions {
        min_count: Some(1000),
        confidence_level: Some(0.99),
        detailed_report: Some(true),
        ..Default::default()
    };
    
    let result = law("benford", &transactions, Some(&options))?;
    
    match result {
        LawkitResult::BenfordAnalysis(_, data) => {
            if !data.compliant {
                println!("警告：潜在的不正が検出されました！");
                println!("カイ二乗: {}, p値: {}", 
                         data.chi_square_statistic, 
                         data.p_value);
            }
        }
        _ => {}
    }
    
    Ok(())
}
```

## サブコマンド

### 分析コマンド

- **`benford`** - 第一桁の分布のベンフォードの法則分析
- **`pareto`** - パレート分布（80/20ルール）分析
- **`zipf`** - ランク頻度分布のジップの法則分析
- **`normal`** - 正規分布テスト
- **`poisson`** - ポアソン分布分析
- **`analyze`** - 自動多法則分析

### ユーティリティコマンド

- **`validate`** - データ形式と構造の検証
- **`diagnose`** - データに関する診断情報の提供
- **`generate`** - 指定された法則に従うテストデータの生成
- **`list`** - 利用可能な統計法則の一覧表示
- **`selftest`** - 内部テストの実行

## 高度な使用法

### カスタム分析ロジック

#### ベンフォードの法則による不正検出

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

fn detect_fraud(transactions: Vec<f64>) -> Result<bool, Box<dyn std::error::Error>> {
    let data = json!(transactions);
    
    let options = LawkitOptions {
        min_count: Some(1000),
        confidence_level: Some(0.99),
        statistical_tests: Some(vec!["chi_square".to_string(), "kolmogorov_smirnov".to_string()]),
        ..Default::default()
    };
    
    let result = law("benford", &data, Some(&options))?;
    
    match result {
        LawkitResult::BenfordAnalysis(_, data) => {
            Ok(!data.compliant && data.p_value < 0.01)
        }
        _ => Ok(false)
    }
}
```

#### 多法則分析

```rust
let options = LawkitOptions {
    laws: Some(vec!["benford".to_string(), "pareto".to_string(), "normal".to_string()]),
    verbose: Some(true),
    include_metadata: Some(true),
    ..Default::default()
};

let result = law("analyze", &dataset, Some(&options))?;

match result {
    LawkitResult::MultiLawAnalysis(analyses) => {
        for (law_name, analysis) in analyses {
            println!("{}: {}", law_name, if analysis.compliant { "✓" } else { "✗" });
        }
    }
    _ => {}
}
```

### 異なるデータタイプでの作業

#### CSVデータ分析

```rust
use lawkit_core::{law, LawkitOptions};
use csv::Reader;
use serde_json::json;

fn analyze_csv_column(
    file_path: &str,
    column_index: usize
) -> Result<LawkitResult, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut values = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        if let Some(value) = record.get(column_index) {
            if let Ok(num) = value.parse::<f64>() {
                values.push(num);
            }
        }
    }
    
    let data = json!(values);
    Ok(law("benford", &data, None)?)
}
```

### 統合パターン

#### 継続的監視

```rust
use lawkit_core::{law, LawkitOptions, LawkitResult};
use serde_json::Value;
use std::time::Duration;
use tokio;

struct DataMonitor {
    pub threshold: f64,
    pub min_sample_size: usize,
}

impl DataMonitor {
    pub async fn monitor_data_quality(
        &self,
        data_source: impl Fn() -> Value
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // 1時間ごと
        
        loop {
            interval.tick().await;
            
            let data = data_source();
            let options = LawkitOptions {
                min_count: Some(self.min_sample_size),
                threshold: Some(self.threshold),
                ..Default::default()
            };
            
            let result = law("analyze", &data, Some(&options))?;
            
            match result {
                LawkitResult::MultiLawAnalysis(analyses) => {
                    let non_compliant: Vec<_> = analyses
                        .iter()
                        .filter(|(_, a)| !a.compliant)
                        .collect();
                    
                    if !non_compliant.is_empty() {
                        println!("データ品質アラート：{} 個の法則に違反", 
                                 non_compliant.len());
                    }
                }
                _ => {}
            }
        }
    }
}
```

#### テストデータ生成

```rust
use lawkit_core::{law, LawkitResult};
use serde_json::json;

fn generate_benford_data(count: usize, seed: u64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let config = json!({
        "law": "benford",
        "count": count,
        "seed": seed
    });
    
    let result = law("generate", &config, None)?;
    
    match result {
        LawkitResult::GeneratedData(values) => {
            Ok(values.into_iter()
                .filter_map(|v| v.as_f64())
                .collect())
        }
        _ => Err("予期しない結果タイプ".into())
    }
}
```

## エラーハンドリング

### エラータイプ

ライブラリはエラーハンドリングに `anyhow::Error` を使用します：

```rust
use lawkit_core::{law, LawkitOptions};
use anyhow::Result;

fn handle_analysis_errors(data: &Value) -> Result<()> {
    match law("benford", data, None) {
        Ok(result) => {
            println!("分析が正常に完了しました");
        }
        Err(e) => {
            eprintln!("分析エラー: {}", e);
            
            // 特定のエラー条件をチェック
            if e.to_string().contains("insufficient data") {
                eprintln!("分析には少なくとも30のデータポイントが必要です");
            } else if e.to_string().contains("invalid format") {
                eprintln!("データは数値である必要があります");
            }
        }
    }
    
    Ok(())
}
```

## パフォーマンスの考慮事項

### 大規模データセット分析

```rust
use lawkit_core::{law, LawkitOptions};

fn analyze_large_dataset(data: &Value) -> Result<LawkitResult, Box<dyn std::error::Error>> {
    let options = LawkitOptions {
        // データセットサイズを削減するためのフィルター
        filter: Some("value > 100 AND value < 1000000".to_string()),
        
        // 非常に大きなデータセットにはサンプリングを使用
        statistical_tests: Some(vec!["chi_square".to_string()]), // より高速なテスト
        
        // パフォーマンスのため詳細レポートを無効化
        detailed_report: Some(false),
        include_metadata: Some(false),
        
        ..Default::default()
    };
    
    Ok(law("benford", data, Some(&options))?)
}
```

### 最適化のヒント

1. **`filter`オプションを使用してデータを事前フィルター**
2. **不要な計算を避けるため適切な min_count を設定**
3. **大規模分析では詳細レポートを無効化**
4. **可能な場合は`analyze`の代わりに特定の法則を使用**
5. **同じデータセットでの繰り返し分析では結果をキャッシュ**

## テスト

### ユニットテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_benford_compliance() {
        // 既知のベンフォード準拠データ
        let fibonacci = json!([1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]);
        
        let result = law("benford", &fibonacci, None).unwrap();
        
        match result {
            LawkitResult::BenfordAnalysis(_, data) => {
                assert!(data.sample_size == 12);
                // フィボナッチ数列はベンフォードの法則に従う
                assert!(data.compliant);
            }
            _ => panic!("予期しない結果タイプ")
        }
    }
    
    #[test]
    fn test_data_generation() {
        let config = json!({
            "law": "zipf",
            "count": 100,
            "parameters": {"s": 1.2, "n": 50},
            "seed": 42
        });
        
        let result = law("generate", &config, None).unwrap();
        
        match result {
            LawkitResult::GeneratedData(values) => {
                assert_eq!(values.len(), 100);
            }
            _ => panic!("予期しない結果タイプ")
        }
    }
}
```

## バージョン互換性

- **0.2.x**: 現在の安定版
- **最小 Rust バージョン**: 1.70.0
- **依存関係**: 現在のバージョンは `Cargo.toml` を参照

## 関連項目

- [CLI リファレンス](cli-reference_ja.md) - コマンドライン使用法
- [はじめに](../user-guide/getting-started_ja.md) - 基本概念
- [統一API リファレンス](../bindings/unified-api_ja.md) - 言語バインディング
- [高度な分析ガイド](../guides/advanced-analysis_ja.md) - 複雑なユースケース