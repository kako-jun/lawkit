# lawkit の思想

**「複数の統計法則による包括的データ品質・不正検知ツールキット」**

lawkitは単一法則の限界を超え、Benford法則・Pareto原理・Zipf法則・正規分布・ポアソン分布の5つの統計法則を統合分析することで、隠れたパターンと異常を自動検出します。

従来ツールが一つずつ分析する中、lawkitは複数法則の矛盾検出・相関分析・推奨システムにより、データの全体像を把握できます。金融監査・品質管理・不正検知・ビジネス分析での実用性を重視し、JSON/CSV等の構造化出力でAIツールや自動化ワークフローと完全統合します。

## 🚨 重要な開発ルール

### Claude対応時の必須ルール
- **CI事前テスト**: リリース前に`./github-shared/rust-cli-kiln/scripts/testing/quick-check.sh`でテスト実行
- **ドキュメント-テスト同期**: 全ドキュメント例がテストで検証済み、変更時は同期必須
- **バージョン統一**: Cargo.toml/pyproject.toml/package.json の完全同期

### プッシュ前の必須チェック
```bash
./github-shared/rust-cli-kiln/scripts/testing/quick-check.sh
```

### 共有スクリプト利用
```bash
# GitHub設定セットアップ（ラベル・ブランチ保護など）
./github-shared/rust-cli-kiln/scripts/setup/setup-github-workflow.sh

# 3言語ドキュメント整合性チェック
./github-shared/rust-cli-kiln/scripts/docs/check-docs-consistency.sh
```

### コンテキスト効率化ルール
**CLAUDE.mdは目次として使用し、詳細情報は以下の専用ファイルを参照:**

- **📋 タスクリスト**: `.claude/tasks.md` を参照
- **🚀 リリース手順**: `mnt/rust-cli-kiln/release-guide.md` を参照

**重要**: 詳細が必要な時のみ該当ファイルを読むこと。CLAUDE.md自体は最小限に保つ。

## 🎯 現在の開発状況

### ✅ 完成済み機能
- **5統計法則完全実装**: Benford/Pareto/Zipf/Normal/Poisson + 統合分析
- **ruff方式Python実装**: maturin バイナリ埋め込み完了
- **Phase 2.3機能**: --confidence/--sample-size/--min-value オプション
- **3言語ドキュメント**: 英語・日本語・中国語対応
- **99.9%実装率**: 938コマンド例中937個実装済み

### 🏆 v2.4.1 リリース完了 (diffx移植成果)
- **✅ 構造現代化**: .claude/ディレクトリ、release-guide.md完了
- **✅ scripts現代化**: 分類・改善、バージョンハードコーディング除去完了
- **✅ npm現代化**: ユニバーサルバイナリ同梱完了
- **✅ CI/CD改善**: 2幕構成、act1/act2エラー0達成完了
- **✅ リリース成功**: npm/PyPI/GitHub Release全て自動成功

## 📈 統計的特徴

### 実装済み統計法則
- **Benford法則**: 金融監査・不正検知 (自然数の第1桁分布)
- **Pareto原理**: ビジネス分析・集中度測定 (80/20法則)
- **Zipf法則**: テキスト分析・頻度分布 (べき法則)
- **正規分布**: 品質管理・異常値検出 (ベル曲線)
- **ポアソン分布**: 稀少事象・システム信頼性 (離散確率)

### 高度機能
- **統合分析**: 複数法則の矛盾検出・相関分析・推奨システム
- **異常値検出**: LOF・Isolation Forest・DBSCAN
- **時系列分析**: トレンド・季節性・変化点検出
- **国際対応**: 5言語数値形式サポート (EN/JP/CN/HI/AR)