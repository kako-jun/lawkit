# 使用例

実世界のユースケースに基づいたlawkitの実践的な使用例。

## 1. 会計監査における不正検知

### ケース: 経費報告書の検証

```bash
# 経費データの基本分析
lawkit benf expenses_2024.csv --format json

# 詳細な分析（冗長出力）
lawkit benf expenses_2024.csv --verbose

# 高信頼度の監査分析（99%信頼水準）
lawkit benf expenses_2024.csv --confidence 0.99 --verbose

# ノイズとなる小さな金額をフィルタリング
lawkit benf expenses_2024.csv --min-value 50 --threshold high

# 大規模データセットのパフォーマンス最適化
lawkit benf expenses_2024.csv --sample-size 10000 --optimize

# 複数法則による包括的分析
lawkit analyze expenses_2024.csv --laws benford,normal
```

**期待される結果**: 
- ベンフォード法則からの逸脱は人為的操作の可能性を示唆
- 正規分布分析により外れ値を特定
- 複数法則分析により包括的な洞察を提供

### ケース: 売上データの信頼性検証

```bash
# 月次売上分析
lawkit benf monthly_sales.csv --verbose

# 地域別分析
lawkit benf sales_by_region.csv --verbose
```

## 2. ビジネス分析

### ケース: 顧客売上のパレート分析

```bash
# 80/20分析
lawkit pareto customer_sales.csv --threshold 0.8

# 90/10分析（より厳密な上位顧客の特定）
lawkit pareto customer_sales.csv --threshold 0.9

# 可視化データのエクスポート
lawkit pareto customer_sales.csv --format csv > pareto_results.csv
```

**活用方法**:
- 売上の80%を占める重要顧客を特定
- 高価値セグメントに営業活動を集中
- 顧客サービスリソースの最適配分

### ケース: 在庫管理分析

```bash
# 在庫回転率分析
lawkit pareto inventory_turnover.csv --verbose

# 季節パターンの検出
lawkit normal seasonal_demand.csv --verbose
```

## 3. テキスト分析とコンテンツ管理

### ケース: ウェブサイトコンテンツ分析

```bash
# 単語頻度分析
lawkit zipf website_content.txt --verbose

# コンテンツ分布分析
lawkit zipf blog_posts.txt --verbose
```

**ユースケース**:
- SEOキーワード最適化
- コンテンツ戦略の計画
- 自然なコンテンツと人工的なコンテンツの検出

### ケース: ソーシャルメディア分析

```bash
# ハッシュタグ分布分析
lawkit zipf hashtags.csv --verbose

# エンゲージメントパターン分析
lawkit poisson post_engagements.csv --verbose
```

## 4. 品質管理と製造

### ケース: 製造プロセス管理

```bash
# 製品寸法の品質管理
lawkit normal product_dimensions.csv --verbose

# 高信頼度での不良率分析
lawkit poisson defect_rates.csv --confidence 0.99 --verbose
```

**品質メトリクス**:
- 統計的プロセス管理
- 不良パターン分析
- 品質分布評価

### ケース: サービス応答時間分析

```bash
# 応答時間分布
lawkit normal response_times.csv --verbose

# 信頼水準を指定したインシデント頻度分析
lawkit poisson incidents.csv --confidence 0.95 --verbose
```

## 5. 統合分析ワークフロー

### ケース: 包括的データ品質評価

```bash
# データ品質のための多法則比較
lawkit analyze financial_data.csv --laws benford,pareto,normal --purpose audit

# 詳細な品質レポートの生成
lawkit analyze data.csv --laws all --format json > quality_report.json
```

### ケース: 自動異常検出パイプライン

```bash
#!/bin/bash
# 日次データ品質パイプライン

# ステップ1: 基本的な品質チェック
lawkit benf daily_transactions.csv --verbose || exit 1

# ステップ2: 集中度分析
lawkit pareto daily_sales.csv --verbose

# ステップ3: 統計的検証
lawkit normal process_metrics.csv --verbose

# ステップ4: 包括的レポート
lawkit analyze daily_data.csv --laws benford,pareto,normal --format json > daily_report.json
```

## 6. CI/CD統合例

### GitHub Actions統合

```yaml
name: データ品質チェック
on: [push, pull_request]

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: lawkitのインストール
        run: cargo install lawkit
      - name: 品質チェックの実行
        run: |
          lawkit benf data/transactions.csv --format json
          lawkit analyze data/sales.csv --laws benford,pareto --format json
```

### Jenkinsパイプライン統合

```groovy
pipeline {
    agent any
    stages {
        stage('データ品質チェック') {
            steps {
                sh 'lawkit benf ${WORKSPACE}/data/*.csv --verbose'
                sh 'lawkit analyze ${WORKSPACE}/data/sales.csv --laws all --format json > quality_report.json'
                archiveArtifacts artifacts: 'quality_report.json'
            }
        }
    }
}
```

## 7. パフォーマンス最適化例

### 大規模データセット処理

```bash
# 大規模ファイルの最適化処理
lawkit benf large_dataset.csv --quiet

# 複数ファイルの並列処理
find data/ -name "*.csv" | xargs -P 4 -I {} lawkit benf {}
```

### メモリ効率的な分析

```bash
# 大規模データセットの効率的処理
lawkit benf huge_data.csv --format json | jq '.risk_level'

# リアルタイム出力によるストリーミング分析
tail -f live_data.log | lawkit benf --quiet
```

## 8. データ生成とテスト例

### ケース: テストと教育のためのデータ生成

```bash
# 不正検知テスト用のベンフォード法則サンプル生成
lawkit generate benf --samples 10000 > benf_test_data.csv

# 検出能力のテスト
lawkit benf benf_test_data.csv --format json

# 異なるタイプのデータ生成
lawkit generate pareto --samples 5000 > pareto_data.csv
lawkit generate zipf --samples 2000 > zipf_data.txt
lawkit generate normal --samples 1000 > normal_data.csv
lawkit generate poisson --samples 1000 > poisson_data.csv
```

### ケース: 統計教育とデモンストレーション

```bash
# 異なる統計法則のデモンストレーション
for law in benf pareto zipf normal poisson; do
  echo "$law 法則のテスト:"
  lawkit generate $law --samples 1000 > test_data.csv
  lawkit $law test_data.csv --verbose
  echo "---"
done

# 検証能力の表示
lawkit generate benf --samples 5000 > test_benf.csv
lawkit validate test_benf.csv --laws benford
```

### ケース: メソッド検証とクロステスト

```bash
# 生成と即時分析のパイプライン
lawkit generate poisson --samples 1000 > poisson_test.csv
lawkit poisson poisson_test.csv --format json

# 法則間のクロス検証
lawkit generate normal --samples 5000 > normal_data.csv
lawkit analyze normal_data.csv --laws normal,benford,zipf

# 包括的テスト
lawkit list --help  # 利用可能なコマンドを表示
```

### ケース: 継続的インテグレーションテスト

```bash
#!/bin/bash
# 生成データを使用したCI/CDテストスクリプト

echo "統計精度テストの実行..."

# テスト1: ベンフォード法則の精度
lawkit generate benf --samples 10000 > benf_test.csv
BENF_RESULT=$(lawkit benf benf_test.csv --format json | jq -r '.risk_level')
if [ "$BENF_RESULT" != "Low" ]; then
    echo "❌ ベンフォードテスト失敗: Lowリスクを期待、実際は $BENF_RESULT"
    exit 1
fi

# テスト2: 正規分布検出
lawkit generate normal --samples 1000 > normal_test.csv
lawkit normal normal_test.csv --verbose

# テスト3: ポアソン分析
lawkit generate poisson --samples 5000 > poisson_test.csv
lawkit poisson poisson_test.csv --format json

echo "✅ すべての統計精度テストに合格"
```

## 9. コマンドリファレンス例

### 利用可能なコマンド

```bash
# すべての利用可能なコマンドをリスト
lawkit --help

# 個別コマンドのヘルプ
lawkit benf --help
lawkit pareto --help
lawkit zipf --help
lawkit normal --help
lawkit poisson --help
lawkit analyze --help
lawkit validate --help
lawkit diagnose --help
lawkit generate --help
lawkit list --help
```

### 出力形式の例

```bash
# 異なる出力形式
lawkit benf data.csv --format json
lawkit benf data.csv --format csv
lawkit benf data.csv --format yaml
lawkit benf data.csv --format toml
lawkit benf data.csv --format xml

# 冗長性の制御
lawkit benf data.csv --quiet
lawkit benf data.csv --verbose
```

## 設定例

詳細なセットアップ手順と高度な設定オプションについては、[設定ガイド](configuration_ja.md)を参照してください。

## 次のステップ

- [インストールガイド](installation_ja.md) - セットアップとインストール手順
- [CLIリファレンス](../reference/cli-reference_ja.md) - 完全なコマンドドキュメント
- [統合ガイド](../guides/integrations_ja.md) - CI/CD自動化
- [パフォーマンスガイド](../guides/performance_ja.md) - 最適化テクニック