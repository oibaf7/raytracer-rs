use raytracer_rs::hittable::HittableList;
use raytracer_rs::interval::Interval;
use raytracer_rs::sphere::Sphere;
use raytracer_rs::vector::{Color, Vec3};
use raytracer_rs::ray::Ray;

fn ray_color(ray: &Ray, list: &HittableList) -> Color {
    if let Some(rec) = list.hit(ray, &Interval::new(0.0, f64::INFINITY)) {
        return Color::from_vec((rec.normal() + Vec3::new(1.0, 1.0, 1.0)) * 0.5);
    }
    let unit_direction = ray.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    let c = Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
    Color::from_vec(c)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/ image_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u  * (1.0 / image_width as f64);
    let pixel_delta_v =  viewport_v  * (1.0 / image_height as f64);

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;


    println!("P3\n {} {} \n255\n",image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &list);
            println!("{}", pixel_color);
        }
    }

}
