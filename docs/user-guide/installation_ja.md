# インストールガイド

このガイドでは、您のシステムにlawkitをインストールする方法を説明します。

## システム要件

- **OS**: Windows、macOS、Linux
- **Rust**: 1.70.0以上（Cargoからインストールする場合）
- **メモリ**: 最低512MB（大きなファイル処理には2GB+を推奨）

## インストール方法

### 1. Cargoからインストール（推奨）

```bash
# 最新バージョンをインストール
cargo install lawkit

# 特定バージョンをインストール
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

您のプラットフォーム用のバイナリを[GitHub Releases](https://github.com/user/lawkit/releases)からダウンロードしてください。

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

## インストールの確認

インストールが成功したことを確認してください:

```bash
# バージョンを確認
lawkit --version

# ヘルプを表示
lawkit --help

# 利用可能な統計法則を表示
lawkit list
```

## 旧バージョンbenfからの移行

既存の`benf`ツールを使用している場合:

```bash
# 旧バージョンの使用方法
benf data.csv

# 新バージョンの同等の使用方法
lawkit benf data.csv
```

100%の互換性が維持されているため、既存のスクリプトをそのまま使用できます。

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
Rustのバージョンが古いことが原因の可能性があります:
```bash
rustup update stable
```

### PATHの設定
バイナリがPATHにない場合:
```bash
# ~/.bashrc または ~/.zshrc に追加
export PATH="$HOME/.cargo/bin:$PATH"
```

### アクセス権エラー
Linuxでアクセス権エラーが発生した場合:
```bash
sudo chown $USER:$USER ~/.cargo/bin/lawkit
chmod +x ~/.cargo/bin/lawkit
```

## 次のステップ

インストールが完了したら、[はじめに](getting-started_ja.md)で基本的な使い方を学んでください。

- [はじめに](getting-started_ja.md) - 基本的な使い方
- [例](examples_ja.md) - 実世界の例
- [CLIリファレンス](../reference/cli-reference_ja.md) - コマンド詳細