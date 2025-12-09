//! Example: Create a GPX file programmatically
//!
//! This example demonstrates how to:
//! - Build a GPX structure from scratch
//! - Add tracks, segments, and points
//! - Add waypoints
//! - Save to file and convert to XML

use gpx_extractor::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️  Creating GPX Programmatically\n");

    // Create a new GPX
    let mut gpx = Gpx::new();

    // Create a track with multiple segments
    println!("📊 Building track with segments...");
    let mut track = Track::with_name("Morning Run in NYC".to_string());

    // First segment - Central Park
    let segment1 = TrackSegment::with_points(vec![
        Point::with_elevation(40.7829, -73.9654, 15.0), // Central Park South
        Point::with_elevation(40.7851, -73.9683, 16.0),
        Point::with_elevation(40.7873, -73.9658, 17.0),
    ]);
    track.add_segment(segment1);

    // Second segment - Upper West Side
    let segment2 = TrackSegment::with_points(vec![
        Point::with_elevation(40.7900, -73.9700, 18.0),
        Point::with_elevation(40.7920, -73.9720, 19.0),
        Point::with_elevation(40.7940, -73.9740, 20.0),
    ]);
    track.add_segment(segment2);

    gpx.add_track(track);

    // Add waypoints for points of interest
    println!("📍 Adding waypoints...");
    gpx.add_waypoint(Waypoint::with_name(
        40.7829,
        -73.9654,
        "Start - Central Park".to_string(),
    ));

    gpx.add_waypoint(Waypoint::with_name(
        40.7940,
        -73.9740,
        "End - Upper West Side".to_string(),
    ));

    gpx.add_waypoint(Waypoint::new(40.7851, -73.9683)); // Waypoint without name

    // Display statistics
    println!("\n📈 GPX Statistics:");
    let stats = gpx.statistics();
    println!("   Tracks: {}", stats.total_tracks);
    println!("   Waypoints: {}", stats.total_waypoints);
    println!("   Total Points: {}", stats.total_points);
    println!("   Distance: {:.2} km", stats.total_distance_km);

    if let Some((min, max)) = stats.elevation_range {
        println!("   Elevation: {:.1}m - {:.1}m", min, max);
    }

    // Convert to XML
    println!("\n🔄 Converting to XML...");
    let xml = gpx.to_xml();
    println!("   XML size: {} bytes", xml.len());

    // Save to file
    let output_path = "/tmp/created_gpx_example.gpx";
    println!("\n💾 Saving to file: {}", output_path);
    gpx.save_to_file(output_path)?;
    println!("   ✅ File saved successfully!");

    // Verify by re-parsing
    println!("\n🔍 Verifying roundtrip...");
    let reparsed = Gpx::try_from_str(&xml)?;
    let reparsed_stats = reparsed.statistics();

    println!(
        "   Original points: {} | Reparsed points: {}",
        stats.total_points, reparsed_stats.total_points
    );
    println!(
        "   Original distance: {:.2} km | Reparsed distance: {:.2} km",
        stats.total_distance_km, reparsed_stats.total_distance_km
    );

    if stats.total_points == reparsed_stats.total_points {
        println!("   ✅ Roundtrip successful!");
    } else {
        println!("   ⚠️  Roundtrip mismatch!");
    }

    // Show sample XML (first 500 chars)
    println!("\n📄 Sample XML output:");
    println!("{}...", &xml[..xml.len().min(500)]);

    Ok(())
}
