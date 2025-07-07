# lawkit

> **🔍 多法则统计分析工具包 - 发现隐藏模式，自信检测异常**

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/lawkit-core.svg)](https://crates.io/crates/lawkit-core)
[![Documentation](https://img.shields.io/badge/docs-CLI%20%26%20API-blue)](docs/index.md)
[![API Documentation](https://docs.rs/lawkit-core/badge.svg)](https://docs.rs/lawkit-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

使用多种统计法则检测异常、模式和洞察的下一代统计分析工具包。非常适合欺诈检测、数据质量评估和商业智能。

```bash
# 传统工具一次只分析一种模式
$ benf data.csv  # 仅本福德定律

# lawkit提供全面的多法则分析
$ lawkit compare --laws all data.csv
📊 本福德定律: ⚠️  中等风险 (卡方: 15.2)
📈 帕累托分析: ✅ 正态分布 (基尼: 0.31)
📉 齐夫定律: ❌ 高风险 (相关: 0.45)
🔔 正态分布: ✅ 高斯分布 (p值: 0.12)
🎯 泊松分布: ⚠️  中等风险 (λ=2.3)
🧠 建议: 关注齐夫分析 - 检测到异常频率模式
```

## ✨ 主要功能

- **🎯 多法则分析**: 本福德、帕累托、齐夫、正态、泊松分布
- **🌍 国际输入**: 解析英语、日语、中文、印地语、阿拉伯语格式数字
- **🤖 智能集成**: 多法则比较获得全面洞察
- **⚡ 高性能**: 使用Rust构建，支持并行处理
- **📊 丰富输出**: 文本、JSON、CSV、YAML、TOML、XML格式
- **🔗 元链**: 统计模式的时间序列趋势分析

## 📊 性能基准

```bash
# 100K数据点基准测试
# 传统工具逐个分析模式
benf data.csv                 # 单一统计分析: ~2.1秒
pareto data.csv               # 另一个单独分析: ~2.1秒

# lawkit高效多模式分析
lawkit benf data.csv          # 单法则: ~180ms (快11.7倍)
lawkit compare data.csv       # 多法则分析: ~850ms (比顺序执行快2.5倍)
```

| 数据集大小 | 单法则 | 多法则 | 内存使用 |
|-----------|--------|-------|----------|
| 1K点      | 8ms    | 25ms  | 2.1MB    |
| 10K点     | 45ms   | 180ms | 8.4MB    |
| 100K点    | 180ms  | 850ms | 32MB     |
| 1M点      | 2.1秒  | 9.2秒 | 128MB    |

## 🚀 快速开始

### 安装

```bash
# 统计分析工具包（推荐）
cargo install lawkit

# 或下载预构建二进制文件
wget https://github.com/kako-jun/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz
tar -xzf lawkit-linux-x86_64.tar.gz
```

### 基本用法

```bash
# 单法则分析
lawkit benf data.csv
lawkit pareto sales.csv
lawkit normal measurements.csv

# 多法则比较（推荐）
lawkit compare --laws benf,pareto data.csv
lawkit compare --laws all financial_data.csv

# 带过滤的高级分析
lawkit compare --laws all --filter ">=1000" --format json data.csv
```

## 🔍 支持的统计法则

### 1. 本福德定律
**用例**: 财务数据欺诈检测
```bash
lawkit benf transactions.csv --threshold high
```
检测可能表明数据操作的不自然数字分布。

### 2. 帕累托分析（80/20法则）
**用例**: 业务优先级和不平等测量
```bash
lawkit pareto customer_revenue.csv --verbose
```
识别驱动大部分结果的重要少数。

### 3. 齐夫定律
**用例**: 频率分析和文本挖掘
```bash
lawkit zipf --text document.txt
lawkit zipf website_traffic.csv
```
分析排名和频率的幂律分布。

### 4. 正态分布
**用例**: 质量控制和异常值检测
```bash
lawkit normal --quality-control --spec-limits 9.5,10.5 production.csv
lawkit normal --outliers process_data.csv
```
统计过程控制和异常检测。

### 5. 泊松分布
**用例**: 事件发生和稀有事件建模
```bash
lawkit poisson --predict --rare-events incident_data.csv
```
建模和预测离散事件发生。

## 国际数字支持

### 支持的数字格式

#### 1. 全角数字
```bash
echo "１２３４５６ ７８９０１２" | lawkit benf
```

#### 2. 汉数字（基本）
```bash
echo "一二三四五六七八九" | lawkit benf
```

#### 3. 汉数字（位置记数法）
```bash
echo "一千二百三十四 五千六百七十八 九万一千二百" | lawkit benf
```

#### 4. 混合模式
```bash
echo "销售123万元 费用45万6千元 利润78万９千元" | lawkit benf
```

### 转换规则

| 汉字 | 数字 | 备注 |
|------|------|------|
| 一 | 1 | 基本数字 |
| 十 | 10 | 十位 |
| 百 | 100 | 百位 |
| 千 | 1000 | 千位 |
| 万 | 10000 | 万位 |
| 一千二百三十四 | 1234 | 位置记数法 |

### 印地语数字（हिन्दी अंक）
```bash
# 天城文数字
echo "१२३४५६ ७८९०१२" | lawkit benf --lang hi
```

### 阿拉伯数字（الأرقام العربية）
```bash  
# 东阿拉伯-印度数字
echo "١٢٣٤٥٦ ٧٨٩٠١٢" | lawkit benf --lang ar
```

## 文档

有关全面文档，请参阅：
- 📖 **[中文文档](docs/index_zh.md)** - 完整用户指南和API参考

## 贡献

有关开发指南，请参阅[CONTRIBUTING.md](CONTRIBUTING.md)。

## 许可证

MIT许可证 - 请参阅[LICENSE](LICENSE)文件。

## 参考资料

- [本福德定律 - 维基百科](https://zh.wikipedia.org/wiki/本福特定律)
- [使用本福德定律进行欺诈检测](https://example.com/benford-fraud)