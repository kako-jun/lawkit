# benf機能仕様

## 入力方式
- stdin (パイプ)
- コマンドライン引数（文字列データ）
- ファイルパス引数（Excel/PDF/Word等）
- URL引数（HTTP/HTTPS）
- **優先順位**: URL > ファイル > 文字列 > パイプ (複数指定時)

## 出力形式
- デフォルト: 人間可読テキスト + ASCII グラフ (標準出力)
- --format オプション: csv, json, yaml, toml, xml (標準出力)
- --quiet: 数値結果のみ
- --verbose: 詳細統計情報付き
- ファイル保存: リダイレクト使用 (例: benf data.csv --format json > result.json)

## 対応ファイル形式
### Microsoft Office
- Excel (.xlsx, .xls)
- Word (.docx, .doc) 
- PowerPoint (.pptx, .ppt)

### OpenDocument
- Calc (.ods)
- Writer (.odt)

### その他
- PDF (.pdf)
- CSV/TSV
- JSON/XML
- YAML/TOML
- HTML (タグ除去機能付き)

## 国際数字対応
### 日本語数字
- 全角数字（０１２３４５６７８９）
- 漢数字（一二三四五六七八九十百千万）
- 混在パターンの処理

### 中国語金融数字
- 金融数字：壹貳參肆伍陸柒捌玖拾佰仟萬
- 繁体字：萬(繁体字) vs 万(日本語/簡体字)

### その他言語数字
- ヒンディー語：देवनागरी文字（०१२३४५६७८९）
- アラビア語：۰۱۲۳۴۵۶۷۸۹
- ペルシャ語：۰۱۲۳۴۵۶۷۸۹

## 多言語出力
- 英語 (English)
- 日本語 (Japanese)
- 中国語 (Chinese)
- ヒンディー語 (Hindi)
- アラビア語 (Arabic)
- 自動言語検出（LANG環境変数）+ 手動指定（--lang）

## ベンフォード法則統計機能
- 先頭桁分布分析
- カイ二乗検定
- p値計算
- 4段階リスク評価 (Low/Medium/High/Critical)

## ベンフォード法則適用限界
### 適用条件
- 数値数: 最低5個（警告表示）、推奨30個以上
- データ特性: 自然発生的な数値、複数桁、幅広い範囲

### 不適切なデータ
- 制約のある数値: 身長(140-200cm)、年齢(0-120歳)
- 人工的数値: 連番、ID、電話番号
- 小規模・単一ソース・丸め込みデータ
- 対数正規分布に従わないデータ

## 高度なCLIオプション
- `--filter <RANGE>`: 数値フィルタリング（例: --filter ">=100"、"100-500"）
- `--threshold <LEVEL>`: アラート閾値カスタマイズ (auto/low/medium/high/critical/0.05)
- `--min-count <N>`: 最小数値数要件設定
- `--proxy <URL>`: HTTPプロキシサーバー指定
- `--insecure`: SSL証明書検証スキップ
- `--timeout <SECONDS>`: リクエストタイムアウト設定 (1-3600秒)
- `--user-agent <STRING>`: カスタムUser-Agent設定

## 終了コード
- 0: Low/Medium (正常)
- 10: High (高リスク)
- 11: Critical (致命的リスク)