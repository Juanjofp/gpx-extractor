use gpx_extractor::Gpx;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file content from command line argument
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <gpx_directory>", args[0]);
        std::process::exit(1);
    }

    let gpx_directory = &args[1];

    println!("📍 Reading GPX files from directory: {}", gpx_directory);

    let files: Vec<PathBuf> = std::fs::read_dir(gpx_directory)?
        .filter_map(|entry| match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                if path.extension().map_or(false, |ext| ext == "gpx") {
                    Some(Ok(path))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("Found {} GPX files", files.len());

    // Obtener vector de objetos Gpx
    let gpx_items: Vec<Gpx> = files
        .iter()
        .filter_map(
            |file_path| match load_gpx_file(file_path.to_str().unwrap()) {
                Ok(gpx) => Some(gpx),

                Err(e) => {
                    eprintln!("Error loading {}: {}", file_path.display(), e);
                    None
                }
            },
        )
        .collect();

    println!("Successfully loaded {} GPX files", gpx_items.len());

    // // Print info for each GPX file
    // for (i, gpx) in gpx_items.iter().enumerate() {
    //     println!("\n=== GPX File {} ===", i + 1);
    //     print_gpx_info(gpx);
    // }
    // Print info for each GPX file
    gpx_items.iter().enumerate().for_each(|(i, gpx)| {
        println!("\n=== GPX File {} ===", i + 1);
        print_gpx_info(gpx);
    });

    // Sumar todos los km de todos los archivos GPX
    let total_distance: f64 = gpx_items.iter().map(|gpx| gpx.total_distance_km()).sum();

    println!(
        "\n📏 Total distance across all GPX files: {:.2} km",
        total_distance
    );

    Ok(())
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

    if let Some(date) = gpx.date() {
        println!("📅 Date: {}", date);
    }
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
