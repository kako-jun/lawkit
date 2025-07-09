# 使用示例

本指南提供了lawkit在各种实际场景中的详细使用示例。

## 财务分析

### 会计欺诈检测

```bash
# 基本欺诈检测
lawkit benf accounting_records.csv --threshold high

# 详细分析报告
lawkit benf financial_statements.csv --verbose --format json > fraud_report.json

# 批量文件分析
find ./monthly_reports -name "*.xlsx" -exec lawkit benf {} --format csv \; > batch_analysis.csv

# 实时监控
tail -f transaction_log.csv | lawkit benf --format json | jq 'select(.risk_level == "HIGH")'
```

### 销售数据分析

```bash
# 帕累托分析（80/20法则）
lawkit pareto monthly_sales.csv --business-analysis --gini-coefficient

# 客户贡献度分析
lawkit pareto customer_revenue.csv --percentiles 80,90,95 --format json

# 产品销售分布
lawkit pareto product_sales.csv --business-analysis --verbose
```

## 质量控制

### 制造业质量管理

```bash
# 正态分布检验
lawkit normal quality_measurements.csv --quality-control --verbose

# 异常值检测
lawkit normal production_data.csv --outliers --outlier-method ensemble

# 工艺能力分析
lawkit normal process_data.csv --process-capability --format json

# 时间序列质量趋势
lawkit normal daily_quality.csv --enable-timeseries --timeseries-window 30
```

### 服务质量监控

```bash
# 响应时间分析
lawkit normal response_times.csv --outliers --threshold critical

# 系统性能监控
lawkit poisson error_events.csv --predict --time-unit hour

# 多维度质量分析
lawkit analyze performance_metrics.csv --laws normal,poisson --recommend
```

## 市场研究

### 文本分析

```bash
# 社交媒体词频分析
lawkit zipf social_media_posts.txt --text-analysis --ranking

# 新闻内容分析
lawkit zipf news_articles.txt --vocabulary-size 5000 --format json

# 多语言文本分析
lawkit zipf multilingual_content.txt --text-analysis --verbose
```

### 消费者行为分析

```bash
# 购买频率分析
lawkit poisson purchase_events.csv --rare-events --time-unit day

# 用户活跃度分析
lawkit zipf user_activity.csv --ranking --format csv

# 季节性购买模式
lawkit normal seasonal_sales.csv --enable-timeseries --timeseries-window 52
```

## 数据科学

### 数据质量评估

```bash
# 综合数据质量检查
lawkit analyze dataset.csv --laws all --quality-focus --recommend

# 数据分布验证
lawkit normal training_data.csv --quality-control --outliers

# 特征重要性分析
lawkit pareto feature_importance.csv --business-analysis --percentiles 70,80,90,95
```

### 实验数据分析

```bash
# A/B测试结果分析
lawkit normal ab_test_results.csv --quality-control --verbose

# 随机性验证
lawkit benf random_samples.csv --threshold critical --mad-threshold 2.0

# 实验组比较
lawkit analyze control_group.csv treatment_group.csv --conflict-detection
```

## 业务智能

### 销售预测

```bash
# 销售事件预测
lawkit poisson daily_sales.csv --predict --time-unit day --verbose

# 季节性趋势分析
lawkit normal monthly_revenue.csv --enable-timeseries --format json

# 多指标综合分析
lawkit analyze sales_metrics.csv --laws pareto,normal,poisson --recommend
```

### 风险评估

```bash
# 信贷风险分析
lawkit benf loan_applications.csv --threshold high --format json

# 投资组合分析
lawkit pareto portfolio_returns.csv --gini-coefficient --business-analysis

# 操作风险监控
lawkit poisson incident_reports.csv --rare-events --predict
```

## 大数据处理

### 大文件分析

```bash
# 性能优化大文件
lawkit benf huge_dataset.csv --optimize

# 内存优化分析
lawkit analyze large_files/*.csv --optimize

# 分布式处理
find ./big_data -name "*.csv" | xargs -P 8 -I {} lawkit benf {} --format json
```

### 实时数据分析

```bash
# 实时日志监控
tail -f application.log | lawkit benf --format json | \
  jq 'select(.risk_level == "HIGH")' | \
  while read alert; do
    echo "ALERT: $alert" | mail -s "Fraud Detection Alert" admin@company.com
  done

# 流式事件分析
kafka-console-consumer --topic events | \
  lawkit poisson --predict --format json
```

## 国际数据处理

### 多语言数字格式

```bash
# 中文财务数据
lawkit benf chinese_financial.csv --verbose
# 支持：一千二百三十四，１２３４，壹贰叁肆

# 日文数据分析
lawkit benf japanese_data.csv --format json
# 支持：一千二百三十四，１２３４，全角数字

# 混合格式数据
lawkit benf international_data.csv --auto-detect --verbose
```

### 跨国业务分析

```bash
# 多地区销售比较
lawkit analyze us_sales.csv eu_sales.csv asia_sales.csv --recommend

# 国际财务合规检查
find ./regional_reports -name "*_financial.csv" | \
  xargs -I {} lawkit benf {} --threshold critical --format json > compliance_report.json

# 汇率影响分析
lawkit normal exchange_rates.csv --enable-timeseries --outliers
```

## 自动化工作流

### CI/CD集成

```bash
#!/bin/bash
# 数据质量检查脚本

# 分析新数据
lawkit analyze new_data.csv baseline.csv --quality-focus --format json > quality_report.json

# 检查是否有质量问题
risk_level=$(jq -r '.risk_level' quality_report.json)

if [ "$risk_level" = "HIGH" ] || [ "$risk_level" = "CRITICAL" ]; then
    echo "Data quality issue detected!"
    exit 1
fi

echo "Data quality check passed"
```

### 定期监控

```bash
#!/bin/bash
# 每日数据监控脚本

DATE=$(date +%Y-%m-%d)
LOG_FILE="daily_analysis_$DATE.log"

# 财务数据检查
lawkit benf daily_transactions.csv --format json >> $LOG_FILE

# 销售分析
lawkit pareto daily_sales.csv --business-analysis --format json >> $LOG_FILE

# 质量控制
lawkit normal quality_metrics.csv --quality-control --format json >> $LOG_FILE

# 发送报告
python send_daily_report.py $LOG_FILE
```

## 高级用法

### 自定义管道

```bash
# 数据生成和验证管道
lawkit generate benf --samples 10000 | \
  lawkit benf --format json | \
  jq '.chi_square < 20 and .p_value > 0.05'

# 多阶段分析管道
cat raw_data.csv | \
  lawkit normal --outliers --format csv | \
  lawkit pareto --business-analysis --format json | \
  jq '.recommendations[]'
```

### 性能基准测试

```bash
# 性能比较
time lawkit benf large_dataset.csv
time lawkit benf large_dataset.csv --optimize
time lawkit benf large_dataset.csv --optimize
# 内存使用分析
/usr/bin/time -v lawkit benf huge_file.csv --optimize
```

## 故障排除示例

### 常见问题解决

```bash
# 内存不足
lawkit benf large_file.csv --optimize

# 性能优化
lawkit analyze *.csv --optimize
# 编码问题
lawkit benf data.csv --input-encoding utf-8 --debug

# 数据格式问题
lawkit benf messy_data.csv --filter ">=0" --min-count 100
```

## 下一步

- 查看[CLI参考文档](../reference/cli-reference_zh.md)了解所有可用选项
- 阅读[配置指南](configuration_zh.md)了解高级配置
- 参考[性能优化指南](../guides/performance_zh.md)提升分析速度