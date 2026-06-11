use std::arch::x86_64::_mm256_sm4rnds4_epi32;
use crate::hittable::{HitRecord, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::tiles::Tile;
use crate::vector::{Color, Vec3};
use rand::Rng;
use std::sync::{mpsc, Arc};
use crate::min_heap::Heap;
use crate::thread_pool::ThreadPool;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400usize;
const SAMPLES_PER_PIXELS: usize = 100;
const MAX_DEPTH: usize = 20;
const CHUNK_SIZE: usize = 3; //thickness of chunk that goes across all x

pub struct CameraParams {
    pub image_width: usize,
    image_height: usize,
    center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
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
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

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
    pub fn render(&mut self, list: HittableList) {
        let params = self.initialize();
        let mut heap = Heap::new();
        let mut last_seen = -1;
        let chunks_in_y = params.image_height.div_ceil(CHUNK_SIZE);
        let mut tiles = Vec::new();
        let mut id = 0;
        for y in 0..chunks_in_y {
            tiles.push(Tile::new(id, y));
            id += 1;

        }
        let (tx, rx) = mpsc::channel();
        ThreadPool::new(tx, tiles, params, list);
        println!("P3\n {} {} \n255\n", self.image_width, self.image_height);

        while let Ok(result) = rx.recv() {
            heap.add(result);
            while let Some(r) = heap.peek() {
                if r.id() as isize == last_seen + 1 {
                    last_seen += 1;
                    let r = heap.remove().unwrap();
                    for i in r.colors() {
                        println!("{}", i);
                    }
                } else {
                    break;
                }
            }
        }

    }

    //move out later! and fix for refactor
    fn initialize(&mut self) -> CameraParams {
        self.aspect_ratio = ASPECT_RATIO;
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.max_depth = MAX_DEPTH;

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

        let viewport_upper_left =
            self.center - (self.w * self.focus_dist) - viewport_u * 0.5 - viewport_v * 0.5;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius =
            self.focus_dist * f64::tan(self.defocus_angle * std::f64::consts::PI / 180.0);
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        CameraParams {
            image_width: self.image_width,
            image_height: self.image_height,
            center: self.center,
            pixel00_loc: self.pixel00_loc,
            pixel_delta_u: self.pixel_delta_u,
            pixel_delta_v: self.pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            pixel_samples_scale: self.pixel_samples_scale,
            max_depth: self.max_depth,
            vfov: self.vfov,
            lookfrom: self.lookfrom,
            lookat: self.lookat,
            vup: self.vup,
            u: self.u,
            v: self.v,
            w: self.w,
            defocus_angle: self.defocus_angle,
            focus_dist: self.focus_dist,
            defocus_disk_u: self.defocus_disk_u,
            defocus_disk_v: self.defocus_disk_v,
        }
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

    fn get_ray(params: &CameraParams, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = params.pixel00_loc
            + (params.pixel_delta_u * (i + offset.x()))
            + (params.pixel_delta_v * (j + offset.y()));
        let ray_origin = if params.defocus_angle <= 0.0 {
            params.center
        } else {
            Self::defocus_disk_sample(params)
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

    fn defocus_disk_sample(params: &CameraParams) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        params.center + (params.defocus_disk_u * p.x()) + (params.defocus_disk_v * p.y())
    }

    pub fn render_chunk(tile: &Tile, list: &HittableList, params: &CameraParams) -> Vec<Color> {
        let start_x = 0;
        let end_x = params.image_width;
        let start_y = tile.y() * CHUNK_SIZE;
        let end_y = usize::min(params.image_height, CHUNK_SIZE * (tile.y() + 1));

        let mut colors = Vec::new();
        for y in start_y..end_y {
            for x in start_x..end_x {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..params.samples_per_pixel {
                    let r = Self::get_ray(params, x as f64, y as f64);
                    pixel_color += Self::ray_color(&r, params.max_depth, list);
                }
                colors.push(Color::from_vec(pixel_color * params.pixel_samples_scale));
            }
        }

        colors
    }
}
