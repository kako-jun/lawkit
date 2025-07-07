# 配置指南

本指南详细介绍了lawkit的配置选项和自定义设置。

## 命令行选项

### 全局选项

```bash
# 输出格式
--format <FORMAT>     # text, json, csv, yaml, toml, xml (默认: text)

# 输出控制
--quiet              # 最小输出（仅数值）
--verbose            # 详细统计信息

# 数据过滤
--filter <RANGE>     # 过滤数值 (例如: --filter ">=100")
--min-count <N>      # 分析所需的最小数据点数

# 性能选项
--parallel           # 启用并行处理
--threads <N>        # 线程数（默认：CPU核心数）
--streaming          # 流式处理大文件
--chunk-size <N>     # 块大小（默认：10000）
--optimize           # 启用性能优化模式
```

### 特定分析选项

#### 本福德定律 (benf)
```bash
--threshold <LEVEL>   # 警报阈值: low, medium, high, critical
--columns <COLS>      # 指定要分析的列
--mad-threshold <N>   # MAD阈值（默认：4.0）
```

#### 帕累托分析 (pareto)
```bash
--gini-coefficient    # 显示基尼系数
--percentiles <LIST>  # 自定义百分位数 (例如: 70,80,90)
--business-analysis   # 业务分析洞察
```

#### 正态分布 (normal)
```bash
--outliers           # 启用异常值检测
--outlier-method <M> # 方法: iqr, zscore, lof, isolation, dbscan, ensemble
--quality-control    # 质量控制分析
--process-capability # 工艺能力分析
--enable-timeseries  # 时间序列分析
--timeseries-window <N> # 时间序列窗口大小
```

#### 泊松分布 (poisson)
```bash
--predict            # 启用预测功能
--rare-events        # 罕见事件分析
--time-unit <UNIT>   # 时间单位: second, minute, hour, day
```

#### 齐夫定律 (zipf)
```bash
--ranking            # 显示排名分析
--text-analysis      # 文本分析模式
--vocabulary-size <N> # 词汇表大小
```

#### 比较分析 (compare)
```bash
--laws <LIST>        # 指定要比较的法则 (例如: benf,pareto,normal)
--recommend          # 显示分析建议
--conflict-detection # 启用冲突检测
--quality-focus      # 专注于质量分析
```

## 环境变量

```bash
# 设置默认输出格式
export LAWKIT_FORMAT=json

# 设置默认并行度
export LAWKIT_THREADS=8

# 启用调试模式
export LAWKIT_DEBUG=1

# 设置默认输入编码
export LAWKIT_INPUT_ENCODING=utf-8
```

## 配置文件

lawkit支持YAML格式的配置文件：

### 创建配置文件

```yaml
# ~/.config/lawkit/config.yaml
default:
  format: json
  verbose: true
  parallel: true
  threads: 8

benf:
  threshold: medium
  mad_threshold: 4.0

pareto:
  gini_coefficient: true
  business_analysis: true

normal:
  outliers: true
  outlier_method: ensemble
  quality_control: true

compare:
  recommend: true
  conflict_detection: true
```

### 使用配置文件

```bash
# 使用默认配置文件
lawkit benf data.csv

# 指定配置文件
lawkit benf data.csv --config /path/to/config.yaml

# 命令行选项会覆盖配置文件设置
lawkit benf data.csv --format csv  # 覆盖配置文件中的json设置
```

## 性能调优

### 内存优化

```bash
# 大文件流式处理
lawkit benf huge_file.csv --streaming --chunk-size 50000

# 内存使用限制
lawkit compare data.csv --memory-limit 1GB
```

### 并行处理

```bash
# 自动检测CPU核心数
lawkit compare data.csv --parallel

# 手动设置线程数
lawkit compare data.csv --parallel --threads 16

# 负载均衡
lawkit compare *.csv --parallel --load-balance
```

### 缓存设置

```bash
# 启用结果缓存
export LAWKIT_CACHE_ENABLED=1
export LAWKIT_CACHE_DIR=~/.cache/lawkit

# 缓存大小限制
export LAWKIT_CACHE_SIZE=1GB
```

## 输出自定义

### 模板配置

```yaml
# 自定义输出模板
output:
  templates:
    benf_summary: |
      分析结果：{{dataset}}
      风险等级：{{risk_level}}
      卡方值：{{chi_square}}
      p值：{{p_value}}
    
    pareto_business: |
      业务洞察：{{dataset}}
      集中度：{{concentration}}%
      基尼系数：{{gini}}
      建议：{{recommendation}}
```

### 颜色配置

```yaml
colors:
  risk_levels:
    low: green
    medium: yellow
    high: red
    critical: bold_red
  
  statistics:
    significant: blue
    normal: white
    warning: yellow
```

## 国际化设置

### 数字格式

```bash
# 自动检测数字格式
lawkit benf mixed_format.csv --auto-detect

# 指定输入格式
lawkit benf chinese_data.csv --input-format chinese

# 支持的格式
# - ascii: 标准ASCII数字
# - chinese: 中文数字（简体/繁体）
# - japanese: 日文数字（全角/汉字）
# - hindi: 印地语数字
# - arabic: 阿拉伯数字
```

### 时区设置

```bash
# 设置时区
export TZ=Asia/Shanghai
lawkit normal timeseries_data.csv --enable-timeseries
```

## 故障排除

### 调试选项

```bash
# 启用详细日志
lawkit benf data.csv --debug

# 性能分析
lawkit benf data.csv --profile

# 内存使用监控
lawkit benf data.csv --memory-monitor
```

### 常见配置错误

#### 内存不足
```bash
# 减少块大小
lawkit benf large_file.csv --chunk-size 1000

# 启用流式处理
lawkit benf large_file.csv --streaming
```

#### 性能问题
```bash
# 减少线程数
lawkit compare data.csv --threads 2

# 禁用并行处理
lawkit compare data.csv --no-parallel
```

## 下一步

- 查看[使用示例](examples_zh.md)了解实际配置场景
- 参考[CLI参考文档](../reference/cli-reference_zh.md)获取完整选项列表
- 阅读[性能优化指南](../guides/performance_zh.md)了解高级性能设置