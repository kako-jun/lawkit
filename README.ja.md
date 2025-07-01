# benf

ベンフォードの法則を使った異常検知CLIツール（日本語数字対応）

## 概要

`benf`は数値データがベンフォードの法則に従っているかを解析し、データ操作や不正を検出するツールです。ベンフォードの法則とは、自然発生的なデータセットにおいて、先頭桁が1である数値が約30.1%、2が17.6%の頻度で現れるという統計法則です。

**独自機能:**
- 🇯🇵 **日本語数字完全対応** (全角数字・漢数字・混在パターン)
- 📊 多様な入力形式対応 (Excel, PDF, Word, HTML等)
- 🌐 URL直接解析とHTML解析
- 🔍 不正検知特化の危険度表示

## 日本語数字対応の詳細

### 対応する数字形式

#### 1. 全角数字
```bash
echo "１２３４５６ ７８９０１２" | benf
```

#### 2. 漢数字（基本）
```bash
echo "一二三四五六七八九" | benf
```

#### 3. 漢数字（位取り付き）
```bash
echo "一千二百三十四 五千六百七十八 九万一千二百" | benf
```

#### 4. 混在パターン
```bash
echo "売上123万円 経費45万6千円 利益78万９千円" | benf
```

### 変換ルール

| 漢数字 | 数値 | 備考 |
|--------|------|------|
| 一 | 1 | 基本数字 |
| 十 | 10 | 十の位 |
| 百 | 100 | 百の位 |
| 千 | 1000 | 千の位 |
| 万 | 10000 | 万の位 |
| 一千二百三十四 | 1234 | 位取り記法 |
| 十二万三千四百 | 123400 | 大きな数値 |

### 特殊ケースの処理

#### ゼロの扱い
```bash
# これらはベンフォード法則の対象外として除外
echo "０００１ 0.123 00456" | benf
```

#### 小数点を含む数値
```bash
# 整数部分の先頭桁のみを使用
echo "12.34 0.567 123.45" | benf
# 結果: 1, 1, 1（先頭桁のみ）
```

#### 負数の処理
```bash
# 絶対値の先頭桁を使用
echo "-123 -456 -789" | benf
# 結果: 1, 4, 7
```

## インストール

### ソースからビルド
```bash
git clone https://github.com/kako-jun/benf
cd benf
cargo build --release
sudo cp target/release/benf /usr/local/bin/
```

### バイナリリリース
[リリースページ](https://github.com/kako-jun/benf/releases)からダウンロード

## 使用例

### 基本的な使い方
```bash
# CSVファイルの解析
benf 売上データ.csv

# WebサイトのデータをURL指定で解析
benf --url https://example.com/財務報告

# パイプでデータを渡す
cat 取引履歴.txt | benf

# JSON形式で結果を出力
benf 経費データ.xlsx --format json
```

### 日本語数字の実例

#### 全角数字の財務データ
```bash
echo "売上：１２３４万円
経費：５６７万円  
利益：６７８万円" | benf
```

#### 漢数字の会計データ
```bash
echo "一千二百万円
三千四百万円
五千六百万円" | benf
```

#### 混在形式の実データ
```bash
# PDFから抽出した和文データ
benf 決算報告書.pdf
```

### 出力例

#### 標準出力（日本語）
```
ベンフォードの法則 解析結果

データセット: 売上データ.csv
解析対象数値: 1,247個
危険度: 高 ⚠️

先頭桁分布:
1: ████████████████████████████ 28.3% (期待値: 30.1%)
2: ████████████████████ 20.1% (期待値: 17.6%)
3: ██████████ 9.8% (期待値: 12.5%)
...

統計検定:
カイ二乗値: 23.45 (p値: 0.003)
平均絶対偏差: 2.1%

判定: 有意な偏差を検出
```

#### JSON出力
```json
{
  "dataset": "売上データ.csv",
  "numbers_analyzed": 1247,
  "risk_level": "HIGH",
  "digits": {
    "1": {"observed": 28.3, "expected": 30.1, "deviation": -1.8},
    "2": {"observed": 20.1, "expected": 17.6, "deviation": 2.5}
  },
  "statistics": {
    "chi_square": 23.45,
    "p_value": 0.003,
    "mad": 2.1
  },
  "verdict": "SIGNIFICANT_DEVIATION"
}
```

## 実用例

### 会計監査
```bash
# 売上データの不正チェック
benf 月次売上.xlsx --threshold high

# 経費精算の異常検知
find . -name "*経費*.csv" -exec benf {} \; | grep "高"
```

### リアルタイム監視
```bash
# 取引ログの監視
tail -f 取引ログ.txt | benf --format json | jq 'select(.risk_level == "HIGH")'
```

### 自動化スクリプト
```bash
#!/bin/bash
# 日次不正検知
結果=$(benf 日次売上.csv --format json)
危険度=$(echo $結果 | jq -r '.risk_level')
if [ "$危険度" = "HIGH" ]; then
    echo $結果 | mail -s "不正検知アラート" admin@company.com
fi
```

## 危険度レベル

| レベル | カイ二乗p値 | 解釈 |
|--------|-------------|------|
| 低 | p > 0.1 | 正常な分布 |
| 中 | 0.05 < p ≤ 0.1 | 軽微な偏差 |
| 高 | 0.01 < p ≤ 0.05 | 有意な偏差 |
| 重大 | p ≤ 0.01 | 操作の強い証拠 |

## よくある使用場面

- **会計監査**: 売上・経費データの人為的操作検出
- **税務調査**: 申告書数値の不自然な偏りを発見  
- **選挙監視**: 得票数・投票率の改ざん疑惑検証
- **保険査定**: 保険金請求額の詐欺パターン検出
- **品質管理**: 製造データの工程異常発見

## コマンドオプション

| オプション | 説明 |
|------------|------|
| `--url <URL>` | URLからデータを取得 |
| `--format <形式>` | 出力形式: text, csv, json, yaml, toml, xml |
| `--quiet` | 最小限の出力（数値のみ） |
| `--verbose` | 詳細な統計情報 |
| `--filter <範囲>` | 数値フィルター (例: `--filter ">=100"`) |
| `--threshold <レベル>` | アラート閾値: low, medium, high, critical |
| `--proxy <URL>` | HTTPプロキシサーバー |
| `--insecure` | SSL証明書検証をスキップ |
| `--timeout <秒>` | リクエストタイムアウト (デフォルト: 30) |
| `--log-level <レベル>` | ログレベル: debug, info, warn, error |
| `--help, -h` | ヘルプを表示 |
| `--version, -V` | バージョンを表示 |

## 対応ファイル形式

| 形式 | 拡張子 | 備考 |
|------|--------|------|
| Microsoft Excel | .xlsx, .xls | スプレッドシートデータ |
| Microsoft Word | .docx, .doc | 文書解析 |
| Microsoft PowerPoint | .pptx, .ppt | プレゼンデータ |
| OpenDocument | .ods, .odt | OpenOffice/LibreOfficeファイル |
| PDF | .pdf | テキスト抽出 |
| CSV/TSV | .csv, .tsv | 構造化データ |
| JSON/XML | .json, .xml | APIレスポンス |
| YAML/TOML | .yaml, .toml | 設定ファイル、データ定義 |
| HTML | .html | Webページ |
| テキスト | .txt | プレーンテキスト |

## 設定

### 環境変数
- `HTTP_PROXY` / `HTTPS_PROXY`: プロキシ設定
- `NO_PROXY`: プロキシ除外リスト

### ログ出力先
- Linux: `~/.local/state/benf/`
- macOS: `~/Library/Logs/benf/`
- Windows: `%APPDATA%\benf\Logs\`

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 正常終了 |
| 1 | 一般エラー |
| 2 | 引数エラー |
| 3 | ファイル/ネットワークエラー |
| 10 | 高リスク検出 |
| 11 | 重大リスク検出 |

## 開発・貢献

[CONTRIBUTING.md](CONTRIBUTING.md)を参照してください。

## ライセンス

MIT ライセンス - [LICENSE](LICENSE)ファイルを参照

## 参考資料

- [ベンフォードの法則 - Wikipedia](https://ja.wikipedia.org/wiki/ベンフォードの法則)
- [不正検知におけるベンフォードの法則の活用](https://example.com/benford-fraud-jp)