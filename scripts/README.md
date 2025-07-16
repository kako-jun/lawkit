# Scripts Directory

## 📁 Directory Structure

### 🛠️ utils/
ユーティリティスクリプト

- **`create-github-shared-symlink.sh`** - github-sharedシンボリックリンク作成

## 🚀 共有スクリプト

共有スクリプトは `github-shared/rust-cli-kiln/` を参照してください：

```bash
# CI事前テスト
./github-shared/rust-cli-kiln/scripts/testing/quick-check.sh

# ドキュメント整合性チェック  
./github-shared/rust-cli-kiln/scripts/docs/check-docs-consistency.sh

# GitHubワークフロー設定
./github-shared/rust-cli-kiln/scripts/setup/setup-github-workflow.sh
```

## 🎯 使用方法

### 日常開発
```bash
# github-sharedシンボリックリンク作成
./scripts/utils/create-github-shared-symlink.sh

# CI事前テスト（プッシュ前必須）
./github-shared/rust-cli-kiln/scripts/testing/quick-check.sh
```