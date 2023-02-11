use std::fs::{OpenOptions};
use std::io::{Write};
use indicatif::{ProgressBar};
mod color;
use color::{Color, WriteColor};
mod vec3;
use vec3::{Vec3,UnitVector};
use vec3::ray::{Ray, GetDirection};

// Source Material
// https://raytracing.github.io/

// PPM requirements and a PPM Viewer
// https://web.cse.ohio-state.edu/~shen.94/681/Site/ppm_help.html
// https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html


fn ray_color(ray: Ray) -> Color{
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5*(unit_direction.y + 1.0);
    return Color{r: 1.0, b: 1.0, g: 1.0}*(1.0-t) + (Color{r:0.5, g: 0.7, b: 1.0}*t);

}
fn main() {
    
    // Image Properties

    let aspect_ratio = 16.0/9.0;
    let image_width = 256;
    let image_height = (image_width as f64/aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio*viewport_height;
    let focal_length = 1.0;

    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{x:viewport_width, y:0.0, z:0.0};
    let vertical = Vec3{x: 0.0, y: viewport_height, z:0.0};
    let lower_left_corner = origin - horizontal/(2.0 as f64) - vertical/(2.0 as f64) - Vec3{x:0.0,y: 0.0,z: focal_length};


    // Write file and the required ppm header

    let file_name = "image.ppm";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(file_name).unwrap();

    match writeln!(file, "{}", format!("P3 {} {} {}\n", image_width, image_height, 255)) {
        Ok(_) => (),
        Err(_) => println!("Problem writing header in {:?}", file),
    }

    // Render

    let bar = ProgressBar::new(image_height as u64);
    bar.set_message("Scanlines");
    for j in (0..=image_height-1).rev() {
        for i in 0..=image_width-1 {
                let u = (i as f64) / ((image_width-1) as f64);
                let v = (j as f64) / ((image_height-1) as f64);
                let r = Ray{origin: origin, direction: lower_left_corner + horizontal*u + vertical*v - origin};
                let color_pixel = ray_color(r);
                color_pixel.write_color(&mut file);
        }
        bar.inc(1);
    }
    bar.finish();
}
