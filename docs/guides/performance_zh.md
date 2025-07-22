# Performance Guide

Optimize lawkit performance for different use cases and data sizes.

## Performance Overview

lawkit is designed to handle various data sizes efficiently:

- **Small datasets** (< 1,000 records): Instant analysis
- **Medium datasets** (1,000 - 100,000 records): < 5 seconds
- **Large datasets** (100,000 - 1M records): < 30 seconds
- **Very large datasets** (> 1M records): Sampling recommended

## Optimization Strategies

### 1. Basic Analysis Commands

```bash
# Benford's law analysis
lawkit benf data.csv

# Pareto analysis
lawkit pareto data.csv --threshold 0.8

# Zipf's law analysis
lawkit zipf text.txt

# Normal distribution analysis
lawkit normal data.csv

# Poisson distribution analysis
lawkit poisson data.csv

# Integrated analysis
lawkit analyze data.csv --laws benford,pareto,normal
```

### 2. Output Format Optimization

```bash
# Minimize output for faster processing
lawkit benf data.csv --quiet --format json

# Detailed information when needed
lawkit benf data.csv --verbose

# Different output formats
lawkit benf data.csv --format csv
lawkit benf data.csv --format yaml
lawkit benf data.csv --format toml
lawkit benf data.csv --format xml
```

### 3. Integrated Analysis Optimization

```bash
# Multi-law comparison analysis
lawkit analyze data.csv --laws benford,pareto

# Focus on specific analysis
lawkit analyze data.csv --laws benford --focus accuracy

# Optimize for specific purpose
lawkit analyze data.csv --laws all --purpose audit

# Use recommendation system
lawkit analyze data.csv --laws all --recommend
```

## File Format Optimization

### CSV Performance

```bash
# Best performance: properly formatted CSV
lawkit benf optimized.csv

# Quiet mode for fast processing
lawkit benf data.csv --quiet

# Detailed analysis when needed
lawkit benf data.csv --verbose
```

## Benchmarking

### Basic Benchmarking

```bash
# Run with timing information
time lawkit benf data.csv

# Compare different configurations
time lawkit benf data.csv --quiet
time lawkit benf data.csv --verbose
time lawkit analyze data.csv --laws benford
time lawkit analyze data.csv --laws benford,pareto
```

### Custom Benchmarks

```bash
#!/bin/bash
# benchmark_script.sh

echo "Benchmarking lawkit performance..."

# Test different laws
echo "Benford's law test:"
time lawkit benf data.csv --quiet

echo "Pareto analysis test:"
time lawkit pareto data.csv --quiet

echo "Integrated analysis test:"
time lawkit analyze data.csv --laws benford,pareto --quiet

echo "All laws analysis test:"
time lawkit analyze data.csv --laws all --quiet
```

## CPU Optimization

### Basic Optimization

```bash
# Lightweight analysis
lawkit benf data.csv --quiet

# Detailed analysis
lawkit benf data.csv --verbose

# Efficient multi-law execution
lawkit analyze data.csv --laws benford,pareto --quiet
```

## Output Optimization

### Efficient Output Formats

```bash
# Minimize output for faster processing
lawkit benf data.csv --quiet --format json

# Structured output
lawkit analyze data.csv --format json --quiet

# Human-readable format
lawkit benf data.csv --format yaml
```

## Performance Monitoring

### Basic Monitoring

```bash
# Using system tools
time lawkit benf data.csv
/usr/bin/time -v lawkit benf data.csv

# Detailed timing information
time lawkit analyze data.csv --laws all --verbose
```

## Performance Tuning Examples

### Small Data (< 1K records)

```bash
# Minimal overhead configuration
lawkit benf small_data.csv --quiet
```

### Medium Data (1K - 100K records)

```bash
# Balanced configuration
lawkit analyze medium_data.csv --laws benford,pareto
```

### Large Data (100K - 1M records)

```bash
# Optimized for large datasets
lawkit analyze large_data.csv --laws benford --quiet
```

### Very Large Data (> 1M records)

```bash
# Maximum optimization
lawkit benf huge_data.csv --quiet --format json
```

## Troubleshooting Performance Issues

### Common Issues

1. **Slow file reading**
   ```bash
   # Solution: Use quiet mode
   lawkit benf data.csv --quiet
   ```

2. **Slow analysis**
   ```bash
   # Solution: Use single law
   lawkit benf data.csv --quiet
   ```

3. **Too much output**
   ```bash
   # Solution: Use quiet mode
   lawkit analyze data.csv --laws benford --quiet
   ```

### Diagnostic Commands

```bash
# Version information and help
lawkit --version
lawkit --help

# Command-specific help
lawkit benf --help
lawkit pareto --help
lawkit zipf --help
lawkit normal --help
lawkit poisson --help
lawkit analyze --help
lawkit generate --help
lawkit list --help
```

## Future Optimization Features

The following features are planned for future versions:

- Parallel processing support
- Memory limit settings
- Sampling capabilities
- Configuration file support
- Advanced outlier detection
- Time series analysis
- Batch processing mode

Use these basic optimization techniques to achieve the best performance with the current implementation.