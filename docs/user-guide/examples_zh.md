# 示例

基于真实世界用例的lawkit实际使用示例。

## 1. 金融审计中的欺诈检测

### 案例: 费用报告验证

```bash
# 费用数据基础分析
lawkit benf expenses_2024.csv --format json

# 详细输出的详细分析
lawkit benf expenses_2024.csv --verbose

# 高信心度的审计分析（99%信心水平）
lawkit benf expenses_2024.csv --confidence 0.99 --verbose

# 过滤掉添加噪音的小金额
lawkit benf expenses_2024.csv --min-value 50 --threshold high

# 大型数据集的性能优化
lawkit benf expenses_2024.csv --sample-size 10000 --optimize

# 使用多种定律的综合分析
lawkit analyze expenses_2024.csv --laws benford,normal
```

**预期结果**: 
- 偏离本福德定律可能表明人为操纵
- 正态分布分析识别异常值
- 多定律分析提供全面洞察

### 案例: 销售数据可靠性验证

```bash
# 月度销售分析
lawkit benf monthly_sales.csv --verbose

# 区域分析
lawkit benf sales_by_region.csv --verbose
```

## 2. 商务分析

### 案例: 客户销售的帕累托分析

```bash
# 80/20分析
lawkit pareto customer_sales.csv --threshold 0.8

# 90/10分析（更严格的顶级客户识别）
lawkit pareto customer_sales.csv --threshold 0.9

# 导出可视化数据
lawkit pareto customer_sales.csv --format csv > pareto_results.csv
```

**应用示例**:
- 识别贡献80%收入的关键客户
- 将销售努力集中在高价值细分市场
- 优化客户服务资源分配

### 案例: 库存管理分析

```bash
# 库存周转分析
lawkit pareto inventory_turnover.csv --verbose

# 季节模式检测
lawkit normal seasonal_demand.csv --verbose
```

## 3. 文本分析和内容管理

### 案例: 网站内容分析

```bash
# 词频分析
lawkit zipf website_content.txt --verbose

# 内容分布分析
lawkit zipf blog_posts.txt --verbose
```

**用例**:
- SEO关键词优化
- 内容策略规划
- 自然与人工内容的检测

### 案例: 社交媒体分析

```bash
# 标签分布分析
lawkit zipf hashtags.csv --verbose

# 参与模式分析
lawkit poisson post_engagements.csv --verbose
```

## 4. 质量控制和制造业

### 案例: 制造过程控制

```bash
# 产品尺寸质量控制
lawkit normal product_dimensions.csv --verbose

# 高信心度的缺陷率分析
lawkit poisson defect_rates.csv --confidence 0.99 --verbose
```

**质量指标**:
- 统计过程控制
- 缺陷模式分析
- 质量分布评估

### 案例: 服务响应时间分析

```bash
# 响应时间分布
lawkit normal response_times.csv --verbose

# 带信心水平的事件频率分析
lawkit poisson incidents.csv --confidence 0.95 --verbose
```

## 5. 集成分析工作流

### 案例: 全面数据质量评估

```bash
# 数据质量的多定律比较
lawkit analyze financial_data.csv --laws benford,pareto,normal --purpose audit

# 生成详细质量报告
lawkit analyze data.csv --laws all --format json > quality_report.json
```

### 案例: 自动异常检测管道

```bash
#!/bin/bash
# 日常数据质量管道

# 步骤 1: 基础质量检查
lawkit benf daily_transactions.csv --verbose || exit 1

# 步骤 2: 集中度分析
lawkit pareto daily_sales.csv --verbose

# 步骤 3: 统计验证
lawkit normal process_metrics.csv --verbose

# 步骤 4: 综合报告
lawkit analyze daily_data.csv --laws benford,pareto,normal --format json > daily_report.json
```

## 6. CI/CD集成示例

### GitHub Actions集成

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

### Jenkins管道集成

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

## 7. 性能优化示例

### 大型数据集处理

```bash
# 大文件的优化处理
lawkit benf large_dataset.csv --quiet

# 多文件的并行处理
find data/ -name "*.csv" | xargs -P 4 -I {} lawkit benf {}
```

### 内存高效分析

```bash
# 高效处理大型数据集
lawkit benf huge_data.csv --format json | jq '.risk_level'

# 实时输出的流式分析
tail -f live_data.log | lawkit benf --quiet
```

## 8. 数据生成和测试示例

### 案例: 测试和教育的数据生成

```bash
# 为欺诈检测测试生成本福德定律样本
lawkit generate benf --samples 10000 > benf_test_data.csv

# 测试我们的检测能力
lawkit benf benf_test_data.csv --format json

# 生成不同类型的数据
lawkit generate pareto --samples 5000 > pareto_data.csv
lawkit generate zipf --samples 2000 > zipf_data.txt
lawkit generate normal --samples 1000 > normal_data.csv
lawkit generate poisson --samples 1000 > poisson_data.csv
```

### 案例: 统计教育和演示

```bash
# 演示不同的统计定律
for law in benf pareto zipf normal poisson; do
  echo "Testing $law law:"
  lawkit generate $law --samples 1000 > test_data.csv
  lawkit $law test_data.csv --verbose
  echo "---"
done

# 显示验证功能
lawkit generate benf --samples 5000 > test_benf.csv
lawkit validate test_benf.csv --laws benford
```

### 案例: 方法验证和交叉测试

```bash
# 生成并立即分析管道
lawkit generate poisson --samples 1000 > poisson_test.csv
lawkit poisson poisson_test.csv --format json

# 定律间的交叉验证
lawkit generate normal --samples 5000 > normal_data.csv
lawkit analyze normal_data.csv --laws normal,benford,zipf

# 全面测试
lawkit list --help  # 显示可用命令
```

### 案例: 持续集成测试

```bash
#!/bin/bash
# 使用生成数据的CI/CD测试脚本

echo "运行统计精度测试..."

# 测试 1: 本福德定律精度
lawkit generate benf --samples 10000 > benf_test.csv
BENF_RESULT=$(lawkit benf benf_test.csv --format json | jq -r '.risk_level')
if [ "$BENF_RESULT" != "Low" ]; then
    echo "❌ 本福德测试失败: 期望低风险，得到 $BENF_RESULT"
    exit 1
fi

# 测试 2: 正态分布检测
lawkit generate normal --samples 1000 > normal_test.csv
lawkit normal normal_test.csv --verbose

# 测试 3: 泊松分析
lawkit generate poisson --samples 5000 > poisson_test.csv
lawkit poisson poisson_test.csv --format json

echo "✅ 所有统计精度测试都通过了"
```

## 9. 命令参考示例

### 可用命令

```bash
# 列出所有可用命令
lawkit --help

# 单个命令帮助
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

### 输出格式示例

```bash
# 不同的输出格式
lawkit benf data.csv --format json
lawkit benf data.csv --format csv
lawkit benf data.csv --format yaml
lawkit benf data.csv --format toml
lawkit benf data.csv --format xml

# 详细度控制
lawkit benf data.csv --quiet
lawkit benf data.csv --verbose
```

## 配置示例

有关详细的设置说明和高级配置选项，请参阅[Configuration Guide](configuration_zh.md)。

## 下一步

- [安装指南](installation_zh.md) - 设置和安装说明
- [CLI参考](../reference/cli-reference_zh.md) - 完整命令文档
- [Integration Guide](../guides/integrations_zh.md) - CI/CD自动化
- [Performance Guide](../guides/performance_zh.md) - 优化技术