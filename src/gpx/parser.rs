use crate::gpx::{point::Point, track::Track, waypoint::Waypoint};
use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};
use std::fmt;

/// GPX metadata containing timestamp and other optional information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    /// Timestamp of when the GPX file was created
    #[serde(rename = "time")]
    pub time: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "gpx")]
pub struct GpxRoot {
    #[serde(rename = "@version", default = "default_version")]
    pub version: String,
    #[serde(rename = "@creator", default = "default_creator")]
    pub creator: String,
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
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

/// Main GPX structure containing tracks, waypoints, and metadata
///
/// This is the primary data structure for working with GPX files.
/// It can be parsed from XML using `TryFrom<&str>` or built programmatically.
///
/// # Examples
///
/// ```
/// use gpx_extractor::Gpx;
///
/// let mut gpx = Gpx::new();
/// // Add tracks, waypoints, etc.
/// ```
#[derive(Debug, Clone)]
pub struct Gpx {
    /// Collection of GPS tracks (recorded routes)
    pub tracks: Vec<Track>,
    /// Collection of waypoints (points of interest)
    pub waypoints: Vec<Waypoint>,
    /// Optional metadata (timestamp, etc.)
    pub metadata: Option<Metadata>,
}

impl Gpx {
    /// Crea un nuevo GPX vacío
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            waypoints: Vec::new(),
            metadata: None,
        }
    }

    /// Obtiene la fecha de la metadata si existe
    pub fn date(&self) -> Option<&str> {
        self.metadata.as_ref()?.time.as_deref()
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

    /// Calcula la elevación acumulada positiva (ascenso total)
    pub fn total_elevation_gain(&self) -> Option<f64> {
        let mut total_gain = 0.0;
        let mut has_elevation = false;

        for track in &self.tracks {
            for segment in &track.segments {
                for window in segment.points.windows(2) {
                    if let (Some(ele1), Some(ele2)) = (window[0].elevation, window[1].elevation) {
                        has_elevation = true;
                        let diff = ele2 - ele1;
                        if diff > 0.0 {
                            total_gain += diff;
                        }
                    }
                }
            }
        }

        if has_elevation {
            Some(total_gain)
        } else {
            None
        }
    }

    /// Calcula la elevación acumulada negativa (descenso total)
    pub fn total_elevation_loss(&self) -> Option<f64> {
        let mut total_loss = 0.0;
        let mut has_elevation = false;

        for track in &self.tracks {
            for segment in &track.segments {
                for window in segment.points.windows(2) {
                    if let (Some(ele1), Some(ele2)) = (window[0].elevation, window[1].elevation) {
                        has_elevation = true;
                        let diff = ele2 - ele1;
                        if diff < 0.0 {
                            total_loss += diff.abs();
                        }
                    }
                }
            }
        }

        if has_elevation {
            Some(total_loss)
        } else {
            None
        }
    }

    /// Cuenta el total de puntos en todos los tracks
    pub fn total_points(&self) -> usize {
        self.tracks.iter().map(|track| track.total_points()).sum()
    }

    /// Cuenta el total de segmentos en todos los tracks
    pub fn total_segments(&self) -> usize {
        self.tracks.iter().map(|track| track.segments.len()).sum()
    }

    /// Calcula la duración total de la ruta basándose en los timestamps de los puntos
    /// Devuelve la duración en segundos entre el primer y último punto con timestamp
    pub fn total_duration_seconds(&self) -> Option<i64> {
        let points = self.get_all_points();

        let times: Vec<chrono::DateTime<chrono::Utc>> = points
            .iter()
            .filter_map(|p| p.time.as_ref().copied())
            .collect();

        if times.is_empty() {
            return None;
        }

        let min_time = times.iter().min()?;
        let max_time = times.iter().max()?;

        Some((*max_time - *min_time).num_seconds())
    }

    /// Calcula la duración total en formato legible (horas:minutos:segundos)
    pub fn total_duration_formatted(&self) -> Option<String> {
        let total_seconds = self.total_duration_seconds()?;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        Some(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
    }

    /// Calcula la velocidad media en km/h si hay distancia y duración
    pub fn average_speed_kmh(&self) -> Option<f64> {
        let distance_km = self.total_distance_km();
        let duration_seconds = self.total_duration_seconds()?;

        if duration_seconds == 0 {
            return None;
        }

        let duration_hours = duration_seconds as f64 / 3600.0;
        Some(distance_km / duration_hours)
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
            elevation_gain: self.total_elevation_gain(),
            elevation_loss: self.total_elevation_loss(),
            duration_seconds: self.total_duration_seconds(),
            average_speed_kmh: self.average_speed_kmh(),
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
            metadata: self.metadata.clone(),
            tracks: self.tracks.clone(),
            waypoints: self.waypoints.clone(),
        };

        match to_string(&gpx_root) {
            Ok(xml) => format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{xml}"),
            Err(e) => {
                eprintln!("Error serializing GPX to XML: {e}");
                String::new()
            }
        }
    }

    /// Saves the GPX to a file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written
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

impl From<Gpx> for String {
    fn from(val: Gpx) -> Self {
        val.to_xml()
    }
}

impl From<&Gpx> for String {
    fn from(val: &Gpx) -> Self {
        val.to_xml()
    }
}

impl Gpx {
    /// Attempts to create a GPX from an XML string
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed into a valid GPX structure
    pub fn try_from_str(s: &str) -> Result<Self, quick_xml::DeError> {
        let gpx_root = from_str::<GpxRoot>(s)?;
        Ok(Gpx {
            tracks: gpx_root.tracks,
            waypoints: gpx_root.waypoints,
            metadata: gpx_root.metadata,
        })
    }
}

impl TryFrom<&str> for Gpx {
    type Error = quick_xml::DeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from_str(s)
    }
}

/// Complete statistics for a GPX file
///
/// Contains computed metrics including distances, elevations, and counts.
#[derive(Debug, Clone)]
pub struct GpxStatistics {
    /// Total number of tracks in the GPX
    pub total_tracks: usize,
    /// Total number of waypoints in the GPX
    pub total_waypoints: usize,
    /// Total number of track segments across all tracks
    pub total_segments: usize,
    /// Total number of points across all tracks
    pub total_points: usize,
    /// Total distance in kilometers
    pub total_distance_km: f64,
    /// Elevation range as (min, max) in meters, if available
    pub elevation_range: Option<(f64, f64)>,
    /// Total elevation gain in meters, if available
    pub elevation_gain: Option<f64>,
    /// Total elevation loss in meters, if available
    pub elevation_loss: Option<f64>,
    /// Total duration in seconds, if timestamps are available
    pub duration_seconds: Option<i64>,
    /// Average speed in km/h, if distance and duration are available
    pub average_speed_kmh: Option<f64>,
}

impl GpxStatistics {
    /// Calcula la ganancia de elevación (diferencia min-max)
    pub fn elevation_difference(&self) -> Option<f64> {
        self.elevation_range.map(|(min, max)| max - min)
    }

    /// Obtiene la duración en formato legible
    pub fn duration_formatted(&self) -> Option<String> {
        let total_seconds = self.duration_seconds?;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        Some(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
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

        if let Some(duration) = self.duration_formatted() {
            summary.push_str(&format!("\n- Duration: {}", duration));
        }

        if let Some(speed) = self.average_speed_kmh {
            summary.push_str(&format!("\n- Average speed: {:.2} km/h", speed));
        }

        if let Some((min_ele, max_ele)) = self.elevation_range {
            use std::fmt::Write;
            let _ = write!(
                &mut summary,
                "\n- Elevation range: {min_ele:.1}m - {max_ele:.1}m"
            );
        }

        if let Some(gain) = self.elevation_gain {
            use std::fmt::Write;
            let _ = write!(&mut summary, "\n- Elevation gain: {gain:.1}m");
        }

        if let Some(loss) = self.elevation_loss {
            use std::fmt::Write;
            let _ = write!(&mut summary, "\n- Elevation loss: {loss:.1}m");
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
    use chrono::TimeZone;

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
        assert_eq!(stats.elevation_difference(), Some(10.0));
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
            elevation_gain: Some(200.0),
            elevation_loss: Some(50.0),
            duration_seconds: Some(7200),
            average_speed_kmh: Some(12.75),
        };

        let summary = stats.summary();
        assert!(summary.contains("Tracks: 2"));
        assert!(summary.contains("Waypoints: 3"));
        assert!(summary.contains("Distance: 25.50 km"));
        assert!(summary.contains("Duration: 02:00:00"));
        assert!(summary.contains("Average speed: 12.75 km/h"));
        assert!(summary.contains("gain: 200.0m"));
        assert!(summary.contains("loss: 50.0m"));
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

    #[test]
    fn test_gpx_metadata_parsing() {
        let xml = r#"
        <gpx version="1.1" creator="StravaGPX">
            <metadata>
                <time>2024-07-11T17:16:43Z</time>
            </metadata>
            <trk>
                <name>Test Track</name>
            </trk>
        </gpx>"#;

        let gpx = Gpx::try_from_str(xml).unwrap();
        assert!(gpx.metadata.is_some());
        assert_eq!(gpx.date(), Some("2024-07-11T17:16:43Z"));
    }

    #[test]
    fn test_gpx_without_metadata() {
        let xml = r#"
        <gpx version="1.1" creator="test">
            <trk>
                <name>Test Track</name>
            </trk>
        </gpx>"#;

        let gpx = Gpx::try_from_str(xml).unwrap();
        assert!(gpx.metadata.is_none());
        assert_eq!(gpx.date(), None);
    }

    #[test]
    fn test_gpx_metadata_without_time() {
        let xml = r#"
        <gpx version="1.1" creator="test">
            <metadata>
            </metadata>
            <trk>
                <name>Test Track</name>
            </trk>
        </gpx>"#;

        let gpx = Gpx::try_from_str(xml).unwrap();
        assert!(gpx.metadata.is_some());
        assert_eq!(gpx.date(), None);
    }

    #[test]
    fn test_gpx_duration_calculation() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());

        let time1 = chrono::Utc.with_ymd_and_hms(2024, 7, 11, 10, 0, 0).unwrap();
        let time2 = chrono::Utc
            .with_ymd_and_hms(2024, 7, 11, 12, 30, 45)
            .unwrap();

        let segment = TrackSegment::with_points(vec![
            Point::with_time(40.7128, -74.0060, Some(10.0), time1),
            Point::with_time(40.7589, -73.9851, Some(15.0), time2),
        ]);

        track.add_segment(segment);
        gpx.add_track(track);

        let duration = gpx.total_duration_seconds();
        assert_eq!(duration, Some(9045)); // 2h 30m 45s = 9045 seconds

        let formatted = gpx.total_duration_formatted();
        assert_eq!(formatted, Some("02:30:45".to_string()));
    }

    #[test]
    fn test_gpx_duration_without_time() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());
        let segment = TrackSegment::with_points(vec![
            Point::new(40.7128, -74.0060),
            Point::new(40.7589, -73.9851),
        ]);
        track.add_segment(segment);
        gpx.add_track(track);

        assert_eq!(gpx.total_duration_seconds(), None);
        assert_eq!(gpx.total_duration_formatted(), None);
    }

    #[test]
    fn test_gpx_average_speed() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());

        let time1 = chrono::Utc.with_ymd_and_hms(2024, 7, 11, 10, 0, 0).unwrap();
        let time2 = chrono::Utc.with_ymd_and_hms(2024, 7, 11, 11, 0, 0).unwrap();

        let segment = TrackSegment::with_points(vec![
            Point::with_time(40.7128, -74.0060, Some(10.0), time1),
            Point::with_time(40.7589, -73.9851, Some(15.0), time2),
        ]);

        track.add_segment(segment);
        gpx.add_track(track);

        let speed = gpx.average_speed_kmh();
        assert!(speed.is_some());
        assert!(speed.unwrap() > 0.0);
    }

    #[test]
    fn test_gpx_average_speed_without_duration() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());
        let segment = TrackSegment::with_points(vec![
            Point::new(40.7128, -74.0060),
            Point::new(40.7589, -73.9851),
        ]);
        track.add_segment(segment);
        gpx.add_track(track);

        assert_eq!(gpx.average_speed_kmh(), None);
    }

    #[test]
    fn test_statistics_with_duration() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());

        let time1 = chrono::Utc.with_ymd_and_hms(2024, 7, 11, 10, 0, 0).unwrap();
        let time2 = chrono::Utc.with_ymd_and_hms(2024, 7, 11, 12, 0, 0).unwrap();

        let segment = TrackSegment::with_points(vec![
            Point::with_time(40.7128, -74.0060, Some(10.0), time1),
            Point::with_time(40.7589, -73.9851, Some(20.0), time2),
        ]);

        track.add_segment(segment);
        gpx.add_track(track);

        let stats = gpx.statistics();
        assert!(stats.duration_seconds.is_some());
        assert!(stats.average_speed_kmh.is_some());

        let summary = stats.summary();
        assert!(summary.contains("Duration:"));
        assert!(summary.contains("Average speed:"));
    }

    #[test]
    fn test_gpx_average_speed_zero_duration() {
        let mut gpx = Gpx::new();
        let mut track = Track::with_name("Test Track".to_string());

        let time1 = chrono::Utc.with_ymd_and_hms(2024, 7, 11, 10, 0, 0).unwrap();

        let segment = TrackSegment::with_points(vec![
            Point::with_time(40.7128, -74.0060, Some(10.0), time1),
            Point::with_time(40.7589, -73.9851, Some(15.0), time1), // Same time
        ]);

        track.add_segment(segment);
        gpx.add_track(track);

        assert_eq!(gpx.average_speed_kmh(), None); // Should handle zero duration
    }
}
