# 配置指南

## 概述

`lawkit`设计为开箱即用，具有合理的默认设置，但为不同的用例提供了多种配置选项。

## 命令行选项

### 全局选项

```bash
# 输出格式
lawkit benf data.csv --format json
lawkit benf data.csv --format yaml
lawkit benf data.csv --format csv
lawkit benf data.csv --format toml
lawkit benf data.csv --format xml

# 国际数字支持（自动识别）
echo "１２３４５６" | lawkit benf      # 日文数字
echo "一千二百三十四" | lawkit benf    # 中文数字

# 详细程度
lawkit benf data.csv --quiet     # 最小输出
lawkit benf data.csv --verbose   # 详细输出
```

### 分析选项

```bash
# 带阈值的帕累托分析
lawkit pareto data.csv --threshold 0.8

# 多定律分析
lawkit analyze data.csv --laws benford,pareto,normal

# 聚焦分析
lawkit analyze data.csv --laws benford --focus accuracy

# 特定目的分析
lawkit analyze data.csv --laws all --purpose audit

# 推荐
lawkit analyze data.csv --laws all --recommend
```

## 输出格式

### 支持的格式

| 格式 | 描述 | 最适用 |
|--------|-------------|----------|
| `text` | 人类可读（默认） | 终端显示 |
| `json` | 机器可读 | API、自动化 |
| `csv` | 表格数据 | 电子表格 |
| `yaml` | 结构化配置 | 配置文件 |
| `toml` | Rust友好 | Rust集成 |
| `xml` | 遗留系统 | 企业 |

### 格式示例

#### JSON输出
```bash
lawkit benf data.csv --format json
```
```json
{
  "dataset": "data.csv",
  "numbers_analyzed": 1000,
  "risk_level": "Low",
  "mean_absolute_deviation": 2.3,
  "chi_square_p_value": 0.85
}
```

#### CSV输出
```bash
lawkit benf data.csv --format csv
```
```csv
dataset,numbers_analyzed,risk_level,mad,chi_square_p
data.csv,1000,Low,2.3,0.85
```

## 多语言支持

### 支持的语言

- **英语** (`en`) - 默认
- **日语** (`ja`) - 日本語
- **中文** (`zh`) - 中文
- **印地语** (`hi`) - हिन्दी
- **阿拉伯语** (`ar`) - العربية

### 国际数字支持

`lawkit`自动识别各种数字格式：

```bash
# 日文数字
echo "１２３４ ５６７８" | lawkit benf

# 中文金融数字
echo "壹万贰千 三千四百" | lawkit benf

# 混合格式
echo "123 ４５６ 七八九" | lawkit benf
```

## 集成分析

### 多定律分析配置

```bash
# 选择特定定律
lawkit analyze data.csv --laws benford,pareto,normal

# 专注于特定分析类型
lawkit analyze data.csv --laws benford --focus accuracy

# 特定目的分析
lawkit analyze data.csv --laws all --purpose audit

# 推荐模式
lawkit analyze data.csv --laws all --recommend

# 验证模式
lawkit validate data.csv --laws all

# 诊断模式
lawkit diagnose data.csv --laws all
```

### 分析目的

| 目的 | 最佳定律 | 用例 |
|---------|-----------|----------|
| `audit` | 本福德 + 正态 | 数据质量审计 |
| `fraud` | 本福德 + 泊松 | 欺诈检测 |
| `business` | 帕累托 + 齐普夫 | 商业分析 |
| `research` | 所有定律 | 一般分析 |

## 批量处理

```bash
# 处理多个文件
for file in *.csv; do
  lawkit benf "$file" --format json > "results_${file%.csv}.json"
done

# 使用不同定律进行分析
lawkit analyze data1.csv --laws benford --format json
lawkit analyze data2.csv --laws pareto --format json
lawkit analyze data3.csv --laws normal --format json
```

## 性能调优

### 大型数据集

```bash
# 使用安静模式提高性能
lawkit benf large_data.csv --quiet

# 专注于特定分析
lawkit analyze large_data.csv --laws benford --quiet
```

### 内存管理

- 文件 > 1GB: 考虑数据预处理
- 使用`--quiet`最小化内存使用
- 使用stdin输入进行流处理

## 故障排除

### 常见问题

1. **"数据不足"** - 提供更多数据或检查文件格式
2. **"未找到数字"** - 检查数据格式和编码
3. **"格式错误"** - 验证文件格式与内容匹配

### 调试模式

```bash
# 启用详细日志记录
lawkit benf data.csv --verbose

# 检查数据解析
lawkit benf data.csv --format json | jq '.numbers_analyzed'
```

## 未来的配置功能

以下功能计划在未来版本中实现：

- 配置文件支持 (`lawkit.toml`)
- 环境变量设置
- 自定义阈值配置
- 基于配置文件的设置
- 数据过滤选项
- 高级分析选项

## 下一步

- [示例](examples_zh.md) - 实际配置示例
- [CLI参考](../reference/cli-reference_zh.md) - 完整的命令文档
- [集成指南](../guides/integrations_zh.md) - CI/CD自动化