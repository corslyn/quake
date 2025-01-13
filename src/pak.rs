use std::{fs::File, io, io::Read};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Pak {
    /// Raw pak data
    pub data: Vec<u8>,
}

pub struct PakHeader {
    pub id: String,      // Should be "PACK"
    pub dir_offset: u32, // Offset to the directory
    pub dir_size: u32,   // Size of the directory
}

#[derive(Debug)]
pub struct PakFile {
    pub name: String, // 56 bytes null terminated ex : "maps/e1m1.bsp"
    pub file_offset: u32,
    pub file_size: u32,
}

impl Pak {
    /// Creates a PAK from a file
    pub fn new(filepath: &str) -> io::Result<Self> {
        let mut pak_file = File::open(filepath)?;
        let mut data: Vec<u8> = Vec::new();
        pak_file.read_to_end(&mut data)?;
        Ok(Pak { data })
    }

    /// Returns a PAK Header
    pub fn read_header(&self) -> io::Result<PakHeader> {
        let mut cursor = io::Cursor::new(&self.data);

        // Read the first 4 bytes as a string
        let mut id_buf = [0u8; 4];
        cursor.read_exact(&mut id_buf)?;
        let id = String::from_utf8_lossy(&id_buf).to_string();

        if id != "PACK" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported PAK file",
            ));
        }

        // Read directory offset and size
        let dir_offset = cursor.read_u32::<LittleEndian>()?;
        let dir_size = cursor.read_u32::<LittleEndian>()?;

        Ok(PakHeader {
            id,
            dir_offset,
            dir_size,
        })
    }

    pub fn read_directory(&self) -> io::Result<Vec<PakFile>> {
        let header = self.read_header()?;

        let file_number = header.dir_size / 64;

        let mut pakfiles: Vec<PakFile> = Vec::new();

        let mut cursor = io::Cursor::new(&self.data);
        cursor.set_position(header.dir_offset.into());
        for _ in 0..file_number {
            let mut file_buf = [0u8; 56]; // 56 bytes
            cursor.read_exact(&mut file_buf)?;
            let name = String::from_utf8_lossy(&file_buf)
                .trim_matches(char::from(0))
                .to_string();
            let file_offset = cursor.read_u32::<LittleEndian>()?;
            let file_size = cursor.read_u32::<LittleEndian>()?;
            pakfiles.push(PakFile {
                name,
                file_offset,
                file_size,
            });
        }

        Ok(pakfiles)
    }

    /// Returns file by path "maps/e1m1.bsp"
    pub fn find_file(&self, path: &str) -> Option<Vec<u8>> {
        let directory = self.read_directory().unwrap();
        for file in directory {
            if file.name == path {
                let start = file.file_offset as usize;
                let end = start + file.file_size as usize;
                return Some(self.data[start..end].to_vec());
            }
        }
        None
    }
}
