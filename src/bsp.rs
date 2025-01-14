use std::io::{self};

use byteorder::{LittleEndian, ReadBytesExt};

mod edges;
mod vertices;

pub struct Bsp {
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct BspHeader {
    pub version: u32, // Must be 29 (BSP29 for Quake)
    pub entities: BspEntry,
    pub planes: BspEntry,

    pub miptex: BspEntry,
    pub vertices: BspEntry,

    pub visilist: BspEntry,
    pub nodes: BspEntry,

    pub texinfo: BspEntry,

    pub faces: BspEntry,

    pub lightmaps: BspEntry,
    pub clipnodes: BspEntry,

    pub leaves: BspEntry,

    pub lfaces: BspEntry,
    pub edges: BspEntry,

    pub ledges: BspEntry,
    pub models: BspEntry,
}

#[derive(Debug)]
pub struct BspEntry {
    pub offset: u32,
    pub size: u32,
}

impl Bsp {
    pub fn new(data: Vec<u8>) -> Self {
        Bsp { data }
    }

    pub fn read_header(&self) -> BspHeader {
        let mut cursor = io::Cursor::new(&self.data);

        // Read the version
        let version = cursor
            .read_u32::<LittleEndian>()
            .expect("Failed to read BSP version");

        // Helper to read a BspEntry
        let read_entry = |cursor: &mut io::Cursor<&Vec<u8>>| -> BspEntry {
            let offset = cursor
                .read_u32::<LittleEndian>()
                .expect("Failed to read entry offset");
            let size = cursor
                .read_u32::<LittleEndian>()
                .expect("Failed to read entry size");
            BspEntry { offset, size }
        };

        BspHeader {
            version,
            entities: read_entry(&mut cursor),
            planes: read_entry(&mut cursor),
            miptex: read_entry(&mut cursor),
            vertices: read_entry(&mut cursor),
            visilist: read_entry(&mut cursor),
            nodes: read_entry(&mut cursor),
            texinfo: read_entry(&mut cursor),
            faces: read_entry(&mut cursor),
            lightmaps: read_entry(&mut cursor),
            clipnodes: read_entry(&mut cursor),
            leaves: read_entry(&mut cursor),
            lfaces: read_entry(&mut cursor),
            edges: read_entry(&mut cursor),
            ledges: read_entry(&mut cursor),
            models: read_entry(&mut cursor),
        }
    }
}
