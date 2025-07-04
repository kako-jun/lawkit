# Changelog

All notable changes to lawkit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete documentation restructure following diffx patterns
- Japanese documentation variants for all guides
- Integration guides for CI/CD, databases, and cloud platforms
- Performance optimization guides
- Project documentation (changelog, roadmap)

### Changed
- Documentation structure now matches diffx sibling project
- FAQ moved to user-guide subdirectory
- All docs organized into proper subdirectories

## [2.0.1] - 2024-07-05

### Fixed
- All clippy warnings resolved
- Code style improvements and modernization
- Redundant field names in struct initialization
- Manual strip operations replaced with strip_prefix
- Manual clamp operations replaced with clamp method
- Manual range contains implementations
- Improved error handling consistency

### Changed
- GitHub Actions CI workflow streamlined
- Code formatting standardized across project

## [2.0.0] - 2024-07-04

### Added
- **Integration Analysis**: Multi-law comparison and conflict detection
- **Recommendation System**: Automated law selection and conflict resolution
- **Zipf Law Analysis**: Text frequency analysis with multi-language support
- **Normal Distribution Analysis**: Statistical normality testing and quality control
- **Poisson Distribution Analysis**: Event prediction and rare event analysis
- **Unified CLI Interface**: `lawkit [subcommand]` structure
- **Backward Compatibility**: Full `benf` command compatibility maintained

### Changed
- **Architecture Redesign**: Monolithic tool → integrated platform
- **Project Structure**: lawkit-cli + lawkit-core workspace architecture
- **Type System**: Unified common foundation + law-specific implementations

### Enhanced
- **Pareto Analysis**: 80/20 principle, Gini coefficient, business insights
- **Multi-language Support**: English, Japanese, Chinese, Hindi, Arabic
- **International Numerals**: Support for various number formats
- **Output Formats**: text, json, csv, yaml, toml, xml

## [1.x.x] - benf Legacy Versions

### [1.2.0] - 2024-06-15
- Enhanced Japanese numeral recognition
- Improved file format detection
- Better error handling for edge cases

### [1.1.0] - 2024-05-20
- Added multiple output formats (JSON, CSV, YAML)
- International numeral support
- Performance improvements for large datasets

### [1.0.0] - 2024-04-10
- Initial release as `benf`
- Basic Benford's Law analysis
- CSV file support
- CLI interface

## Migration Guide

### From benf to lawkit 2.0

**Old Command:**
```bash
benf data.csv
```

**New Command (100% compatible):**
```bash
lawkit benf data.csv
```

**New Capabilities:**
```bash
# Multi-law analysis
lawkit compare data.csv --laws benford,pareto,normal

# Pareto analysis
lawkit pareto sales.csv --gini

# Text analysis
lawkit zipf document.txt --language ja

# Quality control
lawkit normal measurements.csv --outliers
```

### Configuration Migration

Old `benf.toml` configurations are automatically compatible with `lawkit.toml`.

### API Migration

For programmatic usage:

**Old:**
```rust
use benf::analyze_benford;
```

**New:**
```rust
use lawkit_core::laws::benford::BenfordAnalysis;
```

## Development Milestones

### Phase 1: Foundation (Completed)
- ✅ benf → lawkit migration
- ✅ Unified architecture
- ✅ Backward compatibility

### Phase 2: Multi-Law Platform (Completed)
- ✅ Pareto Law implementation
- ✅ Zipf Law implementation  
- ✅ Normal Distribution analysis
- ✅ Poisson Distribution analysis
- ✅ Integration analysis framework

### Phase 3: Advanced Features (Completed)
- ✅ Multi-law comparison
- ✅ Conflict detection
- ✅ Recommendation system
- ✅ Performance optimization

### Phase 4: Documentation & Polish (Completed)
- ✅ Comprehensive documentation
- ✅ Usage examples
- ✅ Integration guides
- ✅ Performance guides

## Future Roadmap

See [roadmap.md](roadmap.md) for planned future enhancements.

## Contributors

- kako-jun - Original author and maintainer
- Claude Code - Development assistance and automation

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Acknowledgments

- Inspired by statistical fraud detection research
- Built with Rust for performance and safety
- Community feedback and contributions