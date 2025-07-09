# lawkit ドキュメント

lawkitは複数の統計法則を用いてデータ品質・不正検知を行うCLIツールキットです。

## 主な機能

- **ベンフォード法則**: 会計データ、選挙結果、自然データの不正検知
- **パレート法則**: 80/20分析、売上分析、在庫管理
- **ジップ法則**: テキスト分析、単語頻度分析
- **正規分布**: 品質管理、異常値検出、工程能力評価
- **ポアソン分布**: イベント発生予測、稀少事象分析
- **統合分析**: 複数法則の比較、矛盾検出、推奨システム

## クイックスタート

```bash
# インストール
cargo install lawkit

# ベンフォード法則分析
lawkit benf data.csv

# パレート分析
lawkit pareto sales.csv

# 複数法則比較
lawkit analyze data.csv --laws benf,pareto,normal
```

## ドキュメント

### ユーザーガイド
- [インストール](user-guide/installation_ja.md)
- [はじめに](user-guide/getting-started_ja.md)  
- [設定](user-guide/configuration_ja.md)
- [使用例](user-guide/examples_ja.md)

### リファレンス
- [CLIリファレンス](reference/cli-reference_ja.md)

### ガイド
- [アーキテクチャガイド](guides/architecture_ja.md) - システム設計とアーキテクチャ概要
- [統合機能](guides/integrations_ja.md)
- [パフォーマンス](guides/performance_ja.md)

### プロジェクト
- [変更履歴](../CHANGELOG.md)
- [ロードマップ](project/roadmap_ja.md)

### その他
- [FAQ](user-guide/faq_ja.md)

## サポート

質問や問題がある場合は、[GitHub Issues](https://github.com/user/lawkit/issues)で報告してください。日本語でのサポートも提供しています。