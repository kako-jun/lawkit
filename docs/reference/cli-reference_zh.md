# CLI参考

`lawkit`的完整命令行界面文档。

## 全局命令

### `lawkit --help`
显示主命令或子命令的帮助信息。

### `lawkit --version`
显示版本信息。

### `lawkit list`
列出所有可用的统计定律及其描述。

```bash
$ lawkit list
Available statistical laws:
  benf    - Benford Law analysis
  pareto  - Pareto Principle (80/20 rule) analysis
  zipf    - Zipf Law analysis
  normal  - Normal Distribution analysis
  poisson - Poisson Distribution analysis

Integration commands:
  analyze  - Multi-law basic analysis and recommendations
  validate - Data validation and consistency checks  
  diagnose - Conflict detection and detailed diagnostics
```

## 统计定律命令

### `lawkit benf` - 本福德定律分析

使用本福德定律检测异常并评估数据质量。

```bash
lawkit benf [OPTIONS] [INPUT]
```

#### 选项
- `--format <FORMAT>, -f` - 输出格式: text, json, csv, yaml, toml, xml (默认: text)
- `--quiet, -q` - 最小输出（仅数字）
- `--verbose, -v` - 启用包含详细分析见解的详细调试输出
- `--filter <RANGE>` - 按范围过滤数字（例如: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析所需的最少数据点数（默认: 10）
- `--threshold <LEVEL>, -t` - 异常检测阈值: low, medium, high, critical（默认: auto）
- `--confidence <LEVEL>` - 测试的统计置信水平（0.01-0.99，默认: 0.95）
- `--sample-size <NUMBER>` - 大型数据集的最大样本量（提高性能）
- `--min-value <VALUE>` - 分析中包含的最小值（过滤添加噪声的小值）

**注意**: `--optimize`选项已被弃用。优化现在会自动应用。
#### 详细输出
`--verbose`标志提供全面的调试和分析信息：

**调试信息：**
- 输入参数检测和验证
- 数据处理策略（自动优化、流式处理）
- 过滤器应用及前后统计
- 数据收集和解析详情

**性能指标：**
- 以毫秒为单位的处理时间
- 以MB为单位的内存使用量
- 大型数据集处理的块数
- 处理的项目计数

**分析洞察：**
- 统计计算步骤
- 置信区间详情
- 算法选择推理
- 数据质量评估

详细输出示例：
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

# 使用JSON格式的详细输出
lawkit benf transactions.json --verbose --format json

# 最小输出的安静模式
lawkit benf data.csv --quiet

# 使用高阈值过滤大型交易
lawkit benf accounts.csv --filter ">=1000" --threshold high

# 用于审计的高置信度分析（99%置信水平）
lawkit benf audit_data.csv --confidence 0.99 --verbose

# 大型数据集的性能优化
lawkit benf big_data.csv --sample-size 50000

# 过滤向分析添加噪声的小值
lawkit benf financial_data.csv --min-value 100
```

### `lawkit pareto` - 帕累托原理分析

分析集中度并应用80/20法则。

```bash
lawkit pareto [OPTIONS] [INPUT]
```

#### 通用选项
- `--format <FORMAT>, -f` - 输出格式: text, json, csv, yaml, toml, xml (默认: text)
- `--quiet, -q` - 最小输出
- `--verbose, -v` - 详细输出
- `--filter <RANGE>` - 按范围过滤数字（例如: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析所需的最少数据点数（默认: 10）

#### 特定选项
- `--concentration <THRESHOLD>, -C` - 集中度阈值 (0.0-1.0)（默认: 0.8）
- `--gini-coefficient` - 计算用于不平等测量的基尼系数
- `--percentiles <PERCENTILES>` - 要计算的自定义百分位数（例如: 70,80,90）
- `--business-analysis` - 启用商业分析洞察

#### 示例
```bash
# 基本帕累托分析
lawkit pareto sales.csv

# 自定义阈值
lawkit pareto data.csv --concentration 0.9

# 包含基尼系数的商业分析
lawkit pareto customers.csv --business-analysis --gini-coefficient

# 自定义百分位数
lawkit pareto revenue.csv --percentiles 70,80,90,95
```

### `lawkit zipf` - 齐普夫定律分析

分析频率分布和排名模式。支持数字数据和文本数据分析。

```bash
lawkit zipf [OPTIONS] [INPUT]
```

#### 通用选项
- `--format <FORMAT>, -f` - 输出格式: text, json, csv, yaml, toml, xml (默认: text)
- `--quiet, -q` - 最小输出
- `--verbose, -v` - 详细输出
- `--filter <RANGE>` - 按范围过滤数字（例如: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析所需的最少数据点数（默认: 10）

#### 特定选项
- `--text, -T` - 启用文本分析模式
- `--words <NUMBER>, -w` - 文本模式中要分析的最大单词数（默认: 1000）

#### 示例
```bash
# 基本齐普夫分析（数字数据）
lawkit zipf frequency_data.csv

# 文本分析模式
lawkit zipf text_document.txt --text

# 带单词限制的文本分析
lawkit zipf large_text.txt --text --words 500

# 详细输出
lawkit zipf rankings.csv --verbose

# JSON输出格式
lawkit zipf data.csv --format json
```

### `lawkit normal` - 正态分布分析

测试正态性并检测异常值。提供高级统计分析功能。

```bash
lawkit normal [OPTIONS] [INPUT]
```

#### 通用选项
- `--format <FORMAT>, -f` - 输出格式: text, json, csv, yaml, toml, xml (默认: text)
- `--quiet, -q` - 最小输出
- `--verbose, -v` - 详细输出
- `--filter <RANGE>` - 按范围过滤数字（例如: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析所需的最少数据点数（默认: 10）

#### 分析选项
- `--test <METHOD>, -T` - 正态性测试方法: shapiro, anderson, ks, all（默认: all）
- `--outliers, -O` - 启用异常值检测
- `--outlier-method <METHOD>` - 异常值检测方法: zscore, modified_zscore, iqr, lof, isolation, dbscan, ensemble（默认: zscore）
- `--quality-control, -Q` - 启用质量控制分析
- `--spec-limits <LOWER,UPPER>` - 质量控制的规格限制（例如: 9.5,10.5）
- `--enable-timeseries` - 启用时间序列分析
- `--timeseries-window <SIZE>` - 时间序列分析窗口大小（默认: 10）

#### 示例
```bash
# 基本正态性测试
lawkit normal data.csv

# 特定测试方法
lawkit normal data.csv --test shapiro

# 异常值检测
lawkit normal data.csv --outliers --outlier-method lof

# 质量控制分析
lawkit normal production_data.csv --quality-control --spec-limits 9.5,10.5

# 时间序列分析
lawkit normal timeseries_data.csv --enable-timeseries --timeseries-window 20

# 详细输出
lawkit normal measurements.csv --verbose

# JSON输出格式
lawkit normal quality_data.csv --format json
```

### `lawkit poisson` - 泊松分布分析

分析事件发生和稀有事件。

```bash
lawkit poisson [OPTIONS] [INPUT]
```

#### 通用选项
- `--format <FORMAT>, -f` - 输出格式: text, json, csv, yaml, toml, xml (默认: text)
- `--quiet, -q` - 最小输出（仅数字）
- `--verbose, -v` - 启用包含详细分析见解的详细调试输出
- `--filter <RANGE>` - 按范围过滤数字（例如: >=100, <1000, 50-500）
- `--min-count <NUMBER>, -c` - 分析所需的最少数据点数（默认: 10）
- `--confidence <LEVEL>` - 测试的统计置信水平（0.01-0.99，默认: 0.95）

#### 分析选项
- `--test <METHOD>, -T` - 拟合优度测试方法: chi_square, ks, variance, all（默认: all）
- `--predict, -p` - 启用概率预测
- `--max-events <NUMBER>` - 分析的最大事件数（默认: 20）
- `--rare-events, -R` - 专注于稀有事件分析

**注意**: `--optimize`选项已被弃用。优化现在会自动应用。

#### 示例
```bash
# 基本泊松分析
lawkit poisson events.csv

# 特定测试方法
lawkit poisson data.csv --test chi_square

# 概率预测模式
lawkit poisson server_logs.csv --predict --max-events 50

# 稀有事件分析
lawkit poisson rare_events.csv --rare-events

# 详细输出
lawkit poisson incidents.csv --verbose

# JSON输出格式
lawkit poisson data.csv --format json

# 关键分析的高置信度水平
lawkit poisson server_errors.csv --confidence 0.99 --verbose
```

## 生成命令

### `lawkit generate` - 样本数据生成

生成遵循特定统计定律的样本数据，用于测试和验证。

```bash
lawkit generate <LAW> [OPTIONS]
```

#### 可用定律
- `benf` - 生成符合本福德定律的数据
- `pareto` - 生成帕累托分布数据
- `zipf` - 生成齐普夫定律数据
- `normal` - 生成正态分布数据
- `poisson` - 生成泊松分布数据

#### 通用生成选项
- `--samples <NUMBER>` - 要生成的样本数（默认: 1000）
- `--seed <NUMBER>` - 用于可重现生成的随机种子
- `--output-file <FILE>` - 输出文件路径（默认: stdout）

#### 定律特定选项

**本福德生成：**
- `--fraud-rate <RATE>` - 用于测试的欺诈注入率 (0.0-1.0)（默认: 0.0）
- `--range <MIN,MAX>` - 生成的数字范围（例如: 1,10000）（默认: 1,100000）

#### 示例
```bash
# 生成本福德定律数据
lawkit generate benf --samples 5000

# 生成带欺诈注入的本福德数据
lawkit generate benf --samples 2000 --fraud-rate 0.1

# 生成带自定义范围的可重现数据
lawkit generate benf --samples 1000 --seed 42 --range 1,50000

# 生成并保存到文件
lawkit generate normal --samples 1000 --output-file test_data.csv
```

## 集成命令

### `lawkit analyze` - 多定律分析

执行基本的多定律分析，并提供综合数据评估的建议。

```bash
lawkit analyze [OPTIONS] [INPUT]
```

### `lawkit validate` - 数据验证

跨多个统计模式验证数据的一致性和质量。

```bash
lawkit validate [OPTIONS] [INPUT]
```

### `lawkit diagnose` - 冲突检测

检测统计定律结果之间的冲突并提供详细诊断。

```bash
lawkit diagnose [OPTIONS] [INPUT]
```

#### 选项
- `--laws <LAWS>` - 要分析的特定定律: benf,pareto,zipf,normal,poisson
- `--focus <FOCUS>` - 分析焦点: quality, concentration, distribution, anomaly
- `--purpose <PURPOSE>` - 分析目的: quality, fraud, concentration, anomaly, distribution, general
- `--recommend` - 启用最佳定律推荐模式
- `--threshold <THRESHOLD>` - 冲突检测阈值 (0.0-1.0)（默认: 0.5）
- `--report <TYPE>` - 集成报告类型: summary, detailed, conflicting（默认: summary）
- `--consistency-check` - 启用一致性检查
- `--cross-validation` - 启用交叉验证分析
- `--confidence-level <LEVEL>` - 置信水平（默认: 0.95）

#### 示例
```bash
# 比较所有定律
lawkit analyze data.csv

# 专注于欺诈检测
lawkit analyze transactions.csv --purpose fraud --recommend

# 自定义定律选择
lawkit analyze data.csv --laws benf,normal --focus quality

# 使用JSON格式的详细输出
lawkit analyze dataset.csv --verbose --format json
```

## 通用选项

所有命令都支持这些通用选项：

### 输入/输出
- `[INPUT]` - 输入数据（文件路径、URL或'-'表示stdin）
- `--format <FORMAT>` - 输出格式: text, json, csv, yaml, toml, xml
- `--quiet, -q` - 最小输出
- `--verbose, -v` - 详细输出
- `--no-color` - 禁用彩色输出

**注意**: 优化会自动应用，因此不再需要`--optimize`选项。

### 数据处理
- `--filter <RANGE>` - 数字过滤 (>=100, <1000, 50-500)
- `--min-count <NUMBER>` - 所需的最少数据点数（默认: 10）

## 输入格式

`lawkit`支持多种输入格式：

- **文本文件** - 由空白或逗号分隔的数字
- **CSV** - 逗号分隔值
- **JSON** - 结构化数据
- **YAML** - YAML配置文件
- **TOML** - TOML配置文件
- **XML** - XML数据文件

## 输出格式

### 文本格式（默认）
包含分析结果、解释和建议的人类可读输出。

### JSON格式
为API和自动化而设计的机器可读结构化输出：
```json
{
  "dataset": "data.csv",
  "numbers_analyzed": 1000,
  "risk_level": "Low",
  "analysis_results": {...}
}
```

### CSV格式
电子表格导入的表格格式：
```csv
dataset,numbers_analyzed,risk_level,score
data.csv,1000,Low,0.85
```

## 退出代码

- `0` - 成功，低风险
- `10` - 检测到中等风险
- `11` - 检测到高风险
- `12` - 检测到严重风险
- `1` - 分析错误
- `2` - 无效参数
- `3` - 文件/网络错误

## 用例示例

### 欺诈检测
```bash
# 金融交易分析
lawkit benf transactions.csv --verbose

# 多定律欺诈检测
lawkit analyze suspicious_data.csv --purpose fraud --recommend
```

### 数据质量评估
```bash
# 综合质量检查
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