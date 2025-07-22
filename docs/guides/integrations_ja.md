# インテグレーション

lawkitを他のツールやワークフローと統合する方法を学びます。

## CI/CD インテグレーション

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
      
      - name: lawkitをインストール
        run: cargo install lawkit
        
      - name: 品質分析
        run: |
          for file in data/*.csv; do
            lawkit analyze "$file" --laws benford,normal --format json > "qa_$(basename "$file" .csv).json"
          done
          
      - name: 結果をアップロード
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

## API インテグレーション

### REST API ラッパー

```python
# Python ラッパーの例
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

### Node.js インテグレーション

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
        reject(new Error(`プロセスが終了コード ${code} で終了しました`));
      }
    });
  });
}
```

## データベースインテグレーション

### PostgreSQL

```sql
-- lawkitを呼び出す関数を作成
CREATE OR REPLACE FUNCTION analyze_benford_law(table_name TEXT, column_name TEXT)
RETURNS JSON AS $$
DECLARE
    result JSON;
BEGIN
    -- データをCSVにエクスポート
    EXECUTE format('COPY (SELECT %I FROM %I) TO ''/tmp/data.csv'' CSV HEADER', column_name, table_name);
    
    -- lawkit分析を実行
    SELECT INTO result system('lawkit benf /tmp/data.csv --format json');
    
    RETURN result;
END;
$$ LANGUAGE plpgsql;
```

### MongoDB

```javascript
// lawkitを使用したMongoDBの集計
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
# Tableau Python インテグレーション
import pandas as pd
import subprocess
import json

def tableau_lawkit_analysis(data_source):
    # Tableauからエクスポート
    df = pd.read_csv(data_source)
    
    # lawkitで分析
    result = subprocess.run([
        'lawkit', 'analyze', data_source, 
        '--laws', 'all', '--format', 'json'
    ], capture_output=True, text=True)
    
    analysis = json.loads(result.stdout)
    
    # Tableau用にDataFrameに変換
    return pd.DataFrame(analysis['recommendations'])
```

### Power BI

```python
# Power BI Python スクリプト
import os
import subprocess
import json

# Power BIからデータを取得
dataset = pd.DataFrame(...)

# 一時ファイルに保存
temp_file = '/tmp/powerbi_data.csv'
dataset.to_csv(temp_file, index=False)

# lawkit分析を実行
result = subprocess.run([
    'lawkit', 'benf', temp_file, '--format', 'json'
], capture_output=True, text=True)

# Power BI用に結果を解析
analysis = json.loads(result.stdout)
risk_level = analysis['risk_level']
```

## クラウドプラットフォーム

### AWS Lambda

```python
import json
import subprocess
import boto3

def lambda_handler(event, context):
    # S3からデータをダウンロード
    s3 = boto3.client('s3')
    s3.download_file(
        event['bucket'], 
        event['key'], 
        '/tmp/data.csv'
    )
    
    # lawkit分析を実行
    result = subprocess.run([
        'lawkit', 'benf', '/tmp/data.csv', '--format', 'json'
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
    
    # 分析を実行
    result = subprocess.run([
        'lawkit', 'analyze', '/tmp/data.csv', 
        '--laws', 'all', '--format', 'json'
    ], capture_output=True, text=True)
    
    return json.loads(result.stdout)
```

## 監視とアラート

### Prometheus メトリクス

```python
from prometheus_client import Gauge, Counter
import subprocess
import json

# メトリクスを定義
risk_level_gauge = Gauge('lawkit_risk_level', 'リスクレベルスコア')
analysis_counter = Counter('lawkit_analyses_total', '実行された分析の総数')

def update_metrics(data_file):
    result = subprocess.run([
        'lawkit', 'benf', data_file, '--format', 'json'
    ], capture_output=True, text=True)
    
    analysis = json.loads(result.stdout)
    risk_level_gauge.set(analysis['risk_level'])
    analysis_counter.inc()
```

### Grafanaダッシュボード

```json
{
  "dashboard": {
    "title": "lawkitデータ品質ダッシュボード",
    "panels": [
      {
        "title": "不正の可能性",
        "type": "stat",
        "targets": [
          {
            "expr": "lawkit_risk_level",
            "legendFormat": "不正スコア"
          }
        ]
      }
    ]
  }
}
```

## カスタムインテグレーション

### 独自の構築

```rust
// サブプロセスとしてlawkitを使用するRustインテグレーション
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

これらのインテグレーション例は、既存のワークフローやツールにlawkitを組み込むのに役立ちます。