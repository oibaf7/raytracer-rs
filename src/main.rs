use raytracer_rs::vector::Color;

mod vector;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n {} {} \n255\n",image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r =  i as f64 / (image_width - 1) as f64;
            let g =  j as f64 / (image_height - 1) as f64;
            let b = 0f64;
            let color = Color::new(r, g, b);

            println!("{}", color);
        }
    }

}
