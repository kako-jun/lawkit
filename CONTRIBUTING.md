# Contributing to lawkit

We welcome contributions to lawkit! This document outlines how to contribute to the project.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Git

### Development Setup

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/lawkit.git
   cd lawkit
   ```

3. Install dependencies and build:
   ```bash
   cargo build
   ```

4. Run tests to ensure everything is working:
   ```bash
   cargo test
   ```

## Development Workflow

### Branch Naming

- Feature branches: `feature/description`
- Bug fixes: `bugfix/description`
- Documentation: `docs/description`

### Code Style

We follow standard Rust conventions:

- **Formatting**: Run `cargo fmt` before committing
- **Linting**: Run `cargo clippy` and fix all warnings
- **Testing**: Ensure all tests pass with `cargo test`

### Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `refactor:` for code refactoring
- `test:` for adding tests
- `chore:` for maintenance tasks

Examples:
```
feat: add zipf law analysis with multi-language support
fix: resolve integer overflow in benford calculation
docs: update README with new CLI options
```

## Project Structure

lawkit uses a workspace structure:

```
lawkit/
├── lawkit-core/        # Core library for statistical analysis
├── lawkit-cli/         # Command-line interface
├── tests/             # Integration tests
├── docs/              # Documentation
└── .github/           # CI/CD workflows
```

### lawkit-core

The core library contains:
- Statistical law implementations (Benford, Pareto, Zipf, Normal, Poisson)
- Data parsing and input handling
- International number support
- Output formatting

### lawkit-cli

The CLI application provides:
- Command-line interface using clap
- Subcommands for each statistical law
- Integration commands for multi-law analysis

## Adding New Features

### Adding a New Statistical Law

1. Create a new module in `lawkit-core/src/laws/`
2. Implement the analysis logic
3. Add result structures with serialization support
4. Create corresponding CLI subcommand in `lawkit-cli/src/subcommands/`
5. Add comprehensive tests
6. Update documentation

### Adding Output Formats

1. Extend the formatter in `lawkit-core/src/common/output/`
2. Add format-specific serialization
3. Update CLI argument parsing
4. Add tests for the new format

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test --package lawkit-core
cargo test --package lawkit

# Run integration tests
cargo test --test integration_tests

# Run benchmarks
cargo bench --package lawkit-core
```

### Test Organization

- **Unit tests**: Located alongside source code
- **Integration tests**: In the `tests/` directory
- **Benchmarks**: In `lawkit-core/benches/`

### Test Data

- Use the `tests/fixtures/` directory for test data
- Include samples for all supported formats
- Test edge cases and error conditions

## Documentation

### Code Documentation

- Document all public APIs with rustdoc comments
- Include examples in documentation
- Explain complex algorithms and mathematical concepts

### User Documentation

- Update README.md for user-facing changes
- Add usage examples for new features
- Update CLI help text

## Quality Assurance

### Before Submitting

1. **Format code**: `cargo fmt`
2. **Check linting**: `cargo clippy`
3. **Run tests**: `cargo test`
4. **Check benchmarks**: `cargo bench --package lawkit-core`
5. **Update documentation**: Ensure docs are current

### Continuous Integration

Our CI pipeline runs:
- Code formatting check
- Clippy linting
- Test suite on multiple platforms
- Benchmark regression testing

## Multi-language Support

lawkit supports multiple languages and international number formats:

- **Languages**: English, Japanese, Chinese, Hindi, Arabic
- **Number formats**: Various international numeral systems
- **Text processing**: Unicode-aware text analysis

When adding features:
- Consider international number support
- Test with non-ASCII input
- Maintain language-neutral core logic

## Performance Considerations

- Use `rayon` for parallel processing where beneficial
- Profile performance-critical code
- Consider memory usage for large datasets
- Benchmark changes against existing implementation

## Getting Help

- **Issues**: Report bugs and request features on GitHub Issues
- **Discussions**: Use GitHub Discussions for questions
- **Code review**: We provide thorough code reviews

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors are recognized in:
- Git commit history
- Release notes for significant contributions
- Project documentation

Thank you for contributing to lawkit! 🚀