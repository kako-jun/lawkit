# 集成

学习如何将lawkit与其他工具和工作流程集成。

## CI/CD 集成

### GitHub Actions

```yaml
name: 数据质量检查

on:
  push:
    paths:
      - 'data/**/*.csv'

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: 安装lawkit
        run: cargo install lawkit
        
      - name: 质量分析
        run: |
          for file in data/*.csv; do
            lawkit analyze "$file" --laws benford,normal --format json > "qa_$(basename "$file" .csv).json"
          done
          
      - name: 上传结果
        uses: actions/upload-artifact@v3
        with:
          name: quality-reports
          path: qa_*.json
```

### GitLab CI

```yaml
stages:
  - quality-check

data-quality:
  stage: quality-check
  script:
    - cargo install lawkit
    - lawkit analyze data/financial.csv --laws all --format json > quality-report.json
  artifacts:
    reports:
      - quality-report.json
```

## API 集成

### REST API 封装器

```python
# Python 封装器示例
import subprocess
import json

class LawkitAPI:
    def __init__(self):
        self.binary = 'lawkit'
    
    def analyze_benford(self, data_file, output_format='json'):
        result = subprocess.run([
            self.binary, 'benf', data_file, 
            '--format', output_format
        ], capture_output=True, text=True)
        
        if output_format == 'json':
            return json.loads(result.stdout)
        return result.stdout
```

### Node.js 集成

```javascript
const { spawn } = require('child_process');

function analyzeBenford(dataFile) {
  return new Promise((resolve, reject) => {
    const lawkit = spawn('lawkit', ['benf', dataFile, '--format', 'json']);
    
    let output = '';
    lawkit.stdout.on('data', (data) => {
      output += data;
    });
    
    lawkit.on('close', (code) => {
      if (code === 0) {
        resolve(JSON.parse(output));
      } else {
        reject(new Error(`进程以代码 ${code} 退出`));
      }
    });
  });
}
```

## 数据库集成

### PostgreSQL

```sql
-- 创建调用lawkit的函数
CREATE OR REPLACE FUNCTION analyze_benford_law(table_name TEXT, column_name TEXT)
RETURNS JSON AS $$
DECLARE
    result JSON;
BEGIN
    -- 导出数据到CSV
    EXECUTE format('COPY (SELECT %I FROM %I) TO ''/tmp/data.csv'' CSV HEADER', column_name, table_name);
    
    -- 运行lawkit分析
    SELECT INTO result system('lawkit benf /tmp/data.csv --format json');
    
    RETURN result;
END;
$$ LANGUAGE plpgsql;
```

### MongoDB

```javascript
// 使用lawkit进行MongoDB聚合
const { MongoClient } = require('mongodb');
const fs = require('fs');

async function analyzeMongoData(collection, field) {
  const data = await collection.find({}).toArray();
  
  // 导出到CSV
  const csv = data.map(doc => doc[field]).join('\n');
  fs.writeFileSync('/tmp/mongo_data.csv', csv);
  
  // 使用lawkit分析
  const result = await analyzeBenford('/tmp/mongo_data.csv');
  return result;
}
```

## 商业智能工具

### Tableau

```python
# Tableau Python 集成
import pandas as pd
import subprocess
import json

def tableau_lawkit_analysis(data_source):
    # 从Tableau导出
    df = pd.read_csv(data_source)
    
    # 使用lawkit分析
    result = subprocess.run([
        'lawkit', 'analyze', data_source, 
        '--laws', 'all', '--format', 'json'
    ], capture_output=True, text=True)
    
    analysis = json.loads(result.stdout)
    
    # 转换回DataFrame供Tableau使用
    return pd.DataFrame(analysis['recommendations'])
```

### Power BI

```python
# Power BI Python 脚本
import os
import subprocess
import json

# 从Power BI获取数据
dataset = pd.DataFrame(...)

# 保存到临时文件
temp_file = '/tmp/powerbi_data.csv'
dataset.to_csv(temp_file, index=False)

# 运行lawkit分析
result = subprocess.run([
    'lawkit', 'benf', temp_file, '--format', 'json'
], capture_output=True, text=True)

# 解析Power BI的结果
analysis = json.loads(result.stdout)
risk_level = analysis['risk_level']
```

## 云平台

### AWS Lambda

```python
import json
import subprocess
import boto3

def lambda_handler(event, context):
    # 从S3下载数据
    s3 = boto3.client('s3')
    s3.download_file(
        event['bucket'], 
        event['key'], 
        '/tmp/data.csv'
    )
    
    # 运行lawkit分析
    result = subprocess.run([
        'lawkit', 'benf', '/tmp/data.csv', '--format', 'json'
    ], capture_output=True, text=True)
    
    # 将结果上传回S3
    analysis = json.loads(result.stdout)
    s3.put_object(
        Bucket=event['output_bucket'],
        Key=f"analysis/{event['key']}.json",
        Body=json.dumps(analysis)
    )
    
    return {
        'statusCode': 200,
        'body': json.dumps(analysis)
    }
```

### Google Cloud Functions

```python
def analyze_data(request):
    from google.cloud import storage
    import subprocess
    import json
    
    # 从Cloud Storage下载
    client = storage.Client()
    bucket = client.bucket('data-bucket')
    blob = bucket.blob('data.csv')
    blob.download_to_filename('/tmp/data.csv')
    
    # 运行分析
    result = subprocess.run([
        'lawkit', 'analyze', '/tmp/data.csv', 
        '--laws', 'all', '--format', 'json'
    ], capture_output=True, text=True)
    
    return json.loads(result.stdout)
```

## 监控和警报

### Prometheus 指标

```python
from prometheus_client import Gauge, Counter
import subprocess
import json

# 定义指标
risk_level_gauge = Gauge('lawkit_risk_level', '风险级别评分')
analysis_counter = Counter('lawkit_analyses_total', '执行的分析总数')

def update_metrics(data_file):
    result = subprocess.run([
        'lawkit', 'benf', data_file, '--format', 'json'
    ], capture_output=True, text=True)
    
    analysis = json.loads(result.stdout)
    risk_level_gauge.set(analysis['risk_level'])
    analysis_counter.inc()
```

### Grafana 仪表板

```json
{
  "dashboard": {
    "title": "lawkit数据质量仪表板",
    "panels": [
      {
        "title": "欺诈可能性",
        "type": "stat",
        "targets": [
          {
            "expr": "lawkit_risk_level",
            "legendFormat": "欺诈评分"
          }
        ]
      }
    ]
  }
}
```

## 自定义集成

### 构建您自己的集成

```rust
// 使用lawkit作为子进程的Rust集成
use std::process::Command;
use serde_json::Value;

fn custom_analysis(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let output = Command::new("lawkit")
        .arg("benf")
        .arg(file_path)
        .arg("--format")
        .arg("json")
        .output()?;
    
    let result: Value = serde_json::from_slice(&output.stdout)?;
    Ok(result)
}
```

这些集成示例帮助您将lawkit集成到现有的工作流程和工具中。