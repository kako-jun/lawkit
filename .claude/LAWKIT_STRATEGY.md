# lawkit統合プラットフォーム戦略

## 戦略的背景
### 現状分析
- ベンフォード法則以外に10個以上の不正検知用自然法則が存在
- 個別CLI実装では重複コード・学習負担・メンテナンス非効率
- Rustエコシステムでの統計CLI分野は未成熟

### 競合ドメイン名状況
- ✅ `pareto` - 取得済み（パレート分布関連）
- ❌ `zips` - 他者に取得済み（ジップ分布関連）  
- ❌ `poisson` - 他者に取得済み（ポアソン分布）
- ❓ `normal` - 意味不明（normalの可能性）

## 統合プラットフォーム設計

### コマンド体系
```bash
# 新しいコマンド体系
lawkit benf data.xlsx              # ベンフォード法則
lawkit zipf text.txt               # ジップの法則
lawkit newcomb elections.csv       # ニューカムの法則
lawkit pareto income.xlsx          # パレート法則
lawkit heaps documents/            # ヒープスの法則
lawkit --list                      # 利用可能な法則一覧
lawkit --help benf                 # 個別法則のヘルプ
```

### 候補となる統計法則（優先度順）
| 優先度 | 法則・性質 | 用途 | CLI化の価値 |
|--------|------------|------|-------------|
| ⭐⭐⭐⭐ | **ベンフォード法則** | 財務監査、異常検知 | ✅実装済み |
| ⭐⭐⭐⭐ | **Zipf法則** | テキスト品質、自然性評価 | NLP系ツールと相性良し |
| ⭐⭐⭐⭐ | **パレート法則** | 売上分析、リソース最適化 | ビジネス分析に強い |
| ⭐⭐⭐⭐ | **正規分布検定** | 外れ値検出、品質管理 | 統計処理の基本 |
| ⭐⭐⭐⭐ | **KS検定** | 分布との一致度評価 | 汎用的な異常検知 |
| ⭐⭐⭐ | **ポアソン分布** | イベント頻度、故障予測 | ログ解析に有用 |
| ⭐⭐⭐ | **エントロピー** | 暗号性、情報量評価 | セキュリティ分析 |

## 実装戦略

### Phase 1: アーキテクチャ変更
- [ ] プロジェクト名変更: benf → lawkit
- [ ] サブコマンド基盤実装
- [ ] benf互換性維持（lawkit benf = 現在のbenf）
- [ ] README・ドキュメント大幅更新

### Phase 2: 第二法則実装
- [ ] ジップの法則実装（文書・テキスト分析用）
- [ ] 統合CLI（--compare, --combination）
- [ ] 法則間の相関分析機能

### Phase 3: エコシステム拡張
- [ ] 残り8法則の段階的実装
- [ ] プラグインアーキテクチャ（外部法則追加）
- [ ] Web API・ダッシュボード（将来機能）

## マーケティング戦略

### 個別コマンド維持戦略
**戦略的価値**:
- SEO・発見性効果：法則名検索流入→lawkitエコシステム誘導
- 専門コミュニティ形成：各分野専門家の入り口
- Cargo名前確保：競合参入阻止・ブランド保護

### 優先確保コマンド名
**Tier 1（即座確保）**:
- `pareto` - ビジネス界での知名度最高（80/20ルール）
- `zipf` - 技術者層での知名度高（テキスト分析）  
- `normal` - 統計学の基本（品質管理）
- `poisson` - 実用性高（ログ解析・故障予測）

**Tier 2（専門分野）**:
- `kstest` - 統計検定の標準
- `entropy` - セキュリティ分野
- `powerlaw` - 新興分野（ネットワーク分析）
- `gini` - 社会科学分野（格差研究）

### 技術的実装
```toml
# Cargo.toml設定例
[package]
name = "lawkit"

[[bin]]
name = "lawkit"
path = "src/main.rs"

# 個別コマンド（統合ツール案内付き）
[[bin]]
name = "benf"
path = "src/standalone/benf.rs"

[[bin]]
name = "pareto"
path = "src/standalone/pareto.rs"
```

## 他ツール連携戦略

### diffx連携例
```bash
# ベンフォード分析結果の時系列比較
lawkit benf Q3-data.xlsx --format json > q3.json
lawkit benf Q4-data.xlsx --format json > q4.json
diffx q3.json q4.json --highlight risk_level

# 複数法則による多角的分析
lawkit benf sales.csv --format json > benf-result.json
lawkit pareto sales.csv --format json > pareto-result.json
diffx benf-result.json pareto-result.json --focus statistics
```

### AI時代のファイル形式対応
| 形式 | 内容 | 対応方法 |
|------|------|----------|
| .npy | NumPy配列 | ndarray-npy |
| .npz | 複数NumPy配列 | zip展開処理 |
| .pt | PyTorchテンソル | tch-rs（要libtorch） |
| .onnx | ONNXモデル | tract-onnx、重み抽出 |
| .h5 | HDF5形式 | hdf5（要libhdf5） |

## ドキュメント戦略

### 階層的情報アーキテクチャ
```
docs/
├── README.md                    # メインランディング（簡潔）
├── QUICKSTART.md               # 高速スタートガイド
├── laws/                       # 法則別詳細ドキュメント
│   ├── README.md              # 法則一覧・概要
│   ├── benford/
│   │   ├── README.md          # ベンフォード法則ガイド
│   │   ├── EXAMPLES.md        # 実用例・ユースケース
│   │   └── THEORY.md          # 理論的背景
│   └── zipf/
│       ├── README.md          # ジップ法則ガイド
│       ├── EXAMPLES.md        # テキスト分析実例
│       └── LINGUISTICS.md     # 言語学的応用
├── integrations/              # 他ツール連携
│   ├── diffx.md               # diffx連携詳細
│   ├── parallel.md            # GNU parallel活用
│   └── pipelines.md           # Unix pipeline例
└── languages/                 # 多言語ドキュメント
    ├── ja/                    # 日本語版
    ├── zh/                    # 中国語版
    └── hi/                    # ヒンディー語版
```

### 期待効果
- **発見性向上**: 法則名検索→専用ドキュメント直リンク
- **メンテナンス性**: 法則専門家による分散貢献
- **SEO最適化**: 各法則の専門キーワード対応
- **学習曲線**: 段階的学習パス提供

## 将来的な拡張計画
- `lawkit tui`: TUIモードで法則を選択・実行
- `lawkit web`: Web UIで可視化
- `lawkit explain`: AIによる結果の自然言語解説
- プラグインアーキテクチャ: 外部法則追加対応