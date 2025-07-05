# インストールガイド

このガイドでは、lawkitをシステムにインストールする方法を説明します。

## 必要な環境

- **OS**: Windows、macOS、Linux
- **Rust**: 1.70.0以上（Cargoからインストールする場合）
- **メモリ**: 最低512MB（大きなファイル処理には2GB以上推奨）

## インストール方法

### 1. Cargoからインストール（推奨）

```bash
# 最新版をインストール
cargo install lawkit

# 特定のバージョンをインストール
cargo install lawkit --version 2.0.0
```

### 2. ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/user/lawkit.git
cd lawkit

# ビルドとインストール
cargo build --release
cargo install --path .
```

### 3. バイナリダウンロード

[GitHub Releases](https://github.com/user/lawkit/releases)から、お使いのプラットフォーム用のバイナリをダウンロードしてください。

#### Windows
```powershell
# PowerShellで実行
Invoke-WebRequest -Uri "https://github.com/user/lawkit/releases/latest/download/lawkit-windows.zip" -OutFile "lawkit.zip"
Expand-Archive lawkit.zip
```

#### macOS
```bash
# Homebrewでインストール（予定）
brew install lawkit

# または手動ダウンロード
curl -L https://github.com/user/lawkit/releases/latest/download/lawkit-macos.tar.gz | tar xz
```

#### Linux
```bash
# 手動ダウンロード
curl -L https://github.com/user/lawkit/releases/latest/download/lawkit-linux.tar.gz | tar xz
sudo mv lawkit /usr/local/bin/
```

## インストール確認

インストールが成功したか確認します：

```bash
# バージョン確認
lawkit --version

# ヘルプ表示
lawkit --help

# 利用可能な統計法則を表示
lawkit list
```

## 旧版benfからの移行

既存の`benf`ツールをお使いの場合：

```bash
# 旧版の使用方法
benf data.csv

# 新版での同等の使用方法
lawkit benf data.csv
```

互換性は100%保持されているため、既存のスクリプトをそのまま使用できます。

## アップデート

### Cargoでインストールした場合
```bash
cargo install lawkit --force
```

### バイナリでインストールした場合
新しいバイナリをダウンロードして置き換えてください。

## アンインストール

### Cargoでインストールした場合
```bash
cargo uninstall lawkit
```

### 手動でインストールした場合
```bash
# バイナリの場所を確認
which lawkit

# ファイルを削除
sudo rm /usr/local/bin/lawkit
```

## トラブルシューティング

### コンパイルエラー
Rustのバージョンが古い場合があります：
```bash
rustup update stable
```

### パス設定
バイナリにパスが通っていない場合：
```bash
# ~/.bashrcまたは~/.zshrcに追加
export PATH="$HOME/.cargo/bin:$PATH"
```

### 権限エラー
Linuxで権限エラーが発生する場合：
```bash
sudo chown $USER:$USER ~/.cargo/bin/lawkit
chmod +x ~/.cargo/bin/lawkit
```

## 次のステップ

インストールが完了したら、[はじめに](getting-started_ja.md)で基本的な使用方法を学んでください。