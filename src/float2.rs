use std::ops::{Div, Mul, Sub, Add, AddAssign};

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    pub fn new(x: f32, y: f32) -> Self {
        Float2 { x, y }
    }

    pub fn dot(&self, other: &Float2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    // 90 degrees clockwise rotation
    pub fn rotate_clockwise(&self) -> Self {
        Float2 { x: self.y, y: -self.x }
    }

    pub fn random_in_range(width: f32, height: f32) -> Self {
        let mut rng = rand::rng();
        Float2 {
            x: rng.random_range(0f32..width),
            y: rng.random_range(0f32..height),
        }
    }
}


impl Sub for Float2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Float2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Float2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Float2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Float2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
    
}

impl Div<f32> for Float2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }   
}

impl Mul<f32> for Float2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<f32> for Float2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
    
}