use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.dir().dot(ray.dir());
        let h = ray.dir().dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let mut root = (h - sqrt) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) * (1.0 / self.radius);
        let mut rec = HitRecord::new(p, outward_normal, root);
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
