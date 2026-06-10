use raytracer_rs::hittable::HittableList;
use std::rc::Rc;

use raytracer_rs::camera::Camera;
use raytracer_rs::material::{Dielectric, Lambertian, Metal};
use raytracer_rs::sphere::Sphere;
use raytracer_rs::vector::{Color, Vec3};

fn main() {
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));
    list.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        mat_center,
    )));
    list.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));
    list.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

    let mut camera = Camera::default();
    camera.render(&list);
}
