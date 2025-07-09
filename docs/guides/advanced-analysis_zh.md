# 高级分析指南

本指南介绍lawkit的高级分析功能，包括复杂数据场景、自定义分析工作流和专业技术。

## 高级统计分析

### 多维度分析

```bash
# 时间序列分析
lawkit normal daily_metrics.csv --enable-timeseries --timeseries-window 30

# 季节性分解
lawkit normal monthly_sales.csv --enable-timeseries --seasonal-decomposition

# 趋势检测
lawkit normal stock_prices.csv --enable-timeseries --trend-detection --changepoint-analysis
```

### 异常值检测方法

#### 集成方法（推荐）
```bash
# 使用多种方法的集成
lawkit normal data.csv --outliers --outlier-method ensemble --verbose

# 输出详细的异常值信息
lawkit normal measurements.csv --outliers --outlier-method ensemble --format json | \
jq '.outliers[] | select(.confidence > 0.8)'
```

#### 特定方法
```bash
# LOF (局部异常因子) - 适用于复杂模式
lawkit normal complex_data.csv --outliers --outlier-method lof --neighbors 20

# 隔离森林 - 适用于高维数据
lawkit normal high_dim_data.csv --outliers --outlier-method isolation --contamination 0.1

# DBSCAN - 适用于密度聚类
lawkit normal clustered_data.csv --outliers --outlier-method dbscan --eps 0.5 --min-samples 5
```

### 工艺能力分析

```bash
# 完整的工艺能力分析
lawkit normal production_data.csv --process-capability --control-limits --format json

# 自定义规格限制
lawkit normal quality_data.csv --process-capability \
  --lower-spec-limit 95 --upper-spec-limit 105 --target 100

# Cpk和Ppk计算
lawkit normal process_measurements.csv --process-capability --verbose | \
grep -E "Cpk|Ppk"
```

## 复杂数据场景

### 混合分布分析

```bash
# 检测多模态分布
lawkit normal mixed_data.csv --multimodal-detection --verbose

# 分离混合组件
lawkit normal bimodal_data.csv --mixture-analysis --components 2 --format json

# 自动确定组件数
lawkit normal unknown_mixture.csv --mixture-analysis --auto-components
```

### 稀有事件分析

```bash
# 泊松稀有事件检测
lawkit poisson incident_data.csv --rare-events --confidence-level 0.99

# 极值理论分析
lawkit normal extreme_data.csv --extreme-value-analysis --threshold auto

# 尾部风险评估
lawkit normal financial_returns.csv --tail-risk --var-level 0.05 --cvar
```

### 时间相关分析

```bash
# 变化点检测
lawkit normal time_series.csv --enable-timeseries --changepoint-detection \
  --method bayesian --sensitivity 0.8

# 漂移检测
lawkit normal sensor_data.csv --enable-timeseries --drift-detection \
  --window-size 100 --threshold 0.05

# 周期性分析
lawkit normal seasonal_data.csv --enable-timeseries --periodicity-analysis \
  --max-period 365
```

## 自定义分析工作流

### 多阶段分析管道

```bash
#!/bin/bash
# advanced_analysis_pipeline.sh

INPUT_FILE="$1"
OUTPUT_DIR="analysis_results"
mkdir -p "$OUTPUT_DIR"

echo "=== 开始高级分析管道 ==="
echo "输入文件: $INPUT_FILE"
echo "输出目录: $OUTPUT_DIR"

# 阶段1: 初步数据质量检查
echo "阶段1: 数据质量检查"
lawkit benf "$INPUT_FILE" --format json > "$OUTPUT_DIR/quality_check.json"

quality_risk=$(jq -r '.risk_level' "$OUTPUT_DIR/quality_check.json")
echo "数据质量风险: $quality_risk"

if [ "$quality_risk" = "HIGH" ] || [ "$quality_risk" = "CRITICAL" ]; then
    echo "警告: 检测到高风险数据质量问题"
    jq '.verdict' "$OUTPUT_DIR/quality_check.json"
fi

# 阶段2: 分布类型识别
echo "阶段2: 分布识别"
lawkit analyze "$INPUT_FILE" --laws all --format json > "$OUTPUT_DIR/distribution_analysis.json"

best_fit=$(jq -r '.recommendations[0].law' "$OUTPUT_DIR/distribution_analysis.json")
echo "最佳拟合分布: $best_fit"

# 阶段3: 深度分析（基于最佳拟合）
echo "阶段3: 深度分析"
case $best_fit in
    "normal")
        lawkit normal "$INPUT_FILE" --outliers --outlier-method ensemble \
          --quality-control --process-capability --enable-timeseries \
          --format json > "$OUTPUT_DIR/deep_analysis.json"
        ;;
    "pareto")
        lawkit pareto "$INPUT_FILE" --gini-coefficient --business-analysis \
          --percentiles "70,80,90,95,99" --format json > "$OUTPUT_DIR/deep_analysis.json"
        ;;
    "poisson")
        lawkit poisson "$INPUT_FILE" --predict --rare-events \
          --time-unit auto --format json > "$OUTPUT_DIR/deep_analysis.json"
        ;;
    *)
        lawkit "$best_fit" "$INPUT_FILE" --verbose --format json > "$OUTPUT_DIR/deep_analysis.json"
        ;;
esac

# 阶段4: 异常检测
echo "阶段4: 异常检测"
lawkit normal "$INPUT_FILE" --outliers --outlier-method ensemble \
  --format json > "$OUTPUT_DIR/anomaly_detection.json"

anomaly_count=$(jq '.outliers | length' "$OUTPUT_DIR/anomaly_detection.json")
echo "检测到异常值数量: $anomaly_count"

# 阶段5: 生成综合报告
echo "阶段5: 生成报告"
python3 <<EOF
import json
import datetime

# 读取所有分析结果
with open('$OUTPUT_DIR/quality_check.json') as f:
    quality = json.load(f)
with open('$OUTPUT_DIR/distribution_analysis.json') as f:
    distribution = json.load(f)
with open('$OUTPUT_DIR/deep_analysis.json') as f:
    deep = json.load(f)
with open('$OUTPUT_DIR/anomaly_detection.json') as f:
    anomaly = json.load(f)

# 生成综合报告
report = {
    "analysis_timestamp": datetime.datetime.now().isoformat(),
    "input_file": "$INPUT_FILE",
    "summary": {
        "data_quality_risk": quality.get('risk_level'),
        "best_fit_distribution": distribution.get('recommendations', [{}])[0].get('law'),
        "anomaly_count": len(anomaly.get('outliers', [])),
        "total_data_points": quality.get('numbers_analyzed')
    },
    "detailed_results": {
        "quality_assessment": quality,
        "distribution_comparison": distribution,
        "specialized_analysis": deep,
        "anomaly_detection": anomaly
    },
    "recommendations": []
}

# 生成建议
if quality.get('risk_level') in ['HIGH', 'CRITICAL']:
    report['recommendations'].append("数据质量需要改进，建议检查数据收集过程")

if len(anomaly.get('outliers', [])) > 0:
    report['recommendations'].append(f"检测到{len(anomaly.get('outliers', []))}个异常值，建议进一步调查")

if distribution.get('conflicts'):
    report['recommendations'].append("检测到分布冲突，数据可能来自多个来源")

# 保存报告
with open('$OUTPUT_DIR/comprehensive_report.json', 'w') as f:
    json.dump(report, f, indent=2, ensure_ascii=False)

print("综合报告已生成: $OUTPUT_DIR/comprehensive_report.json")
EOF

echo "=== 分析管道完成 ==="
echo "结果保存在: $OUTPUT_DIR/"
```

### 批量分析自动化

```bash
#!/bin/bash
# batch_advanced_analysis.sh

DATA_DIR="$1"
RESULTS_DIR="batch_analysis_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

# 批量处理所有文件
find "$DATA_DIR" -name "*.csv" | xargs -P 8 -I {} bash -c "
    echo '处理文件: {}'
    
    # 创建文件特定的结果目录
    file_base=\$(basename {} .csv)
    file_dir='$RESULTS_DIR/\$file_base'
    mkdir -p \"\$file_dir\"
    
    # 运行完整分析
    ./advanced_analysis_pipeline.sh {} > \"\$file_dir/analysis.log\" 2>&1
    
    # 移动结果到正确位置
    mv analysis_results/* \"\$file_dir/\" 2>/dev/null || true
    rmdir analysis_results 2>/dev/null || true
    
    echo '完成: {}'
"

# 生成批量摘要报告
python3 <<EOF
import json
import os
import glob

results_dir = "$RESULTS_DIR"
summary = {
    "batch_analysis_summary": {
        "total_files": 0,
        "high_risk_files": [],
        "anomaly_summary": {},
        "distribution_summary": {}
    }
}

for file_dir in glob.glob(f"{results_dir}/*/"):
    report_file = os.path.join(file_dir, "comprehensive_report.json")
    if os.path.exists(report_file):
        with open(report_file) as f:
            report = json.load(f)
        
        filename = os.path.basename(file_dir.rstrip('/'))
        summary["batch_analysis_summary"]["total_files"] += 1
        
        # 收集高风险文件
        risk = report["summary"].get("data_quality_risk")
        if risk in ["HIGH", "CRITICAL"]:
            summary["batch_analysis_summary"]["high_risk_files"].append({
                "file": filename,
                "risk_level": risk
            })
        
        # 统计异常值
        anomaly_count = report["summary"].get("anomaly_count", 0)
        if anomaly_count > 0:
            summary["batch_analysis_summary"]["anomaly_summary"][filename] = anomaly_count
        
        # 统计分布类型
        dist = report["summary"].get("best_fit_distribution")
        if dist:
            if dist not in summary["batch_analysis_summary"]["distribution_summary"]:
                summary["batch_analysis_summary"]["distribution_summary"][dist] = 0
            summary["batch_analysis_summary"]["distribution_summary"][dist] += 1

# 保存批量摘要
with open(f"{results_dir}/batch_summary.json", "w") as f:
    json.dump(summary, f, indent=2, ensure_ascii=False)

print(f"批量分析完成，摘要保存在: {results_dir}/batch_summary.json")
print(f"处理文件数: {summary['batch_analysis_summary']['total_files']}")
print(f"高风险文件数: {len(summary['batch_analysis_summary']['high_risk_files'])}")
EOF
```

## 专业分析技术

### 金融风险分析

```bash
# VaR (风险价值) 分析
lawkit normal returns.csv --tail-risk --var-level 0.05 --format json | \
jq '.tail_risk.var_95'

# 压力测试
lawkit normal portfolio_values.csv --stress-test --scenarios extreme_market.json

# 流动性风险分析
lawkit poisson trading_volumes.csv --liquidity-analysis --time-unit minute
```

### 质量控制高级功能

```bash
# 多变量控制图
lawkit normal multivariate_quality.csv --multivariate-control-chart \
  --variables "temperature,pressure,flow_rate"

# 工艺能力趋势分析
lawkit normal daily_capability.csv --process-capability --trend-analysis \
  --time-column date --format json

# 六西格玛分析
lawkit normal six_sigma_data.csv --six-sigma-analysis --defect-opportunity 1000000
```

### 预测分析

```bash
# 时间序列预测
lawkit normal time_series.csv --enable-timeseries --forecast \
  --forecast-horizon 30 --confidence-interval 0.95

# 异常预测
lawkit normal sensor_data.csv --anomaly-prediction --prediction-window 24 \
  --alert-threshold 0.8

# 趋势外推
lawkit normal growth_data.csv --trend-extrapolation --extrapolation-periods 12
```

## 可视化和报告

### 高级可视化脚本

```python
#!/usr/bin/env python3
# advanced_visualization.py

import json
import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd
import numpy as np
from scipy import stats
import argparse

def load_lawkit_results(filename):
    """加载lawkit分析结果"""
    with open(filename, 'r') as f:
        return json.load(f)

def plot_benford_analysis(results, output_file):
    """绘制本福德分析图"""
    digits = results['digits']
    
    digit_nums = list(range(1, 10))
    observed = [digits[str(d)]['observed'] for d in digit_nums]
    expected = [digits[str(d)]['expected'] for d in digit_nums]
    
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 6))
    
    # 柱状图比较
    x = np.arange(len(digit_nums))
    width = 0.35
    
    ax1.bar(x - width/2, observed, width, label='观察值', alpha=0.8)
    ax1.bar(x + width/2, expected, width, label='期望值', alpha=0.8)
    ax1.set_xlabel('首位数字')
    ax1.set_ylabel('百分比')
    ax1.set_title('本福德分布比较')
    ax1.set_xticks(x)
    ax1.set_xticklabels(digit_nums)
    ax1.legend()
    ax1.grid(True, alpha=0.3)
    
    # 偏差图
    deviations = [digits[str(d)]['deviation'] for d in digit_nums]
    colors = ['red' if abs(d) > 2 else 'blue' for d in deviations]
    
    ax2.bar(digit_nums, deviations, color=colors, alpha=0.7)
    ax2.axhline(y=0, color='black', linestyle='-', alpha=0.5)
    ax2.axhline(y=2, color='red', linestyle='--', alpha=0.5, label='阈值 ±2%')
    ax2.axhline(y=-2, color='red', linestyle='--', alpha=0.5)
    ax2.set_xlabel('首位数字')
    ax2.set_ylabel('偏差 (%)')
    ax2.set_title('与期望值的偏差')
    ax2.legend()
    ax2.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"本福德分析图保存为: {output_file}")

def plot_anomaly_detection(results, data_file, output_file):
    """绘制异常检测图"""
    # 加载原始数据
    data = pd.read_csv(data_file, header=None, names=['value'])
    
    # 获取异常值索引
    outliers = results.get('outliers', [])
    outlier_indices = [o['index'] for o in outliers]
    
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 8))
    
    # 时间序列图
    ax1.plot(data['value'], color='blue', alpha=0.7, label='正常数据')
    if outlier_indices:
        ax1.scatter(outlier_indices, data.iloc[outlier_indices]['value'], 
                   color='red', s=50, label='异常值', zorder=5)
    ax1.set_title('异常值检测结果')
    ax1.set_ylabel('数值')
    ax1.legend()
    ax1.grid(True, alpha=0.3)
    
    # 箱线图
    box_data = [data[~data.index.isin(outlier_indices)]['value']]
    if outlier_indices:
        box_data.append(data.iloc[outlier_indices]['value'])
        labels = ['正常数据', '异常值']
    else:
        labels = ['数据']
    
    ax2.boxplot(box_data, labels=labels)
    ax2.set_title('数据分布箱线图')
    ax2.set_ylabel('数值')
    ax2.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"异常检测图保存为: {output_file}")

def plot_distribution_comparison(results, output_file):
    """绘制分布比较图"""
    laws = results.get('analysis_results', {})
    
    fig, axes = plt.subplots(2, 3, figsize=(18, 12))
    axes = axes.flatten()
    
    law_names = {
        'benford': '本福德分布',
        'pareto': '帕累托分布', 
        'zipf': '齐夫分布',
        'normal': '正态分布',
        'poisson': '泊松分布'
    }
    
    for i, (law, result) in enumerate(laws.items()):
        if i >= len(axes):
            break
            
        ax = axes[i]
        
        # 绘制适合度指标
        if 'statistics' in result:
            stats_data = result['statistics']
            metrics = []
            values = []
            
            if 'chi_square' in stats_data:
                metrics.append('卡方值')
                values.append(stats_data['chi_square'])
            
            if 'p_value' in stats_data:
                metrics.append('p值')
                values.append(stats_data['p_value'])
            
            if 'mad' in stats_data:
                metrics.append('MAD')
                values.append(stats_data['mad'])
            
            if metrics:
                ax.bar(metrics, values, alpha=0.7)
                ax.set_title(f'{law_names.get(law, law)} - 统计指标')
                ax.tick_params(axis='x', rotation=45)
                
        # 风险等级颜色编码
        risk_level = result.get('risk_level', 'UNKNOWN')
        risk_colors = {'LOW': 'green', 'MEDIUM': 'yellow', 'HIGH': 'red', 'CRITICAL': 'darkred'}
        ax.set_facecolor(risk_colors.get(risk_level, 'white'))
        ax.set_alpha(0.1)
    
    # 移除未使用的子图
    for i in range(len(laws), len(axes)):
        fig.delaxes(axes[i])
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"分布比较图保存为: {output_file}")

def main():
    parser = argparse.ArgumentParser(description='高级lawkit结果可视化')
    parser.add_argument('result_file', help='lawkit分析结果JSON文件')
    parser.add_argument('--data-file', help='原始数据文件（用于异常检测图）')
    parser.add_argument('--output-dir', default='.', help='输出目录')
    
    args = parser.parse_args()
    
    # 加载结果
    results = load_lawkit_results(args.result_file)
    
    # 根据分析类型生成相应图表
    base_name = args.result_file.replace('.json', '')
    
    if 'digits' in results:  # 本福德分析
        plot_benford_analysis(results, f"{args.output_dir}/{base_name}_benford.png")
    
    if 'outliers' in results and args.data_file:  # 异常检测
        plot_anomaly_detection(results, args.data_file, f"{args.output_dir}/{base_name}_anomalies.png")
    
    if 'analysis_results' in results:  # 分布比较
        plot_distribution_comparison(results, f"{args.output_dir}/{base_name}_comparison.png")

if __name__ == "__main__":
    main()
```

### HTML报告生成

```python
#!/usr/bin/env python3
# generate_html_report.py

import json
import jinja2
from datetime import datetime
import argparse

# HTML模板
HTML_TEMPLATE = """
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>lawkit 高级分析报告</title>
    <style>
        body { font-family: 'Microsoft YaHei', Arial, sans-serif; margin: 20px; }
        .header { background: #2c3e50; color: white; padding: 20px; border-radius: 5px; }
        .summary { background: #ecf0f1; padding: 15px; margin: 20px 0; border-radius: 5px; }
        .section { margin: 20px 0; }
        .risk-low { color: green; font-weight: bold; }
        .risk-medium { color: orange; font-weight: bold; }
        .risk-high { color: red; font-weight: bold; }
        .risk-critical { color: darkred; font-weight: bold; }
        table { border-collapse: collapse; width: 100%; margin: 10px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        .metric { display: inline-block; margin: 10px; padding: 10px; background: #f8f9fa; border-radius: 5px; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🔍 lawkit 高级数据分析报告</h1>
        <p>生成时间: {{ timestamp }}</p>
        <p>分析文件: {{ input_file }}</p>
    </div>

    <div class="summary">
        <h2>📊 分析摘要</h2>
        <div class="metric">
            <strong>数据质量风险:</strong>
            <span class="risk-{{ summary.data_quality_risk.lower() }}">
                {{ summary.data_quality_risk }}
            </span>
        </div>
        <div class="metric">
            <strong>最佳拟合分布:</strong> {{ summary.best_fit_distribution }}
        </div>
        <div class="metric">
            <strong>异常值数量:</strong> {{ summary.anomaly_count }}
        </div>
        <div class="metric">
            <strong>数据点总数:</strong> {{ summary.total_data_points }}
        </div>
    </div>

    {% if recommendations %}
    <div class="section">
        <h2>💡 建议</h2>
        <ul>
        {% for rec in recommendations %}
            <li>{{ rec }}</li>
        {% endfor %}
        </ul>
    </div>
    {% endif %}

    <div class="section">
        <h2>🎯 详细分析结果</h2>
        
        {% if detailed_results.quality_assessment %}
        <h3>数据质量评估</h3>
        <table>
            <tr><th>指标</th><th>值</th></tr>
            <tr><td>卡方统计量</td><td>{{ detailed_results.quality_assessment.statistics.chi_square }}</td></tr>
            <tr><td>p值</td><td>{{ detailed_results.quality_assessment.statistics.p_value }}</td></tr>
            <tr><td>MAD</td><td>{{ detailed_results.quality_assessment.statistics.mad }}</td></tr>
            <tr><td>结论</td><td>{{ detailed_results.quality_assessment.verdict }}</td></tr>
        </table>
        {% endif %}

        {% if detailed_results.anomaly_detection and detailed_results.anomaly_detection.outliers %}
        <h3>🚨 异常值检测</h3>
        <table>
            <tr><th>索引</th><th>值</th><th>异常分数</th><th>方法</th></tr>
            {% for outlier in detailed_results.anomaly_detection.outliers[:10] %}
            <tr>
                <td>{{ outlier.index }}</td>
                <td>{{ outlier.value }}</td>
                <td>{{ outlier.score }}</td>
                <td>{{ outlier.method }}</td>
            </tr>
            {% endfor %}
        </table>
        {% if detailed_results.anomaly_detection.outliers|length > 10 %}
        <p><em>显示前10个异常值，总计{{ detailed_results.anomaly_detection.outliers|length }}个</em></p>
        {% endif %}
        {% endif %}
    </div>

    <div class="section">
        <h2>📈 可视化图表</h2>
        <p><em>请查看同目录下的PNG图像文件以获取详细的可视化分析。</em></p>
    </div>

    <div class="section">
        <h2>🔧 技术细节</h2>
        <details>
            <summary>点击查看完整JSON结果</summary>
            <pre style="background: #f4f4f4; padding: 15px; overflow-x: auto;">
{{ raw_json }}
            </pre>
        </details>
    </div>
</body>
</html>
"""

def generate_html_report(result_file, output_file):
    """生成HTML报告"""
    with open(result_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    template = jinja2.Template(HTML_TEMPLATE)
    
    html_content = template.render(
        timestamp=datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
        input_file=data.get('input_file', result_file),
        summary=data.get('summary', {}),
        recommendations=data.get('recommendations', []),
        detailed_results=data.get('detailed_results', {}),
        raw_json=json.dumps(data, indent=2, ensure_ascii=False)
    )
    
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write(html_content)
    
    print(f"HTML报告已生成: {output_file}")

def main():
    parser = argparse.ArgumentParser(description='生成lawkit HTML报告')
    parser.add_argument('result_file', help='分析结果JSON文件')
    parser.add_argument('--output', help='输出HTML文件名')
    
    args = parser.parse_args()
    
    if not args.output:
        args.output = args.result_file.replace('.json', '_report.html')
    
    generate_html_report(args.result_file, args.output)

if __name__ == "__main__":
    main()
```

## 下一步

- 查看[集成指南](integrations_zh.md)了解系统集成方法
- 参考[性能优化指南](performance_zh.md)了解大规模优化技巧
- 阅读[CLI参考文档](../reference/cli-reference_zh.md)了解所有可用命令