use crate::{float2::Float2, float3::{Float3}, transform::{Transform}, triangle::{Triangle2D, Triangle3D}};

#[derive(Default)]
pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Float3>>,
    pub fov: f32, // Field of view
    pub depth_buffer: Vec<Vec<f32>>, 
}

impl RenderTarget {
    pub fn new(width : usize, height: usize) -> Self {
        RenderTarget {
            width,
            height,
            pixels: vec![vec![Float3::zero(); width]; height],
            fov: 60.0, // Default field of view
            depth_buffer: vec![vec![f32::INFINITY; width]; height],
        }
    }
    
    pub fn clear(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                *pixel = Float3::zero();
            }
        }

        for row in &mut self.depth_buffer {
            for depth in row {
                *depth = f32::INFINITY;
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

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub triangles: Vec<Triangle3D>,
    pub transform: Transform,
}

impl Model {
    pub fn new() -> Self {
        Model {
            triangles: Vec::new(),
            transform: Transform { yaw: 0.0, pitch: 0.0, position: Float3::zero() }
        }
    }

    pub fn add_triangle(&mut self, triangle: Triangle3D) -> usize {
        self.triangles.push(triangle);
        self.triangles.len() - 1 // Return the index of the new triangle
    }

    pub fn from(obj: crate::obj::Obj) -> Self {
        let mut model = Model::new();
        for face in obj.faces.iter() {
            let mut triangles = Triangle3D::create_triangles_from_face(&obj, face);
            for triangle in triangles.iter_mut() {
                triangle.set_color(Float3::random());
                model.add_triangle(*triangle);
            }
        }
        model
    }
}

pub fn render(model: &Model, target: &mut RenderTarget) {
    target.clear();
    
    for triangle in model.triangles.iter() {
        let a_screen = vertex_to_screen_space(&triangle.a, target, &model.transform);
        let b_screen = vertex_to_screen_space(&triangle.b, target, &model.transform);
        let c_screen = vertex_to_screen_space(&triangle.c, target, &model.transform);
        let triangle = Triangle2D {
            a: Float2 { x: a_screen.x, y: a_screen.y },
            b: Float2 { x: b_screen.x, y: b_screen.y },
            c: Float2 { x: c_screen.x, y: c_screen.y },
            color: triangle.color,
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
                let (in_triangle, weight) = triangle.contains_point(p);
                if in_triangle {
                    let depths = Float3::new(
                        a_screen.z,
                        b_screen.z,
                        c_screen.z, 
                    );
                    let depth = depths.dot(&weight);
                    if depth > target.depth_buffer[y][x] {
                        continue; // Skip this pixel if it's not closer than the current depth
                    } 
                    
                    target.pixels[y][x] = triangle.color;    
                    target.depth_buffer[y][x] = depth;
                }
            }
        }
    }
}

fn vertex_to_screen_space(vertex : &Float3, target: &RenderTarget, transform: &Transform) -> Float3 {
    let vertex_world = transform.to_world_point(vertex);
    
    let screen_height_world : f32 = (target.fov.to_radians() / 2.0).tan() * 2.0; 
    let pixels_per_world_unit = target.height as f32 / screen_height_world / vertex_world.z;

    let pixel_offset = Float2::new(vertex_world.x, vertex_world.y) * pixels_per_world_unit;
    let vertex_screen = target.size() / 2.0 + pixel_offset;
    Float3::new(vertex_screen.x, vertex_screen.y, vertex_world.z)
}
    