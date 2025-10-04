use gpx_extractor::Gpx;

fn main() {
    // Read file content from command line argument
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <gpx_file>", args[0]);
        std::process::exit(1);
    }

    let gpx_file_name = &args[1];

    println!("📍 Reading GPX file: {}", gpx_file_name);

    let gpx_content = match std::fs::read_to_string(gpx_file_name) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read GPX file: {}", e);
            std::process::exit(1);
        }
    };

    let gpx = Gpx::from(gpx_content.as_str());

    println!("\n🗂️  GPX Analysis:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("📊 Tracks: {}", gpx.tracks.len());
    println!("📍 Waypoints: {}", gpx.waypoints.len());
    println!("🔢 Total points: {}", gpx.total_points());

    if !gpx.tracks.is_empty() {
        for (i, track) in gpx.tracks.iter().enumerate() {
            let default_name = format!("Track {}", i + 1);
            let track_name = track.name.as_deref().unwrap_or(&default_name);
            println!("🛤️  {}: {} segments", track_name, track.segments.len());

            for (j, segment) in track.segments.iter().enumerate() {
                println!("   📈 Segment {}: {} points", j + 1, segment.points.len());
            }
        }
    }

    let distance = gpx.total_distance_km();

    if distance > 0.0 {
        println!("📏 Total distance: {:.2} km", distance);
    }

    if let Some((min_ele, max_ele)) = gpx.elevation_range() {
        println!("⛰️  Elevation: {:.1}m - {:.1}m", min_ele, max_ele);
        println!("📈 Elevation gain: {:.1}m", max_ele - min_ele);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ GPX file processed successfully!");
}
