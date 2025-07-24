# lawkit 文档

lawkit是一个使用多个统计法则进行数据质量评估和欺诈检测的综合统计法则分析工具包。

## 主要特性

- **本福德法则**: 在会计数据、选举结果和自然数据集中进行欺诈检测
- **帕累托分析**: 80/20法则分析、销售分析、库存管理
- **齐普夫法则**: 文本分析、词频分析
- **正态分布**: 质量控制、离群值检测、过程能力评估
- **泊松分布**: 事件发生预测、罕见事件分析
- **集成分析**: 多法则比较、矛盾检测、推荐系统

## 快速开始

```bash
# 安装
cargo install lawkit

# 本福德法则分析
lawkit benf data.csv

# 帕累托分析
lawkit pareto sales.csv

# 多法则比较
lawkit analyze data.csv --laws benf,pareto,normal
```

## 文档

### 用户指南
- [安装](user-guide/installation_zh.md)
- [入门指南](user-guide/getting-started_zh.md)
- [配置](user-guide/configuration_zh.md)
- [示例](user-guide/examples_zh.md)

### 参考
- [CLI参考](reference/cli-reference_zh.md)
- [**API 参考**](reference/api-reference_zh.md) - Rust crate API 文档

### 语言绑定
- [**统一API参考**](bindings/unified-api_zh.md) - lawkit-python 和 lawkit-js 语言绑定

### 指南
- [架构指南](guides/architecture_zh.md) - 系统设计和架构概述
- [集成指南](guides/integrations_zh.md)
- [性能指南](guides/performance_zh.md)
- [高级分析](guides/advanced-analysis_zh.md)


### 其他
- [常见问题](user-guide/faq_zh.md)

## 支持

如有问题或疑问，请在[GitHub Issues](https://github.com/kako-jun/lawkit/issues)上报告。我们提供多语言支持。