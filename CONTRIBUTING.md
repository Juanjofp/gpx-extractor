# Contributing to gpx-extractor

Thank you for your interest in contributing to gpx-extractor! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Rust version and OS information
- Sample GPX file if applicable (anonymized if needed)

### Suggesting Features

Feature requests are welcome! Please include:

- Clear description of the feature
- Use case and motivation
- Example API if applicable
- Any alternatives you've considered

### Pull Requests

1. **Fork the repository** and create a feature branch:

   ```bash
   git checkout -b feature/amazing-feature
   ```

2. **Make your changes**:

   - Write clean, idiomatic Rust code
   - Follow existing code style
   - Add tests for new functionality
   - Update documentation as needed

3. **Run tests and checks**:

   ```bash
   # Format code
   cargo fmt

   # Run clippy
   cargo clippy -- -D warnings

   # Run tests
   cargo test

   # Build documentation
   cargo doc --no-deps
   ```

4. **Commit your changes**:

   ```bash
   git commit -m 'Add amazing feature'
   ```

   Use clear, descriptive commit messages following these guidelines:

   - Use present tense ("Add feature" not "Added feature")
   - Use imperative mood ("Move cursor to..." not "Moves cursor to...")
   - Keep first line under 72 characters
   - Reference issues and PRs when relevant

5. **Push to your fork**:

   ```bash
   git push origin feature/amazing-feature
   ```

6. **Open a Pull Request**:
   - Provide a clear description of the changes
   - Link related issues
   - Ensure CI passes

## Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Cargo

### Building

```bash
# Clone the repository
git clone https://github.com/Juanjofp/gpx-extractor.git
cd gpx-extractor

# Build the library
cargo build

# Build with CLI feature
cargo build --features cli

# Run tests
cargo test

# Run examples
cargo run --example parse_gpx
cargo run --example create_gpx
```

## Code Style

- Follow Rust standard style (enforced by `rustfmt`)
- Use `cargo clippy` and address all warnings
- Write documentation for public APIs
- Include examples in doc comments
- Keep functions focused and modular

## Testing

- Write unit tests for new functionality
- Include integration tests for complex features
- Ensure all tests pass before submitting PR
- Add test cases for edge cases and error conditions

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Documentation

- Document all public APIs with doc comments
- Include examples in documentation
- Update README if adding user-facing features
- Keep CHANGELOG.md updated

```bash
# Generate and view documentation
cargo doc --open
```

## Benchmarks

If your changes affect performance:

```bash
# Run benchmarks
cargo bench
```

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v0.x.0 -m "Release v0.x.0"`
4. Push tag: `git push origin v0.x.0`
5. Publish to crates.io: `cargo publish`

## Questions?

Feel free to open an issue for questions or discussions about contributing!

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
