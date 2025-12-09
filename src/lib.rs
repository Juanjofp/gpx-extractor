//! # gpx-extractor
//!
//! A fast and ergonomic library for parsing and building GPX (GPS Exchange Format) files.
//!
//! ## Features
//!
//! - **Parse GPX files** from XML strings or files
//! - **Build GPX structures** programmatically
//! - **Calculate statistics** (distance, elevation, etc.)
//! - **Serialize to XML** with multiple output options
//! - **Type-safe** with strong error handling
//!
//! ## Quick Start
//!
//! ```rust
//! use gpx_extractor::Gpx;
//! use std::convert::TryFrom;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Parse from XML
//! let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
//! <gpx version="1.1" creator="example">
//!   <trk>
//!     <name>My Route</name>
//!     <trkseg>
//!       <trkpt lat="40.7128" lon="-74.0060">
//!         <ele>10.0</ele>
//!       </trkpt>
//!       <trkpt lat="40.7589" lon="-73.9851">
//!         <ele>15.0</ele>
//!       </trkpt>
//!     </trkseg>
//!   </trk>
//! </gpx>"#;
//!
//! let gpx = Gpx::try_from(xml)?;
//!
//! // Get statistics
//! let stats = gpx.statistics();
//! println!("Distance: {:.2} km", stats.total_distance_km);
//! # Ok(())
//! # }
//! ```
//!
//! ## Examples
//!
//! ### Creating a GPX from scratch
//!
//! ```rust
//! use gpx_extractor::{Gpx, Track, TrackSegment, Point, Waypoint};
//!
//! let mut gpx = Gpx::new();
//!
//! // Add a track
//! let mut track = Track::with_name("Morning Run".to_string());
//! let segment = TrackSegment::with_points(vec![
//!     Point::with_elevation(40.7128, -74.0060, 10.0),
//!     Point::with_elevation(40.7589, -73.9851, 15.0),
//! ]);
//! track.add_segment(segment);
//! gpx.add_track(track);
//!
//! // Add waypoints
//! gpx.add_waypoint(Waypoint::with_name(40.7128, -74.0060, "Start".to_string()));
//!
//! // Convert to XML
//! let xml = gpx.to_xml();
//! ```
//!
//! ### Parsing and analyzing a GPX file
//!
//! ```rust,no_run
//! use gpx_extractor::Gpx;
//! use std::convert::TryFrom;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let xml = std::fs::read_to_string("route.gpx")?;
//! let gpx = Gpx::try_from(xml.as_str())?;
//!
//! println!("📊 GPX Analysis:");
//! println!("Tracks: {}", gpx.tracks.len());
//! println!("Distance: {:.2} km", gpx.total_distance_km());
//!
//! if let Some((min, max)) = gpx.elevation_range() {
//!     println!("Elevation: {:.1}m - {:.1}m", min, max);
//! }
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod gpx;

// Re-export public API
pub use gpx::gpx::{Gpx, GpxStatistics, Metadata};
pub use gpx::point::Point;
pub use gpx::track::{Track, TrackSegment};
pub use gpx::waypoint::Waypoint;

/// Error types for GPX operations
pub mod error {
    /// Re-export quick-xml errors for convenience
    pub use quick_xml::DeError as ParseError;
}

/// Prelude module for convenient imports
///
/// This module provides a convenient way to import the most commonly used types.
///
/// # Example
///
/// ```rust
/// use gpx_extractor::prelude::*;
///
/// let gpx = Gpx::new();
/// let point = Point::new(40.7128, -74.0060);
/// ```
pub mod prelude {
    pub use crate::{Gpx, GpxStatistics, Point, Track, TrackSegment, Waypoint};
}
