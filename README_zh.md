# lawkit

> **🔍 多法则统计分析工具包 - 发现隐藏模式，自信检测异常**

[English README](README.md) | [日本語版 README](README_ja.md) | [中文版 README](README_zh.md)

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io CLI](https://img.shields.io/crates/v/lawkit.svg?label=lawkit-cli)](https://crates.io/crates/lawkit)
[![Docs.rs Core](https://docs.rs/lawkit-core/badge.svg)](https://docs.rs/lawkit-core)
[![npm](https://img.shields.io/npm/v/lawkit-js.svg?label=lawkit-js)](https://www.npmjs.com/package/lawkit-js)
[![PyPI](https://img.shields.io/pypi/v/lawkit-python.svg?label=lawkit-python)](https://pypi.org/project/lawkit-python/)
[![Documentation](https://img.shields.io/badge/📚%20用户指南-Documentation-green)](https://github.com/kako-jun/lawkit/tree/main/docs/index_zh.md)
[![API Reference](https://img.shields.io/badge/🔧%20API%20Reference-docs.rs-blue)](https://docs.rs/lawkit-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

使用多种统计法则检测异常、模式和洞察的下一代统计分析工具包。非常适合欺诈检测、数据质量评估和商业智能。

```bash
# 传统工具一次只分析一种模式
$ other-tool data.csv  # 单一统计分析

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
- **🔍 高级异常值检测**: LOF、隔离森林、DBSCAN、集成方法
- **📈 时间序列分析**: 趋势检测、季节性、变化点分析
- **🚀 内存高效**: 大型数据集流式处理模式

## 📊 性能

在AMD Ryzen 5 PRO 4650U上的真实基准测试结果：

```bash
# 传统工具一次分析一种模式
$ other-tool data.csv         # 单一分析: ~2.1秒
$ lawkit benf data.csv        # 相同分析: ~180ms (快11.7倍)
$ lawkit compare data.csv     # 多法则分析: ~850ms
```

## 为什么选择lawkit？

传统工具一次只分析一种统计模式。`lawkit`提供全面的多法则分析。

- **整体洞察**: 多个统计法则揭示不同方面
- **智能建议**: AI支持的分析集成
- **时间高效**: 多法则并行处理
- **国际就绪**: 5种语言数字解析

## 🏗️ 工作原理

```mermaid
graph LR
    A[数据] --> B[解析和验证]
    B --> C[多法则分析]
    C --> D[风险评估]
    D --> E[建议]
```

lawkit通过多个统计镜头同时分析您的数据，然后整合结果提供全面的洞察和建议。

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

## 规格

### 支持的统计法则

- **本福德定律**: 财务数据欺诈检测
- **帕累托分析**: 80/20法则和不平等测量  
- **齐夫定律**: 频率分析和幂律分布
- **正态分布**: 质量控制和异常值检测
- **泊松分布**: 事件发生和稀有事件建模

### 分析类型

- 单法则分析
- 多法则比较和集成
- 高级异常值检测（LOF、隔离森林、DBSCAN）
- 时间序列分析和趋势检测
- 测试和验证数据生成

### 输出格式

`lawkit`以多种格式输出结果，适用于不同用例：

- **文本格式（默认）**: 人类可读的分析结果
- **JSON格式**: 用于自动化和集成的机器可读格式
- **CSV/YAML/TOML/XML**: 各种结构化数据处理格式

## 安装

### CLI工具

```bash
# 从crates.io（推荐）
cargo install lawkit

# 从发布版本
wget https://github.com/kako-jun/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz
tar -xzf lawkit-linux-x86_64.tar.gz
```

### 包集成

```bash
# Node.js集成
npm install lawkit-js

# Python集成
pip install lawkit-python
lawkit-download-binary  # 下载CLI二进制文件
```

## 文档

有关全面的指南、示例和API文档：

📚 **[用户指南](https://github.com/kako-jun/lawkit/tree/main/docs/index_zh.md)** - 安装、使用和示例  
🔧 **[CLI参考](https://github.com/kako-jun/lawkit/tree/main/docs/reference/cli-reference_zh.md)** - 完整的命令文档  
📊 **[统计法则指南](https://github.com/kako-jun/lawkit/tree/main/docs/user-guide/examples_zh.md)** - 详细的分析示例  
⚡ **[性能指南](https://github.com/kako-jun/lawkit/tree/main/docs/guides/performance_zh.md)** - 优化和大型数据集  
🌍 **[国际支持](https://github.com/kako-jun/lawkit/tree/main/docs/user-guide/configuration_zh.md)** - 多语言数字解析

## 贡献

我们欢迎贡献！详情请参阅我们的[贡献指南](CONTRIBUTING.md)。

## 许可证

此项目根据MIT许可证授权 - 详情请参阅[LICENSE](LICENSE)文件。