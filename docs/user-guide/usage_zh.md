# 使用指南

本指南涵盖了使用lawkit进行统计分析的所有方面。

## 目录

- [基本命令](#基本命令)
- [统计法则](#统计法则)
- [输入格式](#输入格式)
- [输出格式](#输出格式)
- [阈值](#阈值)
- [多语言支持](#多语言支持)
- [集成分析](#集成分析)
- [高级功能](#高级功能)

## 基本命令

### 命令结构

```bash
lawkit <子命令> [选项] <输入>
```

### 可用子命令

- `benf` - 本福德定律分析
- `pareto` - 帕累托法则分析
- `zipf` - 齐夫定律分析
- `normal` - 正态分布分析
- `poisson` - 泊松分布分析
- `analyze` - 多法则集成分析和建议
- `validate` - 使用统计法则验证数据质量
- `diagnose` - 使用统计法则诊断数据问题
- `generate` - 生成测试样本数据
- `list` - 列出可用的统计法则
- `selftest` - 运行自检以验证安装

### 常用选项

- `--format <格式>` - 输出格式 (text, json, csv, yaml, toml, xml)
- `--quiet` - 最少输出
- `--verbose` - 详细分析
- `--threshold <级别>` - 设置分析阈值 (low, medium, high, critical, auto)
- `--focus <焦点>` - 分析焦点区域 (用于analyze命令)
- `--purpose <目的>` - 分析目的 (用于analyze命令)
- `--recommend` - 显示建议 (用于analyze命令)
- `--samples <数量>` - 生成的样本数量 (用于generate命令)

## 统计法则

### 1. 本福德定律 (Benford's Law)

检测财务数据中的欺诈行为：

```bash
# 基本分析
lawkit benf financial_data.csv

# 高敏感度欺诈检测
lawkit benf transactions.csv --threshold high

# JSON格式输出
lawkit benf data.csv --format json
```

### 2. 帕累托法则 (Pareto Principle)

分析业务集中度：

```bash
# 基本80/20分析
lawkit pareto sales_data.csv

# 业务分析模式
lawkit pareto revenue.csv --business-analysis

# 详细输出
lawkit pareto customers.csv --verbose
```

### 3. 齐夫定律 (Zipf's Law)

文本和频率分析：

```bash
# 文本文件分析
lawkit zipf document.txt

# 设置最小计数
lawkit zipf text.txt --min-count 5

# 中文文本分析
lawkit zipf chinese_text.txt
```

### 4. 正态分布 (Normal Distribution)

质量控制和异常检测：

```bash
# 正态性测试
lawkit normal measurements.csv

# 详细的正态性测试
lawkit normal quality_data.csv --verbose

# 高敏感度异常检测
lawkit normal sensor_data.csv --threshold high
```

### 5. 泊松分布 (Poisson Distribution)

事件建模和预测：

```bash
# 基本泊松分析
lawkit poisson events.csv

# 时间序列分析
lawkit poisson daily_counts.csv --time-series

# 详细分析
lawkit poisson incidents.csv --verbose
```

## 输入格式

lawkit支持多种输入格式：

### CSV文件
```csv
value
123.45
678.90
234.56
```

### JSON文件
```json
{
  "data": [123.45, 678.90, 234.56]
}
```

### 文本文件
```
123.45
678.90
234.56
```

### 标准输入
```bash
echo -e "123\\n456\\n789" | lawkit benf -
```

## 输出格式

使用`--format`选项选择输出格式：

### 文本格式（默认）
```bash
lawkit benf data.csv --format text
```

### JSON格式
```bash
lawkit benf data.csv --format json
```

### CSV格式
```bash
lawkit benf data.csv --format csv
```

### YAML格式
```bash
lawkit benf data.csv --format yaml
```

## 阈值

阈值控制分析敏感度：

- `low` - 低敏感度（更少误报）
- `medium` - 中等敏感度（平衡）
- `high` - 高敏感度（更多警报）
- `critical` - 最高敏感度
- `auto` - 自动选择（默认）

示例：
```bash
# 高敏感度欺诈检测
lawkit benf transactions.csv --threshold high

# 低敏感度初步筛选
lawkit benf large_dataset.csv --threshold low
```

## 多语言支持

lawkit能够解析多种语言的数字：

```bash
# 英文数字
echo "one hundred twenty-three" | lawkit benf -

# 日文数字
echo "百二十三" | lawkit benf -

# 中文数字
echo "一百二十三" | lawkit benf -

# 印地语数字
echo "एक सौ तेईस" | lawkit benf -

# 阿拉伯语数字
echo "مائة وثلاثة وعشرون" | lawkit benf -
```

## 集成分析

### 多法则分析
```bash
# 分析所有法则
lawkit analyze data.csv --laws all

# 分析特定法则
lawkit analyze data.csv --laws benf,pareto,normal

# 获取建议
lawkit analyze data.csv --recommend
```

### 数据验证
```bash
# 验证数据一致性
lawkit validate data.csv --laws all

# 针对特定目的验证
lawkit validate financial_data.csv --purpose fraud
```

### 问题诊断
```bash
# 诊断数据问题
lawkit diagnose data.csv --focus conflict

# 详细诊断报告
lawkit diagnose data.csv --focus all --verbose
```

## 高级功能

### 生成测试数据
```bash
# 生成本福德定律样本数据
lawkit generate benf --samples 1000

# 生成带欺诈的数据
lawkit generate benf --samples 1000 --fraud-rate 0.1

# 生成帕累托分布数据
lawkit generate pareto --samples 1000 --concentration 0.8
```

### 自检
```bash
# 运行完整的自检
lawkit selftest

# 这将测试所有统计法则的实现
```

### 批量处理
```bash
# 使用shell循环处理多个文件
for file in *.csv; do
    lawkit benf "$file" --format json > "results/${file%.csv}_benf.json"
done
```

### 管道和过滤
```bash
# 与其他工具结合
cat large_dataset.csv | head -1000 | lawkit benf -

# 过滤和分析
awk -F',' '{print $3}' data.csv | lawkit pareto -
```

## 实际示例

### 财务审计
```bash
# 第1步：使用本福德定律进行初步筛选
lawkit benf transactions.csv --threshold high

# 第2步：检查集中度
lawkit pareto transactions.csv --business-analysis

# 第3步：综合分析
lawkit analyze transactions.csv --purpose fraud --recommend
```

### 质量控制
```bash
# 第1步：检查正态性
lawkit normal measurements.csv --verbose

# 第2步：检测异常值
lawkit normal measurements.csv --threshold critical

# 第3步：验证数据质量
lawkit validate measurements.csv --purpose quality
```

### 文本分析
```bash
# 第1步：词频分析
lawkit zipf document.txt --min-count 10

# 第2步：检查自然性
lawkit zipf document.txt --verbose

# 第3步：比较文档
lawkit analyze doc1.txt --laws zipf --format json
lawkit analyze doc2.txt --laws zipf --format json
```

## 故障排除

### 常见问题

1. **没有数据输出**
   - 检查输入文件格式
   - 使用`--verbose`查看详细信息

2. **意外结果**
   - 验证数据质量
   - 调整阈值设置
   - 使用`diagnose`命令查找问题

3. **性能问题**
   - 对大型数据集使用采样
   - 考虑`--threshold low`以加快处理速度

### 获取帮助

```bash
# 显示一般帮助
lawkit --help

# 显示特定命令的帮助
lawkit benf --help

# 列出所有可用命令
lawkit list
```

## 最佳实践

1. **从简单分析开始**：先使用单个法则，然后再进行多法则分析
2. **了解您的数据**：不同的法则适用于不同类型的数据
3. **使用适当的阈值**：从`auto`开始，根据需要调整
4. **验证结果**：使用多个法则交叉检查发现
5. **保存输出**：对进一步分析使用结构化格式（JSON、YAML）

## 另请参阅

- [安装指南](installation_zh.md) - 设置说明
- [示例](examples_zh.md) - 更多实际用例
- [CLI参考](../reference/cli-reference_zh.md) - 完整的命令文档