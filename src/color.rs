use std::fs::File;

pub struct Color {pub r: u8, pub g: u8, pub b: u8}
pub trait WriteColor {
    fn write_color(&self, file: &mut File);
    
}
impl WriteColor for Color {
    fn write_color(&self, file: &mut File) {
        use std::io::{Write};
        match write!(file, "{}", format!("{} {} {}\n", self.r, self.g, self.b)){
            Ok(_) => (),
            Err(_) => panic!("Error writing to file: {:?}", file),
        }
    }
}