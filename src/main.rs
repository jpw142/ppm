use std::fs::{OpenOptions};
use std::io::{Write};
use indicatif::{ProgressBar};
pub mod color;
use color::{Color, WriteColor};
mod point3;
fn main() {
    
    // Image Properties
    let image_height = 256;
    let image_width = 256;
    let max_ccv = 255;

    // PPM requirements and a PPM Viewer
    // https://web.cse.ohio-state.edu/~shen.94/681/Site/ppm_help.html
    // https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html

    // Write file and the required ppm header
    let file_name = "image.ppm";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(file_name).unwrap();

    match writeln!(file, "{}", format!("P3 {} {} {}\n", image_width, image_height, max_ccv)) {
        Ok(_) => (),
        Err(_) => println!("Problem writing header in {:?}", file),
    }

    // Write file contents according to ppm specifications
    let bar = ProgressBar::new(image_height);
    bar.set_message("Scanlines");
    for j in 0..=image_height-1 {
        for i in 0..=image_width-1 {
                let color = Color{r: ((i as f32/image_width as f32) * max_ccv as f32) as u8 , g: ((j as f32/image_height as f32) * max_ccv as f32) as u8, b: 127};
                color.write_color(&mut file);
        }
        bar.inc(1);
    }
    bar.finish();
}
