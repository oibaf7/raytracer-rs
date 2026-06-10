use rand::Rng;
use std::f64;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees.to_radians()
}

pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::thread_rng().r#gen::<f64>()
}
