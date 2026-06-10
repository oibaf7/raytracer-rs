use std::sync::mpsc::Receiver;
use std::thread::scope;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let color = self.albedo;

        Some((color, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(ray_in.dir(), rec.normal);
        reflected = reflected.unit_vector() + (Vec3::random_unit_vector() * self.fuzz);
        let scattered = Ray::new(rec.p, reflected);
        let color = self.albedo;
        if scattered.dir().dot(rec.normal) > 0.0 {
            return Some((color, scattered));
        }

        None
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let color = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = ray_in.dir().unit_vector();

        let cos = f64::min(ray_in.dir().dot(rec.normal) * -1.0, 1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let cannot_refract = ri * sin > 1.0;
        let direction = if cannot_refract {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, direction);

        Some((color, scattered))
    }
}
