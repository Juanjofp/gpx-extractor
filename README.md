# 🗺️ gpx-extractor

[![Crates.io](https://img.shields.io/crates/v/gpx-extractor.svg)](https://crates.io/crates/gpx-extractor)
[![Documentation](https://docs.rs/gpx-extractor/badge.svg)](https://docs.rs/gpx-extractor)
[![License](https://img.shields.io/crates/l/gpx-extractor.svg)](https://github.com/Juanjofp/gpx-extractor#license)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

A fast and ergonomic Rust library for parsing and building GPX (GPS Exchange Format) files.

## ✨ Features

- 🚀 **Fast parsing** - Efficient XML deserialization with `quick-xml` and `serde`
- 🏗️ **Builder API** - Create GPX structures programmatically
- 📊 **Statistics** - Calculate distances (Haversine), duration, speed, elevation gain/loss, and more
- 🔄 **Roundtrip support** - Parse XML → Modify → Serialize back to XML
- 🛡️ **Type-safe** - Strong typing with comprehensive error handling
- 📦 **Zero-copy where possible** - Minimal allocations
- 🎯 **Idiomatic Rust** - Clean API following Rust best practices

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
gpx-extractor = "0.1"
```

## 📖 Usage Examples

### Parse a GPX file

```rust
use gpx_extractor::Gpx;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("route.gpx")?;
    let gpx = Gpx::try_from(xml.as_str())?;

    println!("📊 Tracks: {}", gpx.tracks.len());
    println!("📏 Distance: {:.2} km", gpx.total_distance_km());

    // Duration and speed (if timestamps are available)
    if let Some(duration) = gpx.total_duration_formatted() {
        println!("⏱️  Duration: {}", duration);
    }

    if let Some(speed) = gpx.average_speed_kmh() {
        println!("🚴 Average speed: {:.2} km/h", speed);
    }

    if let Some((min, max)) = gpx.elevation_range() {
        println!("⛰️  Elevation: {:.1}m - {:.1}m", min, max);
    }

    Ok(())
}
```

### Build a GPX from scratch

```rust
use gpx_extractor::{Gpx, Track, TrackSegment, Point, Waypoint};

let mut gpx = Gpx::new();

// Create a track
let mut track = Track::with_name("Morning Run".to_string());
let segment = TrackSegment::with_points(vec![
    Point::with_elevation(40.7128, -74.0060, 10.0),
    Point::with_elevation(40.7589, -73.9851, 15.0),
]);
track.add_segment(segment);
gpx.add_track(track);

// Add waypoints
gpx.add_waypoint(Waypoint::with_name(
    40.7128, -74.0060, "Start".to_string()
));

// Save to file
gpx.save_to_file("output.gpx")?;
```

### Using the prelude

```rust
use gpx_extractor::prelude::*;

let gpx = Gpx::new();
let point = Point::new(40.7128, -74.0060);
let stats = gpx.statistics();
```

## 🗺️ GPX Structure

```
GPX
├── Metadata (optional)
│   └── time - Timestamp
├── Tracks (trk) - Recorded routes
│   ├── Name - Track name
│   └── Track Segments (trkseg) - Continuous segments
│       └── Track Points (trkpt) - Individual points
│           ├── @lat - Latitude
│           ├── @lon - Longitude
│           ├── ele - Elevation (optional)
│           └── time - Timestamp (optional)
└── Waypoints (wpt) - Points of interest
    ├── @lat - Latitude
    ├── @lon - Longitude
    ├── name - Waypoint name (optional)
    └── time - Timestamp (optional)
```

## 🔧 API Overview

### Core Types

- **`Gpx`** - Main container for GPX data
- **`Track`** - A GPS track (collection of segments)
- **`TrackSegment`** - A continuous part of a track
- **`Point`** - A geographic point (lat, lon, optional elevation)
- **`Waypoint`** - A named point of interest
- **`GpxStatistics`** - Computed statistics for a GPX

### Key Methods

#### Parsing

```rust
Gpx::try_from(xml: &str) -> Result<Gpx, ParseError>
Gpx::try_from_str(xml: &str) -> Result<Gpx, ParseError>
```

#### Statistics

```rust
gpx.total_distance_km() -> f64
gpx.total_duration_seconds() -> Option<i64>
gpx.total_duration_formatted() -> Option<String>  // Format: HH:MM:SS
gpx.average_speed_kmh() -> Option<f64>
gpx.elevation_range() -> Option<(f64, f64)>
gpx.total_elevation_gain() -> Option<f64>
gpx.total_elevation_loss() -> Option<f64>
gpx.statistics() -> GpxStatistics
```

#### Serialization

```rust
gpx.to_xml() -> String
gpx.save_to_file(path: &str) -> Result<(), Box<dyn Error>>
```

## 📦 Optional Features

### CLI Tool

Install the command-line interface:

```bash
cargo install gpx-extractor --features cli
```

Usage:

```bash
# Analyze a single GPX file
gpx-cli route.gpx

# Analyze all GPX files in a directory
gpx-cli ./gpx_files/

# Show detailed statistics
gpx-cli route.gpx --verbose

# Sort files by date
gpx-cli ./gpx_files/ --sort
```

Enable in `Cargo.toml`:

```toml
[dependencies]
gpx-extractor = { version = "0.1", features = ["cli"] }
```

## 📚 Examples

The repository includes several examples:

- [`parse_gpx.rs`](examples/parse_gpx.rs) - Parse and analyze GPX files
- [`create_gpx.rs`](examples/create_gpx.rs) - Build GPX from scratch
- [`gpx_to_xml_demo.rs`](examples/gpx_to_xml_demo.rs) - XML conversion demo
- [`duration_stats.rs`](examples/duration_stats.rs) - Calculate duration and average speed
- [`analyze_gpx_file.rs`](examples/analyze_gpx_file.rs) - Complete GPX file analysis

Run examples with:

```bash
cargo run --example parse_gpx
cargo run --example create_gpx
cargo run --example duration_stats
cargo run --example analyze_gpx_file
```

## 🏗️ Project Structure

```
gpx-extractor/
├── src/
│   ├── lib.rs              # Public API and documentation
│   ├── bin/
│   │   └── gpx-cli.rs      # Optional CLI tool
│   └── gpx/
│       ├── mod.rs          # Module declarations
│       ├── gpx.rs          # Main Gpx struct
│       ├── point.rs        # Point struct
│       ├── track.rs        # Track & TrackSegment
│       └── waypoint.rs     # Waypoint struct
├── examples/               # Usage examples
├── tests/                  # Integration tests
└── benches/                # Benchmarks
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## 📊 Benchmarks

```bash
cargo bench
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## 🙏 Acknowledgments

- Built with [quick-xml](https://github.com/tafia/quick-xml) for efficient XML parsing
- Uses [serde](https://serde.rs/) for serialization/deserialization
- Distance calculations use the Haversine formula

## 📮 Support

- 📖 [Documentation](https://docs.rs/gpx-extractor)
- 🐛 [Issue Tracker](https://github.com/Juanjofp/gpx-extractor/issues)
- 💬 [Discussions](https://github.com/Juanjofp/gpx-extractor/discussions)

---

Made with ❤️ by [Juanjo](https://github.com/Juanjofp)
