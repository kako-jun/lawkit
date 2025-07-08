# 統合機能

lawkitを他のツールやワークフローと統合する方法を学びます。

## CI/CD統合

### GitHub Actions

```yaml
name: データ品質チェック

on:
  push:
    paths:
      - 'data/**/*.csv'

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: lawkitインストール
        run: cargo install lawkit
        
      - name: 品質分析
        run: |
          for file in data/*.csv; do
            lawkit analyze "$file" --laws benford,normal --detect-conflicts --output json > "qa_$(basename "$file" .csv).json"
          done
          
      - name: 結果アップロード
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
    - lawkit analyze data/financial.csv --laws all --output json > quality-report.json
  artifacts:
    reports:
      - quality-report.json
```

## API統合

### REST APIラッパー

```python
# Python ラッパー例
import subprocess
import json

class LawkitAPI:
    def __init__(self):
        self.binary = 'lawkit'
    
    def analyze_benford(self, data_file, output_format='json'):
        result = subprocess.run([
            self.binary, 'benf', data_file, 
            '--output', output_format
        ], capture_output=True, text=True)
        
        if output_format == 'json':
            return json.loads(result.stdout)
        return result.stdout
```

### Node.js統合

```javascript
const { spawn } = require('child_process');

function analyzeBenford(dataFile) {
  return new Promise((resolve, reject) => {
    const lawkit = spawn('lawkit', ['benf', dataFile, '--output', 'json']);
    
    let output = '';
    lawkit.stdout.on('data', (data) => {
      output += data;
    });
    
    lawkit.on('close', (code) => {
      if (code === 0) {
        resolve(JSON.parse(output));
      } else {
        reject(new Error(`Process exited with code ${code}`));
      }
    });
  });
}
```

## データベース統合

### PostgreSQL

```sql
-- lawkit呼び出し関数の作成
CREATE OR REPLACE FUNCTION analyze_benford_law(table_name TEXT, column_name TEXT)
RETURNS JSON AS $$
DECLARE
    result JSON;
BEGIN
    -- データをCSVにエクスポート
    EXECUTE format('COPY (SELECT %I FROM %I) TO ''/tmp/data.csv'' CSV HEADER', column_name, table_name);
    
    -- lawkit分析実行
    SELECT INTO result system('lawkit benf /tmp/data.csv --output json');
    
    RETURN result;
END;
$$ LANGUAGE plpgsql;
```

### MongoDB

```javascript
// MongoDB集計とlawkit
const { MongoClient } = require('mongodb');
const fs = require('fs');

async function analyzeMongoData(collection, field) {
  const data = await collection.find({}).toArray();
  
  // CSVにエクスポート
  const csv = data.map(doc => doc[field]).join('\n');
  fs.writeFileSync('/tmp/mongo_data.csv', csv);
  
  // lawkitで分析
  const result = await analyzeBenford('/tmp/mongo_data.csv');
  return result;
}
```

## ビジネスインテリジェンスツール

### Tableau

```python
# Tableau Python統合
import pandas as pd
import subprocess
import json

def tableau_lawkit_analysis(data_source):
    # Tableauからエクスポート
    df = pd.read_csv(data_source)
    
    # lawkitで分析
    result = subprocess.run([
        'lawkit', 'compare', data_source, 
        '--laws', 'all', '--output', 'json'
    ], capture_output=True, text=True)
    
    analysis = json.loads(result.stdout)
    
    # TableauのDataFrameに変換
    return pd.DataFrame(analysis['recommendations'])
```

### Power BI

```python
# Power BI Pythonスクリプト
import os
import subprocess
import json

# Power BIからデータ取得
dataset = pd.DataFrame(...)

# 一時ファイルに保存
temp_file = '/tmp/powerbi_data.csv'
dataset.to_csv(temp_file, index=False)

# lawkit分析実行
result = subprocess.run([
    'lawkit', 'benf', temp_file, '--output', 'json'
], capture_output=True, text=True)

# Power BI用結果解析
analysis = json.loads(result.stdout)
fraud_score = analysis['fraud_likelihood']
```

## クラウドプラットフォーム

### AWS Lambda

```python
import json
import subprocess
import boto3

def lambda_handler(event, context):
    # S3からデータダウンロード
    s3 = boto3.client('s3')
    s3.download_file(
        event['bucket'], 
        event['key'], 
        '/tmp/data.csv'
    )
    
    # lawkit分析実行
    result = subprocess.run([
        'lawkit', 'benf', '/tmp/data.csv', '--output', 'json'
    ], capture_output=True, text=True)
    
    # 結果をS3にアップロード
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
    
    # Cloud Storageからダウンロード
    client = storage.Client()
    bucket = client.bucket('data-bucket')
    blob = bucket.blob('data.csv')
    blob.download_to_filename('/tmp/data.csv')
    
    # 分析実行
    result = subprocess.run([
        'lawkit', 'compare', '/tmp/data.csv', 
        '--laws', 'all', '--output', 'json'
    ], capture_output=True, text=True)
    
    return json.loads(result.stdout)
```

## 監視・アラート

### Prometheusメトリクス

```python
from prometheus_client import Gauge, Counter
import subprocess
import json

# メトリクス定義
fraud_score_gauge = Gauge('lawkit_fraud_score', '不正可能性スコア')
analysis_counter = Counter('lawkit_analyses_total', '実行した分析の総数')

def update_metrics(data_file):
    result = subprocess.run([
        'lawkit', 'benf', data_file, '--output', 'json'
    ], capture_output=True, text=True)
    
    analysis = json.loads(result.stdout)
    fraud_score_gauge.set(analysis['fraud_likelihood'])
    analysis_counter.inc()
```

### Grafanaダッシュボード

```json
{
  "dashboard": {
    "title": "Lawkit データ品質ダッシュボード",
    "panels": [
      {
        "title": "不正可能性",
        "type": "stat",
        "targets": [
          {
            "expr": "lawkit_fraud_score",
            "legendFormat": "不正スコア"
          }
        ]
      }
    ]
  }
}
```

## カスタム統合

### 独自実装

```rust
// lawkit-coreを使用したRust統合
use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::common::input::parser::parse_csv;

fn custom_analysis(file_path: &str) -> Result<BenfordAnalysis, Box<dyn std::error::Error>> {
    let numbers = parse_csv(file_path)?;
    let analysis = BenfordAnalysis::analyze(&numbers)?;
    Ok(analysis)
}
```

これらの統合例は、lawkitを既存のワークフローやツールに組み込むのに役立ちます。