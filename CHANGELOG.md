# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial public API for library usage
- Comprehensive documentation with examples
- Optional CLI feature with `clap` and `colored`
- Prelude module for convenient imports
- Error module re-exporting quick-xml errors

### Changed

- Refactored project structure for library publication
- Updated README for library usage
- Moved binary to `src/bin/gpx-cli.rs` with optional feature

## [0.1.0] - 2024-12-09

### Added

- GPX parsing from XML using `quick-xml` and `serde`
- GPX building API with fluent interface
- Distance calculation using Haversine formula
- Elevation statistics (range, gain, loss)
- XML serialization with multiple output methods
- Roundtrip support (XML → GPX → XML)
- Statistics generation (`GpxStatistics`)
- Support for:
  - Tracks with multiple segments
  - Track points with lat/lon/elevation
  - Waypoints
  - Metadata (timestamps)
- Comprehensive test suite
- Example programs:
  - `parse_gpx.rs` - Parse and analyze GPX files
  - `create_gpx.rs` - Build GPX from scratch
  - `gpx_to_xml_demo.rs` - XML conversion demonstration

### Technical Details

- Minimum Supported Rust Version (MSRV): 1.70.0
- Dependencies:
  - `quick-xml` 0.31 (XML parsing/serialization)
  - `serde` 1.0 (serialization framework)
  - `chrono` 0.4 (date/time handling)
  - `itertools` 0.14 (iterator utilities)
  - `clap` 4.5 (CLI, optional)
  - `colored` 2.1 (CLI colors, optional)

[Unreleased]: https://github.com/Juanjofp/gpx-extractor/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Juanjofp/gpx-extractor/releases/tag/v0.1.0
