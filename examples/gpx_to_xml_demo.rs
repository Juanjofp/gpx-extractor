use gpx_extractor::{Gpx, Point, Track, TrackSegment, Waypoint};

fn main() {
    println!("🗺️  Demostración: Conversión GPX a XML\n");

    // Crear un GPX programáticamente
    let mut gpx = Gpx::new();

    // Crear un track con varios puntos
    let mut track = Track::with_name("Ruta de Ejemplo".to_string());
    let segment = TrackSegment::with_points(vec![
        Point::with_elevation(40.7128, -74.0060, 10.0), // Nueva York
        Point::with_elevation(40.7589, -73.9851, 15.0), // Central Park
        Point::with_elevation(40.7831, -73.9712, 20.0), // Upper East Side
    ]);
    track.add_segment(segment);
    gpx.add_track(track);

    // Agregar algunos waypoints
    gpx.add_waypoint(Waypoint::with_name(
        40.7128,
        -74.0060,
        "Punto de Inicio".to_string(),
    ));
    gpx.add_waypoint(Waypoint::with_name(
        40.7831,
        -73.9712,
        "Punto Final".to_string(),
    ));

    // Mostrar estadísticas
    let stats = gpx.statistics();
    println!("📊 Estadísticas del GPX:");
    println!("   - Tracks: {}", stats.total_tracks);
    println!("   - Waypoints: {}", stats.total_waypoints);
    println!("   - Puntos: {}", stats.total_points);
    println!("   - Distancia: {:.2} km", stats.total_distance_km);
    if let Some((min, max)) = stats.elevation_range {
        println!("   - Elevación: {:.1}m - {:.1}m", min, max);
    }

    println!("\n🔄 Conversión a XML:");

    // Método 1: usando to_xml()
    let xml_string = gpx.to_xml();
    println!("✅ Método 1 - to_xml(): {} caracteres", xml_string.len());

    // Método 2: usando Display trait
    let display_string = format!("{}", gpx);
    println!("✅ Método 2 - Display: {} caracteres", display_string.len());

    // Método 3: usando Into<String>
    let into_string: String = gpx.clone().into();
    println!(
        "✅ Método 3 - Into<String>: {} caracteres",
        into_string.len()
    );

    // Mostrar el XML generado (formateado para legibilidad)
    println!("\n📄 XML Generado:");
    println!("{}", pretty_format_xml(&xml_string));

    // Guardar en archivo
    let filename = "/tmp/ejemplo_gpx.gpx";
    match gpx.save_to_file(filename) {
        Ok(_) => println!("\n💾 GPX guardado en: {}", filename),
        Err(e) => println!("\n❌ Error guardando archivo: {}", e),
    }

    // Test de roundtrip
    println!("\n🔄 Test de Roundtrip:");
    let reparsed_gpx = Gpx::try_from_str(&xml_string).expect("Error en roundtrip parsing");
    let reparsed_stats = reparsed_gpx.statistics();

    println!(
        "   Original:  {} tracks, {} waypoints, {} puntos",
        stats.total_tracks, stats.total_waypoints, stats.total_points
    );
    println!(
        "   Reparsed:  {} tracks, {} waypoints, {} puntos",
        reparsed_stats.total_tracks, reparsed_stats.total_waypoints, reparsed_stats.total_points
    );

    if stats.total_tracks == reparsed_stats.total_tracks
        && stats.total_waypoints == reparsed_stats.total_waypoints
        && stats.total_points == reparsed_stats.total_points
    {
        println!("   ✅ Roundtrip exitoso!");
    } else {
        println!("   ❌ Error en roundtrip");
    }
}

// Función auxiliar para formatear XML de manera más legible
fn pretty_format_xml(xml: &str) -> String {
    xml.replace("><", ">\n<")
        .replace("<gpx", "\n<gpx")
        .replace("<trk", "\n  <trk")
        .replace("<trkseg", "\n    <trkseg")
        .replace("<trkpt", "\n      <trkpt")
        .replace("<wpt", "\n  <wpt")
        .replace("<name", "\n    <name")
        .replace("<ele", "\n        <ele")
}
