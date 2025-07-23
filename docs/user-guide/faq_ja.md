# よくある質問

## 一般的な質問

### lawkitとは何ですか？

lawkitは、ベンフォードの法則、パレートの原理、ジップの法則、正規分布、ポアソン分布を含む複数の統計法則を実装した包括的な統計分析ツールキットです。不正検出、データ品質評価、ビジネス分析、科学研究に設計されています。

### 他の統計ツールと何が違うのですか？

- **多法則統合**: 単一の分析で複数の統計法則を比較
- **国際対応**: 複数の言語と形式の数値を処理
- **組み込みリスク評価**: 自動異常検出とリスク評価
- **包括的入力対応**: CSV、Excel、PDF、Word、JSONなどから読み取り
- **プロフェッショナル出力**: JSON、CSV、YAML、XMLを含む複数の出力形式

### lawkitは無料で使えますか？

はい、lawkitはMITライセンスの下でリリースされたオープンソースソフトウェアです。個人および商用目的で自由に使用できます。

## インストールとセットアップ

### lawkitのインストール方法は？

**オプション 1: バイナリのダウンロード**
[GitHub Releases](https://github.com/kako-jun/lawkit/releases)からビルド済みバイナリをダウンロードします。

**オプション 2: ソースからビルド**
```bash
git clone https://github.com/kako-jun/lawkit.git
cd lawkit
cargo build --release
```

**オプション 3: ライブラリとして**
`Cargo.toml`に追加:
```toml
[dependencies]
lawkit-core = "2.0"
```

### システム要件は？

- **オペレーティングシステム**: Linux、macOS、Windows
- **メモリ**: 最低512MB RAM（大きなデータセットにはさらに必要）
- **ディスク容量**: インストールに50MB
- **依存関係**: なし（静的リンクバイナリ）

### 「command not found」エラーが出るのはなぜですか？

lawkitバイナリがシステムのPATHにあることを確認してください。ダウンロード後:

```bash
# 実行可能にする (Unix/Linux/macOS)
chmod +x lawkit

# PATHに移動（例）
sudo mv lawkit /usr/local/bin/

# またはシェルプロファイルにPATHを追加
export PATH="/path/to/lawkit:$PATH"
```

## 使用に関する質問

### lawkitはどのファイル形式をサポートしますか？

**入力形式:**
- スプレッドシート: CSV、TSV、Excel (.xlsx, .xls)、OpenDocument (.ods)
- 文書: PDF、Word (.docx)、PowerPoint (.pptx)、プレーンテキスト
- データ: JSON、YAML、TOML、XML
- Web: HTML（テーブル抽出）

**出力形式:**
- text、json、csv、yaml、toml、xml

### CSVファイルの特定の列のデータを分析するには？

lawkitは自動的にすべての列から数値データを抽出します。特定の列に対しては、データを事前処理してください:

```bash
# 標準ツールを使って特定の列を抽出
cut -d',' -f2 data.csv | lawkit benf

# より複雑な抽出にはawkを使用
awk -F',' '{print $2}' data.csv | lawkit pareto
```

### 信頼できる分析にはどの程度のデータポイントが必要ですか？

最小要件は統計法則で異なります:
- **ベンフォードの法則**: 5以上（推奨: 100以上）
- **パレート分析**: 5以上（推奨: 20以上）
- **ジップの法則**: 5以上（推奨: 50以上）
- **正規分布**: 8以上（推奨: 30以上）
- **ポアソン分布**: 10以上（推奨: 50以上）

### リスク評価は何を意味しますか？

lawkitは結果をリスクレベルで分類します:
- **Low**: データは期待される統計パターンに従っている
- **Medium**: いくらかの偏差、調査の価値あり
- **High**: 有意な偏差、問題の可能性あり
- **Critical**: 極端な偏差、早急な注意が必要

闾値のカスタマイズ:
```bash
lawkit benf --threshold high data.csv
```

## 統計分析に関する質問

### ベンフォードの法則はいつ使うべきですか？

ベンフォードの法則は以下に適しています:
- **金融不正検出**: 取引金額、会計データ
- **データ品質評価**: 自然に発生する数値データ
- **科学的検証**: 実験測定値
- **選挙監査**: 投票数と人口統計

**不適合:**
- 割り振り番号（電話番号、ID）
- 制約された範囲（パーセンテージ、評価）
- 一様分布

### パレート分析とジップの法則の違いは何ですか？

**パレート分析（80/20の原則）:**
- ビジネスアプリケーションに焦点
- 不平等のためのジニ係数を計算
- ビジネス洞察と推奨事項を提供
- 最適用途: 売上分析、顧客セグメンテーション、リソース配分

**ジップの法則（べき乗分布）:**
- 頻度分布に焦点
- 順位と頻度の関係を分析
- 数値データ分析をサポート
- 最適用途: 言語分析、都市人口、ウェブサイトトラフィック

### 正規分布テストの精度はどの程度ですか？

lawkitは統計検証を伴う正規性テストを実装しています。分析は以下を提供します:
- 正規性評価のための統計指標
- 信頼区間と有意性テスト
- 外れ値検出機能
- 品質管理指標

### ポアソン分布分析は何を教えてくれますか？

ポアソン分析は以下に有用です:
- **イベントカウント**: 単位あたりの欠陥数、1時間あたりのコール数
- **稀なイベント**: 事故、機器故障
- **品質管理**: プロセス監視
- **容量計画**: サーバー負荷、顧客の到着

分析は以下を提供します:
- λパラメータ（平均率）
- 統計検証
- イベント確率評価
- 品質指標

## 国際化と言語サポート

### 国際数字サポートはどのように機能しますか？

lawkitはさまざまな数値形式を自動認識します:

```bash
# これらはすべて1234.56として認識されます:
echo "1,234.56" | lawkit benf    # 英語
echo "１，２３４．５６" | lawkit benf  # 日本語数字（自動検出）
echo "१,२३४.५६" | lawkit benf    # ヒンディー語数字（自動検出）
```

### 英語以外の言語のテキストを分析できますか？

はい！lawkitは入力データ内の国際数字形式の自動言語検出を備えたUnicodeテキスト分析をサポートしています。

### lawkitはどの言語をサポートしていますか？

lawkitは国際数字形式の自動認識により、すべての分析で統一された英語出力を提供します:
```bash
# 英語出力（統一）
lawkit benf data.csv

# 国際数字は自動的に認識されます
echo "１２３４５６" | lawkit benf      # 日本語数字
echo "一千二百三十四" | lawkit benf    # 中国語数字
echo "१२३४५६" | lawkit benf        # ヒンディー語数字
echo "١٢٣٤٥٦" | lawkit benf        # アラビア語数字
```

## 統合と高度な機能

### 多法則比較はどのように機能しますか？

`analyze`コマンドは複数の統計法則でデータを分析します:

```bash
# 特定の法則で分析
lawkit analyze --laws benf,pareto data.csv

# 適用可能なすべての法則で分析
lawkit analyze --laws all data.csv

# 推奨事項を取得
lawkit analyze --laws all --recommend data.csv
```

機能には以下が含まれます:
- **矛盾検出**: 相反する結果を特定
- **信頼度スコアリング**: 各分析の信頼性を評価
- **推奨事項**: 最も適切な法則を提案
- **メタ分析**: 複数の觖点からの洞察を結合

### 自分のソフトウェアでlawkitを使用できますか？

はい！`lawkit-core`ライブラリを使用してください:

```rust
use lawkit_core::laws::benford::BenfordResult;

fn analyze_data(numbers: &[f64]) {
    let result = BenfordResult::analyze(numbers);
    println!("Chi-square: {}", result.chi_square);
}
```

### lawkitをCI/CDパイプラインと統合するには？

以下の例は[統合ガイド](../guides/integrations_ja.md)を参照してください:
- GitHub Actions
- GitLab CI
- Jenkins
- Dockerコンテナ

## パフォーマンスとトラブルシューティング

### 大きなデータセットでlawkitが遅いです。パフォーマンスを改善するには？

**最適化戦略:**
```bash
# 高速処理のために静音モードを使用
lawkit benf --quiet large_data.csv

# 適切な闾値を使用
lawkit benf --threshold medium large_data.csv

# 効率的な出力形式を選択
lawkit benf --format json large_data.csv  # テキストより高速
```

### 「insufficient data」エラーが出ます。どうすべきでしょうか？

このエラーはデータセットが最小要件を満たさない場合に発生します:

```bash
# データサイズを確認
wc -l data.csv

# データサイズに適した法則を使用
lawkit pareto small_data.csv  # normalよりも低い要件
```

### 分析結果が不正確に思えます。何が原因でしょうか？

**よくある問題:**
1. **間違った統計法則**: すべてのデータがすべての法則に適合するわけではない
2. **データの前処理が必要**: ヘッダーの除去、外れ値のフィルタリング
3. **データ不足**: 信頼できる分析にはデータポイントが少なすぎる
4. **間違った形式**: 数値データが適切にフォーマットされていることを確認

**デバッグ手順:**
```bash
# 詳細な情報のための詳細出力
lawkit benf --verbose data.csv

# 異なる法則でデータを確認
lawkit analyze --laws all data.csv

# データ形式を検証
head -10 data.csv
```

### リアルタイムでデータを分析できますか？

はい、lawkitはパイプ入力をサポートしています:

```bash
# 他のコマンドからデータをパイプ
tail -f logfile.txt | grep "amount:" | lawkit benf

# 継続的データを処理
while true; do
    get_latest_data | lawkit benf --quiet
    sleep 60
done
```

## 高度な使用法

### テストデータを生成するには？

```bash
# テスト用のサンプルデータを生成
lawkit generate --samples 1000 | lawkit benf

# 生成してファイルに保存
lawkit generate --samples 500 > test_data.csv
```

### データ品質を検証するには？

```bash
# 複数の法則を使用して検証
lawkit validate --laws all data.csv

# 特定の領域に焦点を当てる
lawkit validate --laws benf,pareto --focus fraud-detection data.csv
```

### データの問題を診断するには？

```bash
# データの問題を診断
lawkit diagnose --laws all data.csv

# 分析目的を指定
lawkit diagnose --laws all --purpose quality-assessment data.csv
```

## エラーメッセージ

### "Failed to parse input data"

このエラーは通常以下を意味します:
- 入力に非数値データが含まれている
- ファイル形式が不正しい
- エンコーディングの問題

**解決策:**
```bash
# ファイルエンコーディングを確認
file data.csv

# CSV形式を検証
csvlint data.csv

# 数値列のみを抽出
cut -d',' -f2 data.csv | lawkit benf
```

### "No statistical law applicable"

このエラーは以下の場合に発生します:
- データセットが小さすぎる
- データが実装されたいずれの法則にも適合しない
- すべての法則が適用可能性テストに失敗した

**解決策:**
```bash
# データ特性を確認
lawkit analyze --laws all --verbose data.csv

# 異なる闾値で試す
lawkit benf --threshold low data.csv
```

### "Permission denied" or "Access denied"

ファイルアクセス権の問題:
```bash
# ファイルアクセス権を確認
ls -la data.csv

# ファイルを読み取り可能にする
chmod 644 data.csv

# ファイルが存在するか確認
test -f data.csv && echo "File exists" || echo "File not found"
```

## ヘルプの取得

### さらにサポートを受けるには？

- **ドキュメント**: [docs/](../index_ja.md)
- **問題報告**: [GitHub Issues](https://github.com/kako-jun/lawkit/issues)
- **ディスカッション**: [GitHub Discussions](https://github.com/kako-jun/lawkit/discussions)
- **セルフテスト**: `lawkit selftest`を実行してインストールを検証

### バグを報告するには？

1. GitHubの既存の問題を確認
2. 最小限の再現ケースを提供
3. システム情報とlawkitバージョンを含める
4. `lawkit selftest --verbose`を実行して出力を含める

### 新機能をリクエストするには？

1. GitHub Discussionでアイデアを議論
2. ユースケースと期待される動作を説明
3. 実装への貢献を検討
4. [貢献ガイド](../CONTRIBUTING.md)を確認

### コミュニティやフォーラムはありますか？

- **GitHub Discussions**: 一般的な質問とアイデア
- **GitHub Issues**: バグ報告と機能リクエスト
- **Email**: 機密な問題に関する直接連絡

コミュニティからの貢献とフィードバックを歓迎します！