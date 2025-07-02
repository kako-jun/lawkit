# lawkit - 統計法則ツールキット

複数の統計法則を用いてデータ品質・不正検知を行うCLIツールキット。

## 現在の状況 (2025-07-02)
- **benf**: 99.5%完成（エンタープライズレディ）
- **戦略**: lawkit統合プラットフォーム化（zipf/poisson競合対策）
- **方針**: 実装完了後の段階的リリース（倫理的開発）

## 次のTODO
- [ ] lawkitサブコマンドアーキテクチャ設計
- [ ] benf→lawkit移行計画策定

## 詳細ドキュメント（Claude自動参照対象）
### benf固有情報
- **[BENF_FEATURES.md](.claude/BENF_FEATURES.md)** - benf機能仕様（入出力、対応形式、CLI引数）
- **[BENF_ARCHITECTURE.md](.claude/BENF_ARCHITECTURE.md)** - benf設計・実装（型定義、技術スタック、テスト）
- **[BENF_DEVELOPMENT.md](.claude/BENF_DEVELOPMENT.md)** - benf開発記録（フェーズ進捗、技術成果、品質指標）

### lawkit統合情報
- **[LAWKIT_STRATEGY.md](.claude/LAWKIT_STRATEGY.md)** - 統合戦略（競合分析、ドメイン状況、市場戦略）
- **[LAWKIT_ARCHITECTURE.md](.claude/LAWKIT_ARCHITECTURE.md)** - 統合基盤設計（共通基盤、trait設計、移行計画）

## 基本仕様
- **言語**: Rust
- **アーキテクチャ**: サブコマンド方式（将来: `lawkit benf`, `lawkit pareto`等）
- **出力形式**: text, json, csv, yaml, toml, xml
- **多言語対応**: 英語、日本語、中国語、ヒンディー語、アラビア語
- **国際数字対応**: 日本語数字、中国語金融数字、各言語数字

## 現在の戦略 (2025-07-02更新)
**lawkit統合プラットフォーム化**
- **背景**: ベンフォード以外に10+の不正検知法則が存在
- **課題**: benf/pareto確保、zipf/poisson他者確保済み
- **解決策**: 確保失敗した法則をlawkitサブコマンドで品質競争
  - `lawkit zipf` vs 既存`zipf`コマンド
  - 統合基盤の優位性活用（国際数字、多言語、ファイル形式統一）
- **倫理方針**: 実装完了後の段階的リリース（スクワッティング回避）

## benf完成状況
**総合完成度: 99.5%** - エンタープライズレディ状態

### 実装済み機能
- ✅ **国際数字処理**: 5文字体系（日本語、中国語、ヒンディー語、アラビア語、ラテン）
- ✅ **ファイル形式**: Excel, Word, PowerPoint, PDF, CSV/TSV, JSON/XML, YAML/TOML, HTML
- ✅ **多言語出力**: 英語、日本語、中国語、ヒンディー語、アラビア語
- ✅ **ベンフォード解析**: 先頭桁分布、カイ二乗検定、p値計算、4段階リスク評価
- ✅ **データフィルタリング**: --filter、--threshold、--min-count
- ✅ **品質保証**: 包括的テスト（28/28ユニットテスト通過）

### 削除された機能（Unix哲学準拠）
- ❌ **HTTPオプション**: --proxy, --insecure, --timeout, --user-agent（単機能原則違反）
- ❌ **ログ機能**: --log-level（ツール責務外）
- ❌ **設定ファイル**: 複雑性増大のため削除

### 残りタスク（0.5%）
- [ ] 最終品質チェック・リファクタリング
- [ ] パフォーマンス最適化