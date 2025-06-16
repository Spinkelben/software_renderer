
use crate::float2::Float2;
use crate::float3::Float3;
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

    pub fn contains_point(&self, p: Float2) -> (bool, Float3) {
        let area_abp = Self::triangle_area(self.a, self.b, p);
        let area_bcp = Self::triangle_area(self.b, self.c, p);
        let area_cap = Self::triangle_area(self.c, self.a, p);
        let in_triangle = area_abp >= 0.0 && area_bcp >= 0.0 && area_cap >= 0.0;

        let total_area = area_abp + area_bcp + area_cap;
        let inverse_area = 1.0 / total_area;
        let weight = Float3::new(
            area_bcp * inverse_area,
            area_cap * inverse_area,
            area_abp * inverse_area,
        );
        
        (in_triangle && total_area > 0.0, weight)
    }

    pub fn set_color(&mut self, color: Float3) {
        self.color = color;
    }

    pub fn triangle_area(a : Float2, b: Float2, c: Float2) -> f32 {
        let ac = c - a;
        let ab = b - a;
        let ab_perpendicular = ab.rotate_clockwise();
        ac.dot(&ab_perpendicular) / 2.0
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
