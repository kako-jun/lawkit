# 常见问题

## 一般问题

### 什么是lawkit？

lawkit是一个综合的统计分析工具包，实现了多种统计定律，包括本福德定律、帕累托原理、齐普夫定律、正态分布和泊松分布。它专为欺诈检测、数据质量评估、商业分析和科学研究而设计。

### lawkit与其他统计工具有什么不同？

- **多定律集成**：在单一分析中比较多种统计定律
- **国际支持**：处理多种语言和格式的数字
- **内置风险评估**：自动异常检测和风险评估
- **全面输入支持**：从CSV、Excel、PDF、Word、JSON等读取
- **专业输出**：包括JSON、CSV、YAML、XML在内的多种输出格式

### lawkit可以免费使用吗？

是的，lawkit是在MIT许可证下发布的开源软件。您可以自由地将其用于个人和商业目的。

## 安装和设置

### 如何安装lawkit？

**选项 1：下载二进制文件**
从[GitHub Releases](https://github.com/kako-jun/lawkit/releases)下载预编译的二进制文件。

**选项 2：从源代码构建**
```bash
git clone https://github.com/kako-jun/lawkit.git
cd lawkit
cargo build --release
```

**选项 3：作为库使用**
在您的`Cargo.toml`中添加：
```toml
[dependencies]
lawkit-core = "2.0"
```

### 系统要求是什么？

- **操作系统**：Linux、macOS、Windows
- **内存**：最低512MB RAM（大型数据集需要更多）
- **磁盘空间**：安装50MB
- **依赖项**：无（静态链接二进制文件）

### 为什么出现“command not found”错误？

确保 lawkit 二进制文件在您系统的 PATH 中。下载后：

```bash
# 设置为可执行文件 (Unix/Linux/macOS)
chmod +x lawkit

# 移动到 PATH（示例）
sudo mv lawkit /usr/local/bin/

# 或在shell配置文件中添加到PATH
export PATH="/path/to/lawkit:$PATH"
```

## 使用问题

### lawkit支持哪些文件格式？

**输入格式：**
- 电子表格：CSV、TSV、Excel (.xlsx, .xls)、OpenDocument (.ods)
- 文档：PDF、Word (.docx)、PowerPoint (.pptx)、纯文本
- 数据：JSON、YAML、TOML、XML
- 网页：HTML（表格提取）

**输出格式：**
- text、json、csv、yaml、toml、xml

### 如何分析CSV文件中特定列的数据？

lawkit会自动从所有列中提取数值数据。对于特定列，请预处理您的数据：

```bash
# 使用标准工具提取特定列
cut -d',' -f2 data.csv | lawkit benf

# 或使用awk进行更复杂的提取
awk -F',' '{print $2}' data.csv | lawkit pareto
```

### 可靠的分析需要多少数据点？

最低要求因统计定律而异：
- **本福德定律**：5+个点（推荐：100+）
- **帕累托分析**：5+个点（推荐：20+）
- **齐普夫定律**：5+个点（推荐：50+）
- **正态分布**：8+个点（推荐：30+）
- **泊松分布**：10+个点（推荐：50+）

### 风险评估意味着什么？

lawkit将结果分为风险级别：
- **Low**：数据遵循预期的统计模式
- **Medium**：存在一些偏差，值得调查
- **High**：显著偏差，可能存在问题
- **Critical**：极端偏差，需要立即关注

自定义阈值：
```bash
lawkit benf --threshold high data.csv
```

## 统计分析问题

### 什么时候应该使用本福德定律？

本福德定律适用于：
- **金融欺诈检测**：交易金额、会计数据
- **数据质量评估**：自然发生的数值数据
- **科学验证**：实验测量值
- **选举审计**：票数和人口统计

**不适用于：**
- 分配的号码（电话号码、ID）
- 约束范围（百分比、评级）
- 均匀分布

### 帕累托分析和齐普夫定律有什么区别？

**帕累托分析（80/20原则）：**
- 专注于商业应用
- 计算不平等的基尼系数
- 提供商业洞察和建议
- 最适用于：销售分析、客户细分、资源分配

**齐普夫定律（幂律分布）：**
- 专注于频率分布
- 分析排名-频率关系
- 支持数值数据分析
- 最适用于：语言分析、城市人口、网站流量

### 正态分布测试的准确性如何？

lawkit实现了带有统计验证的正态性测试。分析提供：
- 正态性评估的统计指标
- 置信区间和显著性测试
- 异常值检测功能
- 质量控制指标

### 泊松分布分析告诉我什么？

泊松分析适用于：
- **事件计数**：单位缺陷数、每小时通话数
- **稀有事件**：事故、设备故障
- **质量控制**：过程监控
- **容量规划**：服务器负载、客户到达

分析提供：
- Lambda参数（平均率）
- 统计验证
- 事件概率评估
- 质量指标

## 国际化和语言支持

### 国际数字支持如何工作？

lawkit自动识别各种数字格式：

```bash
# 这些都被识别为1234.56：
echo "1,234.56" | lawkit benf    # 英语
echo "１，２３４．５６" | lawkit benf  # 日语数字（自动检测）
echo "१,२३४.५६" | lawkit benf    # 印地语数字（自动检测）
```

### 我可以分析非英语语言的文本吗？

是的！lawkit支持Unicode文本分析，并对输入数据中的国际数字格式进行自动语言检测。

### lawkit支持哪些语言？

lawkit提供统一的英语输出，并自动识别国际数字格式：
```bash
# 英语输出（统一）
lawkit benf data.csv

# 自动识别国际数字
echo "１２３４５６" | lawkit benf      # 日语数字
echo "一千二百三十四" | lawkit benf    # 中文数字
echo "१२३४५६" | lawkit benf        # 印地语数字
echo "١٢٣٤٥٦" | lawkit benf        # 阿拉伯语数字
```

## 集成和高级功能

### 多定律比较如何工作？

`analyze`命令使用多种统计定律分析数据：

```bash
# 使用特定定律分析
lawkit analyze --laws benf,pareto data.csv

# 使用所有适用定律分析
lawkit analyze --laws all data.csv

# 获取建议
lawkit analyze --laws all --recommend data.csv
```

功能包括：
- **矛盾检测**：识别相冲的结果
- **置信度评分**：评估每个分析的可靠性
- **建议**：推荐最适合的定律
- **元分析**：结合多角度的洞察

### 我可以在自己的软件中使用lawkit吗？

可以！使用`lawkit-core`库：

```rust
use lawkit_core::laws::benford::BenfordResult;

fn analyze_data(numbers: &[f64]) {
    let result = BenfordResult::analyze(numbers);
    println!("Chi-square: {}", result.chi_square);
}
```

### 如何将lawkit与CI/CD管道集成？

查看我们的[集成指南](../guides/integrations.md)，其中包含以下示例：
- GitHub Actions
- GitLab CI
- Jenkins
- Docker容器

## 性能和故障排除

### lawkit在大型数据集上运行缓慢。如何提高性能？

**优化策略：**
```bash
# 使用静音模式加快处理
lawkit benf --quiet large_data.csv

# 使用适当的阈值
lawkit benf --threshold medium large_data.csv

# 选择高效的输出格式
lawkit benf --format json large_data.csv  # 比文本格式更快
```

### 我遇到了“insufficient data”错误。应该怎么办？

当您的数据集不满足最低要求时会出现此错误：

```bash
# 检查数据大小
wc -l data.csv

# 使用适合您数据大小的定律
lawkit pareto small_data.csv  # 比normal要求更低
```

### 分析结果似乎不正确。可能是什么原因？

**常见问题：**
1. **错误的统计定律**：不是所有数据都适合所有定律
2. **需要数据预处理**：删除标头、过滤异常值
3. **数据不足**：数据点太少无法进行可靠分析
4. **错误的格式**：确保数值数据格式正确

**调试步骤：**
```bash
# 详细输出获取更多信息
lawkit benf --verbose data.csv

# 使用不同定律检查数据
lawkit analyze --laws all data.csv

# 验证数据格式
head -10 data.csv
```

### 我可以实时分析数据吗？

可以，lawkit支持管道输入：

```bash
# 从其他命令管道数据
tail -f logfile.txt | grep "amount:" | lawkit benf

# 处理连续数据
while true; do
    get_latest_data | lawkit benf --quiet
    sleep 60
done
```

## 高级用法

### 如何生成测试数据？

```bash
# 生成用于测试的样本数据
lawkit generate --samples 1000 | lawkit benf

# 生成并保存到文件
lawkit generate --samples 500 > test_data.csv
```

### 如何验证数据质量？

```bash
# 使用多种定律验证
lawkit validate --laws all data.csv

# 专注于特定领域
lawkit validate --laws benf,pareto --focus fraud-detection data.csv
```

### 如何诊断数据问题？

```bash
# 诊断数据问题
lawkit diagnose --laws all data.csv

# 指定分析目的
lawkit diagnose --laws all --purpose quality-assessment data.csv
```

## 错误消息

### "Failed to parse input data"

这通常意味着：
- 输入中包含非数值数据
- 文件格式不正确
- 编码问题

**解决方案：**
```bash
# 检查文件编码
file data.csv

# 验证CSV格式
csvlint data.csv

# 仅提取数值列
cut -d',' -f2 data.csv | lawkit benf
```

### "No statistical law applicable"

当出现以下情况时会发生：
- 数据集太小
- 数据不符合任何已实现的定律
- 所有定律都未通过适用性测试

**解决方案：**
```bash
# 检查数据特征
lawkit analyze --laws all --verbose data.csv

# 尝试不同的阈值
lawkit benf --threshold low data.csv
```

### "Permission denied" 或 "Access denied"

文件权限问题：
```bash
# 检查文件权限
ls -la data.csv

# 设置文件为可读
chmod 644 data.csv

# 检查文件是否存在
test -f data.csv && echo "File exists" || echo "File not found"
```

## 获取帮助

### 在哪里可以获得更多帮助？

- **文档**: [docs/](index.md)
- **问题报告**: [GitHub Issues](https://github.com/kako-jun/lawkit/issues)
- **讨论**: [GitHub Discussions](https://github.com/kako-jun/lawkit/discussions)
- **自测**: 运行`lawkit selftest`验证安装

### 如何报告错误？

1. 检查GitHub上的现有问题
2. 提供最小重现案例
3. 包含系统信息和lawkit版本
4. 运行`lawkit selftest --verbose`并包含输出

### 如何请求新功能？

1. 在GitHub Discussion中开启讨论想法
2. 解释用例和期望行为
3. 考虑贡献实现
4. 查看我们的[贡献指南](../CONTRIBUTING.md)

### 有社区或论坛吗？

- **GitHub Discussions**: 一般问题和想法
- **GitHub Issues**: 错误报告和功能请求
- **电子邮件**: 敏感问题的直接联系

我们欢迎社区的贡献和反馈！