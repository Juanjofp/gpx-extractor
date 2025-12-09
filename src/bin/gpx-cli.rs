use clap::Parser;
use colored::Colorize;
use gpx_extractor::Gpx;
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gpx-cli")]
#[command(about = "GPX file analyzer and processor", long_about = None)]
#[command(version)]
struct Cli {
    /// GPX file or directory to process
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Show detailed statistics
    #[arg(short, long)]
    verbose: bool,

    /// Sort GPX files by date
    #[arg(short, long)]
    sort: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.path.is_dir() {
        process_directory(&cli.path, &cli)?;
    } else {
        process_file(&cli.path, &cli)?;
    }

    Ok(())
}

fn process_file(path: &PathBuf, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let gpx = Gpx::try_from(content.as_str())?;

    println!("{}", format!("📄 {}", path.display()).cyan());
    print_gpx_info(&gpx, cli.verbose);

    Ok(())
}

fn process_directory(path: &PathBuf, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        format!("📍 Reading GPX files from directory: {}", path.display()).cyan()
    );

    let files: Vec<PathBuf> = std::fs::read_dir(path)?
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "gpx") {
                    Some(Ok(path))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("{}", format!("Found {} GPX files", files.len()).green());

    // Load all GPX files
    let mut gpx_items: Vec<Gpx> = files
        .iter()
        .filter_map(|file| match load_gpx_file(file.to_str().unwrap()) {
            Ok(gpx) => Some(gpx),
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("⚠️  Error loading {}: {}", file.display(), e).yellow()
                );
                None
            }
        })
        .collect();

    // Sort by date if requested
    if cli.sort {
        gpx_items.sort_by(|a, b| match (a.date(), b.date()) {
            (Some(date_a), Some(date_b)) => date_a.cmp(date_b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });
    }

    println!(
        "{}",
        format!("Successfully loaded {} GPX files", gpx_items.len()).green()
    );

    // Print info for each GPX file
    gpx_items.iter().enumerate().for_each(|(i, gpx)| {
        println!("\n{}", format!("═══ GPX File #{} ═══", i + 1).bold());
        print_gpx_info(gpx, cli.verbose);
    });

    // Calculate total distance
    let total_distance: f64 = gpx_items.iter().map(|gpx| gpx.total_distance_km()).sum();

    println!(
        "\n{}",
        format!(
            "📏 Total distance across all files: {:.2} km",
            total_distance
        )
        .green()
        .bold()
    );

    Ok(())
}

fn load_gpx_file(gpx_file_name: &str) -> Result<Gpx, Box<dyn std::error::Error>> {
    let gpx_content = std::fs::read_to_string(gpx_file_name)?;
    let gpx = Gpx::try_from(gpx_content.as_str())?;
    Ok(gpx)
}

fn print_gpx_info(gpx: &Gpx, verbose: bool) {
    if !verbose {
        // Compact format
        println!(
            "  📊 Tracks: {} | 📍 Waypoints: {} | 🔢 Points: {} | 📏 Distance: {:.2} km",
            gpx.tracks.len(),
            gpx.waypoints.len(),
            gpx.total_points(),
            gpx.total_distance_km()
        );
        return;
    }

    // Detailed format
    println!("\n🗂️  GPX Analysis:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if let Some(date) = gpx.date() {
        println!("{}", format!("📅 Date: {}", date).cyan());
    }
    println!("📊 Tracks: {}", gpx.tracks.len());
    println!("📍 Waypoints: {}", gpx.waypoints.len());
    println!("🔢 Total points: {}", gpx.total_points());

    if !gpx.tracks.is_empty() {
        println!("\n{}", "Track Details:".bold());
        for (i, track) in gpx.tracks.iter().enumerate() {
            println!(
                "  Track #{}: {} ({} segments, {} points)",
                i + 1,
                track.name.as_deref().unwrap_or("Unnamed"),
                track.segments.len(),
                track.total_points()
            );
        }
    }

    let distance = gpx.total_distance_km();

    if distance > 0.0 {
        println!(
            "{}",
            format!("\n📏 Total distance: {:.2} km", distance).green()
        );
    }

    if let Some((min_ele, max_ele)) = gpx.elevation_range() {
        println!("⛰️  Elevation range: {:.1}m - {:.1}m", min_ele, max_ele);
    }

    if let Some(gain) = gpx.total_elevation_gain() {
        println!("📈 Total elevation gain: {:.1}m", gain);
    }

    if let Some(loss) = gpx.total_elevation_loss() {
        println!("📉 Total elevation loss: {:.1}m", loss);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}", "✅ GPX file processed successfully!".green());
}
