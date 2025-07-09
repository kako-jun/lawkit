# CLI 参考文档

lawkit命令行界面的完整参考文档。

## 全局选项

所有子命令都支持以下全局选项：

```bash
lawkit [GLOBAL_OPTIONS] <SUBCOMMAND> [OPTIONS] [INPUT]
```

### 通用选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--help, -h` | 显示帮助信息 | - |
| `--version, -V` | 显示版本信息 | - |
| `--format <FORMAT>` | 输出格式 | `text` |
| `--quiet, -q` | 静默模式（最少输出） | `false` |
| `--verbose, -v` | 详细输出 | `false` |

### 输出格式

| 格式 | 描述 | 文件扩展名 |
|------|------|------------|
| `text` | 人类可读的文本格式 | `.txt` |
| `json` | JSON格式 | `.json` |
| `csv` | 逗号分隔值 | `.csv` |
| `yaml` | YAML格式 | `.yaml` |
| `toml` | TOML格式 | `.toml` |
| `xml` | XML格式 | `.xml` |

## 子命令

### benf - 本福德定律分析

```bash
lawkit benf [OPTIONS] [INPUT]
```

检测数据是否符合本福德定律，常用于欺诈检测和数据质量评估。

#### 选项

通用选项（--format, --verbose, --quiet）之外，没有特别的选项。

#### 示例

```bash
# 基本分析
lawkit benf financial_data.csv

# 详细输出
lawkit benf accounting.csv --verbose

# JSON输出
lawkit benf transactions.csv --format json

# 静默模式
lawkit benf data.csv --quiet
```

### pareto - 帕累托分析

```bash
lawkit pareto [OPTIONS] [INPUT]
```

分析数据的集中度，应用80/20法则进行业务洞察。

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--threshold <RATIO>` | 集中度阈值 | `0.8` |

#### 示例

```bash
# 基本帕累托分析
lawkit pareto sales_data.csv

# 自定义阈值
lawkit pareto customer_revenue.csv --concentration 0.9

# 带基尼系数的业务分析
lawkit pareto product_sales.csv --business-analysis --gini-coefficient

# 自定义百分位数
lawkit pareto inventory.csv --percentiles 70,80,90,95
```

### zipf - 齐夫定律分析

```bash
lawkit zipf [OPTIONS] [INPUT]
```

分析数据的频率分布，特别适用于文本分析和排名数据。

#### 选项

通用选项（--format, --verbose, --quiet）之外，没有特别的选项。

#### 示例

```bash
# 数值频率分析
lawkit zipf frequency_data.csv

# 详细输出
lawkit zipf rankings.csv --verbose

# JSON输出
lawkit zipf data.csv --format json
```

### normal - 正态分布分析

```bash
lawkit normal [OPTIONS] [INPUT]
```

检验数据是否符合正态分布，进行质量控制和异常检测。

#### 选项

通用选项（--format, --verbose, --quiet）之外，没有特别的选项。

#### 示例

```bash
# 基本正态性检验
lawkit normal quality_data.csv

# 详细输出
lawkit normal measurements.csv --verbose

# JSON输出
lawkit normal process_data.csv --format json
```

### poisson - 泊松分布分析

```bash
lawkit poisson [OPTIONS] [INPUT]
```

分析事件发生率，适用于预测和稀有事件分析。

#### 选项

通用选项（--format, --verbose, --quiet）之外，没有特别的选项。

#### 示例

```bash
# 基本泊松分析
lawkit poisson event_counts.csv

# 详细输出
lawkit poisson incidents.csv --verbose

# JSON输出
lawkit poisson failures.csv --format json
```

### analyze - 多法则分析

```bash
lawkit analyze [OPTIONS] [INPUT]
```

同时应用多种统计法则，提供综合分析和建议。

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--laws <LIST>` | 指定法则（如"benf,pareto,normal"） | `all` |
| `--focus <FOCUS>` | 分析重点（quality,concentration,distribution,anomaly） | `general` |
| `--purpose <PURPOSE>` | 分析目的（quality,fraud,concentration,anomaly,distribution,general） | `general` |
| `--recommend` | 显示分析建议 | `false` |

#### 支持的法则

| 法则代码 | 完整名称 | 描述 |
|----------|----------|------|
| `benf` | Benford Law | 本福德定律 |
| `pareto` | Pareto Principle | 帕累托法则 |
| `zipf` | Zipf Law | 齐夫定律 |
| `normal` | Normal Distribution | 正态分布 |
| `poisson` | Poisson Distribution | 泊松分布 |
| `all` | All Laws | 所有法则 |

#### 示例

```bash
# 所有法则比较
lawkit analyze dataset.csv

# 特定法则组合
lawkit analyze financial.csv --laws benf,pareto --recommend

# 专注质量分析
lawkit analyze measurements.csv --focus quality --verbose

# JSON输出
lawkit analyze data.csv --format json
```

### validate - 数据验证

```bash
lawkit validate [OPTIONS] [INPUT]
```

验证数据一致性和质量。

#### 示例

```bash
# 基本验证
lawkit validate data.csv

# 详细输出
lawkit validate dataset.csv --verbose
```

### diagnose - 诊断分析

```bash
lawkit diagnose [OPTIONS] [INPUT]
```

检测冲突并提供详细诊断。

#### 示例

```bash
# 基本诊断
lawkit diagnose data.csv

# 详细输出
lawkit diagnose dataset.csv --verbose
```

### generate - 数据生成

```bash
lawkit generate <LAW> [OPTIONS]
```

生成符合特定统计法则的样本数据，用于测试和教学。

#### 支持的法则

```bash
lawkit generate benf [OPTIONS]      # 本福德定律数据
lawkit generate pareto [OPTIONS]    # 帕累托分布数据
lawkit generate zipf [OPTIONS]      # 齐夫分布数据
lawkit generate normal [OPTIONS]    # 正态分布数据
lawkit generate poisson [OPTIONS]   # 泊松分布数据
```

#### 通用选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--samples <N>` | 样本数量 | `1000` |

#### 示例

```bash
# 生成本福德定律数据
lawkit generate benf --samples 5000

# 生成帕累托分布数据
lawkit generate pareto --samples 2000

# 生成正态分布数据
lawkit generate normal --samples 1000

# 生成泊松分布数据
lawkit generate poisson --samples 500

# 生成齐夫分布数据
lawkit generate zipf --samples 1000
```

### list - 列出信息

```bash
lawkit list
```

列出可用的统计法则和描述。

#### 示例

```bash
# 基本列表
lawkit list
```

## 退出代码

| 代码 | 含义 |
|------|------|
| `0` | 成功执行 |
| `1` | 一般错误 |
| `2` | 命令行参数错误 |
| `10` | 低风险检测 |
| `11` | 中等风险检测 |
| `12` | 高风险检测 |
| `13` | 关键风险检测 |

## 管道和重定向

```bash
# 标准输入
echo "123 456 789" | lawkit benf

# 输出重定向
lawkit benf data.csv > results.txt

# 错误重定向
lawkit benf data.csv 2> errors.log

# 管道链
lawkit generate benf --samples 1000 | lawkit benf --format json

# 批量处理
find . -name "*.csv" -exec lawkit benf {} \;
```

## 高级用法

### 批量处理
```bash
# 多文件处理
for file in *.csv; do
    lawkit benf "$file" --format json > "${file%.csv}_result.json"
done

# 性能监控
time lawkit benf large_file.csv --verbose
```

### 自动化脚本
```bash
#!/bin/bash
for file in *.csv; do
    result=$(lawkit benf "$file" --format json)
    risk=$(echo "$result" | jq -r '.risk_level')
    if [ "$risk" = "HIGH" ]; then
        echo "Alert: $file has high risk"
    fi
done
```

更多详细示例请参考[使用示例](../user-guide/examples_zh.md)。