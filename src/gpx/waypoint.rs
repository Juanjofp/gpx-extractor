use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A waypoint representing a point of interest
///
/// Waypoints mark specific locations such as destinations, landmarks,
/// or important points along a route.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Waypoint {
    /// Latitude in decimal degrees (WGS84)
    #[serde(rename = "@lat")]
    pub lat: f64,
    /// Longitude in decimal degrees (WGS84)
    #[serde(rename = "@lon")]
    pub lon: f64,
    /// Optional name describing the waypoint
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Elevation in meters above sea level
    #[serde(rename = "ele", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
    /// Timestamp of when the waypoint was created
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
}

impl Waypoint {
    /// Crea un nuevo waypoint con coordenadas básicas
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            lat,
            lon,
            name: None,
            elevation: None,
            time: None,
        }
    }

    /// Crea un nuevo waypoint con nombre
    pub fn with_name(lat: f64, lon: f64, name: String) -> Self {
        Self {
            lat,
            lon,
            name: Some(name),
            elevation: None,
            time: None,
        }
    }

    /// Crea un nuevo waypoint completo
    pub fn with_details(
        lat: f64,
        lon: f64,
        name: Option<String>,
        elevation: Option<f64>,
        time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            lat,
            lon,
            name,
            elevation,
            time,
        }
    }

    /// Obtiene el nombre del waypoint o un nombre por defecto
    pub fn display_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| format!("Waypoint ({:.4}, {:.4})", self.lat, self.lon))
    }

    /// Verifica si el waypoint tiene elevación
    pub fn has_elevation(&self) -> bool {
        self.elevation.is_some()
    }

    /// Verifica si el waypoint tiene timestamp
    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    /// Obtiene una descripción completa del waypoint
    pub fn description(&self) -> String {
        let mut desc = format!(
            "{} at ({:.6}, {:.6})",
            self.display_name(),
            self.lat,
            self.lon
        );

        if let Some(elevation) = self.elevation {
            use std::fmt::Write;
            let _ = write!(&mut desc, ", elevation: {elevation:.1}m");
        }

        if let Some(time) = self.time {
            use std::fmt::Write;
            let _ = write!(
                &mut desc,
                ", time: {}",
                time.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }

        desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_waypoint_new() {
        let waypoint = Waypoint::new(40.7128, -74.0060);
        assert_eq!(waypoint.lat, 40.7128);
        assert_eq!(waypoint.lon, -74.0060);
        assert!(waypoint.name.is_none());
        assert!(waypoint.elevation.is_none());
        assert!(waypoint.time.is_none());
        assert!(!waypoint.has_elevation());
        assert!(!waypoint.has_time());
    }

    #[test]
    fn test_waypoint_with_name() {
        let waypoint = Waypoint::with_name(40.7128, -74.0060, "New York City".to_string());
        assert_eq!(waypoint.lat, 40.7128);
        assert_eq!(waypoint.lon, -74.0060);
        assert_eq!(waypoint.name, Some("New York City".to_string()));
        assert_eq!(waypoint.display_name(), "New York City");
    }

    #[test]
    fn test_waypoint_display_name_default() {
        let waypoint = Waypoint::new(40.7128, -74.0060);
        assert_eq!(waypoint.display_name(), "Waypoint (40.7128, -74.0060)");
    }

    #[test]
    fn test_waypoint_with_details() {
        let time = Utc.with_ymd_and_hms(2024, 6, 9, 10, 30, 0).unwrap();
        let waypoint = Waypoint::with_details(
            40.7128,
            -74.0060,
            Some("NYC".to_string()),
            Some(10.5),
            Some(time),
        );

        assert_eq!(waypoint.lat, 40.7128);
        assert_eq!(waypoint.lon, -74.0060);
        assert_eq!(waypoint.name, Some("NYC".to_string()));
        assert_eq!(waypoint.elevation, Some(10.5));
        assert_eq!(waypoint.time, Some(time));
        assert!(waypoint.has_elevation());
        assert!(waypoint.has_time());
    }

    #[test]
    fn test_waypoint_description() {
        let time = Utc.with_ymd_and_hms(2024, 6, 9, 10, 30, 0).unwrap();
        let waypoint = Waypoint::with_details(
            40.712800,
            -74.006000,
            Some("NYC".to_string()),
            Some(10.5),
            Some(time),
        );

        let desc = waypoint.description();
        assert!(desc.contains("NYC"));
        assert!(desc.contains("40.712800"));
        assert!(desc.contains("-74.006000"));
        assert!(desc.contains("elevation: 10.5m"));
        assert!(desc.contains("2024-06-09 10:30:00 UTC"));
    }

    #[test]
    fn test_waypoint_description_minimal() {
        let waypoint = Waypoint::new(40.7128, -74.0060);
        let desc = waypoint.description();
        assert!(desc.contains("Waypoint (40.7128, -74.0060)"));
        assert!(desc.contains("40.712800"));
        assert!(desc.contains("-74.006000"));
        assert!(!desc.contains("elevation"));
        assert!(!desc.contains("time"));
    }

    #[test]
    fn test_waypoint_has_flags() {
        let mut waypoint = Waypoint::new(40.7128, -74.0060);
        assert!(!waypoint.has_elevation());
        assert!(!waypoint.has_time());

        waypoint.elevation = Some(100.0);
        assert!(waypoint.has_elevation());
        assert!(!waypoint.has_time());

        waypoint.time = Some(Utc::now());
        assert!(waypoint.has_elevation());
        assert!(waypoint.has_time());
    }
}
