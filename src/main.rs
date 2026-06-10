use raytracer_rs::hittable::HittableList;
use std::rc::Rc;
use rand::random;
use raytracer_rs::camera::Camera;
use raytracer_rs::material::{Dielectric, Lambertian, Metal};
use raytracer_rs::sphere::Sphere;
use raytracer_rs::vector::{Color, Vec3};

fn main() {
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut list = HittableList::new();

    list.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * random::<f64>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_unit_vector() * Vec3::random_unit_vector();
                    let sphere_material = Rc::new(Lambertian::new(Color::from_vec(albedo)));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(random::<f64>() * 0.5 + 0.5, random::<f64>() * 0.5 + 0.5, random::<f64>() * 0.5 + 0.5);
                    let fuzz = random::<f64>() * 0.5 + 0.5;
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    list.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    list.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    list.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));


    let mut camera = Camera::default();
    camera.image_width = 600;
    camera.samples_per_pixel = 100;
    camera.vfov = 20.0;
    camera.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.render(&list);
}
