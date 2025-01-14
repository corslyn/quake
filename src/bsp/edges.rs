use std::io::{Cursor, Read};

use super::{Bsp, BspHeader};

#[derive(Debug)]
pub struct Edge {
    pub start_vertex: u16,
    pub end_vertex: u16,
}

impl Bsp {
    pub fn read_edges(&self, header: &BspHeader) -> Vec<Edge> {
        let start = header.edges.offset as usize;
        let end = start + header.edges.size as usize;

        let mut edges = Vec::new();
        let mut cursor = Cursor::new(&self.data[start..end]);

        while (cursor.position() as usize) < header.edges.size as usize {
            let mut buffer = [0u8; 4]; // 2 u16 = 4 bytes
            if cursor.read_exact(&mut buffer).is_err() {
                break;
            }

            let start_vertex = u16::from_le_bytes(buffer[0..2].try_into().unwrap());
            let end_vertex = u16::from_le_bytes(buffer[2..4].try_into().unwrap());

            edges.push(Edge {
                start_vertex,
                end_vertex,
            });
        }

        edges
    }
}
