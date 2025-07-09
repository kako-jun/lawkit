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
| `--debug` | 启用调试模式 | `false` |

### 输出格式

| 格式 | 描述 | 文件扩展名 |
|------|------|------------|
| `text` | 人类可读的文本格式 | `.txt` |
| `json` | JSON格式 | `.json` |
| `csv` | 逗号分隔值 | `.csv` |
| `yaml` | YAML格式 | `.yaml` |
| `toml` | TOML格式 | `.toml` |
| `xml` | XML格式 | `.xml` |

### 性能选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--optimize` | 启用优化模式 | `false` |

## 子命令

### benf - 本福德定律分析

```bash
lawkit benf [OPTIONS] [INPUT]
```

检测数据是否符合本福德定律，常用于欺诈检测和数据质量评估。

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--threshold <LEVEL>` | 风险阈值：low, medium, high, critical | `medium` |
| `--mad-threshold <N>` | MAD（平均绝对偏差）阈值 | `4.0` |
| `--columns <COLS>` | 指定分析的列（逗号分隔） | 全部数值列 |
| `--filter <RANGE>` | 数值过滤器（如">=100"） | 无 |
| `--min-count <N>` | 最小数据点数 | `30` |

#### 示例

```bash
# 基本分析
lawkit benf financial_data.csv

# 高敏感度欺诈检测
lawkit benf accounting.csv --threshold critical --mad-threshold 2.0

# 特定列分析
lawkit benf sales.csv --columns "revenue,profit" --verbose

# 繁体字金融数字支持（防伪大写）
echo "壹萬貳仟參佰肆拾伍 陸萬柒仟捌佰玖拾" | lawkit benf

# 过滤小额交易
lawkit benf transactions.csv --filter ">=1000" --format json
```

#### 输出字段

```json
{
  "dataset": "文件名",
  "numbers_analyzed": 1247,
  "risk_level": "MEDIUM",
  "digits": {
    "1": {"observed": 28.3, "expected": 30.1, "deviation": -1.8},
    "2": {"observed": 20.1, "expected": 17.6, "deviation": 2.5}
  },
  "statistics": {
    "chi_square": 15.42,
    "p_value": 0.051,
    "mad": 1.2
  },
  "verdict": "POSSIBLE_DEVIATION"
}
```

### pareto - 帕累托分析

```bash
lawkit pareto [OPTIONS] [INPUT]
```

分析数据的集中度，应用80/20法则进行业务洞察。

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--gini-coefficient` | 计算并显示基尼系数 | `false` |
| `--percentiles <LIST>` | 自定义百分位数（如"70,80,90"） | `80` |
| `--business-analysis` | 提供业务分析洞察 | `false` |
| `--threshold <RATIO>` | 集中度阈值 | `0.8` |

#### 示例

```bash
# 基本帕累托分析
lawkit pareto sales_data.csv

# 包含基尼系数的详细分析
lawkit pareto customer_revenue.csv --gini-coefficient --verbose

# 自定义百分位数分析
lawkit pareto product_sales.csv --percentiles "70,80,90,95" --business-analysis

# 业务洞察模式
lawkit pareto inventory.csv --business-analysis --format json
```

#### 输出字段

```json
{
  "dataset": "sales_data.csv",
  "concentration_80": 75.3,
  "gini_coefficient": 0.65,
  "percentiles": {
    "80": 75.3,
    "90": 88.1,
    "95": 94.2
  },
  "business_insights": {
    "concentration_level": "HIGH",
    "efficiency": "GOOD",
    "recommendations": ["关注前20%客户", "优化长尾产品"]
  }
}
```

### zipf - 齐夫定律分析

```bash
lawkit zipf [OPTIONS] [INPUT]
```

分析数据的频率分布，特别适用于文本分析和排名数据。

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--ranking` | 显示排名分析 | `false` |
| `--text-analysis` | 文本分析模式 | `false` |
| `--vocabulary-size <N>` | 词汇表大小限制 | `1000` |
| `--exponent <F>` | 指数参数 | `1.0` |

#### 示例

```bash
# 数值频率分析
lawkit zipf frequency_data.csv --ranking

# 文本分析
lawkit zipf document.txt --text-analysis --vocabulary-size 5000

# 排名分析
lawkit zipf rankings.csv --ranking --verbose --format json
```

### normal - 正态分布分析

```bash
lawkit normal [OPTIONS] [INPUT]
```

检验数据是否符合正态分布，进行质量控制和异常检测。

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--outliers` | 启用异常值检测 | `false` |
| `--outlier-method <METHOD>` | 异常检测方法 | `iqr` |
| `--quality-control` | 质量控制分析 | `false` |
| `--process-capability` | 工艺能力分析 | `false` |
| `--enable-timeseries` | 时间序列分析 | `false` |
| `--timeseries-window <N>` | 时间序列窗口大小 | `30` |

#### 异常检测方法

| 方法 | 描述 | 适用场景 |
|------|------|----------|
| `iqr` | 四分位数法 | 通用异常检测 |
| `zscore` | Z分数法 | 正态分布数据 |
| `lof` | 局部异常因子 | 复杂模式检测 |
| `isolation` | 隔离森林 | 高维数据 |
| `dbscan` | 密度聚类 | 多模态分布 |
| `ensemble` | 集成方法 | 最高准确度 |

#### 示例

```bash
# 基本正态性检验
lawkit normal quality_data.csv

# 异常值检测
lawkit normal measurements.csv --outliers --outlier-method ensemble

# 质量控制分析
lawkit normal process_data.csv --quality-control --process-capability

# 时间序列分析
lawkit normal daily_metrics.csv --enable-timeseries --timeseries-window 60
```

### poisson - 泊松分布分析

```bash
lawkit poisson [OPTIONS] [INPUT]
```

分析事件发生率，适用于预测和稀有事件分析。

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--predict` | 启用预测功能 | `false` |
| `--rare-events` | 稀有事件分析 | `false` |
| `--time-unit <UNIT>` | 时间单位 | `hour` |
| `--lambda <F>` | 期望参数λ | 自动计算 |

#### 时间单位

| 单位 | 描述 |
|------|------|
| `second` | 每秒事件率 |
| `minute` | 每分钟事件率 |
| `hour` | 每小时事件率 |
| `day` | 每日事件率 |
| `week` | 每周事件率 |

#### 示例

```bash
# 基本泊松分析
lawkit poisson event_counts.csv

# 预测功能
lawkit poisson incidents.csv --predict --time-unit day

# 稀有事件分析
lawkit poisson failures.csv --rare-events --verbose
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
| `--recommend` | 显示分析建议 | `false` |
| `--conflict-detection` | 检测法则间冲突 | `false` |
| `--quality-focus` | 专注质量分析 | `false` |

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
lawkit analyze financial.csv --laws "benf,pareto" --recommend

# 冲突检测
lawkit analyze data.csv --conflict-detection --quality-focus

# 质量专注分析
lawkit analyze measurements.csv --quality-focus --format json
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
| `--seed <N>` | 随机种子 | 随机 |
| `--output-format <FORMAT>` | 输出格式 | `text` |

#### 特定法则选项

##### Benford生成
```bash
lawkit generate benf --samples 5000 --fraud-rate 0.1
```

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--fraud-rate <F>` | 欺诈注入率 | `0.0` |

##### Pareto生成
```bash
lawkit generate pareto --samples 2000 --concentration 0.8
```

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--concentration <F>` | 集中度 | `0.8` |
| `--scale <F>` | 规模参数 | `1.0` |

##### Normal生成
```bash
lawkit generate normal --samples 1000 --mean 100 --stddev 15
```

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--mean <F>` | 均值 | `0.0` |
| `--stddev <F>` | 标准差 | `1.0` |

##### Poisson生成
```bash
lawkit generate poisson --samples 500 --lambda 2.5
```

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--lambda <F>` | 期望参数λ | `1.0` |

##### Zipf生成
```bash
lawkit generate zipf --samples 1000 --exponent 1.0 --vocabulary-size 100
```

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--exponent <F>` | 指数参数 | `1.0` |
| `--vocabulary-size <N>` | 词汇表大小 | `1000` |

#### 示例

```bash
# 生成测试数据
lawkit generate benf --samples 10000 > test_data.csv

# 生成并分析
lawkit generate normal --mean 50 --stddev 10 --samples 1000 | lawkit normal --verbose

# 欺诈检测测试
lawkit generate benf --fraud-rate 0.2 --samples 5000 | lawkit benf --threshold critical
```

### selftest - 自检

```bash
lawkit selftest [OPTIONS]
```

运行内置测试套件，验证lawkit功能正常。

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--comprehensive` | 运行全面测试 | `false` |
| `--laws <LIST>` | 测试特定法则 | `all` |

#### 示例

```bash
# 基本自检
lawkit selftest

# 全面测试
lawkit selftest --comprehensive

# 测试特定法则
lawkit selftest --laws "benf,pareto"
```

### list - 列出信息

```bash
lawkit list <ITEM>
```

列出支持的格式、法则等信息。

#### 支持的项目

```bash
lawkit list formats      # 支持的文件格式
lawkit list laws         # 可用的统计法则
lawkit list examples     # 使用示例
```

## 环境变量

| 变量名 | 描述 | 默认值 |
|--------|------|--------|
| `LAWKIT_FORMAT` | 默认输出格式 | `text` |
| `LAWKIT_OPTIMIZE` | 默认优化模式 | `false` |
| `LAWKIT_DEBUG` | 调试模式 | `false` |
| `LAWKIT_CONFIG` | 配置文件路径 | `~/.config/lawkit/config.yaml` |

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

## 配置文件

lawkit支持YAML配置文件：

```yaml
# ~/.config/lawkit/config.yaml
default:
  format: json
  verbose: true
  optimize: true

benf:
  threshold: medium
  mad_threshold: 4.0

pareto:
  gini_coefficient: true
  business_analysis: true

normal:
  outliers: true
  outlier_method: ensemble

analyze:
  recommend: true
  conflict_detection: true
```

## 管道和重定向

```bash
# 标准输入
echo "123 456 789" | lawkit benf

# 输出重定向
lawkit benf data.csv > results.txt

# 错误重定向
lawkit benf data.csv 2> errors.log

# 管道链
lawkit generate benf --samples 1000 | lawkit benf --format json | jq '.risk_level'

# 批量处理
find . -name "*.csv" | xargs -I {} lawkit benf {} --format json
```

## 高级用法

### 批量处理
```bash
# GNU parallel
find . -name "*.csv" | parallel lawkit benf {} --format json

# 使用配置文件
lawkit benf data.csv --config custom_config.yaml

# 性能监控
time lawkit benf large_file.csv --optimize --verbose
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