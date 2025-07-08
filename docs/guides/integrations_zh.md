# 集成指南

本指南介绍如何将lawkit集成到各种开发工作流、CI/CD管道和第三方工具中。

## CI/CD 集成

### GitHub Actions

创建 `.github/workflows/data-quality.yml`：

```yaml
name: Data Quality Check

on:
  push:
    paths:
      - 'data/**/*.csv'
      - 'reports/**/*.xlsx'
  pull_request:
    paths:
      - 'data/**/*.csv'

jobs:
  data-quality:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Install lawkit
      run: cargo install lawkit
      
    - name: Run data quality checks
      run: |
        echo "## Data Quality Report" > report.md
        for file in data/**/*.csv; do
          echo "### Analysis: $file" >> report.md
          lawkit benf "$file" --format json > temp.json
          risk=$(jq -r '.risk_level' temp.json)
          if [ "$risk" = "HIGH" ] || [ "$risk" = "CRITICAL" ]; then
            echo "❌ **$file**: High risk detected ($risk)" >> report.md
            echo "::error file=$file::High fraud risk detected"
            exit 1
          else
            echo "✅ **$file**: Normal ($risk)" >> report.md
          fi
        done
        
    - name: Comment PR
      uses: actions/github-script@v6
      if: github.event_name == 'pull_request'
      with:
        script: |
          const fs = require('fs');
          const report = fs.readFileSync('report.md', 'utf8');
          github.rest.issues.createComment({
            issue_number: context.issue.number,
            owner: context.repo.owner,
            repo: context.repo.repo,
            body: report
          });
```

### GitLab CI

创建 `.gitlab-ci.yml`：

```yaml
stages:
  - data-quality
  - deploy

data-quality-check:
  stage: data-quality
  image: rust:latest
  before_script:
    - cargo install lawkit
  script:
    - |
      for file in data/*.csv; do
        echo "Analyzing $file..."
        result=$(lawkit benf "$file" --format json)
        risk=$(echo "$result" | jq -r '.risk_level')
        
        if [ "$risk" = "HIGH" ] || [ "$risk" = "CRITICAL" ]; then
          echo "ERROR: High risk detected in $file"
          exit 1
        fi
      done
  artifacts:
    reports:
      junit: quality-report.xml
    expire_in: 1 week
  only:
    changes:
      - data/**/*.csv
      - reports/**/*.xlsx
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any
    
    stages {
        stage('Setup') {
            steps {
                sh 'cargo install lawkit'
            }
        }
        
        stage('Data Quality Check') {
            steps {
                script {
                    def dataFiles = sh(
                        script: 'find data/ -name "*.csv"',
                        returnStdout: true
                    ).trim().split('\n')
                    
                    def highRiskFiles = []
                    
                    dataFiles.each { file ->
                        def result = sh(
                            script: "lawkit benf '${file}' --format json",
                            returnStdout: true
                        )
                        
                        def jsonResult = readJSON text: result
                        
                        if (jsonResult.risk_level in ['HIGH', 'CRITICAL']) {
                            highRiskFiles.add(file)
                        }
                    }
                    
                    if (highRiskFiles.size() > 0) {
                        error("High risk detected in: ${highRiskFiles.join(', ')}")
                    }
                }
            }
        }
        
        stage('Generate Report') {
            steps {
                sh '''
                    echo "<h1>Data Quality Report</h1>" > quality-report.html
                    for file in data/*.csv; do
                        lawkit benf "$file" --format json | \
                        jq -r '"<p><strong>" + .dataset + "</strong>: " + .risk_level + "</p>"' >> quality-report.html
                    done
                '''
                publishHTML([
                    allowMissing: false,
                    alwaysLinkToLastBuild: true,
                    keepAll: true,
                    reportDir: '.',
                    reportFiles: 'quality-report.html',
                    reportName: 'Data Quality Report'
                ])
            }
        }
    }
    
    post {
        failure {
            emailext (
                subject: "Data Quality Check Failed: ${env.JOB_NAME} - ${env.BUILD_NUMBER}",
                body: "Data quality issues detected. Please check the build log.",
                to: "${env.CHANGE_AUTHOR_EMAIL}"
            )
        }
    }
}
```

## 数据库集成

### PostgreSQL

```sql
-- 创建存储过程调用lawkit
CREATE OR REPLACE FUNCTION check_data_quality(table_name TEXT, column_name TEXT)
RETURNS JSON AS $$
DECLARE
    result JSON;
    temp_file TEXT;
BEGIN
    -- 导出数据到临时文件
    temp_file := '/tmp/data_export_' || extract(epoch from now()) || '.csv';
    
    EXECUTE format('COPY (SELECT %I FROM %I) TO %L WITH CSV HEADER', 
                   column_name, table_name, temp_file);
    
    -- 调用lawkit分析
    SELECT INTO result (
        SELECT row_to_json(t) FROM (
            SELECT * FROM (
                SELECT 'lawkit benf ' || temp_file || ' --format json'
            ) AS cmd
        ) AS t
    );
    
    -- 清理临时文件
    PERFORM pg_file_unlink(temp_file);
    
    RETURN result;
END;
$$ LANGUAGE plpgsql;

-- 使用示例
SELECT check_data_quality('transactions', 'amount');
```

### MySQL

```sql
-- MySQL存储过程
DELIMITER //

CREATE PROCEDURE CheckDataQuality(
    IN table_name VARCHAR(255),
    IN column_name VARCHAR(255),
    OUT risk_level VARCHAR(50)
)
BEGIN
    DECLARE temp_file VARCHAR(500);
    DECLARE cmd VARCHAR(1000);
    
    SET temp_file = CONCAT('/tmp/mysql_export_', UNIX_TIMESTAMP(), '.csv');
    
    -- 导出数据
    SET @sql = CONCAT('SELECT ', column_name, ' FROM ', table_name, 
                     ' INTO OUTFILE "', temp_file, '"');
    PREPARE stmt FROM @sql;
    EXECUTE stmt;
    DEALLOCATE PREPARE stmt;
    
    -- 调用lawkit（需要系统支持）
    SET cmd = CONCAT('lawkit benf ', temp_file, ' --format json');
    -- 这里需要使用UDF或外部脚本来执行
    
END //

DELIMITER ;
```

### MongoDB

```javascript
// MongoDB集成脚本
function analyzeCollection(collectionName, fieldName) {
    const fs = require('fs');
    const { exec } = require('child_process');
    const { promisify } = require('util');
    const execAsync = promisify(exec);
    
    // 导出数据
    const tempFile = `/tmp/mongo_export_${Date.now()}.csv`;
    
    db[collectionName].find({}, {[fieldName]: 1}).forEach(
        function(doc) {
            fs.appendFileSync(tempFile, doc[fieldName] + '\n');
        }
    );
    
    // 分析数据
    return execAsync(`lawkit benf ${tempFile} --format json`)
        .then(result => {
            fs.unlinkSync(tempFile); // 清理
            return JSON.parse(result.stdout);
        });
}

// 使用示例
analyzeCollection('transactions', 'amount')
    .then(result => {
        print(`Risk Level: ${result.risk_level}`);
        print(`Chi-square: ${result.statistics.chi_square}`);
    });
```

## 云平台集成

### AWS Lambda

```python
import json
import subprocess
import tempfile
import boto3

def lambda_handler(event, context):
    """AWS Lambda函数分析S3中的数据"""
    
    s3 = boto3.client('s3')
    bucket = event['bucket']
    key = event['key']
    
    # 下载文件到临时目录
    with tempfile.NamedTemporaryFile(suffix='.csv') as temp_file:
        s3.download_file(bucket, key, temp_file.name)
        
        # 运行lawkit分析
        result = subprocess.run([
            'lawkit', 'benf', temp_file.name, '--format', 'json'
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            return {
                'statusCode': 500,
                'body': json.dumps({'error': result.stderr})
            }
        
        analysis = json.loads(result.stdout)
        
        # 如果检测到高风险，发送SNS警报
        if analysis['risk_level'] in ['HIGH', 'CRITICAL']:
            sns = boto3.client('sns')
            sns.publish(
                TopicArn='arn:aws:sns:region:account:fraud-alerts',
                Subject=f'Data Quality Alert: {key}',
                Message=f'High risk detected in {key}: {analysis["risk_level"]}'
            )
        
        return {
            'statusCode': 200,
            'body': json.dumps(analysis)
        }
```

### Google Cloud Functions

```python
import functions_framework
import subprocess
import tempfile
import json
from google.cloud import storage

@functions_framework.http
def analyze_data(request):
    """Google Cloud Function分析数据"""
    
    request_json = request.get_json()
    bucket_name = request_json['bucket']
    file_name = request_json['file']
    
    # 下载文件
    client = storage.Client()
    bucket = client.bucket(bucket_name)
    blob = bucket.blob(file_name)
    
    with tempfile.NamedTemporaryFile(suffix='.csv') as temp_file:
        blob.download_to_filename(temp_file.name)
        
        # 运行分析
        result = subprocess.run([
            'lawkit', 'analyze', temp_file.name, 
            '--laws', 'all', '--format', 'json'
        ], capture_output=True, text=True)
        
        if result.returncode == 0:
            return json.loads(result.stdout)
        else:
            return {'error': result.stderr}, 500
```

### Azure Functions

```csharp
using System;
using System.IO;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Azure.WebJobs;
using Microsoft.Azure.WebJobs.Extensions.Http;
using Microsoft.AspNetCore.Http;
using System.Diagnostics;
using Newtonsoft.Json;

public static class DataAnalysisFunction
{
    [FunctionName("AnalyzeData")]
    public static async Task<IActionResult> Run(
        [HttpTrigger(AuthorizationLevel.Function, "post", Route = null)] HttpRequest req)
    {
        string requestBody = await new StreamReader(req.Body).ReadToEndAsync();
        dynamic data = JsonConvert.DeserializeObject(requestBody);
        
        string filePath = data?.filePath;
        
        if (string.IsNullOrEmpty(filePath))
        {
            return new BadRequestObjectResult("filePath is required");
        }
        
        try
        {
            var process = new Process()
            {
                StartInfo = new ProcessStartInfo
                {
                    FileName = "lawkit",
                    Arguments = $"benf {filePath} --format json",
                    RedirectStandardOutput = true,
                    UseShellExecute = false,
                    CreateNoWindow = true,
                }
            };
            
            process.Start();
            string result = await process.StandardOutput.ReadToEndAsync();
            process.WaitForExit();
            
            if (process.ExitCode == 0)
            {
                return new OkObjectResult(JsonConvert.DeserializeObject(result));
            }
            else
            {
                return new StatusCodeResult(500);
            }
        }
        catch (Exception ex)
        {
            return new StatusCodeResult(500);
        }
    }
}
```

## 编程语言集成

### Python

```python
import subprocess
import json
import tempfile
import pandas as pd

class LawkitAnalyzer:
    def __init__(self):
        self.lawkit_path = 'lawkit'
    
    def analyze_dataframe(self, df, column=None, law='benf', **kwargs):
        """分析pandas DataFrame"""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.csv', delete=False) as f:
            if column:
                df[column].to_csv(f.name, index=False, header=False)
            else:
                df.to_csv(f.name, index=False)
            
            return self.analyze_file(f.name, law, **kwargs)
    
    def analyze_file(self, filepath, law='benf', format='json', **kwargs):
        """分析文件"""
        cmd = [self.lawkit_path, law, filepath, '--format', format]
        
        # 添加额外参数
        for key, value in kwargs.items():
            if isinstance(value, bool) and value:
                cmd.append(f'--{key.replace("_", "-")}')
            elif not isinstance(value, bool):
                cmd.extend([f'--{key.replace("_", "-")}', str(value)])
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise Exception(f"Lawkit error: {result.stderr}")
        
        if format == 'json':
            return json.loads(result.stdout)
        else:
            return result.stdout
    
    def batch_analyze(self, files, law='benf', **kwargs):
        """批量分析多个文件"""
        results = {}
        for file in files:
            try:
                results[file] = self.analyze_file(file, law, **kwargs)
            except Exception as e:
                results[file] = {'error': str(e)}
        return results

# 使用示例
analyzer = LawkitAnalyzer()

# 分析DataFrame
df = pd.read_csv('sales_data.csv')
result = analyzer.analyze_dataframe(df, 'amount', law='benf', verbose=True)
print(f"Risk Level: {result['risk_level']}")

# 分析文件
result = analyzer.analyze_file('financial.csv', law='analyze', 
                             laws='all', recommend=True)

# 批量分析
files = ['q1.csv', 'q2.csv', 'q3.csv', 'q4.csv']
results = analyzer.batch_analyze(files, law='pareto', 
                                business_analysis=True)
```

### R

```r
# R集成包装器
library(jsonlite)
library(processx)

LawkitAnalyzer <- R6::R6Class("LawkitAnalyzer",
  public = list(
    lawkit_path = "lawkit",
    
    initialize = function(lawkit_path = "lawkit") {
      self$lawkit_path <- lawkit_path
    },
    
    analyze_vector = function(data, law = "benf", ...) {
      # 创建临时文件
      temp_file <- tempfile(fileext = ".csv")
      write.csv(data, temp_file, row.names = FALSE, col.names = FALSE)
      
      result <- self$analyze_file(temp_file, law, ...)
      unlink(temp_file)
      return(result)
    },
    
    analyze_file = function(filepath, law = "benf", format = "json", ...) {
      args <- c(law, filepath, "--format", format)
      
      # 添加额外参数
      dots <- list(...)
      for (name in names(dots)) {
        value <- dots[[name]]
        arg_name <- paste0("--", gsub("_", "-", name))
        
        if (is.logical(value) && value) {
          args <- c(args, arg_name)
        } else if (!is.logical(value)) {
          args <- c(args, arg_name, as.character(value))
        }
      }
      
      result <- processx::run(self$lawkit_path, args, echo_cmd = FALSE)
      
      if (result$status != 0) {
        stop("Lawkit error: ", result$stderr)
      }
      
      if (format == "json") {
        return(fromJSON(result$stdout))
      } else {
        return(result$stdout)
      }
    },
    
    analyze_laws = function(data, laws = "all", ...) {
      if (is.character(data)) {
        # 文件路径
        return(self$analyze_file(data, "analyze", laws = laws, ...))
      } else {
        # 数据向量
        return(self$analyze_vector(data, "analyze", laws = laws, ...))
      }
    }
  )
)

# 使用示例
analyzer <- LawkitAnalyzer$new()

# 分析向量
sales_data <- c(1234, 5678, 9012, 3456, 7890)
result <- analyzer$analyze_vector(sales_data, law = "benf", verbose = TRUE)
cat("Risk Level:", result$risk_level, "\n")

# 分析文件
result <- analyzer$analyze_file("financial.csv", law = "pareto", 
                               gini_coefficient = TRUE)

# 比较多种法则
comparison <- analyzer$analyze_laws("data.csv", laws = "benf,pareto,normal",
                                   recommend = TRUE)
```

### JavaScript/Node.js

```javascript
const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

class LawkitAnalyzer {
    constructor(lawkitPath = 'lawkit') {
        this.lawkitPath = lawkitPath;
    }
    
    async analyzeFile(filepath, law = 'benf', options = {}) {
        return new Promise((resolve, reject) => {
            const args = [law, filepath];
            
            // 添加格式选项
            if (options.format) {
                args.push('--format', options.format);
            } else {
                args.push('--format', 'json');
            }
            
            // 添加其他选项
            Object.entries(options).forEach(([key, value]) => {
                if (key !== 'format') {
                    const argName = `--${key.replace(/_/g, '-')}`;
                    if (typeof value === 'boolean' && value) {
                        args.push(argName);
                    } else if (typeof value !== 'boolean') {
                        args.push(argName, String(value));
                    }
                }
            });
            
            const process = spawn(this.lawkitPath, args);
            let stdout = '';
            let stderr = '';
            
            process.stdout.on('data', (data) => {
                stdout += data.toString();
            });
            
            process.stderr.on('data', (data) => {
                stderr += data.toString();
            });
            
            process.on('close', (code) => {
                if (code === 0) {
                    try {
                        if (options.format === 'json' || !options.format) {
                            resolve(JSON.parse(stdout));
                        } else {
                            resolve(stdout);
                        }
                    } catch (e) {
                        reject(new Error(`Failed to parse output: ${e.message}`));
                    }
                } else {
                    reject(new Error(`Lawkit process failed: ${stderr}`));
                }
            });
        });
    }
    
    async analyzeArray(data, law = 'benf', options = {}) {
        const tempFile = path.join(__dirname, `temp_${Date.now()}.csv`);
        
        try {
            // 写入临时文件
            fs.writeFileSync(tempFile, data.join('\n'));
            
            const result = await this.analyzeFile(tempFile, law, options);
            return result;
        } finally {
            // 清理临时文件
            if (fs.existsSync(tempFile)) {
                fs.unlinkSync(tempFile);
            }
        }
    }
    
    async batchAnalyze(files, law = 'benf', options = {}) {
        const results = {};
        
        for (const file of files) {
            try {
                results[file] = await this.analyzeFile(file, law, options);
            } catch (error) {
                results[file] = { error: error.message };
            }
        }
        
        return results;
    }
}

// 使用示例
(async () => {
    const analyzer = new LawkitAnalyzer();
    
    try {
        // 分析文件
        const result = await analyzer.analyzeFile('sales.csv', 'benf', {
            verbose: true,
            threshold: 'high'
        });
        console.log('Risk Level:', result.risk_level);
        
        // 分析数组数据
        const data = [1234, 5678, 9012, 3456, 7890];
        const arrayResult = await analyzer.analyzeArray(data, 'pareto', {
            gini_coefficient: true
        });
        console.log('Concentration:', arrayResult.concentration_80);
        
        // 批量分析
        const files = ['q1.csv', 'q2.csv', 'q3.csv'];
        const batchResults = await analyzer.batchAnalyze(files, 'analyze', {
            laws: 'all',
            recommend: true
        });
        console.log('Batch Results:', batchResults);
        
    } catch (error) {
        console.error('Analysis failed:', error.message);
    }
})();

module.exports = LawkitAnalyzer;
```

## 监控和警报系统

### Prometheus集成

```bash
#!/bin/bash
# lawkit_exporter.sh - Prometheus导出器

while true; do
    for file in /data/*.csv; do
        result=$(lawkit benf "$file" --format json 2>/dev/null)
        if [ $? -eq 0 ]; then
            dataset=$(echo "$result" | jq -r '.dataset')
            risk_level=$(echo "$result" | jq -r '.risk_level')
            chi_square=$(echo "$result" | jq -r '.statistics.chi_square')
            p_value=$(echo "$result" | jq -r '.statistics.p_value')
            
            # 将风险等级转换为数值
            case $risk_level in
                "LOW") risk_numeric=1 ;;
                "MEDIUM") risk_numeric=2 ;;
                "HIGH") risk_numeric=3 ;;
                "CRITICAL") risk_numeric=4 ;;
                *) risk_numeric=0 ;;
            esac
            
            # 输出Prometheus指标
            echo "# HELP lawkit_risk_level Risk level from Benford analysis"
            echo "# TYPE lawkit_risk_level gauge"
            echo "lawkit_risk_level{dataset=\"$dataset\"} $risk_numeric"
            
            echo "# HELP lawkit_chi_square Chi-square statistic"
            echo "# TYPE lawkit_chi_square gauge"
            echo "lawkit_chi_square{dataset=\"$dataset\"} $chi_square"
            
            echo "# HELP lawkit_p_value P-value from statistical test"
            echo "# TYPE lawkit_p_value gauge"
            echo "lawkit_p_value{dataset=\"$dataset\"} $p_value"
        fi
    done
    
    sleep 60
done
```

### Grafana仪表板

```json
{
  "dashboard": {
    "title": "Lawkit Data Quality Dashboard",
    "panels": [
      {
        "title": "Risk Level by Dataset",
        "type": "stat",
        "targets": [
          {
            "expr": "lawkit_risk_level",
            "legendFormat": "{{dataset}}"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "thresholds": {
              "steps": [
                {"color": "green", "value": 0},
                {"color": "yellow", "value": 2},
                {"color": "red", "value": 3}
              ]
            }
          }
        }
      },
      {
        "title": "Chi-square Values Over Time",
        "type": "timeseries",
        "targets": [
          {
            "expr": "lawkit_chi_square",
            "legendFormat": "{{dataset}}"
          }
        ]
      }
    ]
  }
}
```

## 下一步

- 查看[性能优化指南](performance_zh.md)了解大规模部署优化
- 参考[高级分析指南](advanced-analysis_zh.md)了解复杂分析场景
- 阅读[配置指南](../user-guide/configuration_zh.md)了解详细配置选项