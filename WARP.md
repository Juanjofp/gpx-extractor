# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

gpx-extractor is a Rust-based GPX (GPS Exchange Format) data extraction tool. The project follows a library + binary architecture pattern where the core functionality is in a library crate (`src/lib.rs`) with a command-line interface provided by the binary (`src/main.rs`).

### Core Architecture

- **Entry Point**: `src/main.rs` - CLI interface that accepts a GPX file path as argument
- **Library**: `src/lib.rs` - Exports the main `Gpx` struct for public API
- **Core Module**: `src/gpx/mod.rs` - Contains the GPX parsing logic with `Point` and `Gpx` data structures
- **Current State**: The GPX parser has a skeleton implementation that needs to be completed

### Data Structures

```rust
// Internal Point representation
struct Point {
    lat: f64,
    lon: f64, 
    elevation: Option<f64>,
}

// Main GPX container (public API)
pub struct Gpx {
    points: Vec<Point>,
}
```

## Essential Commands

### Development Workflow

The project includes a comprehensive development script (`dev.sh`) for all common tasks:

```bash
# Show all available commands
./dev.sh help

# Essential development cycle
./dev.sh fmt        # Format code
./dev.sh clippy     # Lint and check for issues
./dev.sh check      # Verify compilation
./dev.sh test       # Run tests
./dev.sh all        # Run complete verification suite
```

### Core Rust Commands

```bash
# Build and run
cargo build
cargo run <path/to/file.gpx>

# Testing
cargo test                    # Run all tests
cargo test test_gpx_from_str  # Run specific test

# Development checks
cargo check              # Quick compilation check
cargo clippy -- -W clippy::all  # Comprehensive linting
cargo fmt               # Format code
```

### Advanced Formatting (Optional)

For enhanced formatting options, the project supports rustfmt nightly:

```bash
# Setup (one-time)
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly

# Use advanced formatting
./dev.sh fmt-nightly
```

### Security Auditing

```bash
# Install cargo-audit (first time)
cargo install cargo-audit

# Check for vulnerabilities
cargo audit
# or
./dev.sh audit
```

## Project Standards

### Code Style
- **Line width**: 100 characters maximum
- **Indentation**: 4 spaces
- **Edition**: Rust 2024
- **MSRV**: 1.70.0
- **Import organization**: Automatic reordering enabled

### Quality Tools Configuration
- **rustfmt**: Configured for consistent formatting with Unix line endings
- **Clippy**: Enhanced with strict lints including dead code and unused variable warnings
- **cargo-audit**: Available for dependency vulnerability scanning

### VS Code Integration
The project includes VS Code settings for:
- Automatic formatting on save
- Real-time error detection with rust-analyzer
- Integrated code analysis

## Testing Strategy

Tests are co-located with modules using the `#[cfg(test)]` pattern. The current test suite includes basic functionality verification in `src/gpx/mod.rs`.

## Development Notes

- The GPX parsing implementation is currently a stub and needs completion
- The project structure supports both library usage and CLI execution
- All development tools are configured to work together seamlessly
- Spanish comments and documentation are used throughout the project