mod float2;
mod float3;
mod bitmap;
mod math;
mod triangle;
mod obj;
mod render;
mod transform;
use std::time::{Instant, SystemTime};

use triangle::Triangle2D;
use float2::Float2;
use float3::Float3;

use crate::render::Model;

const HEIGHT : usize = 512;
const WIDTH : usize = 512;

fn create_test_image(image: &mut Vec<Vec<Float3>>, triangles : &Vec<Triangle2D>) -> () {
    // Clear the image with black color
    for row in image.iter_mut() {
        for pixel in row.iter_mut() {
            *pixel = Float3::new(0.0, 0.0, 0.0);
        }
    }
    
    for triangle in triangles.iter() {
        let min_x = triangle.a.x.min(triangle.b.x).min(triangle.c.x);
        let max_x = triangle.a.x.max(triangle.b.x).max(triangle.c.x);
        let min_y = triangle.a.y.min(triangle.b.y).min(triangle.c.y);
        let max_y = triangle.a.y.max(triangle.b.y).max(triangle.c.y);
        
        let block_start_x = min_x.floor().clamp(0.0, WIDTH as f32 - 1.0) as usize;
        let block_end_x = max_x.ceil().clamp(0.0, WIDTH as f32 - 1.0) as usize;
        let block_start_y = min_y.floor().clamp(0.0, HEIGHT as f32 - 1.0) as usize;
        let block_end_y = max_y.ceil().clamp(0.0, HEIGHT as f32 - 1.0) as usize;

        for y in block_start_y..=block_end_y {
            for x in block_start_x..=block_end_x {
                let p = Float2::new(x as f32, y as f32);
                if triangle.contains_point(p) {
                    image[y][x] = triangle.color;
                }
            }
        }
    }
    
}

fn update_triangles(triangles: &mut Vec<Triangle2D>, velocities: &mut Vec<Float2>) {
    for (i, triangle) in triangles.iter_mut().enumerate() {
        let velocity = &mut velocities[i];
        triangle.a += *velocity;
        triangle.b += *velocity;
        triangle.c += *velocity;

        // Check for boundary conditions and reverse direction if necessary
        for vertex in [&triangle.a, &triangle.b, &triangle.c] {
            if vertex.x < 0.0 && velocity.x < 0.0 {
                velocity.x = -velocity.x;
            } else if vertex.x >= WIDTH as f32 && velocity.x > 0.0 {
                velocity.x = -velocity.x;
            }

            if vertex.y < 0.0 && velocity.y < 0.0 {
                velocity.y = -velocity.y;
            } else if vertex.y >= HEIGHT as f32 && velocity.y > 0.0 {
                velocity.y = -velocity.y;
            }
        }
    }
}

#[allow(unused)]
fn crazy_triangles() {
    const TRIANGLE_COUNT: usize = 250;
    const FPS : i32 = 30;
    const VIDEO_DURATION : i32 = 60; // seconds
    const FRAME_COUNT : i32 = FPS * VIDEO_DURATION;
    let mut triangles : Vec<Triangle2D> = vec![];
    let mut velocities : Vec<Float2> = vec![];

    let half_size = Float2::new(WIDTH as f32, HEIGHT as f32) / 2.0;
    for _ in 0..TRIANGLE_COUNT {
        let mut triangle = Triangle2D::new(
            half_size + (Float2::random_in_range(WIDTH as f32, HEIGHT as f32) - half_size) * 0.3,
            half_size + (Float2::random_in_range(WIDTH as f32, HEIGHT as f32) - half_size) * 0.3,
            half_size + (Float2::random_in_range(WIDTH as f32, HEIGHT as f32) - half_size) * 0.3,
        );
        triangle.set_color(Float3::random());
        triangles.push(triangle);
        velocities.push((Float2::random_in_range(WIDTH as f32, HEIGHT as f32) - half_size) * 0.1);
    }

    let out_dir = format!("out-{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
    let start = Instant::now();    
    let mut image : Vec<Vec<Float3>> = vec![vec![Float3::new(0.0, 0.0, 0.0); WIDTH]; HEIGHT];

    for frame in 0..FRAME_COUNT {
        let now = Instant::now();
        update_triangles(&mut triangles, &mut velocities);
        create_test_image(&mut image, &triangles);
        bitmap::write_image_to_file(&image, &format!("{}/frame_{:04}.bmp", out_dir, frame)).expect("Failed to write image to file");
        let elapsed = now.elapsed();
        let percent_complete = (frame as f32 / FRAME_COUNT as f32) * 100.0;
        let estimated_time = elapsed * (FRAME_COUNT - frame) as u32;
        println!("Frame {} processed in {:.2?}. {} %. Remaining: {:.2?}", frame, elapsed, percent_complete, estimated_time);
    }
    
    let total_elapsed = start.elapsed();
    println!("All frames processed in {:.2?}.", total_elapsed);
    println!("Images saved to directory: {}", out_dir);
}

fn main() {
    //crazy_triangles();
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <obj_file_path>", args[0]);
        return;
    }

    let obj_file_path = &args[1];
    let obj = obj::Obj::read_from_file(obj_file_path).expect("Failed to read OBJ file");
    println!("OBJ file loaded successfully with {} vertices, {} texture coordinates, {} normals, and {} faces.", 
             obj.vertices.len(), obj.texture_coordinates.len(), obj.normals.len(), obj.faces.len());

    let mut model = Model::new();
    for face in obj.faces.iter() {
        let t = triangle::Triangle3D::create_triangles_from_face(&obj, face);
        for triangle in t.iter() {
            model.add_triangle(*triangle, Float3::random());
        }
    }

    let out_dir = format!("out-{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
    let mut render_target = render::RenderTarget::new(WIDTH, HEIGHT);
    const FPS : i32 = 30;
    const VIDEO_DURATION : i32 = 30; // seconds
    const FRAME_COUNT : i32 = FPS * VIDEO_DURATION;
    let start = Instant::now();
    for frame in 0..FRAME_COUNT {
        let now = Instant::now();
        render_target.clear();
        model.transform.yaw += 0.02; // Rotate the model slightly each frame
        render::render(&model, &mut render_target);
        bitmap::write_image_to_file(&render_target.pixels, &format!("{}/frame_{:04}.bmp", out_dir, frame)).expect("Failed to write image to file");
        let elapsed = now.elapsed();
        let percent_complete = (frame as f32 / FRAME_COUNT as f32) * 100.0;
        let estimated_time = elapsed * (FRAME_COUNT - frame) as u32;
        println!("Frame {} processed in {:.2?}. {} %. Remaining: {:.2?}", frame, elapsed, percent_complete, estimated_time);
    }

    let total_elapsed = start.elapsed();
    println!("All frames processed in {:.2?}.", total_elapsed);
    println!("Images saved to directory: {}", out_dir);
}

