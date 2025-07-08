# lawkit - 統計法則ツールキット

複数の統計法則を用いてデータ品質・不正検知を行うCLIツールキット。

## 🎉 **2.1.0 リリース準備完了 (2025-07-07)** ✅ **全作業完了**

### **最終達成状況**
- ✅ **Core機能**: 5法則完全実装、100%テストカバレッジ
- ✅ **品質保証**: CI/CD完全正常化、全警告解決、179テスト全通過
- ✅ **Generate機能**: 教育・検証ツール完成
- ✅ **統合分析**: 多法則比較・推奨システム
- ✅ **高度機能**: 異常値検出・時系列・並列処理
- ✅ **言語削減**: CLI英語出力統一、5言語入力サポート維持
- ✅ **ドキュメント**: 3言語対応、generate例・業界使用例完備
- ✅ **パッケージ公開**: npm・PyPI両パッケージ公開完了、自動化スクリプト整備
- ✅ **GitHub基盤**: Issue/PRテンプレート完備、スクリプト統一管理
- ✅ **README統一**: 4ファイル短縮化、npm/PyPIバッジ追加、79%サイズ削減
- ✅ **Git管理**: コミット`98996d4`でREADME短縮化完了

### **技術実績**
- **戦略的言語削減**: CLI出力英語統一、5言語入力サポート維持、3言語ドキュメント対応
- **高度分析機能**: LOF・Isolation Forest・DBSCAN・時系列・並列処理・メモリ効率化
- **完全CI/CD**: 179テスト全通過、Clippy警告0個、GitHub Actions緑化
- **パッケージエコシステム**: Rust・npm・PyPI・自動バイナリダウンロード対応

### **確立済み開発ルール**
- ✅ **CI事前テスト**: `./scripts/ci-local.sh` 実行完了、全工程確認済み
- ✅ **完全CI再現**: 個別テストではなくCI全体実行済み
- ✅ **修正作業フロー実証**: 問題発見 → CI実行 → 修正 → 再実行 → 成功後プッシュ

## 📋 **現在の優先度ランキング**

### **🔥 次期実装予定: compare機能刷新 + diffx-core統合**

**実装戦略**: 既存compare機能を3つの明確な機能に再編し、diffx-coreを内部利用

#### **新サブコマンド設計**
1. **`lawkit integrate`** - 複数法則統合分析（現在のcompare summary）
2. **`lawkit validate`** - データ品質検証（現在のcompare consistency+conflict）
3. **`lawkit recommend`** - 法則推奨（現在のcompare recommend）

#### **diffx-core内部利用箇所**
- **統合分析での矛盾検出**: `IntegrationResult`の`conflicts`検出時の複数法則結果オブジェクト差分比較
- **クロスバリデーション**: `cross_validate_laws`でのfold間構造化差分による安定性評価精度向上
- **時系列変化点検出**: `TimeSeriesAnalysis`での統計オブジェクト構造化差分による精密変化検出

#### **実装手順**
1. **Phase 1**: diffx-coreをCargo.tomlに追加（optional feature: "advanced-diff"）
2. **Phase 2**: 既存compare.rsを3つのサブコマンドに分割
   - `integrate.rs` - 統合分析機能
   - `validate.rs` - 品質検証機能  
   - `recommend.rs` - 推奨機能
3. **Phase 3**: diffx-core統合
   - `lawkit-core/src/laws/integration/result.rs:342-362` 矛盾検出強化
   - `lawkit-core/src/laws/integration/analysis.rs:295-315` クロスバリデーション強化
   - `lawkit-core/src/common/timeseries.rs:224-275` 変化点検出強化

#### **技術詳細**
- **Cargo依存**: `diffx-core = { version = "0.1", optional = true }`
- **Feature gate**: `#[cfg(feature = "advanced-diff")]`
- **compare機能削除**: 既存compare機能は完全削除し、新3コマンドで置換
- **クリーンアップ**: `lawkit-cli/src/subcommands/compare.rs`削除、新ファイル3つ作成

### **🥇 最優先 (即座実行推奨)**
1. **リリース準備**: 
   - `2.1.0` 安定版タグ作成
   - GitHub Releases ページ整備
   - バイナリ配布準備

### **🥈 重要 (1-2週間以内)**
2. **パッケージマネージャー対応**:
   - crates.io 公開準備
   - Homebrew formula作成
   - APT/YUM パッケージ準備

3. **外部統合**:
   - Python binding (PyO3)
   - WebAssembly (wasm-bindgen)
   - Docker コンテナ

### **🥉 中期計画 (1-3ヶ月)**
4. **新統計法則追加**:
   - Weibull分布 (生存分析・信頼性工学)
   - LogNormal分布 (金融・環境データ)
   - Beta/Gamma分布 (ベイジアン統計)

5. **エンタープライズ機能**:
   - 設定ファイル対応 (YAML/TOML)
   - バッチ処理モード
   - レポート生成 (PDF/HTML)

## 🎯 **実装済み項目**

### 基本仕様
- **言語**: Rust
- **アーキテクチャ**: サブコマンド方式（`lawkit benf`, `lawkit pareto`, `lawkit zipf`, `lawkit normal`, `lawkit poisson`, `lawkit compare`, `lawkit generate`, `lawkit list`）
- **予定アーキテクチャ**: 新サブコマンド追加（`lawkit integrate`, `lawkit validate`, `lawkit recommend`でcompare機能を置換）
- **出力形式**: text, json, csv, yaml, toml, xml
- **国際数字対応**: 英語、日本語、中国語、ヒンディー語、アラビア語

### 実装完了機能
- ✅ **統計法則**: ベンフォード・パレート・ジップ・正規・ポアソン分布
- ✅ **統合機能**: 複数法則比較・矛盾検出・推奨システム
- ✅ **高度分析**: 異常値検出・時系列分析・並列処理・メモリ効率化
- ✅ **データ生成**: 統計的に正確なサンプルデータ生成機能
- ✅ **セルフテスト**: generate→analyze パイプライン検証機能
- ✅ **後方互換性**: 既存`benf`コマンド完全保持

### 品質保証
- ✅ **全179テスト通過**: 統合57・サブコマンド23・コア99テスト
- ✅ **CI/CD完全正常化**: Clippy警告0個、GitHub Actions緑化
- ✅ **コード品質**: format!警告解決、モダンRust構文採用

### パッケージエコシステム
- ✅ **npm公開**: `lawkit-js@2.1.0` - 自動バイナリダウンロード対応
- ✅ **PyPI公開**: `lawkit-python@2.1.0` - diffx方式準拠
- ✅ **自動化スクリプト**: publish-npm.sh, publish-pypi.sh, publish-all.sh, test-published-packages.sh
- ✅ **GitHub機能**: Issue/PRテンプレート4種類、lawkit特化設定

### ドキュメント
- ✅ **3言語対応**: 英語・日本語・中国語（アラビア語・ヒンディー語削除）
- ✅ **README統一**: 4ファイル短縮化、1,161行→237行（79%削減）
- ✅ **中国語ドキュメント**: 10個の包括的ドキュメントファイル作成
- ✅ **統一フォーマット**: バッジ・構造・リンクの完全統一

## 🔧 **詳細ドキュメント（Claude自動参照対象）**

### 統計法則仕様
- **[BENF_FEATURES.md](.claude/BENF_FEATURES.md)** - benf機能仕様
- **[PARETO_FEATURES.md](.claude/PARETO_FEATURES.md)** - pareto機能仕様
- **[ZIPF_FEATURES.md](.claude/ZIPF_FEATURES.md)** - zipf機能仕様
- **[NORMAL_FEATURES.md](.claude/NORMAL_FEATURES.md)** - normal機能仕様
- **[POISSON_FEATURES.md](.claude/POISSON_FEATURES.md)** - poisson機能仕様
- **[INTEGRATION_FEATURES.md](.claude/INTEGRATION_FEATURES.md)** - 統合機能仕様

### 設計ドキュメント
- **[LAWKIT_INTEGRATION.md](.claude/LAWKIT_INTEGRATION.md)** - 法則間統合仕様
- **[LAWKIT_STRATEGY.md](.claude/LAWKIT_STRATEGY.md)** - 統合戦略
- **[LAWKIT_ARCHITECTURE.md](.claude/LAWKIT_ARCHITECTURE.md)** - 統合基盤設計

## 🚀 **フェーズ4以降: エコシステム拡張**
- **時系列分析**: 変化点検出・トレンド分析・季節分解
- **AI統合**: 機械学習による自動法則選択・異常検知
- **可視化**: D3.js/Plotly連携によるインタラクティブ図表
- **コンプライアンス**: 監査証跡・規制対応

---

**重要指示**: このファイルの次回更新時は、完了済み項目の詳細記録は削除し、現在の優先度と実装済み項目の要約のみ維持してください。