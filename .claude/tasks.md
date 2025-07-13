# TODOリスト

## 🚀 優先度: diffx成果移植 (現在進行中)

### 📂 プロジェクト構造現代化 (高優先度)
- [x] `.claude/`ディレクトリ作成
- [x] `release-guide.md`移植・lawkit対応
- [x] `tasks.md`作成
- [ ] CLAUDE.md目次化（diffxスタイル統一）
- [ ] scripts分類・整理（release/testing/utils）

### 🔧 技術スタック現代化 (高優先度)
- [ ] スクリプト内容をdiffxから完全移植
- [ ] バージョンハードコーディング除去
- [ ] npm実装をユニバーサルバイナリ同梱に現代化
- [ ] Python maturin実装の改善
- [ ] GitHub Actions 2幕構成への移植

### 🎯 最終確認 (必須)
- [ ] CI/CD改善とact1/act2エラー対策適用
- [ ] ci-local.sh実行とエラー0達成

## 🔧 低優先度: 機能改善 (移植完了後)

### 🧹 コードクリーンアップ (後回し)
- [ ] lawkit-python/test_manual.pyの最適化
- [ ] 不要なファイル削除・整理
- [ ] ドキュメントの一貫性確認

### 📦 パッケージ改善 (後回し)
- [ ] npmパッケージの最適化確認
- [ ] Python wheel生成の最適化

### 🔧 エコシステム拡張
- [ ] Homebrew Formula作成
- [ ] TUIモード実装検討
- [ ] VS Code拡張検討

## 💡 長期的機能 (Long-term Features)

- [ ] 3統計法則間の相関分析強化
- [ ] AI推論機能（Claude/GPT連携）
- [ ] Web API（`lawkit serve`）
- [ ] リアルタイム分析

---

## ✅ 完了済み (Completed)

### diffx移植 Phase 1 (2025-07-13)
- [x] diffxプロジェクト分析完了
- [x] `.claude/`ディレクトリ構造作成
- [x] release-guide.mdのlawkit対応完了
- [x] migration-plan-realistic.md理解・適用開始

### Phase 2.3以前 (既存機能)
- [x] 5統計法則完全実装（Benford/Pareto/Zipf/Normal/Poisson）
- [x] 統合分析機能実装
- [x] Phase 2.3機能（--confidence/--sample-size/--min-value）
- [x] 3言語ドキュメント対応（英・日・中）
- [x] 99.9%実装率達成（938コマンド例中937個）
- [x] ruff方式Python実装完了
- [x] maturin バイナリ埋め込み完了
- [x] 基本的なscripts構造（改善必要）
- [x] GitHub Actions基本構成（改善必要）

## 🎯 移植完了基準

### 必須達成項目
1. **ディレクトリ構造**: diffxと同じ構造
2. **CLAUDE.md**: 目次化済み・コンテキスト効率化
3. **スクリプト**: diffxレベルの安定性・エラー0
4. **パッケージング**: maturin + universal npm
5. **CI/CD**: 2幕構成で動作・act1/act2エラー0

### 品質指標
- **ローカルビルド**: `cargo build --release` 成功
- **基本テスト**: `cargo test` 成功
- **ci-local.sh**: エラー0で完了
- **リリース**: 自動化で完了（diffxレベル）

## 📊 進捗状況

### 現在のステータス
- **移植進捗**: 10% (構造作成完了)
- **重要度**: 最高（他作業停止中）
- **完了予定**: 2-3時間以内

### 次のアクション
1. CLAUDE.md目次化
2. scripts分類・移植
3. npm/Python現代化
4. CI/CD改善
5. 最終確認・テスト
EOF < /dev/null