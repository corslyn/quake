use std::io::{Cursor, Read};

use glam::Vec3;

use super::{Bsp, BspHeader};

#[derive(Debug)]
pub struct Plane {
    pub normal: Vec3,
    pub dist: f32,
    pub plane_type: u32,
}

impl Bsp {
    pub fn read_planes(&self, header: &BspHeader) -> Vec<Plane> {
        let start = header.planes.offset as usize;
        let end = start + header.planes.size as usize;

        let mut planes = Vec::new();
        let mut cursor = Cursor::new(&self.data[start..end]);

        while (cursor.position() as usize) < header.planes.size as usize {
            let mut buffer = [0u8; 16]; // 3 floats for normal, 1 float for dist, 1 u32 for plane_type
            if cursor.read_exact(&mut buffer).is_err() {
                break;
            }

            // Extract normal vector
            let x = f32::from_le_bytes(buffer[0..4].try_into().unwrap());
            let y = f32::from_le_bytes(buffer[4..8].try_into().unwrap());
            let z = f32::from_le_bytes(buffer[8..12].try_into().unwrap());
            let normal = Vec3::new(x, y, z);

            // Extract distance
            let dist = f32::from_le_bytes(buffer[12..16].try_into().unwrap());

            // Extract plane type
            let mut plane_type_buf = [0u8; 4];
            cursor.read_exact(&mut plane_type_buf).unwrap();
            let plane_type = u32::from_le_bytes(plane_type_buf);

            planes.push(Plane {
                normal,
                dist,
                plane_type,
            });
        }

        planes
    }
}
