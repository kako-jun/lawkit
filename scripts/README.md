# Scripts Directory

## 📁 Directory Structure

### 🛠️ utils/
ユーティリティスクリプト

- **`check-docs-consistency.sh`** - ドキュメント整合性チェック
- **`create-rust-cli-kiln-symlink.sh`** - rust-cli-kilnシンボリックリンク作成
- **`setup-github-workflow.sh`** - GitHubワークフロー設定

## 🚀 リリース関連スクリプト

リリース関連のスクリプトは `mnt/rust-cli-kiln/release-guide.md` を参照してください。

## 🎯 使用方法

### 日常開発
```bash
# ドキュメント整合性チェック
./scripts/utils/check-docs-consistency.sh

# rust-cli-kilnシンボリックリンク作成
./scripts/utils/create-rust-cli-kiln-symlink.sh

# GitHubワークフロー設定
./scripts/utils/setup-github-workflow.sh
```