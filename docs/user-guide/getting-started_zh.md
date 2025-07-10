# 入门指南

欢迎使用lawkit！本指南将帮助您快速开始使用这个强大的统计分析工具。

## 第一个分析

让我们从最简单的本福德定律分析开始：

```bash
# 分析CSV文件
lawkit benf sales_data.csv

# 分析文本文件
echo "123 456 789 234 567" | lawkit benf

# 分析并输出JSON格式
lawkit benf data.csv --format json
```

## 基本概念

### 支持的统计法则

lawkit支持五种主要的统计法则：

1. **本福德定律**（benf）- 检测数据操纵和欺诈
2. **帕累托定律**（pareto）- 80/20分析
3. **齐夫定律**（zipf）- 词频和排名分析
4. **正态分布**（normal）- 质量控制和异常检测
5. **泊松分布**（poisson）- 事件发生率分析

### 输出格式

```bash
# 文本格式（默认）
lawkit benf data.csv

# JSON格式
lawkit benf data.csv --format json

# CSV格式
lawkit benf data.csv --format csv

# YAML格式
lawkit benf data.csv --format yaml
```

## 常用命令

### 单一分析

```bash
# 本福德定律分析
lawkit benf financial_data.csv

# 帕累托分析
lawkit pareto sales_data.csv --gini-coefficient

# 正态分布检验
lawkit normal quality_data.csv --verbose

# 泊松分布分析
lawkit poisson event_data.csv --verbose
```

### 多法则比较

```bash
# 分析多个法则并获取建议
lawkit analyze data.csv --laws all

# 分析特定法则
lawkit analyze data.csv --laws benf,pareto,normal

# 数据一致性验证
lawkit validate data.csv --laws all
```

### 数据生成

```bash
# 生成本福德定律样本数据
lawkit generate benf --samples 1000

# 生成帕累托分布数据
lawkit generate pareto --samples 5000

# 生成并分析（管道操作）
lawkit generate benf --samples 1000 | lawkit benf --format json
```

## 实际示例

### 财务欺诈检测

```bash
# 高阈值检测
lawkit benf financial_records.csv --threshold high --verbose

# 批量文件分析
find ./accounting_data -name "*.csv" -exec lawkit benf {} \;

# 实时监控
tail -f transaction.log | lawkit benf --format json
```

### 销售数据分析

```bash
# 80/20分析
lawkit pareto monthly_sales.csv --business-analysis

# 季节性趋势分析
lawkit normal sales_history.csv --verbose

# 多维度比较
lawkit analyze Q1_sales.csv --laws all
```

## 配置选项

### 常用参数

```bash
# 详细输出
lawkit benf data.csv --verbose

# 静默模式
lawkit benf data.csv --quiet

# 设置最小数据点数
lawkit benf data.csv --min-count 30

# 过滤数据
lawkit benf data.csv --filter ">=100"
```

### 性能优化

```bash
# 性能优化
lawkit analyze large_dataset.csv --recommend

# 性能优化大文件
lawkit benf huge_file.csv --optimize

# 启用优化模式
lawkit benf data.csv --optimize
```

## 国际数字支持

lawkit支持多种数字格式：

```bash
# 中文数字
echo "一千二百三十四 五千六百七十八" | lawkit benf

# 繁体字金融数字（防伪大写）
echo "壹萬貳仟參佰肆拾伍" | lawkit benf

# 全角数字
echo "１２３４ ５６７８" | lawkit benf

# 混合格式
lawkit benf chinese_financial_report.pdf
```

## 下一步

- 查看[使用示例](examples_zh.md)获取更多实际应用场景
- 参考[CLI参考文档](../reference/cli-reference_zh.md)了解所有可用命令和选项
- 阅读[架构指南](../guides/architecture_zh.md)了解系统设计

如有问题，请在GitHub上提交issue。