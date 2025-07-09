# lawkit Documentation

lawkit is a comprehensive statistical law analysis toolkit for data quality assessment and fraud detection using multiple statistical laws.

## Key Features

- **Benford Law**: Fraud detection in accounting data, election results, and natural datasets
- **Pareto Analysis**: 80/20 rule analysis, sales analysis, inventory management
- **Zipf Law**: Text analysis, word frequency analysis
- **Normal Distribution**: Quality control, outlier detection, process capability assessment
- **Poisson Distribution**: Event occurrence prediction, rare event analysis
- **Integration Analysis**: Multi-law comparison, contradiction detection, recommendation system

## Quick Start

```bash
# Installation
cargo install lawkit

# Benford law analysis
lawkit benf data.csv

# Pareto analysis
lawkit pareto sales.csv

# Multi-law comparison
lawkit analyze data.csv --laws benford,pareto,normal
```

## Documentation

### User Guide
- [Installation](user-guide/installation.md)
- [Getting Started](user-guide/getting-started.md)
- [Configuration](user-guide/configuration.md)
- [Examples](user-guide/examples.md)

### Reference
- [CLI Reference](reference/cli-reference.md)

### Guides
- [Architecture Guide](guides/architecture.md) - System design and architecture overview
- [Integration Guide](guides/integrations.md)
- [Performance Guide](guides/performance.md)
- [Advanced Analysis](guides/advanced-analysis.md)

### Project
- [Changelog](../CHANGELOG.md)
- [Roadmap](project/roadmap.md)

### Other
- [FAQ](user-guide/faq.md)

## Support

For questions or issues, please report them on [GitHub Issues](https://github.com/kako-jun/lawkit/issues). We provide support in multiple languages.