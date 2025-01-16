use std::io::{Cursor, Read};

use glam::Vec3;

use super::{Bsp, BspHeader};

#[derive(Debug)]
pub struct Vertex {
    pub coordinates: Vec3,
}

impl Bsp {
    pub fn read_vertices(&self, header: &BspHeader) -> Vec<Vertex> {
        let start = header.vertices.offset as usize;
        let end = start + header.vertices.size as usize;

        let mut vertices = Vec::new();
        let mut cursor = Cursor::new(&self.data[start..end]);

        while (cursor.position() as usize) < header.vertices.size as usize {
            let mut buffer = [0u8; 12]; // 3 floats, each 4 bytes
            if cursor.read_exact(&mut buffer).is_err() {
                break;
            }

            let x = f32::from_le_bytes(buffer[0..4].try_into().unwrap());
            let y = f32::from_le_bytes(buffer[4..8].try_into().unwrap());
            let z = f32::from_le_bytes(buffer[8..12].try_into().unwrap());

            vertices.push(Vertex {
                coordinates: Vec3::new(x, y, z),
            });
        }

        vertices
    }
}
