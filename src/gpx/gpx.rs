use crate::gpx::{point::Point, track::Track, waypoint::Waypoint};
use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "gpx")]
pub struct GpxRoot {
    #[serde(rename = "@version", default = "default_version")]
    pub version: String,
    #[serde(rename = "@creator", default = "default_creator")]
    pub creator: String,
    #[serde(rename = "trk", default)]
    pub tracks: Vec<Track>,
    #[serde(rename = "wpt", default)]
    pub waypoints: Vec<Waypoint>,
}

fn default_version() -> String {
    "1.1".to_string()
}

fn default_creator() -> String {
    "gpx-extractor".to_string()
}

#[derive(Debug, Clone)]
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

    /// Convierte el GPX a string XML
    pub fn to_xml(&self) -> String {
        let gpx_root = GpxRoot {
            version: default_version(),
            creator: default_creator(),
            tracks: self.tracks.clone(),
            waypoints: self.waypoints.clone(),
        };

        match to_string(&gpx_root) {
            Ok(xml) => format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", xml),
            Err(e) => {
                eprintln!("Error serializing GPX to XML: {}", e);
                String::new()
            }
        }
    }

    /// Guarda el GPX en un archivo
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        use std::fs;
        fs::write(path, self.to_xml())
    }
}

impl Default for Gpx {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Gpx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_xml())
    }
}

impl Into<String> for Gpx {
    fn into(self) -> String {
        self.to_xml()
    }
}

impl Into<String> for &Gpx {
    fn into(self) -> String {
        self.to_xml()
    }
}

impl Gpx {
    /// Intenta crear un GPX desde un string XML, devolviendo un Result
    pub fn try_from_str(s: &str) -> Result<Self, quick_xml::DeError> {
        let gpx_root = from_str::<GpxRoot>(s)?;
        Ok(Gpx {
            tracks: gpx_root.tracks,
            waypoints: gpx_root.waypoints,
        })
    }
}

impl TryFrom<&str> for Gpx {
    type Error = quick_xml::DeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from_str(s)
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
        let gpx = Gpx::try_from_str("<gpx></gpx>").unwrap();
        assert!(gpx.tracks.is_empty());
        assert!(gpx.waypoints.is_empty());
    }

    #[test]
    fn test_gpx_try_from_str_success() {
        let xml = r#"<gpx><trk><name>Test</name></trk></gpx>"#;
        let result = Gpx::try_from_str(xml);
        assert!(result.is_ok());
        let gpx = result.unwrap();
        assert_eq!(gpx.tracks.len(), 1);
    }

    #[test]
    fn test_gpx_try_from_str_error() {
        let invalid_xml = "not valid xml at all";
        let result = Gpx::try_from_str(invalid_xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_gpx_error_handling_preserves_information() {
        let malformed_xml = "<gpx><trk><invalid></trk></gpx>";
        let result = Gpx::try_from_str(malformed_xml);

        match result {
            Ok(_) => panic!("Expected error for malformed XML"),
            Err(e) => {
                // Verificamos que el error contiene información útil
                let error_string = format!("{}", e);
                assert!(!error_string.is_empty());
            }
        }
    }

    #[test]
    fn test_gpx_try_from_trait_success() {
        use std::convert::TryFrom;
        let xml = r#"<gpx><trk><name>TryFrom Test</name></trk></gpx>"#;
        let gpx = Gpx::try_from(xml).unwrap();
        assert_eq!(gpx.tracks.len(), 1);
        assert_eq!(gpx.tracks[0].name.as_ref().unwrap(), "TryFrom Test");
    }

    #[test]
    fn test_gpx_try_from_trait_error() {
        use std::convert::TryFrom;
        let invalid_xml = "not valid xml at all";
        let result = Gpx::try_from(invalid_xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_gpx_try_from_trait_vs_try_from_str() {
        use std::convert::TryFrom;
        let xml = r#"<gpx><trk><name>Comparison Test</name></trk></gpx>"#;

        // Ambos métodos deben dar el mismo resultado
        let gpx1 = Gpx::try_from_str(xml).unwrap();
        let gpx2 = Gpx::try_from(xml).unwrap();

        assert_eq!(gpx1.tracks.len(), gpx2.tracks.len());
        assert_eq!(gpx1.tracks[0].name, gpx2.tracks[0].name);
    }
    #[test]
    fn test_gpx_from_invalid_xml_returns_error() {
        // Ahora debe devolver un error en lugar de un GPX vacío
        let result = Gpx::try_from_str("invalid xml");
        assert!(result.is_err());
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

        let gpx = Gpx::try_from_str(xml).unwrap();
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

    #[test]
    fn test_gpx_to_xml() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());
        let segment = TrackSegment::with_points(vec![
            Point::new(40.7128, -74.0060),
            Point::new(40.7589, -73.9851),
        ]);
        track.add_segment(segment);
        gpx.add_track(track);

        let xml_output = gpx.to_xml();

        assert!(xml_output.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml_output.contains("<gpx"));
        assert!(xml_output.contains("version=\"1.1\""));
        assert!(xml_output.contains("creator=\"gpx-extractor\""));
        assert!(xml_output.contains("Test Track"));
        assert!(xml_output.contains("40.7128"));
        assert!(xml_output.contains("-74.006"));
    }

    #[test]
    fn test_gpx_display_trait() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Display Test".to_string());
        let segment = TrackSegment::with_points(vec![Point::new(1.0, 2.0)]);
        track.add_segment(segment);
        gpx.add_track(track);

        let display_output = format!("{}", gpx);
        assert!(display_output.contains("Display Test"));
        assert!(display_output.contains("<?xml"));
    }

    #[test]
    fn test_gpx_into_string() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Into Test".to_string());
        let segment = TrackSegment::with_points(vec![Point::new(3.0, 4.0)]);
        track.add_segment(segment);
        gpx.add_track(track);

        let string_output: String = gpx.into();
        assert!(string_output.contains("Into Test"));
        assert!(string_output.contains("<?xml"));
    }

    #[test]
    fn test_gpx_roundtrip() {
        let original_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <gpx version="1.1" creator="test">
            <trk>
                <name>Test Track</name>
                <trkseg>
                    <trkpt lat="40.7128" lon="-74.0060">
                        <ele>10.5</ele>
                    </trkpt>
                </trkseg>
            </trk>
            <wpt lat="40.7589" lon="-73.9851">
                <name>Test Waypoint</name>
                <ele>15.0</ele>
            </wpt>
        </gpx>"#;

        let gpx = Gpx::try_from_str(original_xml).unwrap();
        let serialized_xml = gpx.to_xml();
        let reparsed_gpx = Gpx::try_from_str(&serialized_xml).unwrap();

        assert_eq!(gpx.tracks.len(), reparsed_gpx.tracks.len());
        assert_eq!(gpx.waypoints.len(), reparsed_gpx.waypoints.len());
        assert_eq!(gpx.total_points(), reparsed_gpx.total_points());
    }

    #[test]
    fn test_gpx_save_to_file() {
        use std::fs;
        use std::path::Path;

        let mut gpx = Gpx::new();
        let mut track = Track::with_name("File Test".to_string());
        let segment = TrackSegment::with_points(vec![Point::new(5.0, 6.0)]);
        track.add_segment(segment);
        gpx.add_track(track);

        let test_file = "/tmp/test_gpx_output.gpx";

        // Guardar archivo
        let result = gpx.save_to_file(test_file);
        assert!(result.is_ok());

        // Verificar que el archivo existe y tiene contenido
        assert!(Path::new(test_file).exists());
        let file_content = fs::read_to_string(test_file).unwrap();
        assert!(file_content.contains("File Test"));
        assert!(file_content.contains("<?xml"));

        // Limpiar archivo de prueba
        let _ = fs::remove_file(test_file);
    }
}
