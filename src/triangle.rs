use rand::rand_core::le;

use crate::float2::Float2;
use crate::float3::Float3;
use crate::math::point_on_right_side_of_line;
use crate::obj::{FaceElement, Obj};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2D {
    pub a: Float2,
    pub b: Float2,
    pub c: Float2,
    pub color: Float3, // Optional color field
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle3D {
    pub a: Float3,
    pub b: Float3,
    pub c: Float3,
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
        
        ab && bc && ca
    }

    pub fn set_color(&mut self, color: Float3) {
        self.color = color;
    }
}

impl Triangle3D {
    pub fn new(a: Float3, b: Float3, c: Float3) -> Self {
        Triangle3D { a, b, c, color: Float3::new(1.0, 1.0, 1.0) }
    }

    pub fn set_color(&mut self, color: Float3) {
        self.color = color;
    }

    pub fn create_triangles_from_face(obj : &Obj, face : &FaceElement) -> Vec<Triangle3D> {
        let mut triangles = Vec::new();
        if face.vertex_indices.len() < 3 {
            return triangles; // Not enough vertices to form a triangle
        }

        // Create first triangle
        let a = obj.vertices[face.vertex_indices[0]].position;
        let b = obj.vertices[face.vertex_indices[1]].position;
        let c = obj.vertices[face.vertex_indices[2]].position;    
        let mut triangle = Triangle3D::new(a, b, c);

        triangles.push(triangle.clone());
        // Create additional triangles for polygons with more than 3 vertices
        for i in 3 .. face.vertex_indices.len() {
            triangle.b = triangle.c;
            triangle.c = obj.vertices[face.vertex_indices[i]].position;
            triangles.push(triangle.clone());
        }
    
        triangles
    }
    
}
