mod float2;
mod float3;
mod bitmap;
mod math;
mod triangle;
use std::time::{Instant, SystemTime};

use triangle::Triangle2D;
use float2::Float2;
use float3::Float3;

const HEIGHT : usize = 512;
const WIDTH : usize = 512;

fn create_test_image(image: &mut Vec<Vec<Float3>>, triangles : &Vec<Triangle2D>) -> () {


    for (y,row)  in image.iter_mut().enumerate() {
        
        for (x, pixel) in row.iter_mut().enumerate() {
            pixel.x = 0.0;
            pixel.y = 0.0;
            pixel.z = 0.0;
            
            let p = Float2::new(x as f32, y as f32);
            for triangle in triangles.iter() {
                if triangle.contains_point(p) {
                    *pixel = triangle.color;
                    break; // Stop checking other triangles once we find a match
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


fn main() {
    const TRIANGLE_COUNT: usize = 250;
    const FPS : i32 = 30;
    const VIDEO_DURATION : i32 = 5; // seconds
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

