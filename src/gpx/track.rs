use crate::gpx::point::{haversine_distance, Point};
use serde::{Deserialize, Serialize};

/// A continuous segment of a GPS track
///
/// Tracks are divided into segments to represent continuous sections.
/// A break in recording (e.g., GPS turned off) starts a new segment.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackSegment {
    /// Sequential points that make up this segment
    #[serde(rename = "trkpt", default)]
    pub points: Vec<Point>,
}

impl TrackSegment {
    /// Crea un nuevo segmento vacío
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Crea un segmento con puntos
    pub fn with_points(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Agrega un punto al segmento
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    /// Calcula la distancia total del segmento en kilómetros
    pub fn distance_km(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }

        self.points
            .windows(2)
            .map(|window| haversine_distance(&window[0], &window[1]))
            .sum()
    }

    /// Obtiene el rango de elevación del segmento
    pub fn elevation_range(&self) -> Option<(f64, f64)> {
        let elevations: Vec<f64> = self.points.iter().filter_map(|p| p.elevation).collect();

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

    /// Cuenta los puntos del segmento
    pub fn point_count(&self) -> usize {
        self.points.len()
    }
}

impl Default for TrackSegment {
    fn default() -> Self {
        Self::new()
    }
}

/// A GPS track representing a recorded route
///
/// A track consists of one or more segments, each containing sequential points.
/// Tracks typically represent activities like runs, bike rides, or hikes.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Track {
    /// Optional name describing the track
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Track segments making up this track
    #[serde(rename = "trkseg", default)]
    pub segments: Vec<TrackSegment>,
}

impl Track {
    /// Crea un nuevo track vacío
    pub fn new() -> Self {
        Self {
            name: None,
            segments: Vec::new(),
        }
    }

    /// Crea un track con nombre
    pub fn with_name(name: String) -> Self {
        Self {
            name: Some(name),
            segments: Vec::new(),
        }
    }

    /// Agrega un segmento al track
    pub fn add_segment(&mut self, segment: TrackSegment) {
        self.segments.push(segment);
    }

    /// Obtiene todos los puntos de todos los segmentos
    pub fn get_all_points(&self) -> Vec<&Point> {
        self.segments
            .iter()
            .flat_map(|segment| &segment.points)
            .collect()
    }

    /// Calcula la distancia total del track en kilómetros
    pub fn total_distance_km(&self) -> f64 {
        self.segments
            .iter()
            .map(|segment| segment.distance_km())
            .sum()
    }

    /// Obtiene el rango de elevación del track completo
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

    /// Cuenta el total de puntos en el track
    pub fn total_points(&self) -> usize {
        self.segments
            .iter()
            .map(|segment| segment.point_count())
            .sum()
    }

    /// Obtiene el nombre del track o un nombre por defecto
    pub fn display_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| "Unnamed Track".to_string())
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_segment_new() {
        let segment = TrackSegment::new();
        assert!(segment.points.is_empty());
        assert_eq!(segment.point_count(), 0);
        assert_eq!(segment.distance_km(), 0.0);
    }

    #[test]
    fn test_track_segment_with_points() {
        let points = vec![Point::new(40.7128, -74.0060), Point::new(40.7589, -73.9851)];
        let segment = TrackSegment::with_points(points);
        assert_eq!(segment.point_count(), 2);
        assert!(segment.distance_km() > 0.0);
    }

    #[test]
    fn test_track_segment_add_point() {
        let mut segment = TrackSegment::new();
        segment.add_point(Point::new(40.7128, -74.0060));
        assert_eq!(segment.point_count(), 1);
    }

    #[test]
    fn test_track_segment_elevation_range() {
        let points = vec![
            Point::with_elevation(40.7128, -74.0060, 10.0),
            Point::with_elevation(40.7589, -73.9851, 20.0),
            Point::with_elevation(40.7500, -73.9800, 5.0),
        ];
        let segment = TrackSegment::with_points(points);

        let (min, max) = segment.elevation_range().unwrap();
        assert_eq!(min, 5.0);
        assert_eq!(max, 20.0);
    }

    #[test]
    fn test_track_new() {
        let track = Track::new();
        assert!(track.name.is_none());
        assert!(track.segments.is_empty());
        assert_eq!(track.total_points(), 0);
        assert_eq!(track.total_distance_km(), 0.0);
    }

    #[test]
    fn test_track_with_name() {
        let track = Track::with_name("Test Track".to_string());
        assert_eq!(track.name, Some("Test Track".to_string()));
        assert_eq!(track.display_name(), "Test Track");
    }

    #[test]
    fn test_track_display_name_default() {
        let track = Track::new();
        assert_eq!(track.display_name(), "Unnamed Track");
    }

    #[test]
    fn test_track_add_segment() {
        let mut track = Track::new();
        let segment = TrackSegment::with_points(vec![
            Point::new(40.7128, -74.0060),
            Point::new(40.7589, -73.9851),
        ]);

        track.add_segment(segment);
        assert_eq!(track.segments.len(), 1);
        assert_eq!(track.total_points(), 2);
        assert!(track.total_distance_km() > 0.0);
    }

    #[test]
    fn test_track_multiple_segments() {
        let mut track = Track::with_name("Multi-Segment Track".to_string());

        let segment1 = TrackSegment::with_points(vec![
            Point::new(40.7128, -74.0060),
            Point::new(40.7589, -73.9851),
        ]);

        let segment2 = TrackSegment::with_points(vec![
            Point::new(40.7600, -73.9800),
            Point::new(40.7700, -73.9750),
        ]);

        track.add_segment(segment1);
        track.add_segment(segment2);

        assert_eq!(track.segments.len(), 2);
        assert_eq!(track.total_points(), 4);
        assert!(track.total_distance_km() > 0.0);
    }
}
