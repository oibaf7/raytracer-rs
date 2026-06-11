use rand::random;
use raytracer_rs::camera::Camera;
use raytracer_rs::hittable::HittableList;
use raytracer_rs::material::{Dielectric, Lambertian, Metal};
use raytracer_rs::sphere::Sphere;
use raytracer_rs::vector::{Color, Vec3};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    render_large_scene();
}

fn render_small_scene() {
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    // 1. Generate the grid of random spheres (Small)
    for a in -5..5 {
        for b in -5..5 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_unit_vector() * Vec3::random_unit_vector();
                    let sphere_material = Arc::new(Lambertian::new(Color::from_vec(albedo)));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(
                        random::<f64>() * 0.5 + 0.5,
                        random::<f64>() * 0.5 + 0.5,
                        random::<f64>() * 0.5 + 0.5,
                    );
                    let fuzz = random::<f64>() * 0.5 + 0.5;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // 2. Add the three large showcase spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    list.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    list.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    list.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // 3. Camera Setup (Low Resolution / Low Samples)
    let mut camera = Camera::default();
    camera.image_width = 200;
    camera.samples_per_pixel = 10;
    camera.vfov = 20.0;
    camera.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    let start = Instant::now();
    camera.render(list);
    eprintln!("Rendered in {:.2?}", start.elapsed());
}

fn render_medium_scene() {
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    // 1. Generate the grid of random spheres (Medium)
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_unit_vector() * Vec3::random_unit_vector();
                    let sphere_material = Arc::new(Lambertian::new(Color::from_vec(albedo)));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(
                        random::<f64>() * 0.5 + 0.5,
                        random::<f64>() * 0.5 + 0.5,
                        random::<f64>() * 0.5 + 0.5,
                    );
                    let fuzz = random::<f64>() * 0.5 + 0.5;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // 2. Add the three large showcase spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    list.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    list.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    list.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // 3. Camera Setup (Medium Resolution / Balanced Samples)
    let mut camera = Camera::default();
    camera.image_width = 600;
    camera.samples_per_pixel = 100;
    camera.vfov = 20.0;
    camera.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    let start = Instant::now();
    camera.render(list);
    eprintln!("Rendered in {:.2?}", start.elapsed());
}

fn render_large_scene() {
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    // 1. Generate the grid of random spheres (Large)
    for a in -20..20 {
        for b in -20..20 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_unit_vector() * Vec3::random_unit_vector();
                    let sphere_material = Arc::new(Lambertian::new(Color::from_vec(albedo)));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(
                        random::<f64>() * 0.5 + 0.5,
                        random::<f64>() * 0.5 + 0.5,
                        random::<f64>() * 0.5 + 0.5,
                    );
                    let fuzz = random::<f64>() * 0.5 + 0.5;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    list.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // 2. Add the three large showcase spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    list.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    list.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    list.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // 3. Camera Setup (High Resolution / High Samples)
    let mut camera = Camera::default();
    camera.image_width = 1200;
    camera.samples_per_pixel = 250;
    camera.vfov = 20.0;
    camera.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    let start = Instant::now();
    camera.render(list);
    eprintln!("Rendered in {:.2?}", start.elapsed());
}
