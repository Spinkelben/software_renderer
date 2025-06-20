mod float2;
mod float3;
mod bitmap;
mod triangle;
mod obj;
mod render;
mod transform;
use std::{sync::Arc, time::{Instant, SystemTime}};

use pixels::Pixels;
use triangle::Triangle2D;
use float2::Float2;
use float3::Float3;
use winit::{application::ApplicationHandler, dpi::{LogicalSize, Size}, event::WindowEvent, event_loop::{self, ActiveEventLoop}, window::{Window, WindowAttributes, WindowId}};

use crate::{obj::Obj, render::{Model, RenderTarget}};

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
                let (in_triangle, _) = triangle.contains_point(p);
                if in_triangle {
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

#[derive(Default)]
pub struct App {
    window: Option<Arc<winit::window::Window>>,
    pixels: Option<Pixels<'static>>,
    animation: Animation,
    last_frame: Option<Instant>,
}

#[derive(Default)]
struct Animation {
    models: Vec<Model>,
    total_duration: i32,
    rotations: Vec<((i32, i32), (f32, f32))>,
    render_target: RenderTarget,
    start_time: Option<Instant>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {

        self.window = Some(Arc::new(event_loop.create_window(Window::default_attributes()
            .with_inner_size(Size::Logical(LogicalSize {
                width: WIDTH as f64,
                height: HEIGHT as f64,
            }))
            .with_title("Software Renderer".to_string()))
            .expect("Failed to create window")));


        let window_size = self.window.as_ref().unwrap().inner_size();
        let surface_texture = pixels::SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(self.window.as_ref().unwrap()));
        self.pixels = Some(Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap());
        self.animation.start_time = Some(Instant::now());
        self.last_frame = Some(Instant::now());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                println!("Window close requested, exiting application.");
                event_loop.exit();
            },
            winit::event::WindowEvent::RedrawRequested => {    
                // Update model transformations based on the current frame
                let now = Instant::now();
                let elapsed = self.last_frame.map_or(0.0, |last| (now - last).as_secs_f32());
                self.last_frame = Some(now);

                let rotation_list = &self.animation.rotations;
                let elapsed_time = self.animation.start_time
                    .map_or(0, |start| start.elapsed().as_secs_f32() as i32) % self.animation.total_duration as i32;
                let (rotation_yaw, rotation_pitch) = rotation_list.iter()
                    .find(|&&((start, end), _)| {
                        elapsed_time >= start && elapsed_time < end
                    })
                    .map(|&(_, rotation)| rotation)
                    .unwrap_or((0.0, 0.0)); 

                for model in self.animation.models.iter_mut() {
                    model.transform.yaw += rotation_yaw * elapsed * 30.0; 
                    model.transform.pitch += rotation_pitch * elapsed * 30.0; 
                }

                // Render the pixel in software to the render target
                let animation = &mut self.animation;
                animation.render_target.clear();
                for model in animation.models.iter() {
                    render::render(model, &mut animation.render_target);
                }

                // Write the pixels to the pixel buffer used by the window
                let frame = self.pixels.as_mut().unwrap().frame_mut();
                for (y, row) in animation.render_target.pixels.iter().enumerate() {
                    for (x, pixel) in row.iter().enumerate() {
                        let index = (y * WIDTH + x) * 4;
                        frame[index] = (pixel.r() * 255.0) as u8;
                        frame[index + 1] = (pixel.g() * 255.0) as u8;
                        frame[index + 2] = (pixel.b() * 255.0) as u8;
                        frame[index + 3] = 255; // Alpha channel
                    }
                }

                self.pixels.as_mut().unwrap().render().expect("Failed to render pixels");
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //crazy_triangles();
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <obj_file_path>", args[0]);
        return Ok(());
    }

    let event_loop = event_loop::EventLoop::new()?;
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    let mut app = App::default(); 
    
    
    let obj_file_path = &args[1];
    let obj = obj::Obj::read_from_file(obj_file_path).expect("Failed to read OBJ file");
    println!("OBJ file loaded successfully with {} vertices, {} texture coordinates, {} normals, and {} faces.", 
    obj.vertices.len(), obj.texture_coordinates.len(), obj.normals.len(), obj.faces.len());
    
    let mut model = Model::new();
    for face in obj.faces.iter() {
        let mut t = triangle::Triangle3D::create_triangles_from_face(&obj, face);
        for triangle in t.iter_mut() {
            triangle.set_color(Float3::random());
            model.add_triangle(*triangle);
        }
    }
    
    model.transform.position.z = 5.0; // Move the model back in the Z direction
    const FPS : i32 = 30;
    const VIDEO_DURATION : i32 = 30; // seconds
    const FRAME_COUNT : i32 = FPS * VIDEO_DURATION;
    
    let rotation_list = vec![
        ((0,4), (0.04, 0.0)),
        ((4,5), (0.0, 0.0)),
        ((5,9), (0.0, 0.04)),
        ((9, 10), (0.0, 0.0)),
        ((10, 13), (0.06, 0.0)),
        ((13, 17), (0.0, 0.06)),
        ((17, 18), (0.0, 0.0)),
        ((18, 20), (0.04, 0.0)),
        ((20, 23), (0.0, 0.04)),
        ((23, 25), (0.04, 0.0)),
        ((25, 500), (0.0, 0.1))];
        
        app.animation = Animation {
            models: vec![model.clone()],
            total_duration: VIDEO_DURATION,
            rotations: rotation_list.clone(),
            render_target: RenderTarget::new(WIDTH, HEIGHT),
            start_time: None,
        };
        
    event_loop.run_app(&mut app)?;
    return Ok(());

    let start = Instant::now();
    let out_dir = format!("out-{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
    let mut render_target = render::RenderTarget::new(WIDTH, HEIGHT);
    for frame in 0..FRAME_COUNT {
        let now = Instant::now();
        render_target.clear();
        let (rotation_yaw, rotation_pitch) = rotation_list.iter()
            .find(|&&((start, end), _)| {
                frame / FPS >= start && frame / FPS < end
            })
            .map(|&(_, rotation)| rotation)
            .unwrap_or((0.0, 0.0)); 

        model.transform.yaw += rotation_yaw; 
        model.transform.pitch += rotation_pitch; 
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

    Ok(())
}

