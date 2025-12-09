use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A geographic point with latitude, longitude, and optional elevation and timestamp
///
/// Represents a single point in a GPS track or a waypoint location.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Point {
    /// Latitude in decimal degrees (WGS84)
    #[serde(rename = "@lat")]
    pub lat: f64,
    /// Longitude in decimal degrees (WGS84)
    #[serde(rename = "@lon")]
    pub lon: f64,
    /// Elevation in meters above sea level
    #[serde(rename = "ele", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
    /// Timestamp of when the point was recorded
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
}

impl Point {
    /// Crea un nuevo punto con coordenadas básicas
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            lat,
            lon,
            elevation: None,
            time: None,
        }
    }

    /// Crea un nuevo punto con elevación
    pub fn with_elevation(lat: f64, lon: f64, elevation: f64) -> Self {
        Self {
            lat,
            lon,
            elevation: Some(elevation),
            time: None,
        }
    }

    /// Crea un nuevo punto completo con tiempo
    pub fn with_time(lat: f64, lon: f64, elevation: Option<f64>, time: DateTime<Utc>) -> Self {
        Self {
            lat,
            lon,
            elevation,
            time: Some(time),
        }
    }
}

/// Calcula la distancia Haversine entre dos puntos en kilómetros
pub fn haversine_distance(p1: &Point, p2: &Point) -> f64 {
    const R: f64 = 6371.0; // Radio de la Tierra en km

    let lat1_rad = p1.lat.to_radians();
    let lat2_rad = p2.lat.to_radians();
    let delta_lat = (p2.lat - p1.lat).to_radians();
    let delta_lon = (p2.lon - p1.lon).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    R * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let point = Point::new(40.7128, -74.0060);
        assert_eq!(point.lat, 40.7128);
        assert_eq!(point.lon, -74.0060);
        assert!(point.elevation.is_none());
        assert!(point.time.is_none());
    }

    #[test]
    fn test_point_with_elevation() {
        let point = Point::with_elevation(40.7128, -74.0060, 10.5);
        assert_eq!(point.lat, 40.7128);
        assert_eq!(point.lon, -74.0060);
        assert_eq!(point.elevation, Some(10.5));
        assert!(point.time.is_none());
    }

    #[test]
    fn test_haversine_distance() {
        let p1 = Point::new(40.7128, -74.0060); // NYC
        let p2 = Point::new(40.7589, -73.9851); // Central Park

        let distance = haversine_distance(&p1, &p2);
        assert!(distance > 0.0);
        assert!(distance < 10.0); // Distancia aproximada en NYC
        assert!((distance - 5.34).abs() < 1.0); // Aproximadamente 5.34 km
    }

    #[test]
    fn test_haversine_distance_same_point() {
        let p1 = Point::new(40.7128, -74.0060);
        let p2 = Point::new(40.7128, -74.0060);

        let distance = haversine_distance(&p1, &p2);
        assert!((distance - 0.0).abs() < 0.001); // Debe ser prácticamente 0
    }

    #[test]
    fn test_haversine_distance_long_distance() {
        let madrid = Point::new(40.4168, -3.7038);
        let new_york = Point::new(40.7128, -74.0060);

        let distance = haversine_distance(&madrid, &new_york);
        assert!(distance > 5000.0); // Más de 5000 km
        assert!(distance < 7000.0); // Menos de 7000 km
    }
}
