use crate::{float2::Float2, float3::Float3, transform::{self, Transform}, triangle::{Triangle2D, Triangle3D}};


pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Float3>>,
}

impl RenderTarget {
    pub fn new(width : usize, height: usize) -> Self {
        RenderTarget {
            width,
            height,
            pixels: vec![vec![Float3::zero() ; width]; height],
        }
    }
    
    pub fn clear(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                *pixel = Float3::zero();
            }
        }
    }

    pub fn size(&self) -> Float2 {
        Float2 {
            x: self.width as f32,
            y: self.height as f32,
        }
    }
}

pub struct Model {
    pub triangles: Vec<Triangle3D>,
    pub colors: Vec<Float3>,
    pub transform: Transform,
}

impl Model {
    pub fn new() -> Self {
        Model {
            triangles: Vec::new(),
            colors: Vec::new(),
            transform: Transform { yaw: 0.0 }
        }
    }

    pub fn add_triangle(&mut self, triangle: Triangle3D, color: Float3) -> usize {
        self.triangles.push(triangle);
        self.colors.push(color);
        self.triangles.len() - 1 // Return the index of the new triangle
    }
}

pub fn render(model: &Model, target: &mut RenderTarget) {
    target.clear();
    
    for (i, triangle) in model.triangles.iter().enumerate() {
        let color = model.colors[i];
        
        let a_screen = vertex_to_screen_space(&triangle.a, target, &model.transform);
        let b_screen = vertex_to_screen_space(&triangle.b, target, &model.transform);
        let c_screen = vertex_to_screen_space(&triangle.c, target, &model.transform);
        let triangle = Triangle2D {
            a: a_screen,
            b: b_screen,
            c: c_screen,
            color,
        };

        let min_x = triangle.a.x.min(triangle.b.x).min(triangle.c.x);
        let max_x = triangle.a.x.max(triangle.b.x).max(triangle.c.x);
        let min_y = triangle.a.y.min(triangle.b.y).min(triangle.c.y);
        let max_y = triangle.a.y.max(triangle.b.y).max(triangle.c.y);
        
        let block_start_x = min_x.floor().clamp(0.0, target.width as f32 - 1.0) as usize;
        let block_end_x = max_x.ceil().clamp(0.0, target.width as f32 - 1.0) as usize;
        let block_start_y = min_y.floor().clamp(0.0, target.height as f32 - 1.0) as usize;
        let block_end_y = max_y.ceil().clamp(0.0, target.height as f32 - 1.0) as usize;

        for y in block_start_y..=block_end_y {
            for x in block_start_x..=block_end_x {
                let p = Float2::new(x as f32, y as f32);
                if triangle.contains_point(p) {
                    target.pixels[y][x] = color;
                }
            }
        }
    }
}

fn vertex_to_screen_space(vertex : &Float3, target: &RenderTarget, transform: &Transform) -> Float2 {
    let vertex_world = transform.to_world_point(vertex);
    
    let screen_height_world : f32 = 5.0;
    let pixels_per_world_unit = target.height as f32 / screen_height_world;

    let pixel_offset = Float2::new(vertex_world.x, vertex_world.y) * pixels_per_world_unit;
    target.size() / 2.0 + pixel_offset
}
    