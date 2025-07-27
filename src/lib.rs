pub mod camera;
pub mod colour;
pub mod hittable;
pub mod interval;
pub mod ray;
pub mod vec3;

// Constants
pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = 3.1415926535897932385;

// Utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
