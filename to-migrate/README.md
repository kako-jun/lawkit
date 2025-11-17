# to-migrate - 別リポジトリ移行候補

**作成日**: 2025-11-16
**理由**: lawkitリブート - モノリポ複雑化解消

---

## 📦 **移行予定のディレクトリ**

### 1. **lawkit-js/** → `https://github.com/kako-jun/lawkit-js`
- **内容**: JavaScript/TypeScript バインディング
- **パッケージ**: npm (@kako-jun/lawkit)
- **状態**: 別リポジトリ作成済み
- **移行方法**:
  ```bash
  # lawkit-jsリポジトリをクローン
  git clone https://github.com/kako-jun/lawkit-js.git

  # このディレクトリの内容をコピー
  cp -r to-migrate/lawkit-js/* lawkit-js/

  # コミット＆プッシュ
  cd lawkit-js
  git add .
  git commit -m "Initial migration from lawkit monorepo"
  git push
  ```

### 2. **lawkit-python/** → `https://github.com/kako-jun/lawkit-python`
- **内容**: Python バインディング (PyO3)
- **パッケージ**: PyPI (lawkit)
- **状態**: 別リポジトリ作成済み
- **移行方法**:
  ```bash
  # lawkit-pythonリポジトリをクローン
  git clone https://github.com/kako-jun/lawkit-python.git

  # このディレクトリの内容をコピー
  cp -r to-migrate/lawkit-python/* lawkit-python/

  # コミット＆プッシュ
  cd lawkit-python
  git add .
  git commit -m "Initial migration from lawkit monorepo"
  git push
  ```

---

## 🎯 **移行の目的**

### **問題**:
- モノリポ構造が複雑化
- lawkit-python/lawkit-cliでの出力ファイル名衝突
- ワークスペース管理の認知負荷増大

### **解決策**:
- 各言語バインディングを独立したリポジトリに分離
- lawkitコア（Rust CLI）を単一リポジトリに集中
- diffxプロジェクトと同様のシンプル構造を採用

---

## ✅ **移行後の構造**

### **lawkit (このリポジトリ)**:
```
lawkit/
├── lawkit-core/      # Rust コアライブラリ
├── lawkit-cli/       # Rust CLI
├── .claude/          # Claude設定
└── to-migrate/       # この一時ディレクトリ
```

### **lawkit-js (別リポジトリ)**:
```
lawkit-js/
├── src/              # NAPI Rustコード
├── index.d.ts        # TypeScript型定義
├── package.json      # npm設定
└── examples.ts       # サンプルコード
```

### **lawkit-python (別リポジトリ)**:
```
lawkit-python/
├── src/              # PyO3 Rustコード
├── python/           # Pythonラッパー
├── pyproject.toml    # maturin設定
└── tests/            # Pythonテスト
```

---

## 📝 **移行チェックリスト**

- [x] to-migrate/ディレクトリ作成
- [x] lawkit-js, lawkit-pythonをgit mvで移動
- [x] Cargo.tomlワークスペースから除外
- [ ] ビルド確認（出力ファイル名衝突解消）
- [ ] 別リポジトリへの実際のコピー（手動）
- [ ] CIの更新（必要に応じて）
- [ ] ドキュメント更新

---

## 🔄 **diffxとの整合性**

diffxプロジェクトも同様のリブートを実施中：
- **diffx**: Rustコア + CLI
- **diffx-js**: 別リポジトリ（予定）
- **diffx-python**: 別リポジトリ（予定）

lawkitもこの構造に合わせることで、3兄弟プロジェクト（diffx, lawkit, diffai）の保守性を向上。

---

**注意**: このディレクトリは一時的な移行用です。移行完了後は削除予定。
