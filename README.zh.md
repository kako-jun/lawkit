# benf

使用本福德定律检测异常的CLI工具，支持国际数字（日语、中文、印地语、阿拉伯语）。

## 概述

`benf` 分析数值数据以检查其是否遵循本福德定律，该定律指出在许多自然发生的数据集中，数字1作为**首位数字**出现约30.1%的时间，2出现17.6%的时间，以此类推。偏离此定律可能表明数据操作或欺诈。

**注意**：此工具仅分析每个数字的**首位数字**，而不是整个数字序列。

**独特功能：**
- 🌍 **国际数字支持**：英语、日语（全角・汉数字）、中文（中文数字）、印地语（हिन्दी अंक）、阿拉伯语（الأرقام العربية）
- 📊 多种输入格式（Microsoft Excel、Word、PowerPoint、PDF等）
- 🌐 直接URL分析和HTML解析
- 🔍 专注于欺诈检测的风险级别指示器

## 国际数字支持

### 支持的数字格式

#### 1. 全角数字
```bash
echo "１２３４５６ ７８９０１２" | benf
```

#### 2. 汉数字（基本）
```bash
echo "一二三四五六七八九" | benf
```

#### 3. 汉数字（位置记数法）
```bash
echo "一千二百三十四 五千六百七十八 九万一千二百" | benf
```

#### 4. 混合模式
```bash
echo "销售123万元 费用45万6千元 利润78万9千元" | benf
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

#### 小数
```bash
# 仅分析≥1的数字
echo "12.34 0.567 123.45" | benf
# 结果：1,（排除），1（排除<1的数字）
```

#### 负数
```bash
# 使用绝对值的首位数字
echo "-123 -456 -789" | benf
# 结果：1, 4, 7
```

### 中文数字兼容性

当前实现支持与日文汉字相同的基本中文数字：

#### 支持（基本形式）
- 一二三四五六七八九（1-9）- 与日语相同
- 十百千（10, 100, 1000）- 位置标记

#### 计划支持
- **金融形式**：壹贰叁肆伍陆柒捌玖（防欺诈变体）
- **传统**：萬（10,000）vs 日语万
- **地区变体**：繁体vs简体中文

### 印地语数字（हिन्दी अंक）
```bash
# 天城文数字
echo "१२३४५६ ७८९०१२" | benf --lang hi
```

### 阿拉伯数字（الأرقام العربية）
```bash  
# 东阿拉伯-印度数字
echo "١٢٣٤٥٦ ٧٨٩٠١٢" | benf --lang ar
```

### 其他数字系统（未来支持）

#### 附加脚本（计划中）
- **波斯语**：۰۱۲۳۴۵۶۷۸۹（伊朗、阿富汗）
- **孟加拉语**：০১২৩৪৫৬৭৮৯（孟加拉国）
- **泰米尔语**：௦௧௨௩௪௫௬௭௮௯（泰米尔纳德邦）
- **泰语**：๐๑๒๓๔๕๖๗๘๙（泰国）
- **缅甸语**：၀၁၂၃၄၅၆၇၈၉（缅甸）

> **注意**：国际数字支持根据用户需求持续扩展。当前优先级：日语/中文/印地语/阿拉伯语财务文档分析。

## 安装

### 从源码构建
```bash
git clone https://github.com/kako-jun/benf
cd benf
cargo build --release
cp target/release/benf /usr/local/bin/
```

### 二进制版本
从[发布页面](https://github.com/kako-jun/benf/releases)下载

## 快速开始

```bash
# 分析CSV文件
benf data.csv

# 分析网站数据
benf --url https://example.com/financial-report

# 管道数据
echo "123 456 789 101112" | benf

# 自动化JSON输出
benf data.csv --format json
```

## 使用方法

### 基本语法
```bash
benf [OPTIONS] [INPUT]
```

### 输入方法
1. **文件路径**：`benf financial_data.xlsx`
2. **URL**：`benf --url https://api.example.com/data`
3. **字符串**：`benf "123 456 789 101112"`
4. **管道**：`cat data.txt | benf`

优先级：URL > 文件 > 字符串 > 管道

### 选项

| 选项 | 描述 |
|------|------|
| `--url <URL>` | 从URL获取数据 |
| `--format <FORMAT>` | 输出格式：text, csv, json, yaml, toml, xml |
| `--quiet` | 最小输出（仅数字） |
| `--verbose` | 详细统计 |
| `--lang <LANGUAGE>` | 输出语言：en, ja, zh, hi, ar（默认：auto） |
| `--filter <RANGE>` | 过滤数字（例如：`--filter ">=100"`） |
| `--threshold <LEVEL>` | 警报阈值：low, medium, high, critical |
| `--proxy <URL>` | HTTP代理服务器 |
| `--insecure` | 跳过SSL证书验证 |
| `--timeout <SECONDS>` | 请求超时（默认：30） |
| `--log-level <LEVEL>` | 日志级别：debug, info, warn, error |
| `--help, -h` | 显示帮助 |
| `--version, -V` | 显示版本 |

### 支持的文件格式

| 格式 | 扩展名 | 备注 |
|------|--------|------|
| Microsoft Excel | .xlsx, .xls | 电子表格数据 |
| Microsoft Word | .docx, .doc | 文档分析 |
| Microsoft PowerPoint | .pptx, .ppt | 演示文稿数据 |
| OpenDocument | ods, .odt | OpenOffice/LibreOffice文件 |
| PDF | .pdf | 文本提取 |
| CSV/TSV | .csv, .tsv | 结构化数据 |
| JSON/XML | .json, .xml | API响应 |
| YAML/TOML | .yaml, .toml | 配置文件 |
| HTML | .html | 网页 |
| 文本 | .txt | 纯文本 |

## 输出

### 默认输出
```
本福德定律分析结果

数据集：financial_data.csv
分析的数字数量：1,247
风险等级：高 ⚠️

首位数字分布：
1: ████████████████████████████ 28.3%（预期：30.1%）
2: ████████████████████ 20.1%（预期：17.6%）
3: ██████████ 9.8%（预期：12.5%）
...

统计检验：
卡方值：23.45（p值：0.003）
平均绝对偏差：2.1%

判决：检测到显著偏差
```

### JSON输出
```json
{
  "dataset": "financial_data.csv",
  "numbers_analyzed": 1247,
  "risk_level": "HIGH",
  "digits": {
    "1": {"observed": 28.3, "expected": 30.1, "deviation": -1.8},
    "2": {"observed": 20.1, "expected": 17.6, "deviation": 2.5}
  },
  "statistics": {
    "chi_square": 23.45,
    "p_value": 0.003,
    "mad": 2.1
  },
  "verdict": "SIGNIFICANT_DEVIATION"
}
```

## 实际应用示例

Benf遵循Unix哲学，与标准Unix工具配合处理多个文件效果极佳：

### 财务审计工作流

```bash
# 季度财务审计 - 检查所有Excel报告
find ./2024年第4季度 -name "*.xlsx" | while read file; do
    echo "审计中: $file"
    benf "$file" --filter ">=1000" --threshold critical --verbose
    echo "---"
done

# 月度费用报告验证
for dept in 财务部 市场部 销售部; do
    echo "部门: $dept"
    find "./费用/$dept" -name "*.xlsx" -exec benf {} --format json \; | \
    jq '.risk_level' | sort | uniq -c
done

# 税务文件验证（高精度分析）
find ./税务申报 -name "*.pdf" | parallel benf {} --min-count 50 --format csv | \
awk -F, '$3=="Critical" {print "🚨 严重:", $1}'
```

### 自动监控与警报

```bash
# 会计系统导出的日常监控脚本
#!/bin/bash
ALERT_EMAIL="audit@company.com"
find /exports/daily -name "*.csv" -newer /var/log/last-benf-check | while read file; do
    benf "$file" --format json | jq -r 'select(.risk_level=="Critical" or .risk_level=="High") | "\(.dataset): \(.risk_level)"'
done | mail -s "日常本福德警报" $ALERT_EMAIL

# 持续集成欺诈检测
find ./上传报告 -name "*.xlsx" -mtime -1 | \
xargs -I {} sh -c 'benf "$1" || echo "欺诈警报: $1" >> /var/log/fraud-alerts.log' _ {}

# 使用inotify实时文件夹监控
inotifywait -m ./财务上传 -e create --format '%f' | while read file; do
    if [[ "$file" =~ \.(xlsx|csv|pdf)$ ]]; then
        echo "$(date): 分析中 $file" >> /var/log/benf-monitor.log
        benf "./财务上传/$file" --threshold high || \
        echo "$(date): 警报 - 可疑文件: $file" >> /var/log/fraud-alerts.log
    fi
done
```

### 大规模数据处理

```bash
# 企业文件系统合规审计全面处理
find /corporate-data -type f \( -name "*.xlsx" -o -name "*.csv" -o -name "*.pdf" \) | \
parallel -j 16 'echo "{}: $(benf {} --format json 2>/dev/null | jq -r .risk_level // "错误")"' | \
tee compliance-audit-$(date +%Y%m%d).log

# 档案分析 - 高效处理历史数据
find ./档案/2020-2024 -name "*.xlsx" -print0 | \
xargs -0 -n 100 -P 8 -I {} benf {} --filter ">=10000" --format csv | \
awk -F, 'BEGIN{OFS=","} NR>1 && $3~/High|Critical/ {sum++} END{print "高风险文件数:", sum}'

# 带进度跟踪的网络存储扫描
total_files=$(find /nfs/financial-data -name "*.xlsx" | wc -l)
find /nfs/financial-data -name "*.xlsx" | nl | while read num file; do
    echo "[$num/$total_files] 处理中: $(basename "$file")"
    benf "$file" --format json | jq -r '"文件: \(.dataset), 风险: \(.risk_level), 数字数: \(.numbers_analyzed)"'
done | tee network-scan-report.txt
```

### 高级报告与分析

```bash
# 各部门风险分布分析
for dept in */; do
    echo "=== 部门: $dept ==="
    find "$dept" -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | \
    jq -r '.risk_level' | sort | uniq -c | awk '{print $2": "$1" 个文件"}'
    echo
done

# 时间序列风险分析（需要按日期排序的文件）
find ./月度报告 -name "202[0-4]-*.xlsx" | sort | while read file; do
    month=$(basename "$file" .xlsx)
    risk=$(benf "$file" --format json 2>/dev/null | jq -r '.risk_level // "N/A"')
    echo "$month,$risk"
done > risk-timeline.csv

# 统计摘要生成
{
    echo "文件,风险级别,数字数量,卡方值,p值"
    find ./审计样本 -name "*.xlsx" | while read file; do
        benf "$file" --format json 2>/dev/null | \
        jq -r '"\(.dataset),\(.risk_level),\(.numbers_analyzed),\(.statistics.chi_square),\(.statistics.p_value)"'
    done
} > 统计分析.csv

# 期间比较分析
echo "第三季度 vs 第四季度风险级别比较..."
q3_high=$(find ./2024年第3季度 -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | jq -r 'select(.risk_level=="High" or .risk_level=="Critical")' | wc -l)
q4_high=$(find ./2024年第4季度 -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | jq -r 'select(.risk_level=="High" or .risk_level=="Critical")' | wc -l)
echo "第三季度高风险文件: $q3_high"
echo "第四季度高风险文件: $q4_high"
echo "变化: $((q4_high - q3_high))"
```

### 与其他工具集成

```bash
# 数据验证Git预提交钩子
#!/bin/bash
# .git/hooks/pre-commit
changed_files=$(git diff --cached --name-only --diff-filter=A | grep -E '\.(xlsx|csv|pdf)$')
for file in $changed_files; do
    if ! benf "$file" --min-count 10 >/dev/null 2>&1; then
        echo "⚠️  警告: $file 可能包含可疑数据模式"
        benf "$file" --format json | jq '.risk_level'
    fi
done

# 数据库导入验证
psql -c "COPY suspicious_files FROM STDIN CSV HEADER" <<< $(
    echo "文件名,风险级别,卡方值,p值"
    find ./导入数据 -name "*.csv" | while read file; do
        benf "$file" --format json 2>/dev/null | \
        jq -r '"\(.dataset),\(.risk_level),\(.statistics.chi_square),\(.statistics.p_value)"'
    done
)

# Slack/Teams webhook集成
high_risk_files=$(find ./日常上传 -name "*.xlsx" -mtime -1 | \
    xargs -I {} benf {} --format json 2>/dev/null | \
    jq -r 'select(.risk_level=="High" or .risk_level=="Critical") | .dataset')

if [ -n "$high_risk_files" ]; then
    curl -X POST -H 'Content-type: application/json' \
    --data "{\"text\":\"🚨 检测到高风险文件:\n$high_risk_files\"}" \
    $SLACK_WEBHOOK_URL
fi
```

### 专门用例

```bash
# 选举审计（检查选票计数）
find ./选举数据 -name "*.csv" | parallel benf {} --min-count 100 --threshold low | \
grep -E "(HIGH|CRITICAL)" > 选举异常.txt

# 科学数据验证
find ./研究数据 -name "*.xlsx" | while read file; do
    lab=$(dirname "$file" | xargs basename)
    result=$(benf "$file" --format json | jq -r '.risk_level')
    echo "$lab,$file,$result"
done | grep -E "(High|Critical)" > 数据完整性问题.csv

# 供应链发票验证
find ./发票/2024 -name "*.pdf" | parallel 'vendor=$(dirname {} | xargs basename); benf {} --format json | jq --arg v "$vendor" '"'"'{vendor: $v, file: .dataset, risk: .risk_level}'"'"' > 发票分析.jsonl

# 保险理赔分析
find ./理赔 -name "*.xlsx" | while read file; do
    claim_id=$(basename "$file" .xlsx)
    benf "$file" --filter ">=1000" --format json | \
    jq --arg id "$claim_id" '{理赔ID: $id, 风险评估: .risk_level, 总数字数: .numbers_analyzed}'
done | jq -s '.' > 理赔风险评估.json
```

## 示例

### 欺诈检测
```bash
# 监控销售数据
benf sales_report.xlsx --threshold high

# 实时日志监控
tail -f transactions.log | benf --format json | jq 'select(.risk_level == "HIGH")'

# 批量分析
find . -name "*.csv" -exec benf {} \; | grep "HIGH"
```

### 中文数字
```bash
# 全角数字
echo "１２３ ４５６ ７８９" | benf

# 汉数字
echo "一千二百三十四 五千六百七十八" | benf

# 混合模式
benf chinese_financial_report.pdf
```

### 网络分析
```bash
# 财务网站
benf --url https://company.com/earnings --format json

# API端点
benf --url https://api.finance.com/data --proxy http://proxy:8080
```

### 自动化
```bash
# 每日欺诈检查
#!/bin/bash
RESULT=$(benf daily_sales.csv --format json)
RISK=$(echo $RESULT | jq -r '.risk_level')
if [ "$RISK" = "HIGH" ]; then
    echo $RESULT | mail -s "欺诈警报" admin@company.com
fi
```

## 风险级别

| 级别 | 卡方p值 | 解释 |
|------|---------|------|
| 低 | p > 0.1 | 正常分布 |
| 中等 | 0.05 < p ≤ 0.1 | 轻微偏差 |
| 高 | 0.01 < p ≤ 0.05 | 显著偏差 |
| 严重 | p ≤ 0.01 | 操作的强证据 |

## 常见用例

- **会计审计**：检测操纵的财务记录
- **税务调查**：识别可疑的收入申报
- **选举监控**：验证选票计数的真实性
- **保险理赔**：发现欺诈性理赔模式
- **科学数据**：验证研究结果
- **质量控制**：监控制造数据

## ⚠️ 重要限制

**本福德定律不适用于：**
- **受限范围**：成人身高（1.5-2.0米）、年龄（0-100）、温度
- **顺序数据**：发票号码、员工ID、邮政编码
- **分配号码**：电话号码、社会安全号码、彩票号码
- **小数据集**：少于30-50个数字（统计分析不足）
- **单一来源数据**：来自相同过程/机器的相似量级数据
- **四舍五入数据**：大量四舍五入的金额（例如，全部以00结尾）

**最适合：**
- **多尺度自然数据**：金融交易、人口、物理测量
- **多样化来源**：来自不同过程/时间框架的混合数据
- **大数据集**：100+数字用于可靠分析
- **未操作数据**：自然发生，未人为约束

## 历史背景

**发现与发展：**
- **1881年**：西蒙·纽科姆在研究对数表时首次观察到这一现象
- **1938年**：物理学家弗兰克·本福德重新发现并通过广泛研究正式确立了该定律
- **1972年**：会计和欺诈检测学术文献中的首次应用
- **1980年代**：主要会计事务所开始将本福德定律用作标准审计工具
- **1990年代**：马克·尼格里尼在法务会计和税务欺诈检测中普及了其使用
- **2000年代+**：扩展到选举监控、科学数据验证和金融犯罪调查

**现代应用：**
- 美国国税局用于税务审计筛查
- 四大会计事务所的标准工具
- 应用于选举欺诈检测（特别是2009年伊朗选举分析）
- 用于反洗钱调查

## 退出代码

| 代码 | 含义 |
|------|------|
| 0 | 成功 |
| 1 | 一般错误 |
| 2 | 无效参数 |
| 3 | 文件/网络错误 |
| 10 | 检测到高风险 |
| 11 | 检测到严重风险 |

## 配置

Benf尊重标准环境变量：
- `HTTP_PROXY` / `HTTPS_PROXY`：代理设置
- `NO_PROXY`：代理绕过列表

日志写入到：
- Linux：`~/.local/state/benf/`
- macOS：`~/Library/Logs/benf/`
- Windows：`%APPDATA%\benf\Logs\`

## 贡献

有关开发指南，请参阅[CONTRIBUTING.md](CONTRIBUTING.md)。

## 许可证

MIT许可证 - 请参阅[LICENSE](LICENSE)文件。

## 参考资料

- [本福德定律 - 维基百科](https://zh.wikipedia.org/wiki/本福特定律)
- [使用本福德定律进行欺诈检测](https://example.com/benford-fraud)