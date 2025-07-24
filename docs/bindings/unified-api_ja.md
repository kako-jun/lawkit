# lawkit 統一API リファレンス

*lawkit-python および lawkit-js 言語バインディング API ドキュメント*

## 概要

lawkit は統計法分析のための統一APIを提供し、特に詐欺検出、データ品質評価、監査コンプライアンスに有用です。ベンフォードの法則、パレート分布、ジップの法則などの様々な統計法を実装しています。

## メイン関数

### `law(subcommand, data_or_config, options)`

統計法を使用してデータを分析するか、ユーティリティ操作を実行します。

#### パラメータ

- `subcommand` (String): 実行する分析タイプまたはユーティリティコマンド
- `data_or_config` (Value): サブコマンドに応じた入力データまたは設定
- `options` (LawkitOptions, optional): 分析の設定オプション

#### 戻り値

- `Result<LawkitResult, Error>`: 分析結果またはユーティリティ出力

#### 例

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

// ベンフォードの法則分析
let data = json!([123, 456, 789, 111, 222, 333]);
let options = LawkitOptions {
    format: Some("json".to_string()),
    min_count: Some(100),
    ..Default::default()
};

let result = law("benford", &data, Some(&options))?;
```

## サブコマンド

### 分析コマンド

#### `benford` - ベンフォードの法則分析
データが最初の桁の分布でベンフォードの法則に従っているかを分析します。

```rust
let data = json!([/* 数値データ */]);
let result = law("benford", &data, None)?;
```

#### `pareto` - パレート分布分析
データが80/20ルールまたはその他のパレート分布に従っているかをチェックします。

```rust
let data = json!({ "values": [/* データポイント */] });
let result = law("pareto", &data, None)?;
```

#### `zipf` - ジップの法則分析
データがジップの法則に従っているかを分析します（自然言語や都市人口に共通）。

```rust
let data = json!({ "frequencies": [/* 頻度データ */] });
let result = law("zipf", &data, None)?;
```

#### `normal` - 正規分布分析
データが正規（ガウス）分布に従っているかをテストします。

```rust
let data = json!([/* 数値データ */]);
let result = law("normal", &data, None)?;
```

#### `poisson` - ポアソン分布分析
データがポアソン分布に従っているかをチェックします（固定間隔のイベント）。

```rust
let data = json!({ "events": [/* イベント数 */] });
let result = law("poisson", &data, None)?;
```

#### `analyze` - 自動分析
関連する統計法を自動的に検出して適用します。

```rust
let data = json!({ "dataset": [/* 混合データ */] });
let result = law("analyze", &data, None)?;
```

### ユーティリティコマンド

#### `validate` - データ検証
法則分析のためのデータフォーマットと構造を検証します。

```rust
let data = json!({ "data": [/* 検証するデータ */] });
let result = law("validate", &data, None)?;
```

#### `diagnose` - 診断情報
データと潜在的な分析に関する診断情報を提供します。

```rust
let data = json!({ "dataset": [/* データ */] });
let result = law("diagnose", &data, None)?;
```

#### `generate` - テストデータ生成
指定された統計法に従うテストデータを生成します。

```rust
let config = json!({
    "law": "benford",
    "count": 1000,
    "seed": 42
});
let result = law("generate", &config, None)?;
```

#### `list` - 利用可能な法則一覧
利用可能なすべての統計法とその説明を一覧表示します。

```rust
let empty = json!({});
let result = law("list", &empty, None)?;
```

#### `selftest` - セルフテスト
lawkitの機能を検証するために内部テストを実行します。

```rust
let empty = json!({});
let result = law("selftest", &empty, None)?;
```

## オプション

### LawkitOptions 構造体

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

### オプション詳細

#### 出力制御
- **`format`**: 出力フォーマット
  - オプション: `"json"`, `"csv"`, `"text"`, `"markdown"`
  - デフォルト: `"text"`
  
- **`quiet`**: 必須でない出力を抑制
  - デフォルト: `false`
  
- **`verbose`**: 詳細な出力を有効にする
  - デフォルト: `false`
  
- **`no_color`**: カラー出力を無効にする
  - デフォルト: `false`

#### 分析パラメータ
- **`min_count`**: 分析に必要な最小データポイント数
  - デフォルト: `30`（法則によって異なる）
  
- **`confidence_level`**: 統計的信頼水準（0.0-1.0）
  - デフォルト: `0.95`（95%信頼度）
  
- **`significance_level`**: 仮説検定のアルファ水準
  - デフォルト: `0.05`

#### 分析フィルター
- **`filter`**: 分析前にデータをフィルター
  - 例: `"value > 100"` または `"category == 'sales'"`
  
- **`laws`**: チェックする特定の法則（`analyze`コマンド用）
  - 例: `["benford", "pareto"]`
  
- **`focus`**: 特定の側面に分析を集中
  - 例: `"first_digit"` または `"distribution_tail"`

#### 閾値
- **`threshold`**: 法則遵守のカスタム閾値
  - デフォルト: 法則によって異なる
  
- **`tolerance`**: 期待値からの許容偏差
  - デフォルト: 法則によって異なる

#### 高度なオプション
- **`statistical_tests`**: 実行する追加の統計テスト
  - オプション: `["chi_square", "kolmogorov_smirnov", "anderson_darling"]`
  
- **`include_metadata`**: 結果に詳細なメタデータを含める
  - デフォルト: `false`
  
- **`detailed_report`**: 包括的な分析レポートを生成
  - デフォルト: `false`

## 結果タイプ

### LawkitResult 列挙型

```rust
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

### 分析データ構造体

#### BenfordData
```rust
pub struct BenfordData {
    pub digit_frequencies: HashMap<u8, f64>,
    pub expected_frequencies: HashMap<u8, f64>,
    pub chi_square_statistic: f64,
    pub p_value: f64,
    pub compliant: bool,
    pub sample_size: usize,
}
```

#### ParetoData
```rust
pub struct ParetoData {
    pub ratio: (f64, f64),  // 例: (80.0, 20.0)
    pub gini_coefficient: f64,
    pub lorenz_curve: Vec<(f64, f64)>,
    pub compliant: bool,
}
```

## 言語バインディング

### Python

```python
import lawkit_python

# ベンフォードの法則分析
data = [123, 456, 789, 111, 222, 333]
result = lawkit_python.law("benford", data)

# オプション付き
result = lawkit_python.law(
    "benford",
    data,
    format="json",
    min_count=100,
    confidence_level=0.99,
    detailed_report=True
)

# 自動分析
result = lawkit_python.law(
    "analyze",
    {"dataset": data},
    laws=["benford", "pareto", "normal"],
    verbose=True
)
```

### TypeScript/JavaScript

```typescript
import { law, LawkitOptions } from 'lawkit-js';

// ベンフォードの法則分析
const data = [123, 456, 789, 111, 222, 333];
const result = await law('benford', data);

// オプション付き
const options: LawkitOptions = {
    outputFormat: 'json',
    minSampleSize: 100,
    confidenceLevel: 0.99,
    detailedReport: true
};
const result = await law('benford', data, options);

// 自動分析
const analysisOptions: LawkitOptions = {
    lawsToCheck: ['benford', 'pareto', 'normal'],
    includeMetadata: true
};
const result = await law('analyze', { dataset: data }, analysisOptions);
```

## 例

### ベンフォードの法則による詐欺検出

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

// 金融取引の分析
let transactions = json!([10234, 2341, 45632, 1234, 8765, /*...*/]);

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
            println!("警告: 潜在的詐欺が検出されました！");
            println!("カイ二乗: {}, p値: {}", 
                     data.chi_square_statistic, 
                     data.p_value);
        }
    }
    _ => {}
}
```

### データ品質評価

```rust
// 複数の法則でデータ品質をチェック
let options = LawkitOptions {
    laws: Some(vec!["benford".to_string(), 
                    "pareto".to_string(), 
                    "normal".to_string()]),
    verbose: Some(true),
    ..Default::default()
};

let result = law("analyze", &dataset, Some(&options))?;
```

### テストデータ生成

```rust
let config = json!({
    "law": "zipf",
    "count": 10000,
    "parameters": {
        "s": 1.2,
        "n": 100
    },
    "seed": 12345
});

let result = law("generate", &config, None)?;
```

## パフォーマンスの考慮事項

- **大きなデータセット**: lawkitは100MBを超えるデータセットに対して自動的にストリーミング分析を使用
- **メモリ使用量**: 分析前にデータセットサイズを削減するために `filter` オプションを使用
- **並列処理**: `analyze` での複数の法則は並列処理される
- **キャッシング**: 同一データセットでの繰り返し分析では結果がキャッシュされる

## エラーハンドリング

以下に対して詳細なエラーが提供されます：
- 不十分なデータ（`min_count`以下）
- 無効なデータフォーマット
- 統計計算の失敗
- メモリ割り当ての問題
- 無効なサブコマンドまたはオプション

## ベストプラクティス

1. **データサイズ**: 十分なデータポイントを確保する（通常ベンフォードの法則では>1000）
2. **データクリーニング**: 分析前にデータ品質をチェックするために `validate` を使用
3. **複数の法則**: 適用可能な法則を自動検出するために `analyze` を使用
4. **信頼水準**: ユースケースに基づいて調整（詐欺検出：0.99、一般分析：0.95）
5. **フィルタリング**: より正確な分析のために関連するサブセットに事前フィルター