# lawkit 文档

lawkit是一个使用多种统计法则进行数据质量分析和欺诈检测的CLI工具包。

## 主要功能

- **本福德定律**: 会计数据、选举结果、自然数据的欺诈检测
- **帕累托定律**: 80/20分析、销售分析、库存管理
- **齐夫定律**: 文本分析、词频分析
- **正态分布**: 质量管理、异常值检测、工艺能力评估
- **泊松分布**: 事件发生预测、罕见事件分析
- **集成分析**: 多法则比较、矛盾检测、推荐系统

## 快速开始

```bash
# 安装
cargo install lawkit

# 本福德定律分析
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
- [使用示例](user-guide/examples_zh.md)

### 参考文档
- [CLI参考](reference/cli-reference_zh.md)

### 指南
- [架构指南](guides/architecture_zh.md) - 系统设计和架构概述
- [集成功能](guides/integrations_zh.md)
- [性能优化](guides/performance_zh.md)
- [高级分析](guides/advanced-analysis_zh.md)


### 其他
- [常见问题](user-guide/faq_zh.md)

## 支持

如有问题或需要帮助，请在[GitHub Issues](https://github.com/user/lawkit/issues)中报告。我们提供中文支持。