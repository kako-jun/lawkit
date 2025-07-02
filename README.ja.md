# benf

ベンフォードの法則を使った異常検知CLIツール（国際数字対応：日本語、中国語、ヒンディー語、アラビア語）

## 概要

`benf`は数値データがベンフォードの法則に従っているかを解析し、データ操作や不正を検出するツールです。ベンフォードの法則とは、自然発生的なデータセットにおいて、**先頭桁（最初の桁）**が1である数値が約30.1%、2が17.6%の頻度で現れるという統計法則です。

**注意**: このツールは各数値の**先頭桁のみ**を解析対象とし、数値全体の並びは解析しません。

**独自機能:**
- 🌍 **国際数字完全対応**: 英語、日本語（全角・漢数字）、中国語（中文数字）、ヒンディー語（हिन्दी अंक）、アラビア語（الأرقام العربية）
- 📊 多様な入力形式対応 (Microsoft Excel, Word, PowerPoint, PDF等)
- 🌐 URL直接解析とHTML解析
- 🔍 不正検知特化の危険度表示

## 国際数字対応の詳細

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
# 整数部分が1以上の数値のみ解析対象
echo "12.34 0.567 123.45" | benf
# 結果: 1, (除外), 1（1未満の数値は除外）
```

#### 負数の処理
```bash
# 絶対値の先頭桁を使用
echo "-123 -456 -789" | benf
# 結果: 1, 4, 7
```

### 中国語数字互換性

現在の実装は日本の漢字と同じ基本的な中国語数字をサポートしています：

#### サポート済み（基本形式）
- 一二三四五六七八九 (1-9) - 日本語と同じ
- 十百千 (10, 100, 1000) - 位置マーカー

#### 計画中のサポート
- **金融形式**: 壹貳參肆伍陸柒捌玖 (不正防止バリアント)
- **伝統的**: 萬 (10,000) vs 日本語万
- **地域バリアント**: 繁体字vs簡体字中国語

### ヒンディー語数字（हिन्दी अंक）
```bash
# デーヴァナーガリー数字
echo "१२३४५६ ७८९०१२" | benf --lang hi
```

### アラビア語数字（الأرقام العربية）
```bash  
# 東アラビア・インド数字
echo "١٢٣٤٥٦ ٧٨٩٠١٢" | benf --lang ar
```

### その他の数字システム（将来サポート）

#### 追加スクリプト（計画中）
- **ペルシア語**: ۰۱۲۳۴۵۶۷۸۹ (イラン、アフガニスタン)
- **ベンガル語**: ০১২৩৪৫৬৭৮৯ (バングラデシュ)
- **タミル語**: ௦௧௨௩௪௫௬௭௮௯ (タミル・ナードゥ)
- **タイ語**: ๐๑๒๓๔๕๖๗๘๙ (タイ)
- **ミャンマー語**: ၀၁၂၃၄၅၆၇၈၉ (ミャンマー)

> **注意**: 国際数字サポートはユーザー需要に基づいて継続的に拡張されています。現在の優先度: 日本語/中国語/ヒンディー語/アラビア語の財務文書分析。

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

## 実務での活用例

benfはUnix哲学に従い、標準的なUnixツールと組み合わせて複数ファイルの処理に優れています：

### 会計監査ワークフロー

```bash
# 四半期財務監査 - すべてのExcelレポートをチェック
find ./2024年第4四半期 -name "*.xlsx" | while read file; do
    echo "監査中: $file"
    benf "$file" --filter ">=1000" --threshold critical --verbose
    echo "---"
done

# 月次経費レポートの検証
for dept in 経理 マーケティング 営業; do
    echo "部署: $dept"
    find "./経費/$dept" -name "*.xlsx" -exec benf {} --format json \; | \
    jq '.risk_level' | sort | uniq -c
done

# 税務書類の検証（高精度分析）
find ./税務申告 -name "*.pdf" | parallel benf {} --min-count 50 --format csv | \
awk -F, '$3=="Critical" {print "🚨 重大:", $1}'
```

### 自動監視・アラート

```bash
# 会計システム出力の日次監視スクリプト
#!/bin/bash
ALERT_EMAIL="audit@company.com"
find /exports/daily -name "*.csv" -newer /var/log/last-benf-check | while read file; do
    benf "$file" --format json | jq -r 'select(.risk_level=="Critical" or .risk_level=="High") | "\(.dataset): \(.risk_level)"'
done | mail -s "日次ベンフォード・アラート" $ALERT_EMAIL

# 継続的統合による不正検知
find ./アップロード済みレポート -name "*.xlsx" -mtime -1 | \
xargs -I {} sh -c 'benf "$1" || echo "不正アラート: $1" >> /var/log/fraud-alerts.log' _ {}

# inotifyによるリアルタイムフォルダ監視
inotifywait -m ./financial-uploads -e create --format '%f' | while read file; do
    if [[ "$file" =~ \.(xlsx|csv|pdf)$ ]]; then
        echo "$(date): 分析中 $file" >> /var/log/benf-monitor.log
        benf "./financial-uploads/$file" --threshold high || \
        echo "$(date): アラート - 疑わしいファイル: $file" >> /var/log/fraud-alerts.log
    fi
done
```

### 大規模データ処理

```bash
# コンプライアンス監査のための企業ファイルシステム全体処理
find /corporate-data -type f \( -name "*.xlsx" -o -name "*.csv" -o -name "*.pdf" \) | \
parallel -j 16 'echo "{}: $(benf {} --format json 2>/dev/null | jq -r .risk_level // "エラー")"' | \
tee compliance-audit-$(date +%Y%m%d).log

# アーカイブ分析 - 過去データの効率的処理
find ./アーカイブ/2020-2024 -name "*.xlsx" -print0 | \
xargs -0 -n 100 -P 8 -I {} benf {} --filter ">=10000" --format csv | \
awk -F, 'BEGIN{OFS=","} NR>1 && $3~/High|Critical/ {sum++} END{print "高リスクファイル数:", sum}'

# 進捗追跡付きネットワークストレージスキャン
total_files=$(find /nfs/financial-data -name "*.xlsx" | wc -l)
find /nfs/financial-data -name "*.xlsx" | nl | while read num file; do
    echo "[$num/$total_files] 処理中: $(basename "$file")"
    benf "$file" --format json | jq -r '"ファイル: \(.dataset), リスク: \(.risk_level), 数値数: \(.numbers_analyzed)"'
done | tee network-scan-report.txt
```

### 高度なレポート・分析

```bash
# 部署別リスク分布分析
for dept in */; do
    echo "=== 部署: $dept ==="
    find "$dept" -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | \
    jq -r '.risk_level' | sort | uniq -c | awk '{print $2": "$1" ファイル"}'
    echo
done

# 時系列リスク分析（日付順ファイル必要）
find ./月次レポート -name "202[0-4]-*.xlsx" | sort | while read file; do
    month=$(basename "$file" .xlsx)
    risk=$(benf "$file" --format json 2>/dev/null | jq -r '.risk_level // "N/A"')
    echo "$month,$risk"
done > risk-timeline.csv

# 統計サマリー生成
{
    echo "ファイル,リスクレベル,数値数,カイ二乗,p値"
    find ./監査サンプル -name "*.xlsx" | while read file; do
        benf "$file" --format json 2>/dev/null | \
        jq -r '"\(.dataset),\(.risk_level),\(.numbers_analyzed),\(.statistics.chi_square),\(.statistics.p_value)"'
    done
} > 統計分析.csv

# 期間比較分析
echo "第3四半期 vs 第4四半期のリスクレベル比較..."
q3_high=$(find ./2024年第3四半期 -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | jq -r 'select(.risk_level=="High" or .risk_level=="Critical")' | wc -l)
q4_high=$(find ./2024年第4四半期 -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | jq -r 'select(.risk_level=="High" or .risk_level=="Critical")' | wc -l)
echo "第3四半期高リスクファイル: $q3_high"
echo "第4四半期高リスクファイル: $q4_high"
echo "変化: $((q4_high - q3_high))"
```

### 他ツールとの連携

```bash
# データ検証用Gitプレコミットフック
#!/bin/bash
# .git/hooks/pre-commit
changed_files=$(git diff --cached --name-only --diff-filter=A | grep -E '\.(xlsx|csv|pdf)$')
for file in $changed_files; do
    if ! benf "$file" --min-count 10 >/dev/null 2>&1; then
        echo "⚠️  警告: $file に疑わしいデータパターンが含まれている可能性があります"
        benf "$file" --format json | jq '.risk_level'
    fi
done

# データベースインポート検証
psql -c "COPY suspicious_files FROM STDIN CSV HEADER" <<< $(
    echo "ファイル名,リスクレベル,カイ二乗,p値"
    find ./インポートデータ -name "*.csv" | while read file; do
        benf "$file" --format json 2>/dev/null | \
        jq -r '"\(.dataset),\(.risk_level),\(.statistics.chi_square),\(.statistics.p_value)"'
    done
)

# Slack/Teams webhook連携
high_risk_files=$(find ./日次アップロード -name "*.xlsx" -mtime -1 | \
    xargs -I {} benf {} --format json 2>/dev/null | \
    jq -r 'select(.risk_level=="High" or .risk_level=="Critical") | .dataset')

if [ -n "$high_risk_files" ]; then
    curl -X POST -H 'Content-type: application/json' \
    --data "{\"text\":\"🚨 高リスクファイルが検出されました:\n$high_risk_files\"}" \
    $SLACK_WEBHOOK_URL
fi
```

### 専門分野での活用

```bash
# 選挙監査（投票数チェック）
find ./選挙データ -name "*.csv" | parallel benf {} --min-count 100 --threshold low | \
grep -E "(HIGH|CRITICAL)" > 選挙異常.txt

# 科学データ検証
find ./研究データ -name "*.xlsx" | while read file; do
    lab=$(dirname "$file" | xargs basename)
    result=$(benf "$file" --format json | jq -r '.risk_level')
    echo "$lab,$file,$result"
done | grep -E "(High|Critical)" > データ整合性問題.csv

# サプライチェーン請求書検証
find ./請求書/2024 -name "*.pdf" | parallel 'vendor=$(dirname {} | xargs basename); benf {} --format json | jq --arg v "$vendor" '"'"'{vendor: $v, file: .dataset, risk: .risk_level}'"'"' > 請求書分析.jsonl

# 保険金請求分析
find ./請求 -name "*.xlsx" | while read file; do
    claim_id=$(basename "$file" .xlsx)
    benf "$file" --filter ">=1000" --format json | \
    jq --arg id "$claim_id" '{請求ID: $id, リスク評価: .risk_level, 総数値数: .numbers_analyzed}'
done | jq -s '.' > 請求リスク評価.json
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

## ⚠️ 重要な適用限界

**ベンフォードの法則が適用できないデータ:**
- **制約のある範囲**: 成人身長(150-200cm)、年齢(0-100歳)、気温など
- **連番データ**: 請求書番号、社員番号、郵便番号など
- **割り当て番号**: 電話番号、マイナンバー、宝くじ番号など
- **小規模データセット**: 30-50個未満（統計的に不十分）
- **単一ソースデータ**: 同じ工程・機械からの類似した規模のデータ
- **丸め込みデータ**: 大幅に丸められた金額（全て00で終わるなど）

**適用に適したデータ:**
- **多様なスケールの自然データ**: 金融取引、人口、物理測定値
- **多様なソース**: 異なるプロセス・時期からの混合データ
- **大規模データセット**: 100個以上の数値（信頼性の高い解析用）
- **自然発生データ**: 人為的制約を受けていない自然発生的数値

## 歴史的背景

**発見と発展:**
- **1881年**: サイモン・ニューカムが対数表の研究中に現象を最初に観察
- **1938年**: 物理学者フランク・ベンフォードが法則を再発見し、広範な研究で体系化
- **1972年**: 会計・不正検知分野での最初の学術的応用
- **1980年代**: 大手会計事務所が標準的な監査ツールとして採用開始
- **1990年代**: マーク・ニグリーニが法医学会計・税務不正検知での活用を普及
- **2000年代以降**: 選挙監視、科学データ検証、金融犯罪捜査に拡大

**現代の応用例:**
- 米国国税庁（IRS）が税務調査スクリーニングに使用
- Big4会計事務所での標準ツール
- 選挙不正検知に応用（特に2009年イラン大統領選挙分析で注目）
- マネーロンダリング捜査での活用

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
| `--lang <言語>` | 出力言語: en, ja, zh, hi, ar (デフォルト: auto) |
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