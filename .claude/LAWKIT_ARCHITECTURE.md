# lawkit統合アーキテクチャ設計

## 将来のプロジェクト構造
```
src/
├── main.rs                    # サブコマンドルーティング
├── lib.rs                     # 共通ライブラリエクスポート
├── common/                    # 全法則共通機能
│   ├── mod.rs
│   ├── international.rs       # 国際数字処理エンジン
│   ├── output.rs             # 統一出力フォーマット
│   ├── cli.rs                # 共通CLI引数処理
│   ├── statistics.rs         # 基本統計関数
│   └── files.rs              # ファイル形式パーサー群
├── laws/                     # 法則固有実装
│   ├── mod.rs
│   ├── benford/             # ベンフォード法則（移行済み）
│   │   ├── mod.rs
│   │   ├── analysis.rs      # 分析ロジック
│   │   ├── japanese.rs      # 日本語数字（benf固有）
│   │   └── expected.rs      # 期待値定数
│   ├── pareto/             # パレート法則（予定）
│   │   ├── mod.rs
│   │   ├── analysis.rs
│   │   └── business.rs     # ビジネス分析関数
│   ├── zipf/               # ジップ法則（予定）
│   │   ├── mod.rs
│   │   ├── analysis.rs
│   │   └── linguistics.rs  # 言語学関数
│   └── poisson/            # ポアソン分布（予定）
└── subcommands/             # サブコマンド実装
    ├── mod.rs
    ├── benf.rs              # lawkit benf
    ├── pareto.rs            # lawkit pareto
    ├── zipf.rs              # lawkit zipf
    └── poisson.rs           # lawkit poisson
```

## 統合コマンド体系
### メインコマンド
```bash
lawkit --help                          # 全体ヘルプ
lawkit --list                          # 利用可能な法則一覧
lawkit --version                       # バージョン情報
```

### 法則別サブコマンド
```bash
lawkit benf data.xlsx --format json    # ベンフォード法則
lawkit pareto sales.csv --format json  # パレート法則（80/20）
lawkit zipf text.txt --format json     # ジップ法則（単語頻度）
lawkit poisson logs.txt --format json  # ポアソン分布（イベント頻度）
```

### 後方互換性
```bash
# 既存benfコマンドは完全互換維持
benf data.xlsx --format json           # 従来通り動作
```

## 共通基盤の再利用
### benf実装資産の活用
1. **国際数字処理エンジン**: 5文字体系対応を全法則で共用
2. **ファイル形式パーサー**: Excel/PDF/Word等を全法則で利用
3. **多言語出力システム**: 5言語対応を統一
4. **CLI引数システム**: --format, --lang, --threshold等の共通化
5. **エラーハンドリング**: 統一エラー型・終了コード

### 法則固有実装
各法則は独自の分析ロジックのみ実装：
- **benford**: 先頭桁分布分析
- **pareto**: 80/20分布分析  
- **zipf**: 単語頻度分布分析
- **poisson**: イベント発生間隔分析

## trait-based設計
### 統一インターフェース
```rust
pub trait StatisticalLaw {
    type Result: Serialize + Clone;
    type Options: Clone;
    
    fn analyze(&self, data: &[f64], options: Self::Options) -> Result<Self::Result>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn supported_arguments(&self) -> Vec<&'static str>;
}
```

### 実装例
```rust
pub struct BenfordLaw;
impl StatisticalLaw for BenfordLaw {
    type Result = BenfordResult;
    type Options = BenfordOptions;
    
    fn analyze(&self, data: &[f64], options: Self::Options) -> Result<Self::Result> {
        // 既存のbenf実装を活用
    }
}

pub struct ParetoLaw;
impl StatisticalLaw for ParetoLaw {
    type Result = ParetoResult;
    type Options = ParetoOptions;
    // ...
}
```

## 引数互換性管理
### 共通引数（全法則対応）
- `--format`: 出力形式（text/json/csv/yaml/toml/xml）
- `--lang`: 多言語出力（en/ja/zh/hi/ar）
- `--quiet/--verbose`: 出力詳細度
- `--threshold`: リスク評価閾値

### 条件付き引数
- `--filter`: 数値系法則のみ（benf/pareto/poisson）
- `--min-count`: 全法則対応（最小データ数）

### 法則固有引数
- `--benf-*`: ベンフォード法則専用
- `--pareto-*`: パレート法則専用
- `--zipf-*`: ジップ法則専用

## 統合出力形式
### 統一JSON Schema
```json
{
  "law": "benford|pareto|zipf|poisson",
  "dataset_name": "string",
  "numbers_analyzed": "number",
  "risk_level": "Low|Medium|High|Critical",
  "p_value": "number",
  "statistics": {
    // 法則固有の統計情報
  },
  "metadata": {
    "version": "string",
    "timestamp": "ISO8601",
    "options": {}
  }
}
```

### 比較・分析機能（将来実装）
```bash
# 複数法則による多角的分析
lawkit benf data.csv --format json > benf.json
lawkit pareto data.csv --format json > pareto.json
lawkit compare benf.json pareto.json  # 結果比較

# 組み合わせ分析
lawkit analyze data.csv --laws "benf,pareto" --format json
```

## 移行戦略
### Phase 1: 基盤構築
1. 既存benfコードのcommon/laws構造への再編成
2. サブコマンドアーキテクチャ実装
3. `lawkit benf` = 既存benf（100%互換）

### Phase 2: 第二法則実装
1. pareto実装（確保済みドメイン活用）
2. 統合テスト・品質保証
3. ドキュメント更新

### Phase 3: 競合対抗
1. zipf実装（既存zipfコマンド競合対策）
2. poisson実装（既存poissonコマンド競合対策）
3. 統合機能（比較・組み合わせ分析）

## エコシステム戦略
### 個別コマンド戦略
- `benf`: 既存維持（既に確立された市場地位）
- `pareto`: 実装後にリリース（ビジネス分析市場進出）
- `zipf`/`poisson`: lawkitサブコマンドでの競合対抗

### 統合プラットフォーム優位性
- **一貫したUX**: 全法則で同じオプション体系
- **相互運用性**: 結果比較・組み合わせ分析
- **学習効率**: 一つのツールで複数手法習得
- **メンテナンス**: 共通基盤による品質向上