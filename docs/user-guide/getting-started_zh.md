# 入门指南

学习lawkit的基本使用方法。本指南使用实际样本数据解释主要功能。

## 基本用法

### 1. 本福德定律分析

用于会计数据和自然数据的欺诈检测：

```bash
# 基本分析
lawkit benf data.csv

# 详细输出
lawkit benf data.csv --verbose

# 以JSON格式输出
lawkit benf data.csv --format json

# 使用指定阈值分析
lawkit benf data.csv --threshold high

# 审计级分析（99%置信度）
lawkit benf audit_data.csv --confidence 0.99 --verbose

# 大型数据集的优化
lawkit benf large_data.csv --sample-size 10000 --optimize

# 过滤小值（器噪声去除）
lawkit benf financial_data.csv --min-value 100
```

### 2. 帕累托分析（80/20法则）

用于销售分析和库存管理：

```bash
# 基本帕累托分析
lawkit pareto sales.csv

# 指定阈值（用 90/10 分析代替 80/20）
lawkit pareto sales.csv --concentration 0.9

# 同时计算基尼系数
lawkit pareto sales.csv --gini-coefficient
```

### 3. 齐普夫定律分析

文本数据的词频分析：

```bash
# 分析文本文件
lawkit zipf document.txt

# 日语文本分析
lawkit zipf japanese_text.txt

# 指定最少出现次数
lawkit zipf text.txt --min-count 5
```

### 4. 正态分布分析

质量控制和异常值检测：

```bash
# 正态性检验
lawkit normal measurements.csv

# 详细正态性检验
lawkit normal quality_data.csv --verbose

# 同时检测异常值
lawkit normal process_data.csv --outliers
```

### 5. 泊松分布分析

事件发生预测和稀有事件分析：

```bash
# 基本泊松分析
lawkit poisson events.csv

# 详细泊松分析
lawkit poisson events.csv --verbose

# 高置信度分析
lawkit poisson critical_events.csv --confidence 0.99 --verbose
```

### 6. 多定律比较

通过同时应用多种统计定律进行综合分析：

```bash
# 多定律分析和建议
lawkit analyze data.csv --laws benf,pareto,normal

# 数据一致性检查
lawkit validate data.csv --laws all

# 详细诊断报告
lawkit diagnose data.csv --focus conflict
```

## 使用样本数据的实际操作

### 会计数据中的欺诈检测

```bash
# 创建样本会计数据
echo "TransactionID,Amount,Date
1,1234,2024-01-01
2,2345,2024-01-02
3,3456,2024-01-03" > accounting.csv

# 使用本福德定律分析
lawkit benf accounting.csv
```

### 销售数据的帕累托分析

```bash
# 创建样本销售数据
echo "Product,Sales
Product A,80000
Product B,12000
Product C,5000
Product D,2000
Product E,1000" > sales.csv

# 执行帕累托分析
lawkit pareto sales.csv --threshold 0.8
```

## 输出格式

lawkit 支持多种输出格式：

```bash
# 文本格式（默认）
lawkit benf data.csv

# JSON格式
lawkit benf data.csv --format json

# CSV格式
lawkit benf data.csv --format csv

# YAML格式
lawkit benf data.csv --format yaml

# XML格式
lawkit benf data.csv --format xml
```

## 多语言支持

lawkit 支持以下语言：

```bash
# 英语输出（默认统一）
lawkit benf data.csv

# 日语数字被自动识别
echo "１２３４５６ ７８９０" | lawkit benf

# 中文数字也被自动识别
echo "一千二百三十四" | lawkit benf

# 繁体中文（古式）金融数字也被支持
echo "壹萬貳仟參佰肆拾伍" | lawkit benf

# 日语汉字数字被自动识别
echo "五万六千七百八十九" | lawkit benf
```

## 高级功能

### 过滤

```bash
# 仅分析特定范围内的数据
lawkit benf data.csv --filter ">=1000"

# 阈值设置
lawkit pareto sales.csv --concentration 0.95
```

### 性能调优

```bash
# 对于大文件，使用采样
lawkit benf large_data.csv --optimize

# 指定并行处理的线程数
lawkit analyze data.csv --recommend
```

## 常见工作流

### 1. 数据质量检查
```bash
# 全面的数据质量评估
lawkit validate financial_data.csv --laws benf,normal
```

### 2. 欺诈检测管道
```bash
# 使用本福德定律进行初步筛查
lawkit benf transactions.csv --format json > results.json

# 对异常情况进行详细分析
lawkit normal suspicious_data.csv --verbose
```

### 3. 商业分析
```bash
# 销售的帕累托分析
lawkit pareto monthly_sales.csv --gini-coefficient

# 客户分析
lawkit zipf customer_feedback.txt
```

## 下一步

- [Examples](examples.md) - 实际用例
- [CLI Reference](../reference/cli-reference.md) - 所有命令详细信息
- [Architecture](../guides/architecture.md) - 系统设计