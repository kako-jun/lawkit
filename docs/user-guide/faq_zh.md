# 常见问题 (FAQ)

本页面收集了使用lawkit时最常遇到的问题和解决方案。

## 安装和设置

### Q: 如何安装lawkit？

**A:** 最简单的方法是使用Cargo：

```bash
cargo install lawkit
```

如果您没有安装Rust，请先访问[rustup.rs](https://rustup.rs/)安装Rust工具链。

### Q: 安装时出现编译错误怎么办？

**A:** 常见解决方案：

1. 确保Rust版本是1.75或更高：
   ```bash
   rustc --version
   rustup update
   ```

2. 清理缓存并重新安装：
   ```bash
   cargo clean
   cargo install lawkit
   ```

3. 如果仍有问题，请从源码编译：
   ```bash
   git clone https://github.com/user/lawkit.git
   cd lawkit
   cargo build --release
   ```

### Q: Windows上如何安装？

**A:** Windows用户有几个选项：

1. 使用WSL（推荐）：
   ```bash
   wsl --install
   # 在WSL中安装Rust和lawkit
   ```

2. 使用Windows原生Rust：
   ```bash
   # 从rustup.rs安装Rust
   cargo install lawkit
   ```

3. 下载预编译二进制文件

## 基本使用

### Q: 如何分析我的第一个文件？

**A:** 从简单的本福德定律分析开始：

```bash
# 分析CSV文件
lawkit benf your_data.csv

# 查看详细信息
lawkit benf your_data.csv --verbose

# 输出JSON格式
lawkit benf your_data.csv --format json
```

### Q: 支持哪些文件格式？

**A:** lawkit支持多种输入格式：

- **Excel**: .xlsx, .xls
- **CSV/TSV**: .csv, .tsv  
- **文档**: .pdf, .docx, .txt
- **结构化数据**: .json, .yaml, .xml
- **压缩文档**: .zip中的Office文档
- **标准输入**: 通过管道输入

### Q: 如何处理中文数字？

**A:** lawkit自动识别中文数字格式：

```bash
# 中文数字示例
echo "一千二百三十四 五千六百七十八" | lawkit benf

# 全角数字
echo "１２３４ ５６７８" | lawkit benf

# 繁体中文数字
echo "壹萬貳仟參佰肆拾伍" | lawkit benf
```

## 数据分析

### Q: 什么时候使用哪种统计法则？

**A:** 不同法则适用于不同场景：

| 法则 | 适用场景 | 典型用例 |
|------|----------|----------|
| **本福德定律** | 自然发生的数据 | 财务欺诈检测、选举数据验证 |
| **帕累托定律** | 集中度分析 | 销售分析、客户价值分析 |
| **齐夫定律** | 排名/频率数据 | 文本分析、词频统计 |
| **正态分布** | 质量控制 | 制造业QC、性能监控 |
| **泊松分布** | 事件发生 | 故障预测、到达率分析 |

### Q: 如何解读分析结果？

**A:** 重点关注以下指标：

#### 本福德定律
- **风险等级**: LOW(绿色) = 正常，HIGH(红色) = 可疑
- **卡方值**: 越高越偏离期望
- **p值**: <0.05 表示显著偏离
- **MAD**: 平均绝对偏差，<4通常正常

#### 帕累托分析
- **集中度**: 80%左右正常，>90%高度集中
- **基尼系数**: 0-1范围，越高越不平等
- **建议**: 关注业务优化建议

### Q: 数据量太少怎么办？

**A:** 

```bash
# 检查最小数据要求
lawkit benf small_data.csv --min-count 10

# 如果数据不足，会显示警告
# 建议：收集更多数据或使用less严格的分析
```

最小数据要求：
- 本福德定律：推荐100+，最少30
- 帕累托分析：推荐50+，最少20  
- 正态分布：推荐30+，最少10

## 性能和优化

### Q: 分析大文件时内存不足怎么办？

**A:** 使用流式处理：

```bash
# 启用性能优化
lawkit benf large_file.csv --optimize
```

### Q: 如何加速分析？

**A:** 几种优化方法：

```bash
# 启用优化模式
lawkit benf data.csv --optimize

# 减少输出详细度
lawkit benf data.csv --quiet

# 批量处理
find . -name "*.csv" | xargs -I {} lawkit benf {}
```

### Q: 分析卡住或很慢怎么办？

**A:** 调试步骤：

1. 检查文件大小和内容：
   ```bash
   wc -l your_file.csv
   head -10 your_file.csv
   ```

2. 启用调试模式：
   ```bash
   lawkit benf your_file.csv --debug
   ```

3. 尝试性能优化：
   ```bash
   lawkit benf your_file.csv --optimize --verbose
   ```

## 输出和格式

### Q: 如何保存分析结果？

**A:** 

```bash
# 保存为JSON
lawkit benf data.csv --format json > results.json

# 保存为CSV（便于Excel打开）
lawkit benf data.csv --format csv > results.csv

# 批量分析并保存
find . -name "*.csv" -exec lawkit benf {} --format json \; > batch_results.json
```

### Q: 如何自定义输出格式？

**A:** 使用jq处理JSON输出：

```bash
# 只提取风险等级
lawkit benf data.csv --format json | jq -r '.risk_level'

# 提取关键统计数据
lawkit benf data.csv --format json | jq '{dataset, risk_level, chi_square, p_value}'

# 创建摘要报告
lawkit benf data.csv --format json | jq -r '"文件: \(.dataset), 风险: \(.risk_level), 卡方: \(.chi_square)"'
```

## 错误和故障排除

### Q: "No numbers found in input" 错误

**A:** 常见原因和解决方案：

1. **编码问题**：
   ```bash
   lawkit benf data.csv --input-encoding utf-8
   ```

2. **数据格式问题**：
   ```bash
   # 检查文件内容
   head -5 data.csv
   # 尝试不同的分隔符或格式
   ```

3. **空文件或无效数据**：
   ```bash
   # 检查文件大小
   ls -la data.csv
   # 验证数据内容
   ```

### Q: "Permission denied" 错误

**A:** 

```bash
# 检查文件权限
ls -la your_file.csv

# 修改权限
chmod 644 your_file.csv

# 或复制到有权限的目录
cp your_file.csv ~/temp/
lawkit benf ~/temp/your_file.csv
```

### Q: 结果看起来不对怎么办？

**A:** 验证步骤：

1. **检查数据质量**：
   ```bash
   lawkit benf data.csv --verbose --debug
   ```

2. **使用已知数据测试**：
   ```bash
   lawkit generate benf --samples 1000 | lawkit benf --verbose
   ```

3. **比较多种方法**：
   ```bash
   lawkit analyze data.csv --laws all --recommend
   ```

## 高级使用

### Q: 如何集成到自动化流程中？

**A:** 

```bash
#!/bin/bash
# 自动化监控脚本

result=$(lawkit benf daily_data.csv --format json)
risk=$(echo $result | jq -r '.risk_level')

if [ "$risk" = "HIGH" ] || [ "$risk" = "CRITICAL" ]; then
    echo "Alert: High risk detected!" | mail -s "Data Alert" admin@company.com
    echo $result > alert_$(date +%Y%m%d).json
fi
```

### Q: 如何与其他工具集成？

**A:** 

```bash
# 与Python集成
python -c "
import subprocess
import json
result = subprocess.run(['lawkit', 'benf', 'data.csv', '--format', 'json'], 
                       capture_output=True, text=True)
data = json.loads(result.stdout)
print(f'Risk Level: {data[\"risk_level\"]}')
"

# 与R集成
Rscript -e "
system('lawkit benf data.csv --format json > results.json')
library(jsonlite)
results <- fromJSON('results.json')
print(paste('Risk:', results\$risk_level))
"
```

### Q: 如何创建自定义报告？

**A:** 使用模板和脚本：

```bash
#!/bin/bash
# 生成综合报告

echo "# 数据分析报告 - $(date)" > report.md
echo "" >> report.md

for file in *.csv; do
    echo "## 分析文件: $file" >> report.md
    lawkit benf "$file" --format json | jq -r '"- 风险等级: \(.risk_level)\n- 卡方值: \(.chi_square)\n- p值: \(.p_value)"' >> report.md
    echo "" >> report.md
done

# 转换为PDF
pandoc report.md -o report.pdf
```

## 获取帮助

### Q: 在哪里可以获得更多帮助？

**A:** 

1. **内置帮助**：
   ```bash
   lawkit --help
   lawkit benf --help
   ```

2. **文档**：
   - [入门指南](getting-started_zh.md)
   - [配置指南](configuration_zh.md)
   - [使用示例](examples_zh.md)

3. **社区支持**：
   - [GitHub Issues](https://github.com/user/lawkit/issues)
   - [讨论区](https://github.com/user/lawkit/discussions)

4. **报告问题**：
   ```bash
   # 收集诊断信息
   lawkit --version
   lawkit selftest --verbose
   # 在GitHub Issues中提供这些信息
   ```

### Q: 如何贡献或提出功能请求？

**A:** 

1. **功能请求**：在GitHub Issues中创建功能请求
2. **错误报告**：提供重现步骤和样本数据
3. **代码贡献**：查看[贡献指南](../../CONTRIBUTING.md)
4. **文档改进**：通过Pull Request提交改进建议

---

如果您的问题没有在此列出，请在[GitHub Issues](https://github.com/user/lawkit/issues)中提问，我们会及时回复并将常见问题添加到此FAQ中。