# Integrations

Learn how to integrate lawkit with other tools and workflows.

## CI/CD Integration

### GitHub Actions

```yaml
name: Data Quality Check

on:
  push:
    paths:
      - 'data/**/*.csv'

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install lawkit
        run: cargo install lawkit
        
      - name: Quality Analysis
        run: |
          for file in data/*.csv; do
            lawkit compare "$file" --laws benford,normal --detect-conflicts --output json > "qa_$(basename "$file" .csv).json"
          done
          
      - name: Upload Results
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
    - lawkit compare data/financial.csv --laws all --output json > quality-report.json
  artifacts:
    reports:
      - quality-report.json
```

## API Integration

### REST API Wrapper

```python
# Example Python wrapper
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

### Node.js Integration

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

## Database Integration

### PostgreSQL

```sql
-- Create function to call lawkit
CREATE OR REPLACE FUNCTION analyze_benford_law(table_name TEXT, column_name TEXT)
RETURNS JSON AS $$
DECLARE
    result JSON;
BEGIN
    -- Export data to CSV
    EXECUTE format('COPY (SELECT %I FROM %I) TO ''/tmp/data.csv'' CSV HEADER', column_name, table_name);
    
    -- Run lawkit analysis
    SELECT INTO result system('lawkit benf /tmp/data.csv --output json');
    
    RETURN result;
END;
$$ LANGUAGE plpgsql;
```

### MongoDB

```javascript
// MongoDB aggregation with lawkit
const { MongoClient } = require('mongodb');
const fs = require('fs');

async function analyzeMongoData(collection, field) {
  const data = await collection.find({}).toArray();
  
  // Export to CSV
  const csv = data.map(doc => doc[field]).join('\n');
  fs.writeFileSync('/tmp/mongo_data.csv', csv);
  
  // Analyze with lawkit
  const result = await analyzeBenford('/tmp/mongo_data.csv');
  return result;
}
```

## Business Intelligence Tools

### Tableau

```python
# Tableau Python integration
import pandas as pd
import subprocess
import json

def tableau_lawkit_analysis(data_source):
    # Export from Tableau
    df = pd.read_csv(data_source)
    
    # Analyze with lawkit
    result = subprocess.run([
        'lawkit', 'compare', data_source, 
        '--laws', 'all', '--output', 'json'
    ], capture_output=True, text=True)
    
    analysis = json.loads(result.stdout)
    
    # Convert back to DataFrame for Tableau
    return pd.DataFrame(analysis['recommendations'])
```

### Power BI

```python
# Power BI Python script
import os
import subprocess
import json

# Get data from Power BI
dataset = pd.DataFrame(...)

# Save to temp file
temp_file = '/tmp/powerbi_data.csv'
dataset.to_csv(temp_file, index=False)

# Run lawkit analysis
result = subprocess.run([
    'lawkit', 'benf', temp_file, '--output', 'json'
], capture_output=True, text=True)

# Parse results for Power BI
analysis = json.loads(result.stdout)
fraud_score = analysis['fraud_likelihood']
```

## Cloud Platforms

### AWS Lambda

```python
import json
import subprocess
import boto3

def lambda_handler(event, context):
    # Download data from S3
    s3 = boto3.client('s3')
    s3.download_file(
        event['bucket'], 
        event['key'], 
        '/tmp/data.csv'
    )
    
    # Run lawkit analysis
    result = subprocess.run([
        'lawkit', 'benf', '/tmp/data.csv', '--output', 'json'
    ], capture_output=True, text=True)
    
    # Upload results back to S3
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
    
    # Download from Cloud Storage
    client = storage.Client()
    bucket = client.bucket('data-bucket')
    blob = bucket.blob('data.csv')
    blob.download_to_filename('/tmp/data.csv')
    
    # Run analysis
    result = subprocess.run([
        'lawkit', 'compare', '/tmp/data.csv', 
        '--laws', 'all', '--output', 'json'
    ], capture_output=True, text=True)
    
    return json.loads(result.stdout)
```

## Monitoring and Alerting

### Prometheus Metrics

```python
from prometheus_client import Gauge, Counter
import subprocess
import json

# Define metrics
fraud_score_gauge = Gauge('lawkit_fraud_score', 'Fraud likelihood score')
analysis_counter = Counter('lawkit_analyses_total', 'Total analyses performed')

def update_metrics(data_file):
    result = subprocess.run([
        'lawkit', 'benf', data_file, '--output', 'json'
    ], capture_output=True, text=True)
    
    analysis = json.loads(result.stdout)
    fraud_score_gauge.set(analysis['fraud_likelihood'])
    analysis_counter.inc()
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "Lawkit Data Quality Dashboard",
    "panels": [
      {
        "title": "Fraud Likelihood",
        "type": "stat",
        "targets": [
          {
            "expr": "lawkit_fraud_score",
            "legendFormat": "Fraud Score"
          }
        ]
      }
    ]
  }
}
```

## Custom Integrations

### Build Your Own

```rust
// Rust integration using lawkit-core
use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::common::input::parser::parse_csv;

fn custom_analysis(file_path: &str) -> Result<BenfordAnalysis, Box<dyn std::error::Error>> {
    let numbers = parse_csv(file_path)?;
    let analysis = BenfordAnalysis::analyze(&numbers)?;
    Ok(analysis)
}
```

These integration examples help you incorporate lawkit into your existing workflows and tools.