
use std::io::BufRead;

use crate::float3::Float3;

pub struct Vertex {
    pub position: Float3,
    pub scale: f32,
}

impl Vertex {
    pub fn new(position: Float3, scale: Option<f32>) -> Self {
        Vertex { position, scale: scale.unwrap_or(1.0) }
    }
    
}

pub struct FaceElement {
    pub vertex_indices: Vec<usize>,
    pub texture_indices: Option<Vec<usize>>,
    pub normal_indices: Option<Vec<usize>>,
}

pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub texture_coordinates: Vec<Float3>,
    pub normals: Vec<Float3>,
    pub faces: Vec<FaceElement>,
}

impl Obj {
    pub fn new() -> Self {
        Obj {
            vertices: Vec::new(),
            texture_coordinates: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, position: Float3, scale: Option<f32>) -> usize {
        let vertex = Vertex::new(position, scale);
        self.vertices.push(vertex);
        self.vertices.len() - 1 // Return the index of the new vertex
    }

    pub fn add_texture_coordinate(&mut self, texture_coordinate: Float3) -> usize {
        self.texture_coordinates.push(texture_coordinate);
        self.texture_coordinates.len() - 1 // Return the index of the new texture coordinate
    }

    pub fn add_normal(&mut self, normal: Float3) -> usize {
        self.normals.push(normal);
        self.normals.len() - 1 // Return the index of the new normal
    }

    pub fn add_face(&mut self, vertex_indices: Vec<usize>, texture_indices: Option<Vec<usize>>, normal_indices: Option<Vec<usize>>) -> usize {
        let face = FaceElement {
            vertex_indices,
            texture_indices,
            normal_indices,
        };

        self.faces.push(face);
        self.faces.len() - 1 // Return the index of the new face
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.texture_coordinates.clear();
        self.normals.clear();
        self.faces.clear();
    }

    pub fn read_from_file(file_path: &str) -> Result<Self, String> {
        let mut result = Self::new();

        let file = std::fs::File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut reader = std::io::BufReader::new(file);

        let mut line = String::new();
        while (reader.read_line(&mut line).map_err(|e| format!("Failed to read from file: {}", e))?) > 0 {
            line = line.trim().to_string();
 
            if line.is_empty() || line.starts_with('#') {
                line.clear(); // Clear the line for the next iteration
                continue; // Skip empty lines and comments
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "o" => {
                    // Object name, can be ignored for now
                },
                "v" => {
                    let position = Float3::new(
                        parts[1].parse().map_err(|e| format!("Invalid vertex position: {}", e))?,
                        parts[2].parse().map_err(|e| format!("Invalid vertex position: {}", e))?,
                        parts[3].parse().map_err(|e| format!("Invalid vertex position: {}", e))?,
                    );

                    if parts.len() == 5 {
                        let scale = parts[4].parse().map_err(|e| format!("Invalid vertex scale: {}", e))?;
                        result.add_vertex(position, Some(scale));
                    } else {
                        result.add_vertex(position, None);
                    }
                },
                "vn" => {
                    let normal = Float3::new(
                        parts[1].parse().map_err(|e| format!("Invalid normal vector: {}", e))?,
                        parts[2].parse().map_err(|e| format!("Invalid normal vector: {}", e))?,
                        parts[3].parse().map_err(|e| format!("Invalid normal vector: {}", e))?,
                    );
                    result.add_normal(normal);
                },
                "vt" => {
                    let texture_coordinate = Float3::new(
                        parts[1].parse().map_err(|e| format!("Invalid texture coordinate: {}", e))?,
                        parts[2].parse().map_err(|e| format!("Invalid texture coordinate: {}", e))?,
                        if parts.len() > 3 {
                            parts[3].parse().map_err(|e| format!("Invalid texture coordinate: {}", e))?
                        } else {
                            0.0 // Default z-coordinate if not provided
                        },
                    );
                    result.add_texture_coordinate(texture_coordinate);
                },
                "s" => {
                    // Smoothing group, can be ignored for now
                },
                "mtllib" => {
                    // Material library, can be ignored for now
                },
                "usemtl" => {
                    // Material usage, can be ignored for now
                },
                "f" => {
                    let mut vertex_indices = Vec::new();
                    let mut texture_indices = None;
                    let mut normal_indices = None;

                    for part in &parts[1..] {
                        let indices: Vec<&str> = part.split('/').collect();
                        if indices.len() > 0 {
                            let vertex_index = indices[0].parse::<usize>().map_err(|e| format!("Invalid vertex index: {}", e))? - 1; // OBJ indices are 1-based
                            vertex_indices.push(vertex_index);
                            if vertex_index >= result.vertices.len() {
                                return Err(format!("Vertex index {} out of bounds", vertex_index + 1));
                            }

                            if indices.len() > 1 && !indices[1].is_empty() {
                                if texture_indices.is_none() {
                                    texture_indices = Some(Vec::new());
                                }
                                let texture_index = indices[1].parse::<usize>().map_err(|e| format!("Invalid texture index: {}", e))? - 1;
                                if texture_index >= result.texture_coordinates.len() {
                                    return Err(format!("Texture index {} out of bounds", texture_index + 1));
                                }

                                texture_indices.as_mut().unwrap().push(texture_index);
                            }

                            if indices.len() > 2 && !indices[2].is_empty() {
                                if normal_indices.is_none() {
                                    normal_indices = Some(Vec::new());
                                }
                                let normal_index = indices[2].parse::<usize>().map_err(|e| format!("Invalid normal index: {}", e))? - 1;
                                if normal_index >= result.normals.len() {
                                    return Err(format!("Normal index {} out of bounds", normal_index + 1));
                                }

                                normal_indices.as_mut().unwrap().push(normal_index);
                            }
                        }
                    }

                    result.add_face(vertex_indices, texture_indices, normal_indices);
                },
                _ => { todo!("Handle other OBJ commands like vt, vn, f, etc.") },
            }

            line.clear(); // Clear the line for the next iteration
        }

        Ok(result)
    }

}