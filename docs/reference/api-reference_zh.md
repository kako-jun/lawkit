# API 参考 - lawkit-core

提供统计法则分析功能的 `lawkit-core` Rust crate 完整 API 文档。

## 概述

`lawkit-core` crate 是 lawkit 生态系统的核心，为数据质量评估和欺诈检测提供全面的统计法则分析。它可以嵌入到其他 Rust 应用程序中以添加统计分析功能。

**统一API设计**：核心 API 仅公开一个主函数 `law()` 用于所有分析操作。所有功能都通过子命令和选项参数从这个统一接口访问。这种设计确保了所有用例的一致性和简单性。

## 安装

将 `lawkit-core` 添加到您的 `Cargo.toml`：

```toml
[dependencies]
lawkit-core = "0.2.0"
```

### 功能标志

```toml
[dependencies]
lawkit-core = { version = "0.2.0", features = ["all-laws"] }
```

可用功能：
- `benford`（默认）- 本福德定律分析
- `pareto`（默认）- 帕累托分布分析
- `zipf`（默认）- 齐普夫定律分析
- `normal` - 正态分布测试
- `poisson` - 泊松分布分析
- `all-laws` - 启用所有统计法则

## 公共 API

### 核心类型

#### `LawkitResult`

表示统计法则分析的结果。

```rust
#[derive(Debug, PartialEq, Serialize)]
pub enum LawkitResult {
    // 分析结果
    BenfordAnalysis(String, BenfordData),
    ParetoAnalysis(String, ParetoData),
    ZipfAnalysis(String, ZipfData),
    NormalAnalysis(String, NormalData),
    PoissonAnalysis(String, PoissonData),
    
    // 实用程序结果
    ValidationResult(bool, Vec<String>),
    DiagnosticInfo(DiagnosticData),
    GeneratedData(Vec<Value>),
    LawList(Vec<LawInfo>),
    SelfTestResult(TestResults),
    
    // 多法则分析
    MultiLawAnalysis(Vec<(String, AnalysisResult)>),
}
```

#### 分析数据结构

```rust
pub struct BenfordData {
    pub digit_frequencies: HashMap<u8, f64>,
    pub expected_frequencies: HashMap<u8, f64>,
    pub chi_square_statistic: f64,
    pub p_value: f64,
    pub compliant: bool,
    pub sample_size: usize,
}

pub struct ParetoData {
    pub ratio: (f64, f64),  // 例如：(80.0, 20.0)
    pub gini_coefficient: f64,
    pub lorenz_curve: Vec<(f64, f64)>,
    pub compliant: bool,
}
```

### 核心函数

#### `law()`

统计法则分析和实用程序操作的主要函数。这是所有 lawkit 操作的统一 API 入口点。

```rust
pub fn law(
    subcommand: &str,
    data_or_config: &Value,
    options: Option<&LawkitOptions>,
) -> Result<LawkitResult, Error>
```

**参数：**
- `subcommand`：要执行的分析类型或实用程序命令
- `data_or_config`：根据子命令的输入数据或配置
- `options`：分析的可选配置选项

**返回值：**表示分析结果的 `Result<LawkitResult, Error>`

#### LawkitOptions 结构体

```rust
pub struct LawkitOptions {
    // 输出控制
    pub format: Option<String>,
    pub quiet: Option<bool>,
    pub verbose: Option<bool>,
    pub no_color: Option<bool>,
    
    // 分析参数
    pub min_count: Option<usize>,
    pub confidence_level: Option<f64>,
    pub significance_level: Option<f64>,
    
    // 分析过滤器
    pub filter: Option<String>,
    pub laws: Option<Vec<String>>,
    pub focus: Option<String>,
    
    // 阈值
    pub threshold: Option<f64>,
    pub tolerance: Option<f64>,
    
    // 高级选项
    pub statistical_tests: Option<Vec<String>>,
    pub include_metadata: Option<bool>,
    pub detailed_report: Option<bool>,
}
```

**示例：**
```rust
use lawkit_core::{law, LawkitOptions, LawkitResult};
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 用于欺诈检测的金融交易数据
    let transactions = json!([10234, 2341, 45632, 1234, 8765]);
    
    let options = LawkitOptions {
        min_count: Some(1000),
        confidence_level: Some(0.99),
        detailed_report: Some(true),
        ..Default::default()
    };
    
    let result = law("benford", &transactions, Some(&options))?;
    
    match result {
        LawkitResult::BenfordAnalysis(_, data) => {
            if !data.compliant {
                println!("警告：检测到潜在欺诈！");
                println!("卡方统计量：{}，p值：{}", 
                         data.chi_square_statistic, 
                         data.p_value);
            }
        }
        _ => {}
    }
    
    Ok(())
}
```

## 子命令

### 分析命令

- **`benford`** - 首位数字分布的本福德定律分析
- **`pareto`** - 帕累托分布（80/20法则）分析
- **`zipf`** - 排名频率分布的齐普夫定律分析
- **`normal`** - 正态分布测试
- **`poisson`** - 泊松分布分析
- **`analyze`** - 自动多法则分析

### 实用程序命令

- **`validate`** - 验证数据格式和结构
- **`diagnose`** - 提供数据相关的诊断信息
- **`generate`** - 生成遵循指定法则的测试数据
- **`list`** - 列出可用的统计法则
- **`selftest`** - 运行内部测试

## 高级用法

### 自定义分析逻辑

#### 使用本福德定律进行欺诈检测

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

fn detect_fraud(transactions: Vec<f64>) -> Result<bool, Box<dyn std::error::Error>> {
    let data = json!(transactions);
    
    let options = LawkitOptions {
        min_count: Some(1000),
        confidence_level: Some(0.99),
        statistical_tests: Some(vec!["chi_square".to_string(), "kolmogorov_smirnov".to_string()]),
        ..Default::default()
    };
    
    let result = law("benford", &data, Some(&options))?;
    
    match result {
        LawkitResult::BenfordAnalysis(_, data) => {
            Ok(!data.compliant && data.p_value < 0.01)
        }
        _ => Ok(false)
    }
}
```

#### 多法则分析

```rust
let options = LawkitOptions {
    laws: Some(vec!["benford".to_string(), "pareto".to_string(), "normal".to_string()]),
    verbose: Some(true),
    include_metadata: Some(true),
    ..Default::default()
};

let result = law("analyze", &dataset, Some(&options))?;

match result {
    LawkitResult::MultiLawAnalysis(analyses) => {
        for (law_name, analysis) in analyses {
            println!("{}: {}", law_name, if analysis.compliant { "✓" } else { "✗" });
        }
    }
    _ => {}
}
```

### 处理不同数据类型

#### CSV 数据分析

```rust
use lawkit_core::{law, LawkitOptions};
use csv::Reader;
use serde_json::json;

fn analyze_csv_column(
    file_path: &str,
    column_index: usize
) -> Result<LawkitResult, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut values = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        if let Some(value) = record.get(column_index) {
            if let Ok(num) = value.parse::<f64>() {
                values.push(num);
            }
        }
    }
    
    let data = json!(values);
    Ok(law("benford", &data, None)?)
}
```

### 集成模式

#### 持续监控

```rust
use lawkit_core::{law, LawkitOptions, LawkitResult};
use serde_json::Value;
use std::time::Duration;
use tokio;

struct DataMonitor {
    pub threshold: f64,
    pub min_sample_size: usize,
}

impl DataMonitor {
    pub async fn monitor_data_quality(
        &self,
        data_source: impl Fn() -> Value
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // 每小时
        
        loop {
            interval.tick().await;
            
            let data = data_source();
            let options = LawkitOptions {
                min_count: Some(self.min_sample_size),
                threshold: Some(self.threshold),
                ..Default::default()
            };
            
            let result = law("analyze", &data, Some(&options))?;
            
            match result {
                LawkitResult::MultiLawAnalysis(analyses) => {
                    let non_compliant: Vec<_> = analyses
                        .iter()
                        .filter(|(_, a)| !a.compliant)
                        .collect();
                    
                    if !non_compliant.is_empty() {
                        println!("数据质量警报：违反了 {} 个法则", 
                                 non_compliant.len());
                    }
                }
                _ => {}
            }
        }
    }
}
```

#### 测试数据生成

```rust
use lawkit_core::{law, LawkitResult};
use serde_json::json;

fn generate_benford_data(count: usize, seed: u64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let config = json!({
        "law": "benford",
        "count": count,
        "seed": seed
    });
    
    let result = law("generate", &config, None)?;
    
    match result {
        LawkitResult::GeneratedData(values) => {
            Ok(values.into_iter()
                .filter_map(|v| v.as_f64())
                .collect())
        }
        _ => Err("意外的结果类型".into())
    }
}
```

## 错误处理

### 错误类型

该库使用 `anyhow::Error` 进行错误处理：

```rust
use lawkit_core::{law, LawkitOptions};
use anyhow::Result;

fn handle_analysis_errors(data: &Value) -> Result<()> {
    match law("benford", data, None) {
        Ok(result) => {
            println!("分析成功完成");
        }
        Err(e) => {
            eprintln!("分析错误：{}", e);
            
            // 检查特定错误条件
            if e.to_string().contains("insufficient data") {
                eprintln!("分析需要至少 30 个数据点");
            } else if e.to_string().contains("invalid format") {
                eprintln!("数据必须是数值");
            }
        }
    }
    
    Ok(())
}
```

## 性能考虑

### 大数据集分析

```rust
use lawkit_core::{law, LawkitOptions};

fn analyze_large_dataset(data: &Value) -> Result<LawkitResult, Box<dyn std::error::Error>> {
    let options = LawkitOptions {
        // 过滤器以减少数据集大小
        filter: Some("value > 100 AND value < 1000000".to_string()),
        
        // 对于非常大的数据集使用采样
        statistical_tests: Some(vec!["chi_square".to_string()]), // 更快的测试
        
        // 为了性能禁用详细报告
        detailed_report: Some(false),
        include_metadata: Some(false),
        
        ..Default::default()
    };
    
    Ok(law("benford", data, Some(&options))?)
}
```

### 优化提示

1. **使用 `filter` 选项预过滤数据**
2. **设置适当的 min_count 以避免不必要的计算**
3. **对大规模分析禁用详细报告**
4. **尽可能使用特定法则而不是 `analyze`**
5. **为同一数据集的重复分析缓存结果**

## 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_benford_compliance() {
        // 已知的本福德兼容数据
        let fibonacci = json!([1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]);
        
        let result = law("benford", &fibonacci, None).unwrap();
        
        match result {
            LawkitResult::BenfordAnalysis(_, data) => {
                assert!(data.sample_size == 12);
                // 斐波那契数列遵循本福德定律
                assert!(data.compliant);
            }
            _ => panic!("意外的结果类型")
        }
    }
    
    #[test]
    fn test_data_generation() {
        let config = json!({
            "law": "zipf",
            "count": 100,
            "parameters": {"s": 1.2, "n": 50},
            "seed": 42
        });
        
        let result = law("generate", &config, None).unwrap();
        
        match result {
            LawkitResult::GeneratedData(values) => {
                assert_eq!(values.len(), 100);
            }
            _ => panic!("意外的结果类型")
        }
    }
}
```

## 版本兼容性

- **0.2.x**：当前稳定版本
- **最低 Rust 版本**：1.70.0
- **依赖项**：请参阅 `Cargo.toml` 了解当前版本

## 另请参阅

- [CLI 参考](cli-reference_zh.md) - 命令行使用
- [入门指南](../user-guide/getting-started_zh.md) - 基本概念
- [统一API参考](../bindings/unified-api_zh.md) - 语言绑定
- [高级分析指南](../guides/advanced-analysis_zh.md) - 复杂用例