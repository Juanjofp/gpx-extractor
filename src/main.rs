use gpx_extractor::Gpx;

fn main() {
    // Read file content from command line argument
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <gpx_directory>", args[0]);
        std::process::exit(1);
    }

    let gpx_directory = &args[1];

    println!("📍 Reading GPX files from directory: {}", gpx_directory);

    // for entry in std::fs::read_dir(gpx_directory).unwrap() {
    //     let entry = entry.unwrap();
    //     let path = entry.path();

    //     if path.extension().map(|e| e == "gpx").unwrap_or(false) {
    //         print_gpx_info(path.to_str().unwrap());
    //     }
    // }

    let files = std::fs::read_dir(gpx_directory)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|path| {
            path.as_ref()
                .map(|p| p.extension().map(|e| e == "gpx").unwrap_or(false))
                .unwrap_or(false)
        })
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
}

fn load_gpx_file(gpx_file_name: &str) -> Result<Gpx, Box<dyn std::error::Error>> {
    use std::convert::TryFrom;
    let gpx_content = std::fs::read_to_string(gpx_file_name)?;
    let gpx = Gpx::try_from(gpx_content.as_str())?;
    Ok(gpx)
}

fn print_gpx_info(gpx: &Gpx) {
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
