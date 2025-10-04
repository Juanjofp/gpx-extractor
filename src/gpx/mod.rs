// Módulos del paquete GPX
pub mod gpx;
pub mod point;
pub mod track;
pub mod waypoint;

// Re-exportar la estructura principal que se usa desde main.rs
pub use gpx::Gpx;
