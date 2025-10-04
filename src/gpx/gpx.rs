use crate::gpx::{point::Point, track::Track, waypoint::Waypoint};
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GpxRoot {
    #[serde(rename = "trk", default)]
    pub tracks: Vec<Track>,
    #[serde(rename = "wpt", default)]
    pub waypoints: Vec<Waypoint>,
}

#[derive(Debug)]
pub struct Gpx {
    pub tracks: Vec<Track>,
    pub waypoints: Vec<Waypoint>,
}

impl Gpx {
    /// Crea un nuevo GPX vacío
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            waypoints: Vec::new(),
        }
    }

    /// Obtiene todos los puntos de todos los tracks
    pub fn get_all_points(&self) -> Vec<&Point> {
        self.tracks
            .iter()
            .flat_map(|track| track.get_all_points())
            .collect()
    }

    /// Calcula la distancia total aproximada en kilómetros
    pub fn total_distance_km(&self) -> f64 {
        self.tracks
            .iter()
            .map(|track| track.total_distance_km())
            .sum()
    }

    /// Obtiene la elevación mínima y máxima de todos los tracks
    pub fn elevation_range(&self) -> Option<(f64, f64)> {
        let elevations: Vec<f64> = self
            .get_all_points()
            .iter()
            .filter_map(|p| p.elevation)
            .collect();

        if elevations.is_empty() {
            return None;
        }

        Some((
            elevations.iter().fold(f64::INFINITY, |acc, &x| acc.min(x)),
            elevations
                .iter()
                .fold(f64::NEG_INFINITY, |acc, &x| acc.max(x)),
        ))
    }

    /// Cuenta el total de puntos en todos los tracks
    pub fn total_points(&self) -> usize {
        self.tracks.iter().map(|track| track.total_points()).sum()
    }

    /// Cuenta el total de segmentos en todos los tracks
    pub fn total_segments(&self) -> usize {
        self.tracks.iter().map(|track| track.segments.len()).sum()
    }

    /// Obtiene estadísticas completas del GPX
    pub fn statistics(&self) -> GpxStatistics {
        GpxStatistics {
            total_tracks: self.tracks.len(),
            total_waypoints: self.waypoints.len(),
            total_segments: self.total_segments(),
            total_points: self.total_points(),
            total_distance_km: self.total_distance_km(),
            elevation_range: self.elevation_range(),
        }
    }

    /// Agrega un track al GPX
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Agrega un waypoint al GPX
    pub fn add_waypoint(&mut self, waypoint: Waypoint) {
        self.waypoints.push(waypoint);
    }

    /// Verifica si el GPX está vacío
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty() && self.waypoints.is_empty()
    }

    /// Obtiene los nombres de todos los tracks
    pub fn track_names(&self) -> Vec<String> {
        self.tracks
            .iter()
            .map(|track| track.display_name())
            .collect()
    }

    /// Obtiene los nombres de todos los waypoints
    pub fn waypoint_names(&self) -> Vec<String> {
        self.waypoints
            .iter()
            .map(|waypoint| waypoint.display_name())
            .collect()
    }
}

impl Default for Gpx {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for Gpx {
    fn from(s: &str) -> Self {
        match from_str::<GpxRoot>(s) {
            Ok(gpx_root) => Gpx {
                tracks: gpx_root.tracks,
                waypoints: gpx_root.waypoints,
            },
            Err(e) => {
                eprintln!("Error parsing GPX: {}", e);
                Gpx::new()
            }
        }
    }
}

/// Estadísticas completas de un archivo GPX
#[derive(Debug, Clone)]
pub struct GpxStatistics {
    pub total_tracks: usize,
    pub total_waypoints: usize,
    pub total_segments: usize,
    pub total_points: usize,
    pub total_distance_km: f64,
    pub elevation_range: Option<(f64, f64)>,
}

impl GpxStatistics {
    /// Calcula la ganancia de elevación
    pub fn elevation_gain(&self) -> Option<f64> {
        self.elevation_range.map(|(min, max)| max - min)
    }

    /// Obtiene una descripción legible de las estadísticas
    pub fn summary(&self) -> String {
        let mut summary = format!(
            "GPX Statistics:\n\
             - Tracks: {}\n\
             - Waypoints: {}\n\
             - Segments: {}\n\
             - Points: {}\n\
             - Distance: {:.2} km",
            self.total_tracks,
            self.total_waypoints,
            self.total_segments,
            self.total_points,
            self.total_distance_km
        );

        if let Some((min_ele, max_ele)) = self.elevation_range {
            summary.push_str(&format!(
                "\n- Elevation: {:.1}m - {:.1}m (gain: {:.1}m)",
                min_ele,
                max_ele,
                max_ele - min_ele
            ));
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpx::{
        point::Point,
        track::{Track, TrackSegment},
    };

    #[test]
    fn test_gpx_new() {
        let gpx = Gpx::new();
        assert!(gpx.tracks.is_empty());
        assert!(gpx.waypoints.is_empty());
        assert!(gpx.is_empty());
        assert_eq!(gpx.total_points(), 0);
        assert_eq!(gpx.total_distance_km(), 0.0);
    }

    #[test]
    fn test_gpx_from_empty_xml() {
        let gpx: Gpx = Gpx::from("<gpx></gpx>");
        assert!(gpx.tracks.is_empty());
        assert!(gpx.waypoints.is_empty());
    }

    #[test]
    fn test_gpx_with_track() {
        let xml = r#"
        <gpx>
            <trk>
                <name>Test Track</name>
                <trkseg>
                    <trkpt lat="40.7128" lon="-74.0060">
                        <ele>10.0</ele>
                    </trkpt>
                    <trkpt lat="40.7589" lon="-73.9851">
                        <ele>15.0</ele>
                    </trkpt>
                </trkseg>
            </trk>
        </gpx>"#;

        let gpx: Gpx = Gpx::from(xml);
        assert_eq!(gpx.tracks.len(), 1);
        assert_eq!(gpx.tracks[0].segments.len(), 1);
        assert_eq!(gpx.tracks[0].segments[0].points.len(), 2);
        assert_eq!(gpx.total_points(), 2);
        assert!(gpx.total_distance_km() > 0.0);
    }

    #[test]
    fn test_gpx_add_track() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());
        let segment = TrackSegment::with_points(vec![
            Point::new(40.7128, -74.0060),
            Point::new(40.7589, -73.9851),
        ]);
        track.add_segment(segment);

        gpx.add_track(track);
        assert_eq!(gpx.tracks.len(), 1);
        assert_eq!(gpx.total_points(), 2);
        assert!(!gpx.is_empty());
    }

    #[test]
    fn test_gpx_statistics() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());
        let segment = TrackSegment::with_points(vec![
            Point::with_elevation(40.7128, -74.0060, 10.0),
            Point::with_elevation(40.7589, -73.9851, 20.0),
        ]);
        track.add_segment(segment);
        gpx.add_track(track);

        let stats = gpx.statistics();
        assert_eq!(stats.total_tracks, 1);
        assert_eq!(stats.total_waypoints, 0);
        assert_eq!(stats.total_segments, 1);
        assert_eq!(stats.total_points, 2);
        assert!(stats.total_distance_km > 0.0);
        assert_eq!(stats.elevation_range, Some((10.0, 20.0)));
        assert_eq!(stats.elevation_gain(), Some(10.0));
    }

    #[test]
    fn test_gpx_track_names() {
        let mut gpx = Gpx::new();
        gpx.add_track(Track::with_name("Track 1".to_string()));
        gpx.add_track(Track::new()); // Sin nombre

        let names = gpx.track_names();
        assert_eq!(names.len(), 2);
        assert_eq!(names[0], "Track 1");
        assert_eq!(names[1], "Unnamed Track");
    }

    #[test]
    fn test_statistics_summary() {
        let stats = GpxStatistics {
            total_tracks: 2,
            total_waypoints: 3,
            total_segments: 4,
            total_points: 1000,
            total_distance_km: 25.5,
            elevation_range: Some((100.0, 300.0)),
        };

        let summary = stats.summary();
        assert!(summary.contains("Tracks: 2"));
        assert!(summary.contains("Waypoints: 3"));
        assert!(summary.contains("Distance: 25.50 km"));
        assert!(summary.contains("Elevation: 100.0m - 300.0m"));
        assert!(summary.contains("gain: 200.0m"));
    }
}
