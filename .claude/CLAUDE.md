# lawkit - 統計法則ツールキット

複数の統計法則を用いてデータ品質・不正検知を行うCLIツールキット。

## 現在の状況 (2025-07-03)
- **lawkit**: 2.0.0 基盤実装完了 ✅
- **benf→lawkit移行**: 完了（100%後方互換性維持）
- **戦略**: 4つの統計法則実装完了

## 次のTODO
- [x] pareto法則実装 ✅ (2025-07-03完了)
- [x] zipf法則実装（競合対策） ✅ (2025-07-03完了)
- [x] normal法則実装 ✅ (2025-07-03完了)
- [x] 正規分布詳細仕様書作成 ✅ (2025-07-03完了)
- [ ] 統合機能・法則間比較

## 詳細ドキュメント（Claude自動参照対象）
### 統計法則仕様
- **[BENF_FEATURES.md](.claude/BENF_FEATURES.md)** - benf機能仕様（入出力、対応形式、CLI引数）
- **[BENF_ARCHITECTURE.md](.claude/BENF_ARCHITECTURE.md)** - benf設計・実装（型定義、技術スタック、テスト）
- **[PARETO_FEATURES.md](.claude/PARETO_FEATURES.md)** - pareto機能仕様（80/20分析、Gini係数、ビジネス洞察）
- **[PARETO_ARCHITECTURE.md](.claude/PARETO_ARCHITECTURE.md)** - pareto設計・実装（アルゴリズム、リスク評価）
- **[ZIPF_FEATURES.md](.claude/ZIPF_FEATURES.md)** - zipf機能仕様（べき乗法則、テキスト分析、多言語対応）
- **[ZIPF_ARCHITECTURE.md](.claude/ZIPF_ARCHITECTURE.md)** - zipf設計・実装（トークナイザー、相関計算）
- **[NORMAL_FEATURES.md](.claude/NORMAL_FEATURES.md)** - normal機能仕様（正規性検定、異常値検出、品質管理）
- **[NORMAL_ARCHITECTURE.md](.claude/NORMAL_ARCHITECTURE.md)** - normal設計・実装（統計検定、工程能力、管理図）

### 統合プラットフォーム設計
- **[LAWKIT_INTEGRATION.md](.claude/LAWKIT_INTEGRATION.md)** - 法則間統合仕様（共通基盤、比較分析、将来拡張）
- **[LAWKIT_STRATEGY.md](.claude/LAWKIT_STRATEGY.md)** - 統合戦略（競合分析、ドメイン状況、市場戦略）
- **[LAWKIT_ARCHITECTURE.md](.claude/LAWKIT_ARCHITECTURE.md)** - 統合基盤設計（共通基盤、trait設計、移行計画）

### 開発記録
- **[BENF_DEVELOPMENT.md](.claude/BENF_DEVELOPMENT.md)** - benf開発記録（フェーズ進捗、技術成果、品質指標）
- **[LAWKIT_IMPLEMENTATION_PLAN.md](.claude/LAWKIT_IMPLEMENTATION_PLAN.md)** - 実装計画（週別スケジュール、技術仕様）
- **[BENF_MIGRATION_PLAN.md](.claude/BENF_MIGRATION_PLAN.md)** - 移行計画（段階的移行、リスク管理）

## 基本仕様
- **言語**: Rust
- **アーキテクチャ**: サブコマンド方式（実装済み: `lawkit benf`, `lawkit pareto`, `lawkit zipf`, `lawkit normal`, `lawkit list`）
- **出力形式**: text, json, csv, yaml, toml, xml
- **多言語対応**: 英語、日本語、中国語、ヒンディー語、アラビア語
- **国際数字対応**: 日本語数字、中国語金融数字、各言語数字

## lawkit 2.0.0 実装完了 (2025-07-03)
**lawkit統合プラットフォーム完成** ✅
- **サブコマンドシステム**: `lawkit benf`, `lawkit pareto`, `lawkit zipf`, `lawkit normal`, `lawkit list`
- **後方互換性**: 既存`benf`コマンド完全保持
- **アーキテクチャ変更**: 単一ツール→統合プラットフォーム
- **型システム整理**: 共通基盤 + 法則固有実装
- **パレート分析**: 80/20原則・Gini係数・ビジネス洞察
- **Zipf分析**: 単語頻度・べき乗法則・多言語対応
- **正規分布分析**: 正規性検定・異常値検出・品質管理

## 移行実績
**総合完成度: 100%** - 次のフェーズ準備完了

### 実装済み基盤
- ✅ **共通基盤**: 国際数字処理、ファイル形式パーサー、出力システム
- ✅ **ベンフォード法則**: `lawkit benf`（完全移行済み）
- ✅ **パレート法則**: `lawkit pareto`（80/20分析・Gini係数・完全実装）
- ✅ **Zipf法則**: `lawkit zipf`（単語頻度・べき乗法則・多言語テキスト分析）
- ✅ **正規分布**: `lawkit normal`（正規性検定・異常値検出・品質管理）
- ✅ **後方互換性**: `benf`コマンド保持
- ✅ **コンパイル成功**: 全バイナリ生成・動作確認済み

### 次期実装対象
- ✅ **pareto法則**: 80/20分析（完全実装・テスト済み）
- ✅ **zipf法則**: 単語頻度分析（完全実装・多言語対応・競合対策完了）
- ✅ **normal法則**: 正規分布分析（完全実装・3つの正規性検定・品質管理）
- 🔄 **統合機能**: 複数法則比較・組み合わせ分析