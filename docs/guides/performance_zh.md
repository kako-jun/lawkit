# 性能优化指南

本指南介绍如何优化lawkit的性能，处理大规模数据集，并在生产环境中高效运行。

## 性能基准

### 基线性能

在标准硬件（Intel i7-8750H, 16GB RAM）上的性能基准：

| 数据量 | 单法则分析 | 多法则比较 | 内存使用 |
|--------|------------|------------|----------|
| 1K 数据点 | 8ms | 25ms | 2.1MB |
| 10K 数据点 | 45ms | 120ms | 8.3MB |
| 100K 数据点 | 180ms | 850ms | 45MB |
| 1M 数据点 | 2.1s | 8.5s | 180MB |
| 10M 数据点 | 25s | 95s | 1.2GB |

### 优化后性能

启用所有优化选项后的性能提升：

```bash
# 标准模式
time lawkit benf large_dataset.csv
# 结果: 25.3s

# 优化模式
time lawkit benf large_dataset.csv --optimize
# 结果: 8.7s (2.9x 提升)
```

## 并行处理优化

### 自动并行化

```bash
# 自动检测CPU核心数
lawkit compare data.csv --optimize

# 性能优化模式
lawkit compare data.csv --optimize

# 智能优化模式
lawkit compare data.csv --optimize
```

### 批量并行处理

```bash
# 批量处理
find ./data -name "*.csv" | xargs -P 8 -I {} lawkit benf {} --format json

# 高性能批量处理
find ./data -name "*.csv" | \
xargs -P 8 -I {} sh -c 'lawkit benf "{}" --optimize --format json > "${1%.*}.result"' -- {}

# 内存友好的批量处理
find ./data -name "*.csv" | \
xargs -P 4 -I {} lawkit benf {} --optimize --format json
```

### 分布式处理

```bash
# 跨服务器分布式处理
for server in $(cat servers.txt); do
  scp file.csv $server:/tmp/
  ssh $server 'lawkit benf /tmp/file.csv --optimize --format json'
done

# SSH集群处理
for server in server1 server2 server3; do
  ssh $server 'lawkit benf large_files/*.csv --optimize --format json'
done
```

## 内存优化

### 流式处理

```bash
# 启用流式处理
lawkit benf huge_file.csv --optimize

# 优化大文件处理
lawkit benf huge_file.csv --optimize

# 内存优化
lawkit benf huge_file.csv --optimize
```

### 内存使用监控

```bash
# 内存使用分析
/usr/bin/time -v lawkit benf large_file.csv --memory-monitor

# 实时内存监控
while true; do
    ps aux | grep lawkit | grep -v grep
    sleep 1
done

# 详细内存分析
valgrind --tool=massif lawkit benf test_data.csv
```

### 内存优化配置

```yaml
# ~/.config/lawkit/config.yaml
memory:
  chunk_size: 100000
  stream_threshold: 10000000  # 10M records
  gc_frequency: 1000
  memory_limit: "2GB"

streaming:
  enabled: true
  buffer_size: 8192
  compression: true
```

## 磁盘I/O优化

### 输入优化

```bash
# 预处理大文件
# 移除空行和无效数据
grep -v '^$' large_file.csv | grep -E '[0-9]' > cleaned_file.csv
lawkit benf cleaned_file.csv

# 使用压缩格式
gzip -cd compressed_data.csv.gz | lawkit benf --format json

# SSD优化
# 将临时文件放在SSD上
TMPDIR=/ssd/tmp lawkit benf huge_file.csv --optimize
```

### 输出优化

```bash
# 二进制输出格式
lawkit benf data.csv --format binary > results.bin

# 压缩输出
lawkit benf data.csv --format json | gzip > results.json.gz

# 直接写入数据库
lawkit benf data.csv --format json | \
jq -r '.[]' | psql -d analytics -c "COPY results FROM STDIN"
```

## 算法优化

### 采样策略

```bash
# 大数据集采样分析
head -100000 huge_dataset.csv | lawkit benf --format json

# 随机采样
shuf -n 50000 large_file.csv | lawkit benf --format json

# 分层采样
awk 'NR%1000==1' massive_file.csv | lawkit benf --format json
```

### 预计算优化

```bash
# 预计算统计信息
lawkit benf data.csv --precompute-stats --cache-results

# 增量分析
lawkit benf new_data.csv --incremental --base-cache previous_results.cache
```

### 算法选择

```bash
# 快速估算模式
lawkit benf data.csv --fast-estimate

# 精确分析模式
lawkit benf data.csv --precise --all-tests

# 自适应分析
lawkit benf data.csv --adaptive --quality-threshold 0.95
```

## 缓存策略

### 结果缓存

```bash
# 启用结果缓存
export LAWKIT_CACHE_ENABLED=1
export LAWKIT_CACHE_DIR=~/.cache/lawkit

# 缓存配置
lawkit benf data.csv --cache-results --cache-ttl 3600

# 缓存清理
lawkit cache clean --older-than 7d
lawkit cache stats
```

### 智能缓存

```yaml
cache:
  enabled: true
  directory: "~/.cache/lawkit"
  max_size: "1GB"
  ttl: 3600
  compression: true
  
  rules:
    - pattern: "*.csv"
      ttl: 1800
    - pattern: "daily_*"
      ttl: 86400
    - pattern: "archive_*"
      ttl: 604800
```

## 网络优化

### 远程文件处理

```bash
# 直接处理URL
lawkit benf https://example.com/data.csv --optimize

# 优化网络传输
lawkit benf https://example.com/data.csv.gz --decompress --optimize

# 并行下载和分析
curl -s https://example.com/data.csv | lawkit benf --optimize --format json
```

### 分布式数据源

```bash
# 多数据源聚合
parallel -j8 "curl -s {} | lawkit benf --format json" ::: \
  https://api1.com/data \
  https://api2.com/data \
  https://api3.com/data

# 负载均衡处理
for endpoint in api1 api2 api3; do
  curl -s "https://$endpoint.com/data" | \
  lawkit benf --optimize --format json > "$endpoint.result" &
done
wait
```

## 生产环境优化

### 系统调优

```bash
# Linux内核参数优化
echo 'vm.swappiness=1' >> /etc/sysctl.conf
echo 'vm.vfs_cache_pressure=50' >> /etc/sysctl.conf
sysctl -p

# 文件描述符限制
ulimit -n 65536

# 内存映射限制
echo 'vm.max_map_count=262144' >> /etc/sysctl.conf
```

### 容器优化

```dockerfile
# 优化的Docker镜像
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release --features optimize

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/lawkit /usr/local/bin/
COPY --from=builder /app/target/release/benf /usr/local/bin/

# 性能调优
ENV RUST_LOG=warn
ENV LAWKIT_OPTIMIZE=true
ENV LAWKIT_CACHE_ENABLED=1

WORKDIR /data
ENTRYPOINT ["lawkit"]
```

### Kubernetes优化

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: lawkit-analyzer
spec:
  replicas: 3
  selector:
    matchLabels:
      app: lawkit-analyzer
  template:
    metadata:
      labels:
        app: lawkit-analyzer
    spec:
      containers:
      - name: lawkit
        image: lawkit:optimized
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "4Gi"
            cpu: "2"
        env:
        - name: LAWKIT_OPTIMIZE
          value: "true"
        - name: LAWKIT_MEMORY_LIMIT
          value: "3GB"
        volumeMounts:
        - name: cache-volume
          mountPath: /cache
        - name: data-volume
          mountPath: /data
      volumes:
      - name: cache-volume
        emptyDir:
          sizeLimit: 1Gi
      - name: data-volume
        persistentVolumeClaim:
          claimName: data-pvc
```

## 监控和分析

### 性能监控

```bash
#!/bin/bash
# performance_monitor.sh

# 创建性能监控脚本
exec 1> >(tee -a performance.log)
exec 2>&1

echo "=== Performance Test Started: $(date) ==="

# 测试不同配置
configs=(
    "--format json"
    "--format json --optimize"
    "--format json --optimize"
    "--format json --optimize --optimize"
)

for config in "${configs[@]}"; do
    echo "Testing: $config"
    
    /usr/bin/time -f "Time: %e seconds, Memory: %M KB" \
    lawkit benf test_data.csv $config > /dev/null
    
    echo "---"
done

echo "=== Performance Test Completed: $(date) ==="
```

### 性能分析工具

```bash
# CPU性能分析
perf record -g lawkit benf large_dataset.csv --optimize
perf report

# 内存分析
valgrind --tool=memcheck --leak-check=full \
lawkit benf test_data.csv

# 火焰图生成
perf record -F 99 -g lawkit benf large_dataset.csv
perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg
```

### 基准测试脚本

```bash
#!/bin/bash
# benchmark.sh

# 测试数据大小
sizes=(1000 10000 100000 1000000)

# 创建测试数据
for size in "${sizes[@]}"; do
    lawkit generate benf --samples $size > "test_${size}.csv"
done

# 运行基准测试
echo "Size,Standard,Parallel,Optimized,Streaming"

for size in "${sizes[@]}"; do
    file="test_${size}.csv"
    
    # 标准模式
    standard_time=$(time -p lawkit benf "$file" 2>&1 | grep real | awk '{print $2}')
    
    # 优化模式
    optimized_time=$(time -p lawkit benf "$file" --optimize 2>&1 | grep real | awk '{print $2}')
    
    echo "$size,$standard_time,$optimized_time"
done

# 清理测试文件
rm test_*.csv
```

## 故障排除

### 性能问题诊断

```bash
# 系统资源检查
htop
iotop
nethogs

# lawkit特定监控
strace -c lawkit benf large_file.csv
ltrace lawkit benf test_data.csv

# 分析慢查询
lawkit benf slow_file.csv --debug --profile --verbose
```

### 常见性能问题

#### 内存不足
```bash
# 解决方案：启用优化模式
lawkit benf huge_file.csv --optimize

# 或者增加交换空间
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

#### CPU瓶颈
```bash
# 解决方案：减少线程数
lawkit benf data.csv --optimize

# 或者使用nice优先级
nice -n 10 lawkit benf data.csv
```

#### I/O瓶颈
```bash
# 解决方案：使用内存文件系统
sudo mount -t tmpfs -o size=2G tmpfs /tmp/ramdisk
cp large_file.csv /tmp/ramdisk/
lawkit benf /tmp/ramdisk/large_file.csv
```

## 最佳实践

### 生产环境部署

1. **监控设置**
   ```bash
   # 设置监控告警
   lawkit benf data.csv --format json | \
   jq -e '.statistics.chi_square > 20' && \
   echo "Alert: High chi-square detected"
   ```

2. **自动扩缩容**
   ```yaml
   # Kubernetes HPA
   apiVersion: autoscaling/v2
   kind: HorizontalPodAutoscaler
   metadata:
     name: lawkit-hpa
   spec:
     scaleTargetRef:
       apiVersion: apps/v1
       kind: Deployment
       name: lawkit-analyzer
     minReplicas: 2
     maxReplicas: 10
     metrics:
     - type: Resource
       resource:
         name: cpu
         target:
           type: Utilization
           averageUtilization: 70
   ```

3. **资源限制**
   ```bash
   # 使用cgroups限制资源
   cgcreate -g memory,cpu:lawkit
   echo 2G > /sys/fs/cgroup/memory/lawkit/memory.limit_in_bytes
   cgexec -g memory,cpu:lawkit lawkit benf large_file.csv
   ```

### 开发环境优化

```bash
# 开发模式配置
export LAWKIT_DEBUG=1
export LAWKIT_CACHE_ENABLED=0
export RUST_BACKTRACE=1

# 快速迭代
lawkit benf small_sample.csv --fast-estimate --format json
```

## 下一步

- 查看[高级分析指南](advanced-analysis_zh.md)了解复杂分析技巧
- 参考[集成指南](integrations_zh.md)了解系统集成方法
- 阅读[配置指南](../user-guide/configuration_zh.md)了解详细配置选项