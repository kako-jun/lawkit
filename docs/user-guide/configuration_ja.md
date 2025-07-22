# 設定ガイド

## 概要

`lawkit`は適切なデフォルト設定ですぐに使えるように設計されていますが、さまざまなユースケースに対応するためにいくつかの設定オプションを提供しています。

## コマンドラインオプション

### グローバルオプション

```bash
# 出力形式
lawkit benf data.csv --format json
lawkit benf data.csv --format yaml
lawkit benf data.csv --format csv
lawkit benf data.csv --format toml
lawkit benf data.csv --format xml

# 国際数字サポート（自動認識）
echo "１２３４５６" | lawkit benf      # 日本語数字
echo "一千二百三十四" | lawkit benf    # 中国語数字

# 詳細度
lawkit benf data.csv --quiet     # 最小出力
lawkit benf data.csv --verbose   # 詳細出力
```

### 分析オプション

```bash
# 闾値付きパレート分析
lawkit pareto data.csv --threshold 0.8

# 複数法則分析
lawkit analyze data.csv --laws benford,pareto,normal

# 焦点を当てた分析
lawkit analyze data.csv --laws benford --focus accuracy

# 目的固有の分析
lawkit analyze data.csv --laws all --purpose audit

# 推奨事項
lawkit analyze data.csv --laws all --recommend
```

## 出力形式

### サポートされる形式

| 形式 | 説明 | 最適用途 |
|--------|-------------|----------|
| `text` | 人間が読める（デフォルト） | ターミナル表示 |
| `json` | 機械読み取り可能 | API、自動化 |
| `csv` | 表形式データ | スプレッドシート |
| `yaml` | 構造化設定 | 設定ファイル |
| `toml` | Rustフレンドリー | Rust統合 |
| `xml` | レガシーシステム | エンタープライズ |

### 形式例

#### JSON出力
```bash
lawkit benf data.csv --format json
```
```json
{
  "dataset": "data.csv",
  "numbers_analyzed": 1000,
  "risk_level": "Low",
  "mean_absolute_deviation": 2.3,
  "chi_square_p_value": 0.85
}
```

#### CSV出力
```bash
lawkit benf data.csv --format csv
```
```csv
dataset,numbers_analyzed,risk_level,mad,chi_square_p
data.csv,1000,Low,2.3,0.85
```

## 多言語サポート

### サポート言語

- **英語** (`en`) - デフォルト
- **日本語** (`ja`) - 日本語
- **中国語** (`zh`) - 中文
- **ヒンディー語** (`hi`) - हिन्दी
- **アラビア語** (`ar`) - العربية

### 国際数字サポート

`lawkit`はさまざまな数字形式を自動認識します：

```bash
# 日本語数字
echo "１２３４ ５６７８" | lawkit benf

# 中国語金融数字
echo "壹万贰千 三千四百" | lawkit benf

# 混合形式
echo "123 ４５６ 七八九" | lawkit benf
```

## 統合分析

### 複数法則分析設定

```bash
# 特定の法則を選択
lawkit analyze data.csv --laws benford,pareto,normal

# 特定の分析タイプに焦点を当てる
lawkit analyze data.csv --laws benford --focus accuracy

# 目的固有の分析
lawkit analyze data.csv --laws all --purpose audit

# 推奨モード
lawkit analyze data.csv --laws all --recommend

# 検証モード
lawkit validate data.csv --laws all

# 診断モード
lawkit diagnose data.csv --laws all
```

### 分析目的

| 目的 | 最適法則 | ユースケース |
|---------|-----------|----------|
| `audit` | ベンフォード + 正規 | データ品質監査 |
| `fraud` | ベンフォード + ポアソン | 不正検出 |
| `business` | パレート + ジップ | ビジネス分析 |
| `research` | 全法則 | 一般分析 |

## バッチ処理

```bash
# 複数ファイルを処理
for file in *.csv; do
  lawkit benf "$file" --format json > "results_${file%.csv}.json"
done

# 異なる法則で分析
lawkit analyze data1.csv --laws benford --format json
lawkit analyze data2.csv --laws pareto --format json
lawkit analyze data3.csv --laws normal --format json
```

## パフォーマンスチューニング

### 大規模データセット

```bash
# パフォーマンス向上のために静音モードを使用
lawkit benf large_data.csv --quiet

# 特定の分析に焦点を当てる
lawkit analyze large_data.csv --laws benford --quiet
```

### メモリ管理

- 1GB以上のファイル: データ前処理を検討
- 最小メモリ使用のために`--quiet`を使用
- stdin入力でのストリーム処理

## トラブルシューティング

### よくある問題

1. **"データ不足"** - より多くのデータを提供するかファイル形式を確認
2. **"数値が見つからない"** - データ形式とエンコーディングを確認
3. **"形式エラー"** - ファイル形式が内容と一致しているか確認

### デバッグモード

```bash
# 詳細ログを有効化
lawkit benf data.csv --verbose

# データ解析を確認
lawkit benf data.csv --format json | jq '.numbers_analyzed'
```

## 将来の設定機能

以下の機能が将来のバージョンで計画されています：

- 設定ファイルサポート (`lawkit.toml`)
- 環境変数設定
- カスタム闾値設定
- プロファイルベースの設定
- データフィルタリングオプション
- 高度な分析オプション

## 次のステップ

- [例](examples.md) - 実世界の設定例
- [CLIリファレンス](../reference/cli-reference.md) - 完全なコマンドドキュメント
- [統合ガイド](../guides/integrations.md) - CI/CD自動化