# lawkit 统一API参考

*lawkit-python 和 lawkit-js 语言绑定API文档*

## 概述

lawkit 提供用于统计法分析的统一API，特别适用于欺诈检测、数据质量评估和审计合规。它实现了各种统计法，包括本福德定律、帕累托分布、齐普夫定律等。

## 主函数

### `law(subcommand, data_or_config, options)`

使用统计法分析数据或执行实用程序操作。

#### 参数

- `subcommand` (String): 要执行的分析类型或实用程序命令
- `data_or_config` (Value): 根据子命令的输入数据或配置
- `options` (LawkitOptions, optional): 分析的配置选项

#### 返回值

- `Result<LawkitResult, Error>`: 分析结果或实用程序输出

#### 示例

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

// 本福德定律分析
let data = json!([123, 456, 789, 111, 222, 333]);
let options = LawkitOptions {
    format: Some("json".to_string()),
    min_count: Some(100),
    ..Default::default()
};

let result = law("benford", &data, Some(&options))?;
```

## 子命令

### 分析命令

#### `benford` - 本福德定律分析
分析数据在首位数字分布中是否遵循本福德定律。

```rust
let data = json!([/* 数值数据 */]);
let result = law("benford", &data, None)?;
```

#### `pareto` - 帕累托分布分析
检查数据是否遵循80/20法则或其他帕累托分布。

```rust
let data = json!({ "values": [/* 数据点 */] });
let result = law("pareto", &data, None)?;
```

#### `zipf` - 齐普夫定律分析
分析数据是否遵循齐普夫定律（在自然语言和城市人口中常见）。

```rust
let data = json!({ "frequencies": [/* 频率数据 */] });
let result = law("zipf", &data, None)?;
```

#### `normal` - 正态分布分析
测试数据是否遵循正态（高斯）分布。

```rust
let data = json!([/* 数值数据 */]);
let result = law("normal", &data, None)?;
```

#### `poisson` - 泊松分布分析
检查数据是否遵循泊松分布（固定区间内的事件）。

```rust
let data = json!({ "events": [/* 事件计数 */] });
let result = law("poisson", &data, None)?;
```

#### `analyze` - 自动分析
自动检测并应用相关的统计法。

```rust
let data = json!({ "dataset": [/* 混合数据 */] });
let result = law("analyze", &data, None)?;
```

### 实用程序命令

#### `validate` - 数据验证
验证法律分析的数据格式和结构。

```rust
let data = json!({ "data": [/* 要验证的数据 */] });
let result = law("validate", &data, None)?;
```

#### `diagnose` - 诊断信息
提供关于数据和潜在分析的诊断信息。

```rust
let data = json!({ "dataset": [/* 数据 */] });
let result = law("diagnose", &data, None)?;
```

#### `generate` - 测试数据生成
生成遵循指定统计法的测试数据。

```rust
let config = json!({
    "law": "benford",
    "count": 1000,
    "seed": 42
});
let result = law("generate", &config, None)?;
```

#### `list` - 列出可用法则
列出所有可用的统计法及其描述。

```rust
let empty = json!({});
let result = law("list", &empty, None)?;
```

#### `selftest` - 自检
运行内部测试以验证lawkit功能。

```rust
let empty = json!({});
let result = law("selftest", &empty, None)?;
```

## 选项

### LawkitOptions 结构体

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

### 选项详情

#### 输出控制
- **`format`**: 输出格式
  - 选项: `"json"`, `"csv"`, `"text"`, `"markdown"`
  - 默认值: `"text"`
  
- **`quiet`**: 抑制非必要输出
  - 默认值: `false`
  
- **`verbose`**: 启用详细输出
  - 默认值: `false`
  
- **`no_color`**: 禁用彩色输出
  - 默认值: `false`

#### 分析参数
- **`min_count`**: 分析所需的最少数据点
  - 默认值: `30`（因法则而异）
  
- **`confidence_level`**: 统计置信水平（0.0-1.0）
  - 默认值: `0.95`（95%置信度）
  
- **`significance_level`**: 假设检验的α水平
  - 默认值: `0.05`

#### 分析过滤器
- **`filter`**: 分析前过滤数据
  - 示例: `"value > 100"` 或 `"category == 'sales'"`
  
- **`laws`**: 要检查的特定法则（用于`analyze`命令）
  - 示例: `["benford", "pareto"]`
  
- **`focus`**: 将分析聚焦于特定方面
  - 示例: `"first_digit"` 或 `"distribution_tail"`

#### 阈值
- **`threshold`**: 法则遵循的自定义阈值
  - 默认值: 因法则而异
  
- **`tolerance`**: 期望值的可接受偏差
  - 默认值: 因法则而异

#### 高级选项
- **`statistical_tests`**: 要执行的额外统计测试
  - 选项: `["chi_square", "kolmogorov_smirnov", "anderson_darling"]`
  
- **`include_metadata`**: 在结果中包含详细元数据
  - 默认值: `false`
  
- **`detailed_report`**: 生成全面的分析报告
  - 默认值: `false`

## 结果类型

### LawkitResult 枚举

```rust
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

### 分析数据结构体

#### BenfordData
```rust
pub struct BenfordData {
    pub digit_frequencies: HashMap<u8, f64>,
    pub expected_frequencies: HashMap<u8, f64>,
    pub chi_square_statistic: f64,
    pub p_value: f64,
    pub compliant: bool,
    pub sample_size: usize,
}
```

#### ParetoData
```rust
pub struct ParetoData {
    pub ratio: (f64, f64),  // 例如：(80.0, 20.0)
    pub gini_coefficient: f64,
    pub lorenz_curve: Vec<(f64, f64)>,
    pub compliant: bool,
}
```

## 语言绑定

### Python

```python
import lawkit_python

# 本福德定律分析
data = [123, 456, 789, 111, 222, 333]
result = lawkit_python.law("benford", data)

# 带选项
result = lawkit_python.law(
    "benford",
    data,
    format="json",
    min_count=100,
    confidence_level=0.99,
    detailed_report=True
)

# 自动分析
result = lawkit_python.law(
    "analyze",
    {"dataset": data},
    laws=["benford", "pareto", "normal"],
    verbose=True
)
```

### TypeScript/JavaScript

```typescript
import { law, LawkitOptions } from 'lawkit-js';

// 本福德定律分析
const data = [123, 456, 789, 111, 222, 333];
const result = await law('benford', data);

// 带选项
const options: LawkitOptions = {
    outputFormat: 'json',
    minSampleSize: 100,
    confidenceLevel: 0.99,
    detailedReport: true
};
const result = await law('benford', data, options);

// 自动分析
const analysisOptions: LawkitOptions = {
    lawsToCheck: ['benford', 'pareto', 'normal'],
    includeMetadata: true
};
const result = await law('analyze', { dataset: data }, analysisOptions);
```

## 示例

### 使用本福德定律进行欺诈检测

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

// 分析金融交易
let transactions = json!([10234, 2341, 45632, 1234, 8765, /*...*/]);

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
```

### 数据质量评估

```rust
// 使用多个法则检查数据质量
let options = LawkitOptions {
    laws: Some(vec!["benford".to_string(), 
                    "pareto".to_string(), 
                    "normal".to_string()]),
    verbose: Some(true),
    ..Default::default()
};

let result = law("analyze", &dataset, Some(&options))?;
```

### 生成测试数据

```rust
let config = json!({
    "law": "zipf",
    "count": 10000,
    "parameters": {
        "s": 1.2,
        "n": 100
    },
    "seed": 12345
});

let result = law("generate", &config, None)?;
```

## 性能考虑

- **大数据集**: lawkit自动对超过100MB的数据集使用流式分析
- **内存使用**: 使用`filter`选项在分析前减少数据集大小
- **并行处理**: `analyze`中的多个法则并行处理
- **缓存**: 对同一数据集的重复分析会缓存结果

## 错误处理

为以下情况提供详细错误：
- 数据不足（低于`min_count`）
- 无效的数据格式
- 统计计算失败
- 内存分配问题
- 无效的子命令或选项

## 最佳实践

1. **数据大小**: 确保有足够的数据点（本福德定律通常>1000）
2. **数据清理**: 分析前使用`validate`检查数据质量
3. **多法则**: 使用`analyze`自动检测适用的法则
4. **置信水平**: 根据用例调整（欺诈检测：0.99，一般分析：0.95）
5. **过滤**: 将数据预过滤为相关子集以获得更准确的分析