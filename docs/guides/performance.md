# Performance Guide

Optimize lawkit performance for different use cases and data sizes.

## Performance Overview

lawkit is designed to handle various data sizes efficiently:

- **Small datasets** (< 1,000 records): Instant analysis
- **Medium datasets** (1,000 - 100,000 records): < 5 seconds
- **Large datasets** (100,000 - 1M records): < 30 seconds
- **Very large datasets** (> 1M records): Use sampling strategies

## Optimization Strategies

### 1. Sampling for Large Datasets

```bash
# Sample 50,000 records from large dataset
lawkit benf huge_dataset.csv --sample-size 50000

# Sample with custom seed for reproducibility
lawkit benf data.csv --sample-size 10000 --seed 12345
```

### 2. Parallel Processing

```bash
# Use multiple threads
lawkit compare data.csv --threads 8

# Let lawkit auto-detect optimal threads
lawkit compare data.csv --threads auto
```

### 3. Memory Management

```bash
# Set memory limit (in MB)
lawkit benf large_file.csv --memory-limit 2048

# Use streaming mode for very large files
lawkit benf massive_file.csv --streaming
```

### 4. Column Selection

```bash
# Analyze only specific columns
lawkit benf financial.csv --columns "amount,revenue"

# Exclude unnecessary columns
lawkit benf data.csv --exclude-columns "id,timestamp,notes"
```

## File Format Optimization

### CSV Performance

```bash
# Best performance: properly formatted CSV
lawkit benf optimized.csv

# Specify delimiter to avoid auto-detection
lawkit benf data.csv --delimiter ","

# Skip header detection if known
lawkit benf data.csv --no-header
```

### Other Formats

**Excel files:**
```bash
# Specify sheet to avoid scanning all sheets
lawkit benf workbook.xlsx --sheet "Financial Data"

# Limit rows to read
lawkit benf large_workbook.xlsx --max-rows 100000
```

**JSON files:**
```bash
# Use streaming parser for large JSON
lawkit benf large_data.json --streaming

# Specify JSON path for nested data
lawkit benf complex.json --json-path "$.transactions[*].amount"
```

## Benchmarking

### Built-in Benchmarking

```bash
# Run with timing information
lawkit benf data.csv --benchmark

# Detailed performance metrics
lawkit benf data.csv --profile-memory

# Compare different configurations
lawkit benf data.csv --benchmark --threads 1
lawkit benf data.csv --benchmark --threads 4
lawkit benf data.csv --benchmark --threads 8
```

### Custom Benchmarks

```bash
#!/bin/bash
# benchmark_script.sh

echo "Benchmarking lawkit performance..."

# Test different sample sizes
for size in 1000 5000 10000 50000; do
    echo "Testing sample size: $size"
    time lawkit benf large_dataset.csv --sample-size $size --quiet
done

# Test different thread counts
for threads in 1 2 4 8; do
    echo "Testing threads: $threads"
    time lawkit compare data.csv --threads $threads --quiet
done
```

## Memory Usage Optimization

### Configuration

```toml
# lawkit.toml
[performance]
# Limit memory usage
memory_limit = 1024  # MB

# Use streaming for large files
streaming_threshold = 100000  # rows

# Cache settings
cache_enabled = true
cache_size = 512  # MB

[processing]
# Chunk size for streaming
chunk_size = 10000

# Buffer size for file I/O
buffer_size = 8192
```

### Runtime Optimization

```bash
# Monitor memory usage
lawkit benf large_file.csv --monitor-memory

# Use memory-efficient algorithms
lawkit normal huge_dataset.csv --algorithm memory-efficient

# Clear cache between runs
lawkit benf data1.csv --clear-cache
lawkit benf data2.csv
```

## CPU Optimization

### Thread Configuration

```bash
# CPU-intensive analysis with maximum threads
lawkit compare data.csv --threads $(nproc)

# Balanced approach (leave some cores free)
lawkit compare data.csv --threads $(($(nproc) - 2))

# Single-threaded for consistency
lawkit benf data.csv --threads 1
```

### Algorithm Selection

```bash
# Fast algorithms for quick analysis
lawkit benf data.csv --algorithm fast

# Accurate algorithms for precise results
lawkit benf data.csv --algorithm precise

# Balanced algorithms (default)
lawkit benf data.csv --algorithm balanced
```

## I/O Optimization

### Reading Large Files

```bash
# Use memory mapping for large files
lawkit benf huge_file.csv --mmap

# Streaming mode for files larger than memory
lawkit benf massive_file.csv --streaming

# Parallel file reading
lawkit benf data.csv --parallel-io
```

### Output Optimization

```bash
# Minimize output for faster processing
lawkit benf data.csv --quiet --output json

# Stream output for real-time processing
lawkit benf data.csv --stream-output

# Compress output for large results
lawkit compare data.csv --compress-output
```

## Network Performance

### Remote Files

```bash
# Cache remote files locally
lawkit benf https://example.com/data.csv --cache-remote

# Streaming for large remote files
lawkit benf https://example.com/huge.csv --streaming

# Parallel download chunks
lawkit benf https://example.com/data.csv --parallel-download
```

## Performance Monitoring

### Built-in Metrics

```bash
# Detailed performance report
lawkit benf data.csv --performance-report

# Export metrics to file
lawkit benf data.csv --metrics-output metrics.json

# Real-time monitoring
lawkit benf data.csv --monitor
```

### External Monitoring

```bash
# Using system tools
time lawkit benf data.csv
/usr/bin/time -v lawkit benf data.csv

# Memory profiling
valgrind --tool=massif lawkit benf data.csv

# CPU profiling
perf record lawkit benf data.csv
perf report
```

## Performance Tuning Examples

### Small Data (< 1K records)

```bash
# Minimal overhead configuration
lawkit benf small_data.csv \
  --threads 1 \
  --no-cache \
  --algorithm fast
```

### Medium Data (1K - 100K records)

```bash
# Balanced configuration
lawkit compare medium_data.csv \
  --threads 4 \
  --cache-enabled \
  --algorithm balanced
```

### Large Data (100K - 1M records)

```bash
# Optimized for large datasets
lawkit compare large_data.csv \
  --threads 8 \
  --memory-limit 2048 \
  --sample-size 100000 \
  --streaming
```

### Very Large Data (> 1M records)

```bash
# Maximum optimization
lawkit benf huge_data.csv \
  --sample-size 50000 \
  --threads $(nproc) \
  --memory-limit 4096 \
  --streaming \
  --algorithm fast \
  --cache-enabled
```

## Troubleshooting Performance Issues

### Common Issues

1. **Slow file reading**
   ```bash
   # Solution: Use streaming mode
   lawkit benf data.csv --streaming
   ```

2. **High memory usage**
   ```bash
   # Solution: Set memory limit
   lawkit benf data.csv --memory-limit 1024
   ```

3. **Slow analysis**
   ```bash
   # Solution: Use sampling
   lawkit benf data.csv --sample-size 10000
   ```

### Diagnostic Commands

```bash
# Check system resources
lawkit system-info

# Validate file format efficiency
lawkit validate-file data.csv

# Performance recommendations
lawkit recommend-config data.csv
```

Use these optimization techniques to achieve the best performance for your specific use case and hardware configuration.