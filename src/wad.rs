use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};

pub struct Wad {
    pub data: Vec<u8>,
}

pub struct WadHeader {
    pub name: String, // Must be "WAD2"
    pub numentries: u32,
    pub diroffset: u32,
}

#[derive(Debug)]
pub struct WadAsset {
    pub offset: u32,
    pub dsize: u32,        // Size of the entry in WAD file
    pub size: u32,         // Size of the entry in memory
    pub entry_type: char, // "@" = Raw bytes, "B" = Pictures (status bar), "D" = MIP Textures (3D brush models)
    pub compression: char, // 0 = no compression
    pub dummy: u16,       // unused
    pub name: String,     // Max 16 chars, null byte terminated
}

impl Wad {
    pub fn new(data: Vec<u8>) -> Wad {
        Wad { data }
    }

    pub fn read_header(&self) -> WadHeader {
        let mut cursor = io::Cursor::new(&self.data);

        // Read the first 4 bytes as a string
        let mut name_buf = [0u8; 4];
        cursor.read_exact(&mut name_buf).unwrap();
        let name = String::from_utf8_lossy(&name_buf).to_string();
        let numentries = cursor.read_u32::<LittleEndian>().unwrap();
        let diroffset = cursor.read_u32::<LittleEndian>().unwrap();

        WadHeader {
            name,
            numentries,
            diroffset,
        }
    }

    pub fn read_directory(&self) -> Vec<WadAsset> {
        let header = self.read_header();

        let file_number = header.numentries;

        let mut assets: Vec<WadAsset> = Vec::new();

        let mut cursor = io::Cursor::new(&self.data);
        cursor.set_position(header.diroffset.into());
        for _ in 0..file_number {
            let offset = cursor.read_u32::<LittleEndian>().unwrap();
            let dsize = cursor.read_u32::<LittleEndian>().unwrap();
            let size = cursor.read_u32::<LittleEndian>().unwrap();
            let entry_type = char::from(cursor.read_u8().unwrap());
            let compression = char::from(cursor.read_u8().unwrap());

            let dummy = cursor.read_u16::<LittleEndian>().unwrap(); // not used

            let mut file_buf = [0u8; 16]; // 16 bytes
            cursor.read_exact(&mut file_buf).unwrap();
            let name = String::from_utf8_lossy(&file_buf)
                .trim_matches(char::from(0))
                .to_string();

            assets.push(WadAsset {
                offset,
                dsize,
                size,
                entry_type,
                compression,
                dummy,
                name,
            });
        }
        assets
    }

    /// Returns file by path "maps/e1m1.bsp"
    pub fn find_file(&self, path: &str) -> Option<Vec<u8>> {
        let directory = self.read_directory();
        for file in directory {
            if file.name == path {
                let start = file.offset as usize;
                let end = start + file.dsize as usize;
                return Some(self.data[start..end].to_vec());
            }
        }
        None
    }
}
