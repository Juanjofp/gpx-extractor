use chrono::TimeZone;
use gpx_extractor::{Gpx, Point, Track, TrackSegment};

fn main() {
    // Crear un GPX con puntos que tienen timestamps
    let mut gpx = Gpx::new();
    let mut track = Track::with_name("Ruta de ejemplo".to_string());

    // Simular una ruta de 2 horas con varios puntos
    let start_time = chrono::Utc.with_ymd_and_hms(2024, 7, 11, 10, 0, 0).unwrap();

    let points = vec![
        Point::with_time(40.7128, -74.0060, Some(10.0), start_time),
        Point::with_time(
            40.7589,
            -73.9851,
            Some(15.0),
            start_time + chrono::Duration::minutes(30),
        ),
        Point::with_time(
            40.7829,
            -73.9654,
            Some(20.0),
            start_time + chrono::Duration::hours(1),
        ),
        Point::with_time(
            40.8050,
            -73.9654,
            Some(25.0),
            start_time + chrono::Duration::minutes(90),
        ),
        Point::with_time(
            40.8282,
            -73.9261,
            Some(30.0),
            start_time + chrono::Duration::hours(2),
        ),
    ];

    let segment = TrackSegment::with_points(points);
    track.add_segment(segment);
    gpx.add_track(track);

    // Calcular y mostrar estadísticas
    println!("=== Estadísticas de la ruta ===\n");

    // Duración
    if let Some(duration) = gpx.total_duration_seconds() {
        println!("Duración total: {} segundos", duration);
        if let Some(formatted) = gpx.total_duration_formatted() {
            println!("Duración formateada: {}", formatted);
        }
    } else {
        println!("No hay información de tiempo disponible");
    }

    // Distancia
    let distance = gpx.total_distance_km();
    println!("Distancia total: {:.2} km", distance);

    // Velocidad media
    if let Some(speed) = gpx.average_speed_kmh() {
        println!("Velocidad media: {:.2} km/h", speed);
    } else {
        println!("No se puede calcular la velocidad media");
    }

    // Elevación
    if let Some((min, max)) = gpx.elevation_range() {
        println!("\nRango de elevación: {:.1}m - {:.1}m", min, max);
    }

    if let Some(gain) = gpx.total_elevation_gain() {
        println!("Ganancia de elevación: {:.1}m", gain);
    }

    // Estadísticas completas
    println!("\n=== Estadísticas completas ===\n");
    let stats = gpx.statistics();
    println!("{}", stats.summary());

    // Ejemplo sin timestamps
    println!("\n=== Ejemplo sin timestamps ===\n");
    let mut gpx_no_time = Gpx::new();
    let mut track_no_time = Track::with_name("Ruta sin tiempo".to_string());
    let segment_no_time = TrackSegment::with_points(vec![
        Point::new(40.7128, -74.0060),
        Point::new(40.7589, -73.9851),
    ]);
    track_no_time.add_segment(segment_no_time);
    gpx_no_time.add_track(track_no_time);

    println!("Distancia: {:.2} km", gpx_no_time.total_distance_km());
    match gpx_no_time.total_duration_seconds() {
        Some(d) => println!("Duración: {} segundos", d),
        None => println!("Duración: No disponible (puntos sin timestamps)"),
    }
    match gpx_no_time.average_speed_kmh() {
        Some(s) => println!("Velocidad media: {:.2} km/h", s),
        None => println!("Velocidad media: No disponible"),
    }
}
