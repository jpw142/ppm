use std::fs::{OpenOptions};
use std::io::{Write};
use indicatif::{ProgressBar};
pub mod color;
use color::{Color, WriteColor};
mod point3;
fn main() {
    
    // Image Properties
    let image_height = 255;
    let image_width = 255;
    let max_ccv = 255;


    // Write file and header
    let file_name = "lines.ppm";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(file_name).unwrap();
    match writeln!(file, "{}", format!("P3 {} {} {}\n", image_width, image_height, max_ccv)) {
        Ok(_) => (),
        Err(_) => println!("Problem writing header"),
    }

    // Write file contents
    let bar = ProgressBar::new(image_height);
    bar.set_message("Scanlines");
    for j in 0..=image_height-1 {
        for i in 0..=image_width-1 {
                let color = Color{r:i, g:j as u8, b:127};
                color.write_color(&mut file);
        }
        bar.inc(1);
    }
    bar.finish();
}
