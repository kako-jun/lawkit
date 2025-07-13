# lawkit タスクリスト

## ✅ 🎉 **diffx移植100%完了！**

**migration-plan-realistic.md の全Phase 1-6完了**
- ✅ Act1/Act2 エラー0達成  
- ✅ 137テスト全通過
- ✅ 動的バージョン管理適用
- ✅ リリーススクリプト完全移植

## 🏁 **残り作業: なし**

**要求事項達成確認:**
- ✅ 「リリースして」指示でact1/act2エラーなく完了可能
- ✅ diffxとの完全互換（1行も変えず、ツール名のみ差分）  
- ✅ npm pre_download除去（ユニバーサルバイナリ同梱）
- ✅ バージョンハードコーディング完全除去
- ✅ CI/CD品質レベル = diffx同等

**結論: lawkitプロジェクトはdiffxの全改善を完全取り込み済み**

## 📋 完了済み緊急タスク

### ✅ 高優先度 - 全完了
- [x] `.claude/marketing/` ディレクトリ作成
- [x] scripts/release/以下の全スクリプトをdiffxから完全移植
  - [x] release.sh
  - [x] pre-release-check.sh
  - [x] monitor-release.sh
  - [x] cleanup-failed-release.sh
  - [x] validate-dynamic-versions.sh
  - [x] validate-dynamic-versions-simple.sh
  - [x] quick-release-check.sh
  - [x] pre-release-environment-check.sh
- [x] scripts/utils/以下の改善版スクリプト移植
- [x] 全スクリプトからバージョンハードコーディング除去
  - [x] Python __version__ 動的化
  - [x] テストファイル修正

### ✅ 中優先度 - 全完了
- [x] リリーススクリプト動作確認
- [x] act1/act2エラー0達成の最終確認

## 📝 完了タスク

### Phase 1: 基本構造移植
- [x] `.claude/` ディレクトリ作成
- [x] `CLAUDE.md` 目次化
- [x] `scripts/` 分類（release/testing/utils）

### Phase 2: ドキュメント移植  
- [x] `.claude/tasks.md` 作成
- [x] `.claude/release-guide.md` 作成

### Phase 3: スクリプト移植
- [x] ci-local.shテストスクリプト移植

### Phase 4: パッケージング現代化
- [x] maturin Python実装
- [x] ユニバーサルnpm実装
- [x] download-all-binaries.js作成

### Phase 5: CI/CD移植
- [x] GitHub Actions 2幕構成実装
- [x] release-act1.yml, release-act2.yml移植

## 🎯 移植方針

**重要**: diffxから1行も変えずにコピーし、ツール名（diffx→lawkit）だけを置換する。
独自実装は絶対に避け、diffxで実証済みの実装をそのまま使用する。