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
lawkit list                            # 利用可能な法則一覧
lawkit --version                       # バージョン情報
lawkit selftest                        # セルフテスト実行
```

### 法則別サブコマンド（分析系）
```bash
lawkit benf data.xlsx --format json    # ベンフォード法則
lawkit pareto sales.csv --format json  # パレート法則（80/20）
lawkit zipf text.txt --format json     # ジップ法則（単語頻度）
lawkit normal measurements.csv --format json  # 正規分布分析
lawkit poisson logs.txt --format json  # ポアソン分布（イベント頻度）
```

### 統合分析機能
```bash
lawkit compare data.csv --laws "benf,pareto" --format json  # 複数法則比較
lawkit compare data.csv --recommend --purpose fraud        # 推奨分析
lawkit compare data.csv --consistency-check                # 整合性チェック
```

### データ生成機能（教育・検証用）
```bash
lawkit generate benf --samples 1000 --seed 42             # ベンフォード生成
lawkit generate pareto --concentration 0.8 --samples 500  # パレート生成
lawkit generate zipf --exponent 1.0 --vocabulary-size 1000  # ジップ生成
lawkit generate normal --mean 100 --stddev 15 --samples 1000  # 正規分布生成
lawkit generate poisson --lambda 2.5 --time-series --samples 1000  # ポアソン生成
```

### 高度分析オプション
```bash
lawkit normal data.csv --outliers --outlier-method ensemble  # 高度異常値検出
lawkit normal data.csv --enable-timeseries --timeseries-window 20  # 時系列分析
lawkit benf large_data.csv --optimize                              # 大規模データ最適化
```

### パイプライン連携（generate→analyze）
```bash
lawkit generate benf --samples 10000 | lawkit benf --format json  # 生成→分析
lawkit generate normal --mean 50 --stddev 10 | lawkit normal --verbose  # 検証用
```

### 後方互換性
```bash
# 既存benfコマンドは完全互換維持
benf data.xlsx --format json           # 従来通り動作
```

## 共通基盤の再利用
### benf実装資産の活用
1. **国際数字処理エンジン**: 5文字体系対応を全法則で共用（入力処理）
2. **ファイル形式パーサー**: Excel/PDF/Word等を全法則で利用
3. **CLI出力システム**: 英語統一（戦略的言語削減完了）
4. **CLI引数システム**: --format, --quiet, --verbose, --threshold等の共通化
5. **エラーハンドリング**: 統一エラー型・終了コード
6. **高度分析基盤**: 異常値検出・時系列分析・並列処理・メモリ効率化
7. **データ生成エンジン**: 教育・検証・テスト用途のサンプル生成

### 法則固有実装
各法則は独自の分析ロジックのみ実装：
- **benford**: 先頭桁分布分析・不正検知・MAD計算
- **pareto**: 80/20分布分析・ジニ係数・ビジネス洞察・集中度評価
- **zipf**: 単語頻度分布分析・べき乗法則・テキスト分析・多言語対応
- **normal**: 正規性検定・異常値検出・品質管理・時系列分析
- **poisson**: イベント発生分析・稀少事象・確率予測・適合度検定
- **compare**: 複数法則統合・矛盾検出・推奨システム・目的別分析

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
- `--format` (-f): 出力形式（text/json/csv/yaml/toml/xml）
- `--quiet` (-q): 最小出力
- `--verbose` (-v): 詳細出力
- `--filter`: 数値範囲フィルタ（例: >=100, <1000, 50-500）
- `--min-count` (-c): 最小データ数 [default: 10]
- `--optimize`: 大規模データセット最適化

### 削除された引数（戦略的言語削減）
- `--language` / `--lang`: CLI出力英語統一により削除
- 入力数字処理: 5言語対応継続（英/日/中/印/アラブ）

### 高度分析機能
- `--outliers` (-O): 異常値検出
- `--outlier-method`: 異常値検出手法（zscore/modified_zscore/iqr/lof/isolation/dbscan/ensemble）
- `--enable-timeseries`: 時系列分析
- `--timeseries-window`: 時系列分析ウィンドウサイズ [default: 10]

### 法則固有引数
- **benf**: `--threshold` (-t) - 異常検知閾値
- **pareto**: `--concentration` (-C), `--gini-coefficient`, `--percentiles`, `--business-analysis`
- **zipf**: `--text` (-T), `--words` (-w)
- **normal**: `--test` (-T), `--quality-control` (-Q), `--spec-limits`
- **poisson**: `--test` (-T), `--predict` (-p), `--max-events`, `--rare-events` (-R)
- **compare**: `--laws` (-l), `--focus` (-F), `--recommend` (-r), `--report`, `--consistency-check`, `--cross-validation`, `--confidence-level`, `--purpose` (-p)

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

### 比較・分析機能（実装完了）
```bash
# 複数法則による多角的分析
lawkit compare data.csv --laws "benf,pareto" --format json    # 直接比較
lawkit compare data.csv --recommend --purpose fraud          # 推奨分析
lawkit compare data.csv --consistency-check --threshold 0.7  # 整合性チェック

# クロスバリデーション・詳細分析
lawkit compare data.csv --cross-validation --confidence-level 0.95
lawkit compare data.csv --report detailed --focus quality

# データ生成→分析パイプライン
lawkit generate benf --samples 10000 | lawkit benf --format json
lawkit generate pareto --concentration 0.8 | lawkit compare --laws "pareto,normal"
```

## 移行戦略（完了記録）
### ✅ Phase 1: 基盤構築（完了）
1. ✅ 既存benfコードのcommon/laws構造への再編成
2. ✅ サブコマンドアーキテクチャ実装
3. ✅ `lawkit benf` = 既存benf（100%互換）

### ✅ Phase 2: 法則実装（完了）
1. ✅ pareto実装（80/20分析・ジニ係数・ビジネス洞察）
2. ✅ zipf実装（単語頻度・べき乗法則・テキスト分析）
3. ✅ normal実装（正規性検定・異常値検出・品質管理）
4. ✅ poisson実装（イベント発生・稀少事象・確率予測）

### ✅ Phase 3: 統合・高度機能（完了）
1. ✅ compare実装（複数法則比較・矛盾検出・推奨システム）
2. ✅ generate実装（教育・検証用データ生成）
3. ✅ 高度分析機能（異常値検出・時系列・並列処理）
4. ✅ 戦略的言語削減（CLI英語統一・5言語入力対応継続）

### ✅ Phase 4: 品質保証・リリース準備（完了）
1. ✅ CI/CD完全正常化（179テスト全通過）
2. ✅ パッケージ公開（npm・PyPI対応）
3. ✅ GitHub基盤整備（Issue/PRテンプレート）
4. ✅ ドキュメント整備（3言語対応・使用例追加）

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