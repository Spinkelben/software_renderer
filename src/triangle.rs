use crate::float2::Float2;
use crate::float3::Float3;
use crate::math::point_on_right_side_of_line;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2D {
    pub a: Float2,
    pub b: Float2,
    pub c: Float2,
    pub color: Float3, // Optional color field
}

impl Triangle2D {
    pub fn new(a: Float2, b: Float2, c: Float2) -> Self {
        Triangle2D { a, b, c, color: Float3::new(1.0, 1.0, 1.0) }
    }

    pub fn contains_point(&self, p: Float2) -> bool {
        let ab = point_on_right_side_of_line(self.a, self.b, p);
        let bc = point_on_right_side_of_line(self.b, self.c, p);
        let ca = point_on_right_side_of_line(self.c, self.a, p);
        
        ab == bc && bc == ca
    }

    pub fn set_color(&mut self, color: Float3) {
        self.color = color;
    }
}
