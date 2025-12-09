//! Example: Parse a GPX file and display statistics
//!
//! This example demonstrates how to:
//! - Parse GPX from XML string
//! - Access GPX data structures
//! - Calculate statistics

use gpx_extractor::prelude::*;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="example">
  <metadata>
    <time>2024-07-11T17:16:43Z</time>
  </metadata>
  <trk>
    <name>Sample Track</name>
    <trkseg>
      <trkpt lat="40.7128" lon="-74.0060">
        <ele>10.0</ele>
      </trkpt>
      <trkpt lat="40.7589" lon="-73.9851">
        <ele>15.0</ele>
      </trkpt>
      <trkpt lat="40.7831" lon="-73.9712">
        <ele>20.0</ele>
      </trkpt>
    </trkseg>
  </trk>
  <wpt lat="40.7128" lon="-74.0060">
    <name>Start Point</name>
  </wpt>
</gpx>"#;

    println!("🗺️  Parsing GPX Example\n");

    // Parse the GPX
    let gpx = Gpx::try_from(xml)?;

    // Access basic information
    println!("📊 Basic Information:");
    println!("   Tracks: {}", gpx.tracks.len());
    println!("   Waypoints: {}", gpx.waypoints.len());
    println!("   Total Points: {}", gpx.total_points());

    // Get metadata
    if let Some(date) = gpx.date() {
        println!("   Date: {}", date);
    }

    // Calculate statistics
    println!("\n📏 Calculated Statistics:");
    let stats = gpx.statistics();
    println!("   Distance: {:.2} km", stats.total_distance_km);

    if let Some((min, max)) = stats.elevation_range {
        println!("   Elevation Range: {:.1}m - {:.1}m", min, max);
    }

    if let Some(gain) = stats.elevation_gain {
        println!("   Elevation Gain: {:.1}m", gain);
    }

    if let Some(loss) = stats.elevation_loss {
        println!("   Elevation Loss: {:.1}m", loss);
    }

    // Access track details
    println!("\n🛤️  Track Details:");
    for (i, track) in gpx.tracks.iter().enumerate() {
        println!(
            "   Track #{}: {}",
            i + 1,
            track.name.as_deref().unwrap_or("Unnamed")
        );
        println!("      Segments: {}", track.segments.len());
        println!("      Points: {}", track.total_points());
        println!("      Distance: {:.2} km", track.total_distance_km());
    }

    // Access waypoint details
    if !gpx.waypoints.is_empty() {
        println!("\n📍 Waypoints:");
        for (i, wp) in gpx.waypoints.iter().enumerate() {
            println!(
                "   Waypoint #{}: {} ({:.4}, {:.4})",
                i + 1,
                wp.name.as_deref().unwrap_or("Unnamed"),
                wp.lat,
                wp.lon
            );
        }
    }

    println!("\n✅ Parsing completed successfully!");

    Ok(())
}
