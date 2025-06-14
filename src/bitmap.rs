use std::io::Write;

use crate::float3::{Float3};


pub fn write_image_to_file(image : &Vec<Vec<Float3>>, filename: &str) -> Result<(), std::io::Error> {
    // Ensure the directory exists
    if let Some(dir) = std::path::Path::new(filename).parent() {
        std::fs::create_dir_all(dir)?;
    }

    let width : usize = image[0].len();
    let height : usize = image.len();

    // Open the file in write mode
    let mut file = std::fs::File::create(filename)?;
    
    // Write BMP header
    file.write("BM".as_bytes())?;
    let file_size : u32 = 54 + width as u32 * height as u32 * 4; // 54 bytes for header + pixel data
    file.write(&file_size.to_le_bytes())?;
    
    // Reserved bytes (set to 0)
    file.write(&[0, 0, 0, 0])?;
    let pixel_data_offset : i32 = 54;
    file.write(&pixel_data_offset.to_le_bytes())?;
    let dib_header_size: i32 = 40;
    file.write(&dib_header_size.to_le_bytes())?;
    file.write(&(width as i32).to_le_bytes())?;
    file.write(&(height as i32).to_le_bytes())?;
    
    // Write color planes (always 1 for BMP)
    file.write(&[1, 0])?;
    
    // Write bits per pixel (24 for RGB)
    file.write(&[32, 0])?;
    
    // Write compression (0 for no compression)
    file.write(&[0, 0, 0, 0])?;
    let pixel_data_size : u32 = width as u32 * height as u32 * 4; // 4 bytes per pixel for RGB plus padding
    file.write(&pixel_data_size.to_le_bytes())?;
    file.write(&[0, 0, 0, 0])?; // Horizontal resolution (pixels per meter)
    file.write(&[0, 0, 0, 0])?; // Vertical resolution (pixels per meter)
    file.write(&[0, 0, 0, 0])?; // Number of colors in the palette (0 for 24-bit BMP)
    file.write(&[0, 0, 0, 0])?; // Important colors (0 means all colors are important)

    // Write pixel data
    for y in 0..height { 
        for x in 0..width {
            let pixel = image[y][x];
            let r = (pixel.r() * 255.0) as u8;
            let g = (pixel.g() * 255.0) as u8;
            let b = (pixel.b() * 255.0) as u8;
            file.write(&[b, g, r, 0])?; // Write pixel in BGR format with padding
        }
    }

    // Flush the file to ensure all data is written
    file.flush()?;
    println!("Image written to {}", filename);
    Ok(())
}