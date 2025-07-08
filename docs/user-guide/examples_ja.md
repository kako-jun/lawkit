# 使用例

実際のユースケースに基づいた、lawkitの実践的な使用例を紹介します。

## 1. 会計監査での不正検知

### ケース: 経費報告書の検証

```bash
# 経費データの基本分析
lawkit benf expenses_2024.csv --columns "金額" --output json

# 詳細分析（日本語数字対応）
lawkit benf expenses_2024.csv  --verbose

# 複数法則での包括分析
lawkit analyze expenses_2024.csv --laws benford,normal --detect-conflicts
```

**期待される結果**: 
- ベンフォード法則からの逸脱がある場合、人為的操作の可能性
- 正規分布分析で異常値を特定
- 矛盾検出で追加調査が必要な項目を識別

### ケース: 売上データの信頼性検証

```bash
# 月次売上の分析
lawkit benf monthly_sales.csv --min-value 1000 --confidence 0.99

# 地域別分析
lawkit benf sales_by_region.csv --columns "北海道,東北,関東,中部,関西,中国,四国,九州"
```

## 2. ビジネス分析

### ケース: 顧客売上のパレート分析

```bash
# 80/20分析
lawkit pareto customer_sales.csv --threshold 0.8 --gini

# 90/10分析（より厳密な上位顧客特定）
lawkit pareto customer_sales.csv --threshold 0.9

# 可視化用データ出力
lawkit pareto customer_sales.csv --output csv > pareto_results.csv
```

**活用方法**:
- 売上の80%を占める20%の顧客を特定
- Gini係数で売上集中度を評価
- 営業戦略の優先順位付け

### ケース: 在庫管理の最適化

```bash
# 商品別在庫回転率のパレート分析
lawkit pareto inventory_turnover.csv --columns "回転率"

# 在庫金額の分析
lawkit pareto inventory_value.csv --threshold 0.7
```

## 3. 品質管理

### ケース: 製造工程の品質監視

```bash
# 製品寸法の正規性検定
lawkit normal product_dimensions.csv --control-chart

# 工程能力評価
lawkit normal process_data.csv --capability --spec-limits 98.5,101.5

# 異常値検出
lawkit normal quality_measurements.csv --outliers --method iqr
```

**品質管理での活用**:
- 工程が統計的管理状態にあるかの確認
- 工程能力指数（Cp, Cpk）の計算
- 異常値の早期検出

### ケース: 不良品発生パターンの分析

```bash
# 不良品発生のポアソン分析
lawkit poisson defect_counts.csv --interval day

# 予測分析
lawkit poisson defect_history.csv --forecast 30 --confidence 0.95
```

## 4. テキスト分析

### ケース: 顧客フィードバックの分析

```bash
# 日本語テキストの分析
lawkit zipf customer_feedback.txt  --min-frequency 3

# キーワード頻度の抽出
lawkit zipf reviews.txt --max-words 100 --output json
```

### ケース: ソーシャルメディア投稿の分析

```bash
# 複数言語対応分析
lawkit zipf social_posts.txt 
# トレンド分析用データ出力
lawkit zipf tweets.txt --output csv --columns "単語,頻度,順位"
```

## 5. 金融・投資分析

### ケース: 株価データの異常検知

```bash
# 株価変動の正規性分析
lawkit normal stock_returns.csv --columns "日次収益率"

# 異常取引の検出
lawkit benf trading_volumes.csv --min-value 1000000
```

### ケース: ポートフォリオ分析

```bash
# 投資配分のパレート分析
lawkit pareto portfolio_weights.csv --gini

# リスク要因の分析
lawkit normal risk_factors.csv --outliers
```

## 6. 学術研究・データサイエンス

### ケース: 実験データの検証

```bash
# データの自然性検証
lawkit benf experimental_data.csv --verbose

# 測定データの分布確認
lawkit normal measurements.csv --normality-tests shapiro,anderson,ks
```

### ケース: 大規模データセットの要約

```bash
# 包括的データ分析
lawkit analyze large_dataset.csv --laws all --recommend

# サンプリング分析
lawkit benf big_data.csv --sample-size 50000
```

## 7. 運用・監視

### ケース: システムログの異常検知

```bash
# アクセスログの分析
lawkit zipf access.log --min-frequency 10

# レスポンス時間の分析
lawkit normal response_times.csv --outliers
```

### ケース: イベント発生予測

```bash
# サーバーダウンの予測
lawkit poisson downtime_events.csv --forecast 7 --interval day

# アラート頻度の分析
lawkit poisson alert_counts.csv --confidence 0.99
```

## 8. バッチ処理とパイプライン

### 複数ファイルの一括処理

```bash
#!/bin/bash
# 月次レポートの一括分析

for file in reports/2024-*.csv; do
    echo "Processing $file..."
    
    # ベンフォード分析
    lawkit benf "$file" --output json > "results/benf_$(basename "$file" .csv).json"
    
    # パレート分析
    lawkit pareto "$file" --output json > "results/pareto_$(basename "$file" .csv).json"
    
    # 包括分析
    lawkit analyze "$file" --laws benford,pareto --output json > "results/compare_$(basename "$file" .csv).json"
done

echo "Batch processing complete!"
```

### CI/CDでの品質チェック

```yaml
# .github/workflows/data-quality.yml
name: Data Quality Check

on:
  push:
    paths:
      - 'data/**/*.csv'

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install lawkit
        run: cargo install lawkit
        
      - name: Quality Analysis
        run: |
          for file in data/*.csv; do
            lawkit analyze "$file" --laws benford,normal --detect-conflicts --output json > "qa_$(basename "$file" .csv).json"
          done
          
      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: quality-reports
          path: qa_*.json
```

## 9. 統合ワークフロー例

### 包括的データ監査パイプライン

```bash
#!/bin/bash
# comprehensive_audit.sh - 包括的データ監査スクリプト

DATA_FILE="$1"
OUTPUT_DIR="audit_results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# 出力ディレクトリ作成
mkdir -p "$OUTPUT_DIR"

echo "=== 包括的データ監査開始: $DATA_FILE ==="

# 1. 基本統計とデータ品質
echo "1. データ品質分析..."
lawkit analyze "$DATA_FILE" --laws all --recommend --output json > "$OUTPUT_DIR/quality_${TIMESTAMP}.json"

# 2. 不正検知（ベンフォード法則）
echo "2. 不正検知分析..."
lawkit benf "$DATA_FILE" --confidence 0.99 --output json > "$OUTPUT_DIR/fraud_${TIMESTAMP}.json"

# 3. ビジネス分析（パレート法則）
echo "3. ビジネス分析..."
lawkit pareto "$DATA_FILE" --gini --output json > "$OUTPUT_DIR/business_${TIMESTAMP}.json"

# 4. 統計的分析（正規分布）
echo "4. 統計分析..."
lawkit normal "$DATA_FILE" --outliers --capability --output json > "$OUTPUT_DIR/statistics_${TIMESTAMP}.json"

# 5. 要約レポート生成
echo "5. 要約レポート生成..."
cat > "$OUTPUT_DIR/summary_${TIMESTAMP}.md" << EOF
# データ監査レポート

**対象ファイル**: $DATA_FILE  
**分析日時**: $(date)  
**分析ID**: ${TIMESTAMP}

## 実行した分析

1. **データ品質分析**: [quality_${TIMESTAMP}.json](quality_${TIMESTAMP}.json)
2. **不正検知分析**: [fraud_${TIMESTAMP}.json](fraud_${TIMESTAMP}.json)  
3. **ビジネス分析**: [business_${TIMESTAMP}.json](business_${TIMESTAMP}.json)
4. **統計的分析**: [statistics_${TIMESTAMP}.json](statistics_${TIMESTAMP}.json)

## 推奨事項

詳細は quality_${TIMESTAMP}.json の recommend セクションを参照してください。

EOF

echo "=== 監査完了: 結果は $OUTPUT_DIR に保存されました ==="
```

使用方法:
```bash
chmod +x comprehensive_audit.sh
./comprehensive_audit.sh financial_data.csv
```

これらの例を参考に、あなたの具体的なユースケースに合わせてlawkitを活用してください。