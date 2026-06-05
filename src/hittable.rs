use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64) -> Self {
        Self {
            p,
            normal,
            t,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.dir().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        }
    }
    
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub  struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}