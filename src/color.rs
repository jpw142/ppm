use std::fs::File;
pub struct Color {pub r: f64, pub g: f64, pub b: f64}
pub trait WriteColor {
    fn write_color(&self, file: &mut File);
    
}
impl WriteColor for Color {
    fn write_color(&self, file: &mut File) {
        use std::io::{Write};
        match write!(file, "{}", format!("{} {} {}\n", (self.r*255.999) as u8, (self.g * 255.999) as u8, (self.b * 255.999) as u8)){
            Ok(_) => (),
            Err(_) => panic!("Error writing to file: {:?}", file),
        }
    }
} 
// Multiply by a scalar f64
impl std::ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl std::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            b: self.b + rhs.b,
            g: self.g + rhs.g,
        }
    }
}