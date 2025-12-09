use gpx_extractor::Gpx;
use std::fs;

fn main() {
    // Leer el archivo GPX de prueba
    let xml_content = fs::read_to_string("test_with_metadata.gpx")
        .expect("No se pudo leer el archivo test_with_metadata.gpx");

    // Parsear el GPX
    let gpx = Gpx::try_from_str(&xml_content).expect("Error al parsear el GPX");

    println!("=== Análisis del archivo GPX ===\n");

    // Información básica
    println!("Tracks: {}", gpx.tracks.len());
    println!("Waypoints: {}", gpx.waypoints.len());
    println!("Puntos totales: {}", gpx.total_points());

    // Duración
    match gpx.total_duration_seconds() {
        Some(duration) => {
            println!("\n=== Duración ===");
            println!("Total en segundos: {}", duration);
            if let Some(formatted) = gpx.total_duration_formatted() {
                println!("Formateado: {}", formatted);
            }
        }
        None => println!("\nNo hay información de tiempo en este archivo GPX"),
    }

    // Distancia
    println!("\n=== Distancia ===");
    println!("Total: {:.2} km", gpx.total_distance_km());

    // Velocidad
    match gpx.average_speed_kmh() {
        Some(speed) => {
            println!("\n=== Velocidad ===");
            println!("Media: {:.2} km/h", speed);
        }
        None => println!("\nNo se puede calcular la velocidad media"),
    }

    // Elevación
    if let Some((min, max)) = gpx.elevation_range() {
        println!("\n=== Elevación ===");
        println!("Mínima: {:.1}m", min);
        println!("Máxima: {:.1}m", max);
        println!("Diferencia: {:.1}m", max - min);
    }

    if let Some(gain) = gpx.total_elevation_gain() {
        println!("Ganancia acumulada: {:.1}m", gain);
    }

    if let Some(loss) = gpx.total_elevation_loss() {
        println!("Pérdida acumulada: {:.1}m", loss);
    }

    // Resumen completo
    println!("\n{}", "=".repeat(50));
    println!("\n{}", gpx.statistics().summary());
}
