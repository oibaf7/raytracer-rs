use crate::hittable::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{Color, Vec3};
use rand::Rng;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400usize;
const SAMPLES_PER_PIXELS: usize = 100;
const MAX_DEPTH: usize = 20;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub samples_per_pixel: usize,
    pixel_samples_scale: f64,
    pub max_depth: usize,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, list: &HittableList) {
        self.initialize();
        println!("P3\n {} {} \n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for s in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color += Self::ray_color(&r, self.max_depth, &list);
                }
                println!(
                    "{}",
                    Color::from_vec(pixel_color * self.pixel_samples_scale)
                );
            }
        }
    }

    //move out later!
    fn initialize(&mut self) {
        self.aspect_ratio = ASPECT_RATIO;
        // self.image_width = IMAGE_WIDTH;
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        // self.samples_per_pixel = SAMPLES_PER_PIXELS;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.max_depth = MAX_DEPTH;
        //self.vfov = 90.0;
        // self.lookfrom = Vec3::new(0.0, 0.0, 0.0);
        // self.lookat = Vec3::new(0.0, 0.0, -1.0);
        // self.vup = Vec3::new(0.0, 1.0, 0.0);

        let theta = self.vfov * std::f64::consts::PI / 180.0;
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = self.lookfrom;

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.w.cross(self.vup).unit_vector();
        self.v = self.u.cross(self.w);

        let viewport_u = self.u * viewport_width * -1.0;
        let viewport_v = self.v * -1.0 * viewport_height;
        self.pixel_delta_u = viewport_u * (1.0 / self.image_width as f64);
        self.pixel_delta_v = viewport_v * (1.0 / self.image_height as f64);

        let viewport_upper_left = self.center - (self.w * self.focus_dist) - viewport_u * 0.5 - viewport_v * 0.5;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        
        let defocus_radius = self.focus_dist * f64::tan(self.defocus_angle * std::f64::consts::PI / 180.0);
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(ray: &Ray, depth: usize, list: &HittableList) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(mut rec) = list.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            if let Some(t) = rec.mat.scatter(ray, &rec) {
                return t.0.vec() * Self::ray_color(&t.1, depth - 1, list);
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let unit_direction = ray.dir().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let c = Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
        c
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i + offset.x()))
            + (self.pixel_delta_v * (j + offset.y()));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(
            rand::thread_rng().r#gen::<f64>() - 0.5,
            rand::thread_rng().r#gen::<f64>() - 0.5,
            0.0,
        )
    }
    
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y()) 
    }
}
