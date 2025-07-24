# lawkit ドキュメント

lawkitは、複数の統計法則を使用したデータ品質評価と不正検出のための包括的統計法則分析ツールキットです。

## 主要機能

- **ベンフォード法則**: 会計データ、選挙結果、自然データセットでの不正検出
- **パレート分析**: 80/20の法則分析、販売分析、在庫管理
- **ジップ法則**: テキスト分析、単語頻度分析
- **正規分布**: 品質管理、外れ値検出、プロセス能力評価
- **ポアソン分布**: 事象発生予測、まれな事象分析
- **統合分析**: 複数法則比較、矛盾検出、推奨システム

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
- [はじめ方](user-guide/getting-started_ja.md)
- [設定](user-guide/configuration_ja.md)
- [例](user-guide/examples_ja.md)

### リファレンス
- [CLIリファレンス](reference/cli-reference_ja.md)
- [**API リファレンス**](reference/api-reference_ja.md) - Rust クレート API ドキュメント

### 言語バインディング
- [**統一API リファレンス**](bindings/unified-api_ja.md) - lawkit-python および lawkit-js 言語バインディング

### ガイド
- [アーキテクチャガイド](guides/architecture_ja.md) - システム設計とアーキテクチャの概要
- [統合ガイド](guides/integrations_ja.md)
- [性能ガイド](guides/performance_ja.md)
- [高度な分析](guides/advanced-analysis_ja.md)


### その他
- [FAQ](user-guide/faq_ja.md)

## サポート

質問や問題については、[GitHub Issues](https://github.com/kako-jun/lawkit/issues)にて報告してください。複数言語でのサポートを提供しています。