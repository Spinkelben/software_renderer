use std::ops::{Add, Mul};

use rand::Rng;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,

}

impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Float3 { x, y, z }
    }

    pub fn r(&self) -> f32 {
        self.x
    }

    pub fn g(&self) -> f32 {
        self.y
    }

    pub fn b(&self) -> f32 {
        self.z
    }

    pub fn random() -> Self {
        Self::random_in_range(1.0, 1.0, 1.0)
    }

    pub fn random_in_range(width: f32, height: f32, depth: f32) -> Self {
        let mut rng = rand::rng();
        Float3 {
            x: rng.random_range(0f32..width),
            y: rng.random_range(0f32..height),
            z: rng.random_range(0f32..depth),
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
}

impl Mul<f32> for Float3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Float3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Float3> for f32 {
    type Output = Float3;

    fn mul(self, vec: Float3) -> Self::Output {
        Float3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}

impl Mul for Float3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Float3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }    
}

impl Add for Float3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Float3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }    
}