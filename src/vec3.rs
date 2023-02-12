#[derive(Clone, Copy)]
pub struct Vec3 {pub x: f64, pub y: f64, pub z: f64}

// Vec3 Utilities
// Multiply Vector by an f64
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
// Divide Vector by an f64
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x/rhs,
            y: self.y/rhs,
            z: self.z/rhs,
        }
    }
}
impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x/rhs,
            y: self.y/rhs,
            z: self.z/rhs,
        }
    }
}

// Subtract Vector by Vector
impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

}

// Add Vector by Vector
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Get Length of a vector
pub trait Length {
    fn length_squared(&self) -> f64;
    fn length(&self) -> f64; 
}
impl Length for Vec3 {
    fn length_squared(&self) -> f64 {
        return &self.x* &self.x + &self.y* &self.y + &self.z* &self.z;
    }
    fn length(&self) -> f64 {
        return f64::sqrt(self.length_squared());

    }
}

// Normalize Vector
pub trait UnitVector {
    fn unit_vector(&self) -> Vec3;
}
impl UnitVector for Vec3 {
    fn unit_vector(&self) -> Vec3 {
        return self/self.length();
    }
}

// Dot Product
pub trait DotProduct {
    fn dot(self, vec: Vec3) -> f64;
}
impl DotProduct for Vec3 {
    fn dot(self, vec: Vec3) -> f64 {
        return self.x*vec.x + self.y*vec.y + self.z*vec.z;
    }
}
pub mod ray {
    use super::Vec3;
    pub struct Ray {pub origin: super::Vec3, pub direction: Vec3}

    pub trait Origin {
        fn origin(self) -> super::Vec3;
    }
    impl Origin for Ray {
        fn origin(self) -> super::Vec3 {
            return self.origin;
        }
    }

    pub trait GetDirection {
        fn direction(self) -> Vec3;
    }
    impl GetDirection for Ray {
        fn direction(self) -> Vec3 {
            return self.direction;
        }
    }

    pub trait At {
        fn at(&self, t: f64) -> super::Vec3;
    }
    impl At for Ray {
        fn at(&self, t: f64) -> super::Vec3{
            return super::Vec3{x: (self.origin.x + self.direction.x * t), y: (self.origin.y + self.direction.y * t), z: (self.origin.z + self.direction.z * t) }
        }
    }
}

