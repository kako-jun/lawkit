# 例

実世界のユースケースに基づいたlawkitの実用的な使用例。

## 1. 金融監査における不正検出

### ケース: 経費報告書の検証

```bash
# 経費データの基本分析
lawkit benf expenses_2024.csv --format json

# 詳細出力付きの詳細分析
lawkit benf expenses_2024.csv --verbose

# 高信頼度の監査分析（99%信頼水準）
lawkit benf expenses_2024.csv --confidence 0.99 --verbose

# ノイズを追加する小さな金額を除外
lawkit benf expenses_2024.csv --min-value 50 --threshold high

# 大規模データセットのパフォーマンス最適化
lawkit benf expenses_2024.csv --sample-size 10000 --optimize

# 複数の法則を使った包括的分析
lawkit analyze expenses_2024.csv --laws benford,normal
```

**期待される結果**: 
- ベンフォードの法則からの逸脱は人為的操作を示す可能性がある
- 正規分布分析は外れ値を特定する
- 複数法則分析は包括的な洞察を提供する

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

# 90/10分析（より厳格な上位顧客の特定）
lawkit pareto customer_sales.csv --threshold 0.9

# 可視化データのエクスポート
lawkit pareto customer_sales.csv --format csv > pareto_results.csv
```

**適用例**:
- 売上の80%を占める主要顧客を特定
- 高価値セグメントに営業努力を集中
- 顧客サービスリソース配分の最適化

### ケース: 在庫管理分析

```bash
# 在庫回転分析
lawkit pareto inventory_turnover.csv --verbose

# 季節パターン検出
lawkit normal seasonal_demand.csv --verbose
```

## 3. テキスト分析とコンテンツ管理

### ケース: Webサイトコンテンツ分析

```bash
# 単語頻度分析
lawkit zipf website_content.txt --verbose

# コンテンツ分布分析
lawkit zipf blog_posts.txt --verbose
```

**ユースケース**:
- SEOキーワード最適化
- コンテンツ戦略計画
- 自然なコンテンツと人為的コンテンツの検出

### ケース: ソーシャルメディア分析

```bash
# ハッシュタグ分布分析
lawkit zipf hashtags.csv --verbose

# エンゲージメントパターン分析
lawkit poisson post_engagements.csv --verbose
```

## 4. 品質管理と製造業

### ケース: 製造プロセス管理

```bash
# 製品寸法品質管理
lawkit normal product_dimensions.csv --verbose

# 高信頼度での欠陥率分析
lawkit poisson defect_rates.csv --confidence 0.99 --verbose
```

**品質指標**:
- 統計的プロセス管理
- 欠陥パターン分析
- 品質分布評価

### ケース: サービス応答時間分析

```bash
# 応答時間分布
lawkit normal response_times.csv --verbose

# 信頼水準付きインシデント頻度分析
lawkit poisson incidents.csv --confidence 0.95 --verbose
```

## 5. 統合分析ワークフロー

### ケース: 包括的データ品質評価

```bash
# データ品質のための複数法則比較
lawkit analyze financial_data.csv --laws benford,pareto,normal --purpose audit

# 詳細品質レポートの生成
lawkit analyze data.csv --laws all --format json > quality_report.json
```

### ケース: 自動異常検出パイプライン

```bash
#!/bin/bash
# 日次データ品質パイプライン

# ステップ 1: 基本品質チェック
lawkit benf daily_transactions.csv --verbose || exit 1

# ステップ 2: 集中度分析
lawkit pareto daily_sales.csv --verbose

# ステップ 3: 統計検証
lawkit normal process_metrics.csv --verbose

# ステップ 4: 包括レポート
lawkit analyze daily_data.csv --laws benford,pareto,normal --format json > daily_report.json
```

## 6. CI/CD統合例

### GitHub Actions統合

```yaml
name: Data Quality Check
on: [push, pull_request]

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install lawkit
        run: cargo install lawkit
      - name: Run quality checks
        run: |
          lawkit benf data/transactions.csv --format json
          lawkit analyze data/sales.csv --laws benford,pareto --format json
```

### Jenkinsパイプライン統合

```groovy
pipeline {
    agent any
    stages {
        stage('Data Quality Check') {
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
# 大きなファイルの最適化処理
lawkit benf large_dataset.csv --quiet

# 複数ファイルの並列処理
find data/ -name "*.csv" | xargs -P 4 -I {} lawkit benf {}
```

### メモリ効率分析

```bash
# 大規模データセットを効率的に処理
lawkit benf huge_data.csv --format json | jq '.risk_level'

# リアルタイム出力のストリーミング分析
tail -f live_data.log | lawkit benf --quiet
```

## 8. データ生成とテスト例

### ケース: テストと教育のためのデータ生成

```bash
# 不正検出テストのためのベンフォードの法則サンプル生成
lawkit generate benf --samples 10000 > benf_test_data.csv

# 検出機能のテスト
lawkit benf benf_test_data.csv --format json

# さまざまなタイプのデータ生成
lawkit generate pareto --samples 5000 > pareto_data.csv
lawkit generate zipf --samples 2000 > zipf_data.txt
lawkit generate normal --samples 1000 > normal_data.csv
lawkit generate poisson --samples 1000 > poisson_data.csv
```

### ケース: 統計教育とデモンストレーション

```bash
# さまざまな統計法則のデモンストレーション
for law in benf pareto zipf normal poisson; do
  echo "Testing $law law:"
  lawkit generate $law --samples 1000 > test_data.csv
  lawkit $law test_data.csv --verbose
  echo "---"
done

# 検証機能の示例
lawkit generate benf --samples 5000 > test_benf.csv
lawkit validate test_benf.csv --laws benford
```

### ケース: 手法検証とクロステスト

```bash
# 生成と即座分析パイプライン
lawkit generate poisson --samples 1000 > poisson_test.csv
lawkit poisson poisson_test.csv --format json

# 法則間のクロス検証
lawkit generate normal --samples 5000 > normal_data.csv
lawkit analyze normal_data.csv --laws normal,benford,zipf

# 包括テスト
lawkit list --help  # 利用可能コマンドの表示
```

### ケース: 継続的統合テスト

```bash
#!/bin/bash
# 生成データを使ったCI/CDテストスクリプト

echo "統計精度テストを実行中..."

# テスト 1: ベンフォードの法則精度
lawkit generate benf --samples 10000 > benf_test.csv
BENF_RESULT=$(lawkit benf benf_test.csv --format json | jq -r '.risk_level')
if [ "$BENF_RESULT" != "Low" ]; then
    echo "❌ ベンフォードテスト失敗: Low riskを期待したが $BENF_RESULT を取得"
    exit 1
fi

# テスト 2: 正規分布検出
lawkit generate normal --samples 1000 > normal_test.csv
lawkit normal normal_test.csv --verbose

# テスト 3: ポアソン分析
lawkit generate poisson --samples 5000 > poisson_test.csv
lawkit poisson poisson_test.csv --format json

echo "✅ すべての統計精度テストが成功しました"
```

## 9. コマンドリファレンス例

### 利用可能コマンド

```bash
# 利用可能なすべてのコマンドを一覧表示
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

### 出力形式例

```bash
# さまざまな出力形式
lawkit benf data.csv --format json
lawkit benf data.csv --format csv
lawkit benf data.csv --format yaml
lawkit benf data.csv --format toml
lawkit benf data.csv --format xml

# 詳細度制御
lawkit benf data.csv --quiet
lawkit benf data.csv --verbose
```

## 設定例

詳細なセットアップ手順と高度な設定オプションについては[Configuration Guide](configuration.md)を参照してください。

## 次のステップ

- [Installation Guide](installation.md) - セットアップとインストール手順
- [CLI Reference](../reference/cli-reference.md) - 完全なコマンドドキュメント
- [Integration Guide](../guides/integrations.md) - CI/CD自動化
- [Performance Guide](../guides/performance.md) - 最適化技法