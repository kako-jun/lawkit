# CLI参考文档

`lawkit`命令行界面完整文档。

## 全局命令

### `lawkit --help`
显示主命令或子命令的帮助信息。

### `lawkit --version`
显示版本信息。

### `lawkit list`
列出所有可用的统计法则及其描述。

```bash
$ lawkit list
可用的统计法则:
  benf    - 本福特定律分析
  pareto  - 帕累托原理 (80/20法则) 分析
  zipf    - 齐普夫定律分析
  normal  - 正态分布分析
  poisson - 泊松分布分析

集成命令:
  analyze  - 多法则基本分析和推荐
  validate - 数据验证和一致性检查
  diagnose - 冲突检测和详细诊断
```

## 统计法则命令

### `lawkit benf` - 本福特定律分析

使用本福特定律检测异常并评估数据质量。

```bash
lawkit benf [OPTIONS] [INPUT]
```

#### 选项
- `--format <FORMAT>` - 输出格式: text, json, csv, yaml, toml, xml (默认: text)
- `--quiet, -q` - 最小输出（仅数字）
- `--verbose, -v` - 启用详细调试输出和分析洞察
- `--filter <RANGE>` - 按范围过滤数字 (例: >=100, <1000, 50-500)
- `--min-count <NUMBER>` - 分析所需的最小数据点数 (默认: 10)
- `--threshold <LEVEL>` - 异常检测阈值: low, medium, high, critical (默认: auto)
- `--confidence <LEVEL>` - 统计检验的置信水平 (0.01-0.99, 默认: 0.95)
- `--sample-size <NUMBER>` - 大数据集的最大样本大小（提高性能）
- `--min-value <VALUE>` - 分析中包含的最小值（过滤增加噪音的小值）

#### 详细输出
`--verbose`标志提供全面的调试和分析信息：

**调试信息:**
- 输入参数检测和验证
- 数据处理策略（自动优化、流处理）
- 过滤器应用的前后统计
- 数据收集和解析详情

**性能指标:**
- 处理时间（毫秒）
- 内存使用量（MB）
- 大数据集的处理块数
- 已处理项目数

**分析洞察:**
- 统计计算步骤
- 置信区间详情
- 算法选择原因
- 数据质量评估

详细输出示例:
```bash
$ echo "123 456 789" | lawkit benf --verbose
Debug: input argument = None
Debug: Reading from stdin, using automatic optimization
Debug: Using automatic optimization (streaming + incremental + memory efficiency)
Debug: Collected 3 numbers from stream
Debug: Streaming analysis successful - 3 items processed
Debug: Processed 3 numbers in 1 chunks
Debug: Memory used: 0.00 MB
Debug: Processing time: 1 ms

# 标准分析输出如下...
```

#### 示例
```bash
# 基本分析
lawkit benf data.csv

# JSON格式的详细输出
lawkit benf transactions.json --verbose --format json

# 最小输出的安静模式
lawkit benf data.csv --quiet

# 高阈值下的大交易过滤
lawkit benf accounts.csv --filter ">=1000" --threshold high

# 审计用高置信度分析（99%置信水平）
lawkit benf audit_data.csv --confidence 0.99 --verbose

# 大数据集的性能优化
lawkit benf big_data.csv --sample-size 50000

# 过滤给分析增加噪音的小值
lawkit benf financial_data.csv --min-value 100
```

### `lawkit pareto` - 帕累托原理分析

分析集中度并应用80/20法则。

```bash
lawkit pareto [OPTIONS] [INPUT]
```

#### 特定选项
- `--concentration <THRESHOLD>` - 集中度阈值 (0.0-1.0) (默认: 0.8)
- `--gini-coefficient` - 计算不平等测量的基尼系数
- `--percentiles <PERCENTILES>` - 计算自定义百分位数 (例: 70,80,90)
- `--business-analysis` - 启用商业分析洞察

#### 示例
```bash
# 基本帕累托分析
lawkit pareto sales.csv

# 自定义阈值
lawkit pareto data.csv --concentration 0.9

# 带基尼系数的商业分析
lawkit pareto customers.csv --business-analysis --gini-coefficient

# 自定义百分位数
lawkit pareto revenue.csv --percentiles 70,80,90,95
```

### `lawkit zipf` - 齐普夫定律分析

分析频率分布和排名模式。

```bash
lawkit zipf [OPTIONS] [INPUT]
```

#### 详细输出
`--verbose`标志提供全面的调试和分析信息：

**调试信息:**
- 输入参数检测和验证
- 文本模式vs数值模式判定
- 数据处理策略（流处理分析）
- 数据收集和解析详情

**性能指标:**
- 处理时间（毫秒）
- 内存使用量（MB）
- 处理块数
- 已处理项目数

详细输出示例:
```bash
$ echo "1 2 3 4 5" | lawkit zipf --verbose
Debug: input argument = None
Debug: text mode = false
Debug: Reading from stdin, using automatic optimization
Debug: Collected 5 numbers from input

# 标准分析输出如下...
```

#### 示例
```bash
# 基本齐普夫分析
lawkit zipf frequency_data.csv

# 详细输出
lawkit zipf rankings.csv --verbose

# JSON输出格式
lawkit zipf data.csv --format json
```

### `lawkit normal` - 正态分布分析

测试正态性并检测异常值。

```bash
lawkit normal [OPTIONS] [INPUT]
```

#### 详细输出
`--verbose`标志提供全面的调试和分析信息：

**调试信息:**
- 输入参数检测和验证
- 数据处理策略（自动优化、流处理）
- 数据收集和解析详情
- 流处理分析指标

**性能指标:**
- 处理时间（毫秒）
- 内存使用量（MB）
- 大数据集的处理块数
- 已处理项目数

详细输出示例:
```bash
$ echo "50 51 49 52 48" | lawkit normal --verbose
Debug: input argument = None
Debug: Reading from stdin, using automatic optimization
Debug: Collected 5 numbers from stream
Debug: Memory used: 0.00 MB

# 标准分析输出如下...
```

#### 示例
```bash
# 基本正态性测试
lawkit normal data.csv

# 详细输出
lawkit normal measurements.csv --verbose

# JSON输出格式
lawkit normal quality_data.csv --format json
```

### `lawkit poisson` - 泊松分布分析

分析事件发生和罕见事件。

```bash
lawkit poisson [OPTIONS] [INPUT]
```

#### 选项
- `--format <FORMAT>` - 输出格式: text, json, csv, yaml, toml, xml (默认: text)
- `--quiet, -q` - 最小输出（仅数字）
- `--verbose, -v` - 启用详细调试输出和分析洞察
- `--min-count <NUMBER>` - 分析所需的最小数据点数 (默认: 10)
- `--confidence <LEVEL>` - 统计检验的置信水平 (0.01-0.99, 默认: 0.95)

#### 示例
```bash
# 基本泊松分析
lawkit poisson events.csv

# 详细输出
lawkit poisson incidents.csv --verbose

# JSON输出格式
lawkit poisson data.csv --format json

# 关键分析的高置信水平
lawkit poisson server_errors.csv --confidence 0.99 --verbose
```

## 生成命令

### `lawkit generate` - 样本数据生成

生成遵循特定统计法则的样本数据用于测试和验证。

```bash
lawkit generate <LAW> [OPTIONS]
```

#### 可用法则
- `benf` - 生成符合本福特定律的数据
- `pareto` - 生成帕累托分布数据
- `zipf` - 生成齐普夫定律数据
- `normal` - 生成正态分布数据
- `poisson` - 生成泊松分布数据

#### 通用生成选项
- `--samples <NUMBER>` - 生成的样本数 (默认: 1000)
- `--seed <NUMBER>` - 可重现生成的随机种子
- `--output-file <FILE>` - 输出文件路径 (默认: stdout)

#### 法则特定选项

**本福特生成:**
- `--fraud-rate <RATE>` - 测试用欺诈注入率 (0.0-1.0) (默认: 0.0)
- `--range <MIN,MAX>` - 生成的数字范围 (例: 1,10000) (默认: 1,100000)

#### 示例
```bash
# 生成本福特定律数据
lawkit generate benf --samples 5000

# 生成带欺诈注入的本福特数据
lawkit generate benf --samples 2000 --fraud-rate 0.1

# 生成自定义范围的可重现数据
lawkit generate benf --samples 1000 --seed 42 --range 1,50000

# 生成并保存到文件
lawkit generate normal --samples 1000 --output-file test_data.csv
```

## 集成命令

### `lawkit analyze` - 多法则分析

执行带推荐的基本多法则分析以进行全面数据评估。

```bash
lawkit analyze [OPTIONS] [INPUT]
```

### `lawkit validate` - 数据验证

跨多个统计模式验证数据一致性和质量。

```bash
lawkit validate [OPTIONS] [INPUT]
```

### `lawkit diagnose` - 冲突检测

检测统计法则结果之间的冲突并提供详细诊断。

```bash
lawkit diagnose [OPTIONS] [INPUT]
```

#### 选项
- `--laws <LAWS>` - 要分析的特定法则: benf,pareto,zipf,normal,poisson
- `--focus <FOCUS>` - 分析焦点: quality, concentration, distribution, anomaly
- `--purpose <PURPOSE>` - 分析目的: quality, fraud, concentration, anomaly, distribution, general
- `--recommend` - 启用最优法则推荐模式
- `--threshold <THRESHOLD>` - 冲突检测阈值 (0.0-1.0) (默认: 0.5)
- `--report <TYPE>` - 集成报告类型: summary, detailed, conflicting (默认: summary)
- `--consistency-check` - 启用一致性检查
- `--cross-validation` - 启用交叉验证分析
- `--confidence-level <LEVEL>` - 置信水平 (默认: 0.95)

#### 示例
```bash
# 比较所有法则
lawkit analyze data.csv

# 专注于欺诈检测
lawkit analyze transactions.csv --purpose fraud --recommend

# 自定义法则选择
lawkit analyze data.csv --laws benf,normal --focus quality

# JSON格式的详细输出
lawkit analyze dataset.csv --verbose --format json
```

## 通用选项

所有命令都支持这些通用选项:

### 输入/输出
- `[INPUT]` - 输入数据（文件路径、URL或stdin用'-'）
- `--format <FORMAT>` - 输出格式: text, json, csv, yaml, toml, xml
- `--quiet, -q` - 最小输出
- `--verbose, -v` - 详细输出

### 数据处理
- `--filter <RANGE>` - 数字过滤 (>=100, <1000, 50-500)
- `--min-count <NUMBER>` - 所需的最小数据点数 (默认: 10)

## 输入格式

`lawkit`支持多种输入格式:

- **文本文件** - 用空格/逗号分隔的数字
- **CSV** - 逗号分隔值
- **JSON** - 结构化数据
- **YAML** - YAML配置文件
- **TOML** - TOML配置文件
- **XML** - XML数据文件

## 输出格式

### 文本格式（默认）
包含分析结果、解释和推荐的人类可读输出。

### JSON格式
用于API和自动化的机器可读结构化输出:
```json
{
  "dataset": "data.csv",
  "numbers_analyzed": 1000,
  "risk_level": "Low",
  "analysis_results": {...}
}
```

### CSV格式
用于电子表格导入的表格格式:
```csv
dataset,numbers_analyzed,risk_level,score
data.csv,1000,Low,0.85
```

## 退出代码

- `0` - 成功，低风险
- `10` - 检测到中等风险
- `11` - 检测到高风险
- `12` - 检测到关键风险
- `1` - 分析错误
- `2` - 无效参数
- `3` - 文件/网络错误

## 按用例的示例

### 欺诈检测
```bash
# 金融交易分析
lawkit benf transactions.csv --verbose

# 多法则欺诈检测
lawkit analyze suspicious_data.csv --purpose fraud --recommend
```

### 数据质量评估
```bash
# 全面质量检查
lawkit analyze dataset.csv --purpose quality --verbose

# 专注于正态性
lawkit normal dataset.csv --verbose
```

### 商业智能
```bash
# 80/20分析
lawkit pareto sales.csv --threshold 0.8

# 客户分析
lawkit zipf customer_frequency.csv --verbose
```

### 异常检测
```bash
# 正态性和异常值分析
lawkit normal data.csv --verbose

# 事件分析
lawkit poisson incidents.csv --verbose
```