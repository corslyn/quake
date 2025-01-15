use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use super::{Bsp, BspHeader};

#[derive(Debug)]
pub struct Face {
    pub plane_id: u16,
    pub side: u16,
    pub ledge_id: u32,
    pub ledge_num: u16,
    pub texinfo_id: u16,
    pub typelight: u8,
    pub baselight: u8,
    pub light: [u8; 2],
    pub lightmap: u32,
}

impl Bsp {
    pub fn read_faces(&self, header: &BspHeader) -> Vec<Face> {
        let start = header.faces.offset as usize;
        let end = (header.faces.offset + header.faces.size) as usize;

        let mut faces = Vec::new();
        let mut cursor = Cursor::new(&self.data[start..end]);

        while (cursor.position() as usize) < (end - start) {
            let plane_id = cursor.read_u16::<LittleEndian>().unwrap();
            let side = cursor.read_u16::<LittleEndian>().unwrap();
            let ledge_id = cursor.read_u32::<LittleEndian>().unwrap();
            let ledge_num = cursor.read_u16::<LittleEndian>().unwrap();
            let texinfo_id = cursor.read_u16::<LittleEndian>().unwrap();
            let typelight = cursor.read_u8().unwrap();
            let baselight = cursor.read_u8().unwrap();

            let mut light = [0; 2];
            for i in 0..2 {
                light[i] = cursor.read_u8().unwrap();
            }

            let lightmap = cursor.read_u32::<LittleEndian>().unwrap();

            faces.push(Face {
                plane_id,
                side,
                ledge_id,
                ledge_num,
                texinfo_id,
                typelight,
                baselight,
                light,
                lightmap,
            });
        }

        faces
    }
}
