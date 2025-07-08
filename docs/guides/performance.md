# Performance Guide

Optimize lawkit performance for different use cases and data sizes.

## Performance Overview

lawkit is designed to handle various data sizes efficiently:

- **Small datasets** (< 1,000 records): Instant analysis
- **Medium datasets** (1,000 - 100,000 records): < 5 seconds
- **Large datasets** (100,000 - 1M records): < 30 seconds
- **Very large datasets** (> 1M records): Use sampling strategies

## Optimization Strategies

### 1. Advanced Outlier Detection

```bash
# Use LOF (Local Outlier Factor) for sophisticated outlier detection
lawkit benf data.csv --outlier-method lof --outlier-k 5

# Use Isolation Forest-style anomaly detection
lawkit benf data.csv --outlier-method isolation --outlier-depth 8

# Use DBSCAN-style density-based outlier detection
lawkit benf data.csv --outlier-method dbscan --outlier-eps 0.5 --outlier-min-pts 3

# Use ensemble of multiple methods (recommended)
lawkit benf data.csv --outlier-method ensemble
```

### 2. Time Series Analysis

```bash
# Analyze time-based data patterns
lawkit benf time_series.csv --enable-timeseries --timestamp-column "date"

# Generate forecasts with confidence intervals
lawkit benf sales_data.csv --forecast-steps 5 --enable-timeseries

# Detect seasonal patterns and trends
lawkit benf monthly_data.csv --detect-seasonality --enable-timeseries

# Find changepoints in data
lawkit benf process_data.csv --detect-changepoints --enable-timeseries
```

### 3. Parallel Processing

```bash
# Enable optimizations for large datasets (includes parallel processing)
lawkit analyze data.csv --optimize

# Performance comparison
time lawkit analyze data.csv          # Standard processing
time lawkit analyze data.csv --optimize  # Optimized processing
```

### 4. Memory-Efficient Processing

```bash
# Enable optimizations for memory-efficient processing
lawkit benf massive_file.csv --optimize

# For very large datasets (optimizations include memory management)
lawkit benf large_file.csv --optimize

# Standard processing vs optimized
lawkit benf data.csv          # Standard mode
lawkit benf data.csv --optimize  # Optimized mode
```

### 5. Traditional Sampling

```bash
# Sample 50,000 records from large dataset
lawkit benf huge_dataset.csv --sample-size 50000

# Sample with custom seed for reproducibility
lawkit benf data.csv --sample-size 10000 --seed 12345
```

### 6. Column Selection

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
# Use optimized processing for large JSON
lawkit benf large_data.json --optimize

# Specify JSON path for nested data
lawkit benf complex.json --json-path "$.transactions[*].amount"
```

## Advanced Analysis Features

### Outlier Detection Methods

lawkit provides multiple sophisticated outlier detection algorithms:

1. **Local Outlier Factor (LOF)**
   - Detects outliers based on local density
   - Best for clustered data with varying densities
   - Parameter: `k` (number of neighbors, default: 5)

2. **Isolation Score**
   - Based on Isolation Forest principles
   - Fast for high-dimensional data
   - Parameter: `max_depth` (tree depth, default: 8)

3. **DBSCAN-style Detection**
   - Density-based spatial clustering
   - Good for irregular cluster shapes
   - Parameters: `eps` (neighborhood size), `min_pts` (minimum points)

4. **Ensemble Method**
   - Combines multiple algorithms for robust detection
   - Automatically tunes parameters
   - Provides consensus-based results

### Time Series Analysis Capabilities

Advanced time series features include:

- **Trend Analysis**: Linear regression with R² scores
- **Seasonality Detection**: Automatic period identification
- **Changepoint Detection**: Statistical significance testing
- **Forecasting**: Confidence intervals and uncertainty quantification
- **Anomaly Detection**: Context-aware outlier identification

### Parallel Processing Architecture

The unified `--optimize` flag automatically provides:

- **Automatic Thread Detection**: Uses available CPU cores efficiently
- **Memory Management**: Streams large files and manages memory usage
- **Parallel Processing**: Distributes work across available resources
- **Intelligent Chunking**: Processes data in optimal sizes

This single flag eliminates the need for users to understand complex performance tuning parameters.

- **Automatic Thread Detection**: Uses available CPU cores
- **Chunk-based Processing**: Memory-efficient data splitting
- **Load Balancing**: Work queue distribution
- **Result Aggregation**: Seamless result combination
- **Performance Monitoring**: Speedup and efficiency metrics

### Memory Management Features

Memory-efficient processing includes:

- **Streaming Processors**: Handle data larger than RAM
- **Incremental Statistics**: Welford's online algorithm
- **Chunk Iterators**: Fixed-memory data processing
- **Resource Monitoring**: Peak memory tracking

## Benchmarking

### Built-in Benchmarking

```bash
# Run with timing information
lawkit benf data.csv --benchmark

# Detailed performance metrics
lawkit benf data.csv --profile-memory

# Compare different configurations
time lawkit benf data.csv          # Standard processing
time lawkit benf data.csv --optimize  # Optimized processing
time lawkit benf large_data.csv --optimize  # Large dataset optimization
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

# Performance comparison
echo "Testing standard mode:"
time lawkit analyze data.csv --quiet
echo "Testing optimized mode:"
time lawkit analyze data.csv --optimize --quiet
```

## Memory Usage Optimization

### Configuration

```toml
# lawkit.toml
[performance]
# Limit memory usage
memory_limit = 1024  # MB

# Use optimizations for large files
optimization_threshold = 100000  # rows

# Cache settings
cache_enabled = true
cache_size = 512  # MB

[processing]
# Optimization configuration
optimize_large_datasets = true

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
# Optimized analysis for large datasets
lawkit analyze data.csv --optimize

# Standard processing for comparison
lawkit analyze data.csv

# Performance comparison
time lawkit benf data.csv
time lawkit benf data.csv --optimize
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

# Optimized mode for files larger than memory
lawkit benf massive_file.csv --optimize

# Optimized file processing
lawkit benf data.csv --optimize
```

### Output Optimization

```bash
# Minimize output for faster processing
lawkit benf data.csv --quiet --output json

# Quiet mode for real-time processing
lawkit benf data.csv --quiet

# Efficient output format for large results
lawkit analyze data.csv --format json
```

## Network Performance

### Remote Files

```bash
# Cache remote files locally
lawkit benf https://example.com/data.csv --cache-remote

# Optimized processing for large remote files
lawkit benf https://example.com/huge.csv --optimize

# Optimized download processing
lawkit benf https://example.com/data.csv --optimize
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
  --quiet \
  --no-cache \
  --algorithm fast
```

### Medium Data (1K - 100K records)

```bash
# Balanced configuration
lawkit analyze medium_data.csv \
  --optimize \
  --cache-enabled \
  --algorithm balanced
```

### Large Data (100K - 1M records)

```bash
# Optimized for large datasets
lawkit analyze large_data.csv \
  --optimize \
  --sample-size 100000
```

### Very Large Data (> 1M records)

```bash
# Maximum optimization with advanced features
lawkit benf huge_data.csv \
  --sample-size 50000 \
  --optimize \
  --memory-limit 4096 \
  --outlier-method ensemble \
  --incremental-stats

# Time series analysis for large datasets
lawkit benf timeseries_data.csv \
  --enable-timeseries \
  --optimize \
  --memory-limit 2048 \
  --forecast-steps 10
```

### Advanced Analysis Performance

```bash
# Optimized outlier detection for large datasets
lawkit benf data.csv \
  --outlier-method lof \
  --outlier-k 10 \
  --optimize

# Memory-efficient time series processing
lawkit benf timeseries.csv \
  --enable-timeseries \
  --optimize \
  --incremental-stats

# Parallel comparison analysis
lawkit analyze datasets/*.csv \
  --optimize \
  --memory-limit 1024
```

## Troubleshooting Performance Issues

### Common Issues

1. **Slow file reading**
   ```bash
   # Solution: Use optimization mode
   lawkit benf data.csv --optimize
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